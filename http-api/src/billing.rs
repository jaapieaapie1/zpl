//! API keys and usage, with zpl-console (github.com/jaapieaapie1/zpl-console).
//!
//! Enabled by `ZPL_KEYS_PUBLIC_KEYS` (the console's `kid:key` public keys). Then:
//! - `POST /render` needs `Authorization: Bearer zpl_live_…` (or `X-API-Key`). Keys are
//!   verified offline; the console is never asked per request.
//! - Every 30 s the revocation list is fetched from `ZPL_CONSOLE_INTERNAL_URL` with
//!   `ZPL_CONSOLE_TOKEN`: revoked keys get 401, live keys of suspended organisations 402.
//!   When the console is unreachable the last list stays in force.
//! - Each rendered request is published as one usage message to the `zpl.usage` exchange
//!   on `ZPL_AMQP_URL`, from a background task: the response does not wait for RabbitMQ.
//!   The task publishes in batches of up to [`PUBLISH_BATCH`], awaiting their publisher
//!   confirms together, and resends what was not confirmed after reconnecting (consumers
//!   deduplicate on `request_id`). Up to `ZPL_USAGE_QUEUE` messages (default
//!   [`DEFAULT_QUEUE_LIMIT`]) wait in memory, e.g. while RabbitMQ restarts; when that is
//!   full a response waits up to [`QUEUE_FULL_WAIT`] for room before its usage is dropped
//!   (logged, at most every 10 s).
//! - Test keys render free, with a watermark on every label.
//! - Each key is rate limited (`ZPL_RATE_LIMIT_PER_SECOND`, `ZPL_RATE_LIMIT_BURST`), see
//!   [`crate::rate_limit`]: over it, 429 with `Retry-After`.
//!
//! `ZPL_CONSOLE_URL` is the console's public address, named in error messages.

use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use lapin::options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::{SendTimeoutError, TrySendError};
use uuid::Uuid;

use crate::console::{KeyEnv, RevocationList, TokenError, UsageEvent, Verifier, USAGE_EXCHANGE};
use crate::log_every::LogEvery;
use crate::rate_limit::{Decision, RateLimiter, DEFAULT_BURST, DEFAULT_PER_SECOND};

/// Usage messages kept in memory (a few hundred bytes each) while RabbitMQ is slow or
/// away; beyond this a response waits for room, see [`QUEUE_FULL_WAIT`].
pub const DEFAULT_QUEUE_LIMIT: usize = 100_000;
/// How long a response waits for room in a full usage queue before dropping its usage
/// (unbilled). Slowing responses down is better than losing usage, up to a point.
pub const QUEUE_FULL_WAIT: Duration = Duration::from_secs(2);
/// Most messages published before awaiting their confirms.
pub const PUBLISH_BATCH: usize = 1_000;
/// A batch whose confirms take longer than this is resent on a new connection.
const CONFIRM_TIMEOUT: Duration = Duration::from_secs(30);
/// Usage that waited for room, and usage dropped (requests, then labels).
static QUEUE_FULL: LogEvery<3> = LogEvery::new(Duration::from_secs(10));
const REVOCATION_INTERVAL: Duration = Duration::from_secs(30);
/// Namespace for request ids derived from `Idempotency-Key` and the request's content,
/// so a retried request maps to the same id and the console counts it once.
const IDEMPOTENCY_NAMESPACE: Uuid = Uuid::from_u128(0x7a3c_51f0_9d6e_4b8a_a2f1_3e5c_7d9b_0c14);

pub struct Billing {
    verifier: Verifier,
    revoked: RwLock<Revoked>,
    usage: Option<mpsc::Sender<UsageEvent>>,
    rate_limit: Option<RateLimiter>,
    console_url: String,
}

#[derive(Default)]
struct Revoked {
    keys: HashSet<Uuid>,
    orgs: HashSet<Uuid>,
}

/// The authenticated key behind a request.
#[derive(Clone, Debug)]
pub struct Caller {
    pub key_id: Uuid,
    pub org_id: Uuid,
    pub env: KeyEnv,
    pub idempotency_key: Option<String>,
}

impl Caller {
    pub fn is_test(&self) -> bool {
        self.env == KeyEnv::Test
    }
}

fn var(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

impl Billing {
    /// From the environment; `None` when `ZPL_KEYS_PUBLIC_KEYS` is unset (no API keys,
    /// as for local use). Starts the revocation and usage tasks, whose handle the server
    /// awaits at shutdown to flush usage.
    pub fn from_env() -> Result<Option<(Arc<Billing>, tokio::task::JoinHandle<()>)>, String> {
        let Some(keys) = var("ZPL_KEYS_PUBLIC_KEYS") else { return Ok(None) };
        let verifier = Verifier::from_config(&keys).map_err(|e| format!("ZPL_KEYS_PUBLIC_KEYS: {e}"))?;
        let console_url = var("ZPL_CONSOLE_URL").unwrap_or_else(|| "https://caretly.dev/console".into());
        let rate_limit = RateLimiter::from_env()?;
        if rate_limit.is_none() {
            eprintln!("ZPL_RATE_LIMIT_PER_SECOND or ZPL_RATE_LIMIT_BURST is 0: API keys are not rate limited");
        }

        let queue_limit = match var("ZPL_USAGE_QUEUE") {
            None => DEFAULT_QUEUE_LIMIT,
            Some(v) => v
                .trim()
                .parse::<usize>()
                .ok()
                .filter(|&n| n > 0)
                .ok_or_else(|| format!("ZPL_USAGE_QUEUE: expected a positive whole number, got {v:?}"))?,
        };
        let (usage, publisher) = match var("ZPL_AMQP_URL") {
            Some(url) => {
                let (tx, rx) = mpsc::channel(queue_limit);
                (Some(tx), tokio::spawn(publish(url, rx)))
            }
            None => {
                eprintln!("ZPL_AMQP_URL is not set: usage is not reported");
                (None, tokio::spawn(async {}))
            }
        };
        let billing = Arc::new(Billing { verifier, revoked: RwLock::default(), usage, rate_limit, console_url });
        match (var("ZPL_CONSOLE_INTERNAL_URL"), var("ZPL_CONSOLE_TOKEN")) {
            (Some(url), Some(token)) => {
                tokio::spawn(poll_revocations(Arc::downgrade(&billing), url, token));
            }
            _ => eprintln!("ZPL_CONSOLE_INTERNAL_URL / ZPL_CONSOLE_TOKEN not set: revoked keys keep working"),
        }
        Ok(Some((billing, publisher)))
    }

    /// For tests: keys only, no console or RabbitMQ, the default rate limit.
    pub fn with_verifier(verifier: Verifier, usage: Option<mpsc::Sender<UsageEvent>>) -> Arc<Billing> {
        Self::with_rate_limit(verifier, usage, RateLimiter::new(DEFAULT_PER_SECOND, DEFAULT_BURST))
    }

    /// For tests: like [`with_verifier`](Self::with_verifier) with the given rate limit.
    pub fn with_rate_limit(
        verifier: Verifier,
        usage: Option<mpsc::Sender<UsageEvent>>,
        rate_limit: Option<RateLimiter>,
    ) -> Arc<Billing> {
        let console_url = "https://console.test".into();
        Arc::new(Billing { verifier, revoked: RwLock::default(), usage, rate_limit, console_url })
    }

    /// Takes one request from the caller's rate limit. Gives the `RateLimit-Limit` and
    /// `RateLimit-Remaining` headers for the response (none without a limit), or the 429
    /// to send instead.
    pub fn rate_limit(&self, caller: &Caller) -> Result<HeaderMap, Response> {
        let mut headers = HeaderMap::new();
        let Some(limiter) = &self.rate_limit else { return Ok(headers) };
        let decision = limiter.check(caller.key_id);
        let remaining = match decision {
            Decision::Allowed { remaining } => remaining,
            Decision::Limited { .. } => 0,
        };
        headers.insert("ratelimit-limit", HeaderValue::from(limiter.burst()));
        headers.insert("ratelimit-remaining", HeaderValue::from(remaining));
        let Decision::Limited { retry_after } = decision else { return Ok(headers) };
        let (per_second, burst) = (limiter.per_second(), limiter.burst());
        let message = format!(
            "Too many requests for this API key: the limit is {per_second} per second, with bursts of {burst}. \
             Retry after {retry_after} s."
        );
        let mut response = (StatusCode::TOO_MANY_REQUESTS, message).into_response();
        headers.insert(header::RETRY_AFTER, HeaderValue::from(retry_after));
        response.headers_mut().extend(headers);
        Err(response)
    }

    /// Replaces the revocation list (from the console, or in tests).
    pub fn set_revocations(&self, list: &RevocationList) {
        let revoked = Revoked {
            keys: list.revoked.iter().map(|r| r.key_id).collect(),
            orgs: list.suspended_orgs.iter().copied().collect(),
        };
        *self.revoked.write().expect("revocation lock") = revoked;
    }

    /// The caller's key, or the error response to send instead.
    pub fn authenticate(&self, headers: &HeaderMap) -> Result<Caller, Response> {
        let console = &self.console_url;
        let Some(token) = api_key(headers) else {
            return Err(unauthorized(format!(
                "An API key is required. Create one at {console}/keys and send it as \
                 `Authorization: Bearer <key>`."
            )));
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs() as i64);
        let claims = match self.verifier.verify(token, now) {
            Ok(claims) => claims,
            Err(TokenError::Expired) => {
                return Err(unauthorized(format!("This API key has expired. Renew it at {console}/keys.")))
            }
            Err(_) => {
                return Err(unauthorized(format!(
                    "This API key is not valid. Check that it was copied completely, or create a new one at {console}/keys."
                )))
            }
        };
        let revoked = self.revoked.read().expect("revocation lock");
        if revoked.keys.contains(&claims.sub) {
            return Err(unauthorized(format!("This API key was revoked. Create a new one at {console}/keys.")));
        }
        if claims.env == KeyEnv::Live && revoked.orgs.contains(&claims.org) {
            return Err((
                StatusCode::PAYMENT_REQUIRED,
                format!(
                    "Billing for this account is not active, so live keys are paused. Update it at {console}/billing; \
                     test keys keep working."
                ),
            )
                .into_response());
        }
        let idempotency_key = headers
            .get("idempotency-key")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.chars().take(255).collect());
        Ok(Caller { key_id: claims.sub, org_id: claims.org, env: claims.env, idempotency_key })
    }

    /// Reports a rendered request; returns the `X-Labels-Billed` value (0 for test keys).
    /// `fingerprint` hashes the rendered content and settings; with an `Idempotency-Key`
    /// it goes into the request id, so reusing the key for other labels is billed again.
    /// When the usage queue is full this waits up to [`QUEUE_FULL_WAIT`] for room.
    pub async fn record(
        &self,
        caller: &Caller,
        labels: usize,
        format: &'static str,
        fingerprint: Option<&blake3::Hash>,
    ) -> usize {
        let labels = labels.min(u32::MAX as usize);
        if let Some(usage) = &self.usage {
            let request_id = match (&caller.idempotency_key, fingerprint) {
                (Some(key), Some(fingerprint)) => {
                    let name = format!("{}:{key}:{}", caller.key_id, fingerprint.to_hex());
                    Uuid::new_v5(&IDEMPOTENCY_NAMESPACE, name.as_bytes())
                }
                _ => Uuid::new_v4(),
            };
            let event = UsageEvent {
                request_id,
                key_id: caller.key_id,
                org_id: caller.org_id,
                env: caller.env,
                labels: labels as u32,
                format,
                idempotency_key: caller.idempotency_key.clone(),
                at: chrono_now(),
            };
            queue_usage(usage, event).await;
        }
        if caller.is_test() { 0 } else { labels }
    }
}

/// Queues `event` for publishing, waiting for room when the queue is full. Waits and drops
/// are counted and logged at most every 10 s, not per request.
async fn queue_usage(usage: &mpsc::Sender<UsageEvent>, event: UsageEvent) {
    let labels = u64::from(event.labels);
    let event = match usage.try_send(event) {
        Ok(()) => return,
        Err(TrySendError::Closed(_)) => return note_queue_full(&[(DROPPED, 1), (DROPPED_LABELS, labels)]),
        Err(TrySendError::Full(event)) => event,
    };
    note_queue_full(&[(WAITED, 1)]);
    if let Err(SendTimeoutError::Timeout(_) | SendTimeoutError::Closed(_)) = usage.send_timeout(event, QUEUE_FULL_WAIT).await {
        note_queue_full(&[(DROPPED, 1), (DROPPED_LABELS, labels)]);
    }
}

const WAITED: usize = 0;
const DROPPED: usize = 1;
const DROPPED_LABELS: usize = 2;

fn note_queue_full(adds: &[(usize, u64)]) {
    QUEUE_FULL.add(adds, |[waited, dropped, labels]| {
        eprintln!(
            "usage queue full: since the last report {waited} response(s) waited for room and \
             {dropped} usage message(s) of {labels} label(s) were dropped (not billed)"
        );
    });
}

/// `Authorization: Bearer <key>` (scheme case-insensitive) or `X-API-Key: <key>`.
fn api_key(headers: &HeaderMap) -> Option<&str> {
    if let Some(value) = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok()) {
        let (scheme, key) = value.trim().split_once(' ')?;
        return scheme.eq_ignore_ascii_case("bearer").then(|| key.trim()).filter(|k| !k.is_empty());
    }
    headers.get("x-api-key").and_then(|v| v.to_str().ok()).map(str::trim).filter(|k| !k.is_empty())
}

fn unauthorized(message: String) -> Response {
    let mut response = (StatusCode::UNAUTHORIZED, message).into_response();
    response
        .headers_mut()
        .insert(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer realm=\"zpl\""));
    response
}

/// Current UTC time as RFC 3339 with milliseconds, without a date crate.
fn chrono_now() -> String {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs() as i64;
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
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60,
        now.subsec_millis()
    )
}

async fn poll_revocations(billing: std::sync::Weak<Billing>, url: String, token: String) {
    let url = format!("{}/internal/revocations", url.trim_end_matches('/'));
    let http = match reqwest::Client::builder().timeout(Duration::from_secs(10)).build() {
        Ok(http) => http,
        Err(e) => return eprintln!("revocation client: {e}"),
    };
    let mut tick = tokio::time::interval(REVOCATION_INTERVAL);
    let mut failing = false;
    loop {
        tick.tick().await;
        let Some(billing) = billing.upgrade() else { return };
        let result = async {
            let response = http.get(&url).bearer_auth(&token).send().await?.error_for_status()?;
            response.json::<RevocationList>().await
        }
        .await;
        match result {
            Ok(list) => {
                billing.set_revocations(&list);
                if failing {
                    eprintln!("revocation list available again");
                    failing = false;
                }
            }
            Err(e) if !failing => {
                eprintln!("fetching revocations failed, keeping the last list: {e}");
                failing = true;
            }
            Err(_) => {}
        }
    }
}

/// Publishes usage with publisher confirms, reconnecting as needed. Ends once every sender
/// is gone and the queue is empty (server shutdown).
async fn publish(url: String, mut rx: mpsc::Receiver<UsageEvent>) {
    // Messages taken from the queue and not yet confirmed: kept across reconnects.
    let mut pending: Vec<Vec<u8>> = Vec::new();
    // Log a failure once, not every retry (DNS is often not ready when a pod starts).
    let mut failing = false;
    loop {
        match publish_until_error(&url, &mut rx, &mut pending, &mut failing).await {
            Ok(()) => return,
            Err(e) => {
                if !failing {
                    eprintln!("publishing usage failed, retrying every 2 s ({} unconfirmed message(s) kept): {e}", pending.len());
                    failing = true;
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}

async fn publish_until_error(
    url: &str,
    rx: &mut mpsc::Receiver<UsageEvent>,
    pending: &mut Vec<Vec<u8>>,
    failing: &mut bool,
) -> Result<(), lapin::Error> {
    let connection = Connection::connect(url, ConnectionProperties::default()).await?;
    if std::mem::take(failing) {
        eprintln!("connected to RabbitMQ again, publishing usage");
    }
    let channel = connection.create_channel().await?;
    channel.confirm_select(ConfirmSelectOptions::default()).await?;
    let durable = ExchangeDeclareOptions { durable: true, ..Default::default() };
    channel
        .exchange_declare(USAGE_EXCHANGE, ExchangeKind::Fanout, durable, FieldTable::default())
        .await?;
    let properties = BasicProperties::default()
        .with_delivery_mode(2)
        .with_content_type("application/json".into());
    loop {
        if !fill_batch(rx, pending).await {
            return Ok(());
        }
        // Publish the whole batch, then await its confirms: one round trip per batch
        // instead of per message.
        let mut confirms = Vec::with_capacity(pending.len());
        for body in pending.iter() {
            let confirm = channel
                .basic_publish(USAGE_EXCHANGE, "", BasicPublishOptions::default(), body, properties.clone())
                .await?;
            confirms.push(confirm);
        }
        let confirmed = tokio::time::timeout(CONFIRM_TIMEOUT, async {
            let mut confirmed = Vec::with_capacity(confirms.len());
            for confirm in confirms {
                // A failed confirm (channel or connection lost) leaves that message unconfirmed.
                confirmed.push(confirm.await.is_ok_and(|c| c.is_ack()));
            }
            confirmed
        })
        .await;
        let Ok(confirmed) = confirmed else {
            return Err(lapin::Error::InvalidChannelState(lapin::ChannelState::Error));
        };
        // Keep what the broker refused or never confirmed, to send again on a fresh
        // connection; consumers drop duplicates by `request_id`.
        let mut acked = confirmed.into_iter();
        pending.retain(|_| !acked.next().unwrap_or(false));
        if !pending.is_empty() {
            return Err(lapin::Error::InvalidChannelState(lapin::ChannelState::Error));
        }
    }
}

/// Tops `pending` up to [`PUBLISH_BATCH`] messages from the queue, waiting for one when it
/// is empty. `false` once the queue is closed and drained (shutdown) with nothing pending.
async fn fill_batch(rx: &mut mpsc::Receiver<UsageEvent>, pending: &mut Vec<Vec<u8>>) -> bool {
    if pending.is_empty() {
        match rx.recv().await {
            Some(event) => pending.push(serde_json::to_vec(&event).expect("usage serializes")),
            None => return false,
        }
    }
    while pending.len() < PUBLISH_BATCH {
        match rx.try_recv() {
            Ok(event) => pending.push(serde_json::to_vec(&event).expect("usage serializes")),
            Err(_) => break,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_headers() {
        let mut h = HeaderMap::new();
        assert_eq!(api_key(&h), None);
        h.insert(header::AUTHORIZATION, HeaderValue::from_static("bearer  zpl_test_x "));
        assert_eq!(api_key(&h), Some("zpl_test_x"));
        h.insert(header::AUTHORIZATION, HeaderValue::from_static("Basic abc"));
        assert_eq!(api_key(&h), None);
        let mut h = HeaderMap::new();
        h.insert("x-api-key", HeaderValue::from_static("zpl_live_y"));
        assert_eq!(api_key(&h), Some("zpl_live_y"));
    }

    fn event(labels: u32) -> UsageEvent {
        UsageEvent {
            request_id: Uuid::new_v4(),
            key_id: Uuid::nil(),
            org_id: Uuid::nil(),
            env: KeyEnv::Live,
            labels,
            format: "pdf",
            idempotency_key: None,
            at: chrono_now(),
        }
    }

    #[tokio::test]
    async fn full_queue_waits_for_room() {
        let (tx, mut rx) = mpsc::channel(1);
        queue_usage(&tx, event(1)).await;
        // The queue is full: the next one waits until the publisher takes the first.
        let consumer = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            let first = rx.recv().await.unwrap();
            (first, rx)
        });
        let start = std::time::Instant::now();
        queue_usage(&tx, event(2)).await;
        assert!(start.elapsed() >= Duration::from_millis(40));
        let (first, mut rx) = consumer.await.unwrap();
        assert_eq!((first.labels, rx.recv().await.unwrap().labels), (1, 2));
        // A closed queue drops at once.
        drop(rx);
        let start = std::time::Instant::now();
        queue_usage(&tx, event(3)).await;
        assert!(start.elapsed() < QUEUE_FULL_WAIT);
    }

    #[tokio::test]
    async fn batches_keep_what_is_pending() {
        let (tx, mut rx) = mpsc::channel(PUBLISH_BATCH * 2);
        for labels in 0..(PUBLISH_BATCH + 5) as u32 {
            tx.try_send(event(labels)).unwrap();
        }
        let mut pending = vec![b"unconfirmed".to_vec()];
        assert!(fill_batch(&mut rx, &mut pending).await);
        assert_eq!(pending.len(), PUBLISH_BATCH);
        assert_eq!(pending[0], b"unconfirmed", "unconfirmed messages go first");
        let second: serde_json::Value = serde_json::from_slice(&pending[1]).unwrap();
        assert_eq!(second["labels"], 0);
        pending.clear();
        assert!(fill_batch(&mut rx, &mut pending).await);
        assert_eq!(pending.len(), 6);
        // Shutdown: once the queue is closed and empty, the publisher is done...
        drop(tx);
        assert!(fill_batch(&mut rx, &mut pending).await, "...but not with messages pending");
        pending.clear();
        assert!(!fill_batch(&mut rx, &mut pending).await);
    }

    #[test]
    fn rfc3339_now() {
        let now = chrono_now();
        assert_eq!(now.len(), 24, "{now}");
        assert!(now.starts_with("20") && now.ends_with('Z') && now.as_bytes()[10] == b'T');
    }
}
