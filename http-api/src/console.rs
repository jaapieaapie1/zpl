//! The zpl-console formats the renderer needs: API key tokens and usage messages.
//!
//! A copy of the verifying half of `zpl-console-protocol` (github.com/jaapieaapie1/zpl-console,
//! private, so it cannot be a dependency of this public repository). The test vectors below
//! were signed by the console's code and keep the two in step.
//!
//! Keys are `zpl_<env>_<JWT>`, the JWT signed with Ed25519 (`alg: EdDSA`). The header's
//! `kid` names the console signing key; the prefix must match the `env` claim.

use std::collections::HashMap;

use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const ISSUER: &str = "zpl-console";
/// RabbitMQ exchange usage is published to (fanout, durable).
pub const USAGE_EXCHANGE: &str = "zpl.usage";
/// Clock skew allowed when checking `exp`, in seconds.
const LEEWAY: i64 = 60;

/// Billed (`live`) or free with watermarked labels (`test`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyEnv {
    Live,
    Test,
}

impl KeyEnv {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "live" => Some(KeyEnv::Live),
            "test" => Some(KeyEnv::Test),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Claims {
    pub iss: String,
    /// The API key id; renewed tokens of one key share it.
    pub sub: Uuid,
    /// The organisation billed for the key.
    pub org: Uuid,
    pub env: KeyEnv,
    pub iat: i64,
    #[serde(default)]
    pub exp: Option<i64>,
}

#[derive(Deserialize)]
struct Header {
    alg: String,
    kid: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    Malformed,
    UnknownKid,
    BadSignature,
    Expired,
}

/// Console public keys, by `kid`.
#[derive(Clone, Default)]
pub struct Verifier {
    keys: HashMap<String, VerifyingKey>,
}

impl Verifier {
    /// `kid:base64url-public-key` entries separated by commas or whitespace, as printed by
    /// the console's `GET /internal/public-keys`.
    pub fn from_config(config: &str) -> Result<Self, String> {
        let mut keys = HashMap::new();
        for entry in config.split([',', ' ', '\n']).map(str::trim).filter(|e| !e.is_empty()) {
            let (kid, key) = entry.split_once(':').ok_or_else(|| format!("{entry}: expected kid:key"))?;
            let bytes: [u8; 32] = B64
                .decode(key)
                .ok()
                .and_then(|b| b.try_into().ok())
                .ok_or_else(|| format!("{kid}: expected a base64url Ed25519 public key"))?;
            let key = VerifyingKey::from_bytes(&bytes).map_err(|e| format!("{kid}: {e}"))?;
            keys.insert(kid.to_string(), key);
        }
        if keys.is_empty() {
            return Err("no public keys".into());
        }
        Ok(Verifier { keys })
    }

    /// Checks prefix, signature, issuer and expiry at Unix time `now`.
    pub fn verify(&self, token: &str, now: i64) -> Result<Claims, TokenError> {
        let rest = token.strip_prefix("zpl_").ok_or(TokenError::Malformed)?;
        let (env, jwt) = rest.split_once('_').ok_or(TokenError::Malformed)?;
        let env = KeyEnv::parse(env).ok_or(TokenError::Malformed)?;
        let (signed, signature) = jwt.rsplit_once('.').ok_or(TokenError::Malformed)?;
        let (header, claims) = signed.split_once('.').ok_or(TokenError::Malformed)?;
        let header: Header = decode_json(header)?;
        if header.alg != "EdDSA" {
            return Err(TokenError::Malformed);
        }
        let key = self.keys.get(&header.kid).ok_or(TokenError::UnknownKid)?;
        let signature: [u8; 64] = B64
            .decode(signature)
            .ok()
            .and_then(|b| b.try_into().ok())
            .ok_or(TokenError::Malformed)?;
        key.verify_strict(signed.as_bytes(), &Signature::from_bytes(&signature))
            .map_err(|_| TokenError::BadSignature)?;
        let claims: Claims = decode_json(claims)?;
        if claims.iss != ISSUER || claims.env != env {
            return Err(TokenError::Malformed);
        }
        if claims.exp.is_some_and(|exp| exp + LEEWAY < now) {
            return Err(TokenError::Expired);
        }
        Ok(claims)
    }
}

fn decode_json<T: for<'de> Deserialize<'de>>(part: &str) -> Result<T, TokenError> {
    let bytes = B64.decode(part).map_err(|_| TokenError::Malformed)?;
    serde_json::from_slice(&bytes).map_err(|_| TokenError::Malformed)
}

/// `kind` of [`UsageAggregate`], and its AMQP `type` property. Consumers tell it from the
/// older one-message-per-request format, which has no `kind`, by this field.
pub const AGGREGATE_KIND: &str = "aggregate";

/// Usage of one key in one format over a short window (`ZPL_USAGE_WINDOW_MS`), published to
/// [`USAGE_EXCHANGE`]. Requests with an `Idempotency-Key` are listed one by one in
/// `idempotent` (not in `labels` and `requests`), so consumers count each `request_id` once
/// however often it is retried, on whichever renderer. `at` and `until` (RFC 3339) are the
/// times of the first and last request counted, always on the same UTC day.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAggregate {
    /// Always [`AGGREGATE_KIND`].
    pub kind: &'static str,
    /// Unique (UUIDv7); a redelivered message keeps it, so consumers store it once.
    pub id: Uuid,
    pub key_id: Uuid,
    pub org_id: Uuid,
    pub env: KeyEnv,
    pub format: &'static str,
    /// Labels and requests without an `Idempotency-Key`.
    pub labels: u32,
    pub requests: u32,
    pub at: String,
    pub until: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub idempotent: Vec<IdempotentRequest>,
}

/// A request with an `Idempotency-Key`: its id is derived from the key and the rendered
/// content, so a retry has the same id and is counted once.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IdempotentRequest {
    pub request_id: Uuid,
    pub labels: u32,
    pub idempotency_key: String,
    pub at: String,
}

/// `GET /internal/revocations`.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct RevocationList {
    pub revoked: Vec<Revocation>,
    #[serde(default)]
    pub suspended_orgs: Vec<Uuid>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Revocation {
    pub key_id: Uuid,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    // Signed by zpl-console-protocol's Signer with a throwaway key: key id 0xb of
    // organisation 0xa, live and test without expiry, and a live one that has expired.
    pub(crate) const KEYS: &str = "test:wXB2130mLMGs6OVDgkxC7DmfFI5YjA3ByCJDB0S5oiY";
    pub(crate) const LIVE: &str = "zpl_live_eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCIsImtpZCI6InRlc3QifQ.eyJpc3MiOiJ6cGwtY29uc29sZSIsInN1YiI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwYiIsIm9yZyI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwYSIsImVudiI6ImxpdmUiLCJpYXQiOjE3OTAwMDAwMDB9.8lbtMAmWi4kI6aF_6QsFfQ8UBIJ3QL5qkiGAVxmY_DYUqMy7KMXWQzACr2Y3bnAli-Pfs-dCEQF7jR7mY76UAQ";
    pub(crate) const TEST: &str = "zpl_test_eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCIsImtpZCI6InRlc3QifQ.eyJpc3MiOiJ6cGwtY29uc29sZSIsInN1YiI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwYiIsIm9yZyI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwYSIsImVudiI6InRlc3QiLCJpYXQiOjE3OTAwMDAwMDB9.xXHDWeh5o16YgTwok67LlUn8d4RLTgaua_tayeTe_jrnXi7RR0NV0k3O27qFI_FRjxcHPqUdLSIMgnDpGxbCDQ";
    pub(crate) const EXPIRED: &str = "zpl_live_eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCIsImtpZCI6InRlc3QifQ.eyJpc3MiOiJ6cGwtY29uc29sZSIsInN1YiI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwYiIsIm9yZyI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwYSIsImVudiI6ImxpdmUiLCJpYXQiOjE3OTAwMDAwMDAsImV4cCI6MTAwMDAwMDAwMH0.Khkwpm6RbzuETde9fQ1RYEBunFpTEmjGKPPUGaDMaoOrNdtzUV2m1TO5DWUiNWSyTx455Ad0VgapyjSpUqsiAg";

    #[test]
    fn verifies_console_tokens() {
        let verifier = Verifier::from_config(KEYS).unwrap();
        let live = verifier.verify(LIVE, 1_800_000_000).unwrap();
        assert_eq!(live.env, KeyEnv::Live);
        assert_eq!(live.sub, Uuid::from_u128(0xb));
        assert_eq!(live.org, Uuid::from_u128(0xa));
        assert_eq!(live.exp, None);
        assert_eq!(verifier.verify(TEST, 1_800_000_000).unwrap().env, KeyEnv::Test);
        assert_eq!(verifier.verify(EXPIRED, 1_800_000_000), Err(TokenError::Expired));
    }

    #[test]
    fn rejects_tampering() {
        let verifier = Verifier::from_config(KEYS).unwrap();
        assert_eq!(verifier.verify(&LIVE.replacen("zpl_live_", "zpl_test_", 1), 0), Err(TokenError::Malformed));
        let mut forged = LIVE.to_string();
        forged.pop();
        forged.push(if LIVE.ends_with('A') { 'B' } else { 'A' });
        assert!(matches!(verifier.verify(&forged, 0), Err(TokenError::BadSignature | TokenError::Malformed)));
        let other = Verifier::from_config("other:wXB2130mLMGs6OVDgkxC7DmfFI5YjA3ByCJDB0S5oiY").unwrap();
        assert_eq!(other.verify(LIVE, 0), Err(TokenError::UnknownKid));
        assert_eq!(verifier.verify("Bearer nonsense", 0), Err(TokenError::Malformed));
        assert!(Verifier::from_config("").is_err());
    }
}
