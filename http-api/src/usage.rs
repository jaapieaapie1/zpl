//! Usage metering: requests are added up in memory per key, format and UTC day, and every
//! window (`ZPL_USAGE_WINDOW_MS`, default [`DEFAULT_WINDOW`]) the totals go to RabbitMQ as
//! one [`UsageAggregate`] per key and format, instead of one message per request.
//!
//! - Recording a request only updates a map: a response never waits for RabbitMQ, and
//!   usage is never dropped because a queue is full. The map holds one entry per key and
//!   format in use (plus each `Idempotency-Key` request until it is published).
//! - Messages go out with publisher confirms; what the broker did not confirm is sent
//!   again (same `id`, which consumers store once) after reconnecting. While a cut window
//!   is unconfirmed, e.g. RabbitMQ is away or blocked, no new window is cut: requests keep
//!   adding up in the map, and the next window covers the whole time.
//! - At shutdown [`Publisher::finish`] cuts what is left and publishes it, retrying until
//!   its deadline; anything still unsent then is logged with its label count.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use lapin::options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::console::{IdempotentRequest, KeyEnv, UsageAggregate, AGGREGATE_KIND, USAGE_EXCHANGE};

/// How often usage is published, unless `ZPL_USAGE_WINDOW_MS` says otherwise.
pub const DEFAULT_WINDOW: Duration = Duration::from_secs(1);
/// Most messages published before awaiting their confirms.
pub const PUBLISH_BATCH: usize = 1_000;
/// Most `Idempotency-Key` requests listed in one message; more go in further messages.
pub const MAX_IDEMPOTENT_PER_MESSAGE: usize = 1_000;
/// Most labels or requests in one message, so each fits a signed 32-bit column.
pub const MAX_COUNT_PER_MESSAGE: u64 = i32::MAX as u64;
/// A batch whose confirms take longer than this (e.g. RabbitMQ blocked publishing) is sent
/// again on a new connection.
const CONFIRM_TIMEOUT: Duration = Duration::from_secs(30);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const RETRY_EVERY: Duration = Duration::from_secs(2);
const MS_PER_DAY: i64 = 86_400_000;

/// What a group of requests is counted by. The UTC day keeps every message within one
/// day, so daily and monthly totals stay exact however long a window gets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GroupKey {
    org_id: Uuid,
    key_id: Uuid,
    env: KeyEnv,
    format: &'static str,
    day: i64,
}

#[derive(Debug)]
struct Group {
    labels: u64,
    requests: u64,
    first_ms: i64,
    last_ms: i64,
    /// By request id: a retry within the window is listed once.
    idempotent: HashMap<Uuid, (u32, String, i64)>,
}

/// One rendered request, as [`Usage::add`] takes it.
#[derive(Clone, Debug)]
pub struct Request {
    pub org_id: Uuid,
    pub key_id: Uuid,
    pub env: KeyEnv,
    pub format: &'static str,
    pub labels: u32,
    /// The request id and `Idempotency-Key` of a request that has one.
    pub idempotent: Option<(Uuid, String)>,
    /// Unix milliseconds.
    pub at_ms: i64,
}

/// Usage not yet handed to the publisher.
#[derive(Default)]
pub struct Usage {
    groups: Mutex<HashMap<GroupKey, Group>>,
}

impl Usage {
    pub fn new() -> Arc<Usage> {
        Arc::new(Usage::default())
    }

    pub fn add(&self, request: Request) {
        let key = GroupKey {
            org_id: request.org_id,
            key_id: request.key_id,
            env: request.env,
            format: request.format,
            day: request.at_ms.div_euclid(MS_PER_DAY),
        };
        let mut groups = self.groups.lock().expect("usage lock");
        let group = groups.entry(key).or_insert_with(|| Group {
            labels: 0,
            requests: 0,
            first_ms: request.at_ms,
            last_ms: request.at_ms,
            idempotent: HashMap::new(),
        });
        group.first_ms = group.first_ms.min(request.at_ms);
        group.last_ms = group.last_ms.max(request.at_ms);
        match request.idempotent {
            Some((request_id, idempotency_key)) => {
                group.idempotent.entry(request_id).or_insert((request.labels, idempotency_key, request.at_ms));
            }
            None => {
                group.labels += u64::from(request.labels);
                group.requests += 1;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.groups.lock().expect("usage lock").is_empty()
    }

    /// Takes everything added so far, as messages with fresh ids.
    pub fn cut(&self) -> Vec<UsageAggregate> {
        let groups = std::mem::take(&mut *self.groups.lock().expect("usage lock"));
        let mut messages = Vec::with_capacity(groups.len());
        for (key, group) in groups {
            let (at, until) = (rfc3339(group.first_ms), rfc3339(group.last_ms));
            let mut idempotent: Vec<IdempotentRequest> = group
                .idempotent
                .into_iter()
                .map(|(request_id, (labels, idempotency_key, at_ms))| IdempotentRequest {
                    request_id,
                    labels,
                    idempotency_key,
                    at: rfc3339(at_ms),
                })
                .collect();
            idempotent.sort_by(|a, b| a.at.cmp(&b.at));
            let (mut labels, mut requests) = (group.labels, group.requests);
            let mut idempotent = idempotent.into_iter().peekable();
            // Normally one message; more only past the per-message limits.
            loop {
                let chunk: Vec<_> = idempotent.by_ref().take(MAX_IDEMPOTENT_PER_MESSAGE).collect();
                let (l, r) = (labels.min(MAX_COUNT_PER_MESSAGE), requests.min(MAX_COUNT_PER_MESSAGE));
                labels -= l;
                requests -= r;
                messages.push(UsageAggregate {
                    kind: AGGREGATE_KIND,
                    id: Uuid::now_v7(),
                    key_id: key.key_id,
                    org_id: key.org_id,
                    env: key.env,
                    format: key.format,
                    labels: l as u32,
                    requests: r as u32,
                    at: at.clone(),
                    until: until.clone(),
                    idempotent: chunk,
                });
                if labels == 0 && requests == 0 && idempotent.peek().is_none() {
                    break;
                }
            }
        }
        messages
    }
}

/// Unix milliseconds now.
pub fn now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

/// RFC 3339 UTC with milliseconds, without a date crate.
pub fn rfc3339(ms: i64) -> String {
    let (secs, millis) = (ms.div_euclid(1000), ms.rem_euclid(1000));
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    // Civil date from days since 1970-01-01 (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z", rem / 3600, rem % 3600 / 60, rem % 60)
}

/// A serialized message waiting for its confirm.
struct Pending {
    id: String,
    body: Vec<u8>,
    labels: u64,
}

impl Pending {
    fn new(message: &UsageAggregate) -> Pending {
        let labels = u64::from(message.labels) + message.idempotent.iter().map(|i| u64::from(i.labels)).sum::<u64>();
        let body = serde_json::to_vec(message).expect("usage serializes");
        Pending { id: message.id.to_string(), body, labels }
    }
}

/// The background task publishing [`Usage`].
pub struct Publisher {
    task: tokio::task::JoinHandle<()>,
    stop: oneshot::Sender<Instant>,
}

impl Publisher {
    pub fn start(url: String, usage: Arc<Usage>, window: Duration) -> Publisher {
        let (stop, stopped) = oneshot::channel();
        Publisher { task: tokio::spawn(run(url, usage, window, stopped)), stop }
    }

    /// Publishes what is left, trying for at most `within`. Call once no more requests are
    /// being recorded.
    pub async fn finish(self, within: Duration) {
        let _ = self.stop.send(Instant::now() + within);
        // The task gives up by itself at the deadline; this is only a backstop.
        let _ = tokio::time::timeout(within + Duration::from_secs(2), self.task).await;
    }
}

async fn run(url: String, usage: Arc<Usage>, window: Duration, mut stopped: oneshot::Receiver<Instant>) {
    let mut tick = tokio::time::interval(window);
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    let mut pending: Vec<Pending> = Vec::new();
    let mut link: Option<(Connection, Channel)> = None;
    let mut retry_at = Instant::now();
    let mut deadline: Option<Instant> = None;
    // Log a failure once, not every retry (DNS is often not ready when a pod starts).
    let mut failing = false;
    loop {
        match deadline {
            None => tokio::select! {
                _ = tick.tick() => {}
                at = &mut stopped => deadline = Some(at.unwrap_or_else(|_| Instant::now())),
            },
            Some(deadline) if Instant::now() >= deadline => {
                pending.extend(usage.cut().iter().map(Pending::new));
                if !pending.is_empty() {
                    let labels: u64 = pending.iter().map(|p| p.labels).sum();
                    eprintln!(
                        "shutting down with {} usage message(s) of {labels} label(s) not published to RabbitMQ (lost)",
                        pending.len()
                    );
                }
                return;
            }
            Some(deadline) => {
                // Shutting down: retry soon, not on the next tick.
                let wait = retry_at.saturating_duration_since(Instant::now());
                if link.is_none() && !wait.is_zero() {
                    tokio::time::sleep(wait.min(deadline.saturating_duration_since(Instant::now()))).await;
                    continue;
                }
            }
        }
        // Cut a new window only once the last one is confirmed (or when stopping): until
        // then requests keep adding up in memory, which stays small.
        if pending.is_empty() || deadline.is_some() {
            pending.extend(usage.cut().iter().map(Pending::new));
        }
        if pending.is_empty() {
            if deadline.is_some() {
                return;
            }
            continue;
        }
        if link.is_none() {
            if Instant::now() < retry_at {
                continue;
            }
            match tokio::time::timeout(CONNECT_TIMEOUT, connect(&url)).await {
                Ok(Ok(connected)) => {
                    if std::mem::take(&mut failing) {
                        eprintln!("connected to RabbitMQ again, publishing usage");
                    }
                    link = Some(connected);
                }
                result => {
                    let error = match result {
                        Ok(Err(e)) => e.to_string(),
                        _ => "timed out connecting".into(),
                    };
                    note_failure(&mut failing, &error, &pending);
                    retry_at = Instant::now() + RETRY_EVERY;
                    continue;
                }
            }
        }
        let (_, channel) = link.as_ref().expect("connected");
        // When shutting down, give up at the deadline even while confirms are outstanding.
        let published = match deadline {
            None => publish_pending(channel, &mut pending).await.map_err(|e| e.to_string()),
            Some(deadline) => {
                let left = deadline.saturating_duration_since(Instant::now());
                match tokio::time::timeout(left, publish_pending(channel, &mut pending)).await {
                    Ok(result) => result.map_err(|e| e.to_string()),
                    Err(_) => Err("no confirms before the shutdown deadline".into()),
                }
            }
        };
        if let Err(e) = published {
            note_failure(&mut failing, &e, &pending);
            if let Some((connection, _)) = link.take() {
                let _ = tokio::time::timeout(Duration::from_secs(1), connection.close(200, "reconnecting")).await;
            }
            retry_at = Instant::now() + RETRY_EVERY;
        }
    }
}

fn note_failure(failing: &mut bool, error: &str, pending: &[Pending]) {
    if !std::mem::replace(failing, true) {
        let labels: u64 = pending.iter().map(|p| p.labels).sum();
        eprintln!(
            "publishing usage failed, retrying every 2 s; {} message(s) of {labels} label(s) wait, and new usage \
             adds up in memory meanwhile: {error}",
            pending.len()
        );
    }
}

async fn connect(url: &str) -> Result<(Connection, Channel), lapin::Error> {
    let connection = Connection::connect(url, ConnectionProperties::default()).await?;
    let channel = connection.create_channel().await?;
    channel.confirm_select(ConfirmSelectOptions::default()).await?;
    let durable = ExchangeDeclareOptions { durable: true, ..Default::default() };
    channel.exchange_declare(USAGE_EXCHANGE, ExchangeKind::Fanout, durable, FieldTable::default()).await?;
    Ok((connection, channel))
}

/// Publishes `pending` in batches, awaiting each batch's confirms together. Confirmed
/// messages are removed; on an error the rest stays for the next connection.
async fn publish_pending(channel: &Channel, pending: &mut Vec<Pending>) -> Result<(), lapin::Error> {
    let properties = BasicProperties::default()
        .with_delivery_mode(2)
        .with_content_type("application/json".into())
        .with_type(AGGREGATE_KIND.into());
    while !pending.is_empty() {
        let n = pending.len().min(PUBLISH_BATCH);
        let mut confirms = Vec::with_capacity(n);
        for message in &pending[..n] {
            let properties = properties.clone().with_message_id(message.id.as_str().into());
            let confirm = channel
                .basic_publish(USAGE_EXCHANGE, "", BasicPublishOptions::default(), &message.body, properties)
                .await?;
            confirms.push(confirm);
        }
        let acked = tokio::time::timeout(CONFIRM_TIMEOUT, async {
            let mut acked = Vec::with_capacity(confirms.len());
            for confirm in confirms {
                // A failed confirm (channel or connection lost) leaves that message unconfirmed.
                acked.push(confirm.await.is_ok_and(|c| c.is_ack()));
            }
            acked
        })
        .await
        .map_err(|_| lapin::Error::InvalidChannelState(lapin::ChannelState::Error))?;
        let unconfirmed: Vec<Pending> = pending.drain(..n).zip(acked).filter(|(_, ok)| !ok).map(|(p, _)| p).collect();
        if !unconfirmed.is_empty() {
            pending.splice(0..0, unconfirmed);
            return Err(lapin::Error::InvalidChannelState(lapin::ChannelState::Error));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = MS_PER_DAY;

    fn request(key: u128, format: &'static str, labels: u32, at_ms: i64) -> Request {
        Request {
            org_id: Uuid::from_u128(0xa),
            key_id: Uuid::from_u128(key),
            env: KeyEnv::Live,
            format,
            labels,
            idempotent: None,
            at_ms,
        }
    }

    fn total(messages: &[UsageAggregate]) -> (u64, u64) {
        messages.iter().fold((0, 0), |(l, r), m| {
            let idem_labels: u64 = m.idempotent.iter().map(|i| u64::from(i.labels)).sum();
            (l + u64::from(m.labels) + idem_labels, r + u64::from(m.requests) + m.idempotent.len() as u64)
        })
    }

    #[test]
    fn adds_up_per_key_and_format() {
        let usage = Usage::new();
        let t = 20_000 * DAY + 5_000;
        usage.add(request(1, "pdf", 3, t));
        usage.add(request(1, "pdf", 2, t + 400));
        usage.add(request(1, "png", 1, t + 100));
        usage.add(request(2, "pdf", 7, t + 900));
        let mut messages = usage.cut();
        assert!(usage.is_empty() && usage.cut().is_empty(), "a cut takes everything");
        messages.sort_by_key(|m| (m.key_id, m.format));
        let counts: Vec<_> = messages.iter().map(|m| (m.key_id.as_u128(), m.format, m.labels, m.requests)).collect();
        assert_eq!(counts, [(1, "pdf", 5, 2), (1, "png", 1, 1), (2, "pdf", 7, 1)]);
        assert_eq!((messages[0].at.as_str(), messages[0].until.as_str()), (rfc3339(t).as_str(), rfc3339(t + 400).as_str()));
        assert!(messages.iter().all(|m| m.kind == "aggregate" && m.idempotent.is_empty()));
        let mut ids: Vec<_> = messages.iter().map(|m| m.id).collect();
        ids.dedup();
        assert_eq!(ids.len(), 3, "every message has its own id");
        let json = serde_json::to_value(&messages[0]).unwrap();
        assert_eq!(json["kind"], "aggregate");
        assert_eq!(json["env"], "live");
        assert!(json.get("idempotent").is_none());
    }

    #[test]
    fn a_window_never_spans_two_days() {
        let usage = Usage::new();
        usage.add(request(1, "pdf", 1, 3 * DAY - 1));
        usage.add(request(1, "pdf", 1, 3 * DAY));
        let mut messages = usage.cut();
        messages.sort_by(|a, b| a.at.cmp(&b.at));
        assert_eq!(messages.len(), 2);
        assert!(messages[0].until.ends_with("T23:59:59.999Z") && messages[1].at.ends_with("T00:00:00.000Z"));
    }

    #[test]
    fn idempotent_requests_are_listed_once() {
        let usage = Usage::new();
        let id = Uuid::from_u128(0x1d);
        let mut retried = request(1, "pdf", 4, 1_000);
        retried.idempotent = Some((id, "order-1".into()));
        usage.add(retried.clone());
        usage.add(Request { at_ms: 1_500, ..retried });
        usage.add(request(1, "pdf", 2, 1_200));
        let messages = usage.cut();
        assert_eq!(messages.len(), 1);
        let m = &messages[0];
        assert_eq!((m.labels, m.requests), (2, 1), "idempotent requests are not in the totals");
        assert_eq!(m.idempotent.len(), 1);
        assert_eq!((m.idempotent[0].request_id, m.idempotent[0].labels), (id, 4));
        assert_eq!(m.idempotent[0].at, rfc3339(1_000));
        assert_eq!(m.until, rfc3339(1_500));
    }

    #[test]
    fn large_groups_are_split_without_losing_anything() {
        let usage = Usage::new();
        for i in 0..(MAX_IDEMPOTENT_PER_MESSAGE as u128 * 2 + 3) {
            let mut r = request(1, "png", 1, 10);
            r.idempotent = Some((Uuid::from_u128(i), format!("k{i}")));
            usage.add(r);
        }
        usage.add(request(1, "png", u32::MAX, 10));
        usage.add(request(1, "png", u32::MAX, 11));
        let messages = usage.cut();
        assert!(messages.len() >= 3);
        assert!(messages.iter().all(|m| u64::from(m.labels) <= MAX_COUNT_PER_MESSAGE));
        assert!(messages.iter().all(|m| m.idempotent.len() <= MAX_IDEMPOTENT_PER_MESSAGE));
        let expected = 2 * u64::from(u32::MAX) + MAX_IDEMPOTENT_PER_MESSAGE as u64 * 2 + 3;
        assert_eq!(total(&messages), (expected, 2 + MAX_IDEMPOTENT_PER_MESSAGE as u64 * 2 + 3));
    }

    #[test]
    fn totals_equal_per_request_sums() {
        // Whatever the mix, the messages add up to exactly what one message per request
        // (and each idempotent request once) would have.
        let usage = Usage::new();
        let (mut labels, mut requests) = (0u64, 0u64);
        for i in 0..5_000u32 {
            let mut r = request(u128::from(i % 7), if i % 3 == 0 { "png" } else { "pdf" }, i % 11, i64::from(i) * 997);
            if i % 5 == 0 {
                r.idempotent = Some((Uuid::from_u128(u128::from(i % 50)), "k".into()));
            } else {
                labels += u64::from(i % 11);
                requests += 1;
            }
            usage.add(r);
        }
        // Each idempotent id (i % 50, for i a multiple of 5) counts once, with its first labels.
        let mut first = HashMap::new();
        for i in (0..5_000u32).step_by(5) {
            first.entry((i % 50, i % 7, i % 3 == 0, (i64::from(i) * 997).div_euclid(DAY))).or_insert(i % 11);
        }
        let messages = usage.cut();
        let idem: u64 = messages.iter().flat_map(|m| &m.idempotent).map(|i| u64::from(i.labels)).sum();
        let plain: u64 = messages.iter().map(|m| u64::from(m.labels)).sum();
        assert_eq!(plain, labels);
        assert_eq!(messages.iter().map(|m| u64::from(m.requests)).sum::<u64>(), requests);
        assert_eq!(idem, first.values().map(|&l| u64::from(l)).sum::<u64>());
    }

    #[test]
    fn rfc3339_times() {
        assert_eq!(rfc3339(0), "1970-01-01T00:00:00.000Z");
        assert_eq!(rfc3339(1_792_000_000_123), "2026-10-14T17:46:40.123Z");
        let now = rfc3339(now_ms());
        assert_eq!(now.len(), 24, "{now}");
    }

    #[tokio::test]
    async fn shutdown_without_rabbitmq_gives_up_at_the_deadline() {
        let usage = Usage::new();
        usage.add(request(1, "pdf", 3, now_ms()));
        // Nothing listens on port 9: every connection attempt fails.
        let publisher = Publisher::start("amqp://127.0.0.1:9/%2f".into(), usage.clone(), Duration::from_millis(20));
        let start = Instant::now();
        publisher.finish(Duration::from_millis(300)).await;
        assert!(start.elapsed() >= Duration::from_millis(300) && start.elapsed() < Duration::from_secs(2));
        assert!(usage.is_empty(), "taken for publishing (and logged as lost)");
    }

    async fn next(consumer: &mut lapin::Consumer) -> (Option<String>, Option<String>, serde_json::Value) {
        use futures_util::StreamExt;
        let delivery = tokio::time::timeout(Duration::from_secs(5), consumer.next()).await.unwrap().unwrap().unwrap();
        delivery.ack(lapin::options::BasicAckOptions::default()).await.unwrap();
        let kind = delivery.properties.kind().as_ref().map(|k| k.to_string());
        let id = delivery.properties.message_id().as_ref().map(|k| k.to_string());
        (kind, id, serde_json::from_slice(&delivery.data).unwrap())
    }

    /// Against a real RabbitMQ when `ZPL_TEST_AMQP_URL` is set (e.g. a throwaway vhost):
    /// windows are published as they end, and what is left at shutdown is published too.
    #[tokio::test]
    async fn publishes_windows_and_flushes_at_shutdown() {
        use lapin::options::{BasicConsumeOptions, QueueBindOptions, QueueDeclareOptions};
        let Ok(url) = std::env::var("ZPL_TEST_AMQP_URL") else { return };
        let (connection, channel) = connect(&url).await.unwrap();
        let exclusive = QueueDeclareOptions { exclusive: true, auto_delete: true, ..Default::default() };
        let queue = channel.queue_declare("", exclusive, FieldTable::default()).await.unwrap();
        let queue = queue.name().as_str().to_string();
        channel.queue_bind(&queue, USAGE_EXCHANGE, "", QueueBindOptions::default(), FieldTable::default()).await.unwrap();
        let mut consumer =
            channel.basic_consume(&queue, "test", BasicConsumeOptions::default(), FieldTable::default()).await.unwrap();

        let usage = Usage::new();
        let publisher = Publisher::start(url.clone(), usage.clone(), Duration::from_millis(100));
        for _ in 0..50 {
            usage.add(request(1, "pdf", 2, now_ms()));
        }
        let (kind, id, first) = next(&mut consumer).await;
        assert_eq!(kind.as_deref(), Some("aggregate"));
        assert_eq!(id.as_deref(), first["id"].as_str());
        assert_eq!(first["kind"], "aggregate");
        assert_eq!((first["labels"].as_u64(), first["requests"].as_u64()), (Some(100), Some(50)));
        publisher.finish(Duration::from_secs(5)).await;

        // With an hour-long window, only shutdown publishes this.
        let publisher = Publisher::start(url, usage.clone(), Duration::from_secs(3600));
        tokio::time::sleep(Duration::from_millis(100)).await; // past the interval's immediate first tick
        usage.add(request(3, "png", 1, now_ms()));
        let start = Instant::now();
        publisher.finish(Duration::from_secs(5)).await;
        assert!(start.elapsed() < Duration::from_secs(2));
        assert!(usage.is_empty());
        let (_, _, last) = next(&mut consumer).await;
        assert_eq!(last["key_id"], Uuid::from_u128(3).to_string());
        assert_eq!((last["format"].as_str(), last["labels"].as_u64()), (Some("png"), Some(1)));
        connection.close(200, "done").await.unwrap();
    }
}
