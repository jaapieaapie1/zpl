//! API keys and usage, with zpl-console (github.com/jaapieaapie1/zpl-console).
//!
//! Enabled by `ZPL_KEYS_PUBLIC_KEYS` (the console's `kid:key` public keys). Then:
//! - `POST /render` needs `Authorization: Bearer zpl_live_…` (or `X-API-Key`). Keys are
//!   verified offline; the console is never asked per request.
//! - Every 30 s the revocation list is fetched from `ZPL_CONSOLE_INTERNAL_URL` with
//!   `ZPL_CONSOLE_TOKEN`: revoked keys get 401, live keys of suspended organisations 402.
//!   When the console is unreachable the last list stays in force.
//! - Rendered requests are added up per key and format in memory and published to the
//!   `zpl.usage` exchange on `ZPL_AMQP_URL` once per window (`ZPL_USAGE_WINDOW_MS`,
//!   default 1000), one message per key and format, see [`crate::usage`]. The response
//!   never waits for RabbitMQ, and usage is kept (adding up) while RabbitMQ is away.
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
use uuid::Uuid;

use crate::console::{KeyEnv, RevocationList, TokenError, Verifier};
use crate::rate_limit::{Decision, RateLimiter, DEFAULT_BURST, DEFAULT_PER_SECOND};
use crate::usage::{Publisher, Usage, DEFAULT_WINDOW};

const REVOCATION_INTERVAL: Duration = Duration::from_secs(30);
/// Namespace for request ids derived from `Idempotency-Key` and the request's content,
/// so a retried request maps to the same id and the console counts it once.
const IDEMPOTENCY_NAMESPACE: Uuid = Uuid::from_u128(0x7a3c_51f0_9d6e_4b8a_a2f1_3e5c_7d9b_0c14);

pub struct Billing {
    verifier: Verifier,
    revoked: RwLock<Revoked>,
    usage: Option<Arc<Usage>>,
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
    /// as for local use). Starts the revocation and usage tasks; the server finishes the
    /// usage publisher (when there is one) at shutdown to publish what is left.
    pub fn from_env() -> Result<Option<(Arc<Billing>, Option<Publisher>)>, String> {
        let Some(keys) = var("ZPL_KEYS_PUBLIC_KEYS") else { return Ok(None) };
        let verifier = Verifier::from_config(&keys).map_err(|e| format!("ZPL_KEYS_PUBLIC_KEYS: {e}"))?;
        let console_url = var("ZPL_CONSOLE_URL").unwrap_or_else(|| "https://caretly.dev/console".into());
        let rate_limit = RateLimiter::from_env()?;
        if rate_limit.is_none() {
            eprintln!("ZPL_RATE_LIMIT_PER_SECOND or ZPL_RATE_LIMIT_BURST is 0: API keys are not rate limited");
        }

        let window = match var("ZPL_USAGE_WINDOW_MS") {
            None => DEFAULT_WINDOW,
            Some(v) => v
                .trim()
                .parse::<u64>()
                .ok()
                .filter(|&n| n > 0)
                .map(Duration::from_millis)
                .ok_or_else(|| format!("ZPL_USAGE_WINDOW_MS: expected a positive whole number, got {v:?}"))?,
        };
        let (usage, publisher) = match var("ZPL_AMQP_URL") {
            Some(url) => {
                let usage = Usage::new();
                (Some(usage.clone()), Some(Publisher::start(url, usage, window)))
            }
            None => {
                eprintln!("ZPL_AMQP_URL is not set: usage is not reported");
                (None, None)
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
    pub fn with_verifier(verifier: Verifier, usage: Option<Arc<Usage>>) -> Arc<Billing> {
        Self::with_rate_limit(verifier, usage, RateLimiter::new(DEFAULT_PER_SECOND, DEFAULT_BURST))
    }

    /// For tests: like [`with_verifier`](Self::with_verifier) with the given rate limit.
    pub fn with_rate_limit(
        verifier: Verifier,
        usage: Option<Arc<Usage>>,
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
    /// Only adds to the usage in memory: never waits.
    pub fn record(&self, caller: &Caller, labels: usize, format: &'static str, fingerprint: Option<&blake3::Hash>) -> usize {
        let labels = labels.min(u32::MAX as usize);
        if let Some(usage) = &self.usage {
            let idempotent = match (&caller.idempotency_key, fingerprint) {
                (Some(key), Some(fingerprint)) => {
                    let name = format!("{}:{key}:{}", caller.key_id, fingerprint.to_hex());
                    Some((Uuid::new_v5(&IDEMPOTENCY_NAMESPACE, name.as_bytes()), key.clone()))
                }
                _ => None,
            };
            usage.add(crate::usage::Request {
                org_id: caller.org_id,
                key_id: caller.key_id,
                env: caller.env,
                format,
                labels: labels as u32,
                idempotent,
                at_ms: crate::usage::now_ms(),
            });
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
}
