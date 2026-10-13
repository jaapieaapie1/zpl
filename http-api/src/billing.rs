//! API keys and usage, with zpl-console (github.com/jaapieaapie1/zpl-console).
//!
//! Enabled by `ZPL_KEYS_PUBLIC_KEYS` (the console's `kid:key` public keys). Then:
//! - `POST /render` needs `Authorization: Bearer zpl_live_…` (or `X-API-Key`). Keys are
//!   verified offline; the console is never asked per request.
//! - Every 30 s the revocation list is fetched from `ZPL_CONSOLE_INTERNAL_URL` with
//!   `ZPL_CONSOLE_TOKEN`: revoked keys get 401, live keys of suspended organisations 402.
//!   When the console is unreachable the last list stays in force.
//! - Each rendered request is published as one usage message to the `zpl.usage` exchange
//!   on `ZPL_AMQP_URL`, from a background task: the response never waits for RabbitMQ.
//!   Up to [`QUEUE_LIMIT`] messages wait in memory while RabbitMQ is away.
//! - Test keys render free, with a watermark on every label.
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
use uuid::Uuid;

use crate::console::{KeyEnv, RevocationList, TokenError, UsageEvent, Verifier, USAGE_EXCHANGE};

/// Usage messages kept while RabbitMQ is unreachable; beyond this they are dropped
/// (unbilled) rather than growing without bound.
pub const QUEUE_LIMIT: usize = 10_000;
const REVOCATION_INTERVAL: Duration = Duration::from_secs(30);
/// Namespace for request ids derived from `Idempotency-Key`, so a retried request maps
/// to the same id and the console counts it once.
const IDEMPOTENCY_NAMESPACE: Uuid = Uuid::from_u128(0x7a3c_51f0_9d6e_4b8a_a2f1_3e5c_7d9b_0c14);

pub struct Billing {
    verifier: Verifier,
    revoked: RwLock<Revoked>,
    usage: Option<mpsc::Sender<UsageEvent>>,
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
        let console_url = var("ZPL_CONSOLE_URL").unwrap_or_else(|| "https://zpl.jaapelst.nl/console".into());

        let (usage, publisher) = match var("ZPL_AMQP_URL") {
            Some(url) => {
                let (tx, rx) = mpsc::channel(QUEUE_LIMIT);
                (Some(tx), tokio::spawn(publish(url, rx)))
            }
            None => {
                eprintln!("ZPL_AMQP_URL is not set: usage is not reported");
                (None, tokio::spawn(async {}))
            }
        };
        let billing = Arc::new(Billing { verifier, revoked: RwLock::default(), usage, console_url });
        match (var("ZPL_CONSOLE_INTERNAL_URL"), var("ZPL_CONSOLE_TOKEN")) {
            (Some(url), Some(token)) => {
                tokio::spawn(poll_revocations(Arc::downgrade(&billing), url, token));
            }
            _ => eprintln!("ZPL_CONSOLE_INTERNAL_URL / ZPL_CONSOLE_TOKEN not set: revoked keys keep working"),
        }
        Ok(Some((billing, publisher)))
    }

    /// For tests: keys only, no console or RabbitMQ.
    pub fn with_verifier(verifier: Verifier, usage: Option<mpsc::Sender<UsageEvent>>) -> Arc<Billing> {
        Arc::new(Billing { verifier, revoked: RwLock::default(), usage, console_url: "https://console.test".into() })
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
    pub fn record(&self, caller: &Caller, labels: usize, format: &'static str) -> usize {
        let labels = labels.min(u32::MAX as usize);
        if let Some(usage) = &self.usage {
            let request_id = match &caller.idempotency_key {
                Some(key) => Uuid::new_v5(&IDEMPOTENCY_NAMESPACE, format!("{}:{key}", caller.key_id).as_bytes()),
                None => Uuid::new_v4(),
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
            if usage.try_send(event).is_err() {
                eprintln!("usage queue full or closed: dropped usage of {labels} label(s) for key {}", caller.key_id);
            }
        }
        if caller.is_test() { 0 } else { labels }
    }
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
    let mut pending: Option<UsageEvent> = None;
    // Log a failure once, not every retry (DNS is often not ready when a pod starts).
    let mut failing = false;
    loop {
        match publish_until_error(&url, &mut rx, &mut pending, &mut failing).await {
            Ok(()) => return,
            Err(e) => {
                if !failing {
                    eprintln!("publishing usage failed, retrying every 2 s: {e}");
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
    pending: &mut Option<UsageEvent>,
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
        let event = match pending.take() {
            Some(event) => event,
            None => match rx.recv().await {
                Some(event) => event,
                None => return Ok(()),
            },
        };
        let body = serde_json::to_vec(&event).expect("usage serializes");
        *pending = Some(event);
        let confirm = channel
            .basic_publish(USAGE_EXCHANGE, "", BasicPublishOptions::default(), &body, properties.clone())
            .await?
            .await?;
        if confirm.is_nack() {
            // The broker refused it; retry it on a fresh connection.
            return Err(lapin::Error::InvalidChannelState(lapin::ChannelState::Error));
        }
        *pending = None;
    }
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

    #[test]
    fn rfc3339_now() {
        let now = chrono_now();
        assert_eq!(now.len(), 24, "{now}");
        assert!(now.starts_with("20") && now.ends_with('Z') && now.as_bytes()[10] == b'T');
    }
}
