//! Request building, response handling and retry policy shared by the async and blocking
//! clients. Neither client does anything here that depends on how it waits for I/O.

use std::fmt;
use std::time::Duration;

use bytes::Bytes;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, RETRY_AFTER};

use crate::error::{Error, Result};
use crate::options::RenderOptions;

/// The hosted API.
pub(crate) const DEFAULT_BASE_URL: &str = "https://caretly.dev";
/// Default per-request timeout.
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
/// Default number of retries after the first attempt.
pub(crate) const DEFAULT_MAX_RETRIES: u32 = 2;
/// The environment variable read when no API key is given.
pub(crate) const API_KEY_ENV: &str = "CARETLY_API_KEY";
/// `User-Agent` of every request.
pub(crate) const USER_AGENT: &str = concat!("caretly-rust/", env!("CARGO_PKG_VERSION"));

const BACKOFF_START: Duration = Duration::from_millis(500);
const BACKOFF_CAP: Duration = Duration::from_secs(8);

/// Settings collected by a client builder.
#[derive(Default, Clone)]
pub(crate) struct BuilderState {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub timeout: Option<Duration>,
    pub max_retries: Option<u32>,
}

impl fmt::Debug for BuilderState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

/// Resolved client settings.
#[derive(Clone)]
pub(crate) struct Config {
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
    /// Headers sent with every request: `Authorization` when a key is set.
    pub default_headers: HeaderMap,
    has_key: bool,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.base_url)
            .field(
                "api_key",
                &if self.has_key { "<redacted>" } else { "<none>" },
            )
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

impl BuilderState {
    /// Resolves the settings, reading `CARETLY_API_KEY` when no key was given.
    pub fn into_config(self) -> Result<Config> {
        let api_key = match self.api_key {
            Some(key) => Some(key),
            None => std::env::var(API_KEY_ENV).ok(),
        }
        .map(|key| key.trim().to_owned())
        .filter(|key| !key.is_empty());

        let mut default_headers = HeaderMap::new();
        if let Some(key) = &api_key {
            let mut value = HeaderValue::from_str(&format!("Bearer {key}")).map_err(|_| {
                Error::InvalidArgument(
                    "API key contains characters not allowed in an HTTP header".into(),
                )
            })?;
            value.set_sensitive(true);
            default_headers.insert(AUTHORIZATION, value);
        }

        let base_url = self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_owned());
        let base_url = base_url.trim().trim_end_matches('/').to_owned();
        let parsed = reqwest::Url::parse(&base_url)
            .map_err(|e| Error::InvalidArgument(format!("invalid base URL {base_url:?}: {e}")))?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.query().is_some() {
            return Err(Error::InvalidArgument(format!(
                "invalid base URL {base_url:?}: use an http or https URL without a query"
            )));
        }

        Ok(Config {
            base_url,
            timeout: self.timeout.unwrap_or(DEFAULT_TIMEOUT),
            max_retries: self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES),
            default_headers,
            has_key: api_key.is_some(),
        })
    }
}

/// Generates the setters shared by the async and blocking `ClientBuilder`s. Each builder
/// is a struct with a `state: BuilderState` field.
macro_rules! builder_setters {
    () => {
        /// Sets the API key, sent as `Authorization: Bearer <key>`.
        ///
        /// Without this call the key is read from the `CARETLY_API_KEY` environment
        /// variable. An empty key sends no `Authorization` header (for self-hosted servers
        /// running without keys) and skips the environment variable.
        pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
            self.state.api_key = Some(api_key.into());
            self
        }

        /// Sets the base URL of the API. Default: `https://caretly.dev`.
        pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
            self.state.base_url = Some(base_url.into());
            self
        }

        /// Sets the timeout for each attempt, from sending the request to reading the
        /// whole response. Default: 30 seconds.
        pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
            self.state.timeout = Some(timeout);
            self
        }

        /// Sets how many times a call is retried after a 429, a 503 or a connection
        /// error. Default: 2. Use 0 to disable retries.
        pub fn max_retries(mut self, max_retries: u32) -> Self {
            self.state.max_retries = Some(max_retries);
            self
        }
    };
}
pub(crate) use builder_setters;

/// The body of a render call before encoding.
pub(crate) enum Payload {
    /// One raw ZPL body.
    Label(Vec<u8>),
    /// Several named files, sent as `multipart/form-data`.
    Files(Vec<(String, Vec<u8>)>),
}

impl Payload {
    /// Collects the files of a `render_files` call.
    pub fn files<I, N, D>(files: I) -> Result<Payload>
    where
        I: IntoIterator<Item = (N, D)>,
        N: Into<String>,
        D: Into<Vec<u8>>,
    {
        let files: Vec<(String, Vec<u8>)> = files
            .into_iter()
            .map(|(n, d)| (n.into(), d.into()))
            .collect();
        if files.is_empty() {
            return Err(Error::InvalidArgument(
                "render_files needs at least one file".into(),
            ));
        }
        Ok(Payload::Files(files))
    }
}

/// A render request ready to send, and to send again on a retry.
#[derive(Debug, Clone)]
pub(crate) struct Prepared {
    pub url: String,
    pub headers: HeaderMap,
    pub body: Bytes,
}

/// Validates the options and encodes the request.
pub(crate) fn prepare(
    config: &Config,
    payload: Payload,
    options: &RenderOptions,
) -> Result<Prepared> {
    options.validate()?;
    let url = format!("{}/render{}", config.base_url, options.query());

    let mut headers = HeaderMap::new();
    let body = match payload {
        Payload::Label(zpl) => {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("text/plain; charset=utf-8"),
            );
            zpl
        }
        Payload::Files(files) => {
            let boundary = format!("caretly-{}", uuid::Uuid::new_v4().simple());
            let content_type = format!("multipart/form-data; boundary={boundary}");
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(&content_type).expect("boundary is ASCII"),
            );
            encode_multipart(&files, &boundary)
        }
    };

    let body = if options.compressed() {
        headers.insert("content-encoding", HeaderValue::from_static("gzip"));
        gzip(&body)?
    } else {
        body
    };

    let idempotency_key = match &options.idempotency_key {
        Some(key) => Some(key.clone()),
        // Retries must not be billed twice, so give the call a key of its own.
        None if config.max_retries > 0 => Some(uuid::Uuid::new_v4().to_string()),
        None => None,
    };
    if let Some(key) = idempotency_key {
        headers.insert(
            "idempotency-key",
            HeaderValue::from_str(&key).expect("validated above"),
        );
    }

    Ok(Prepared {
        url,
        headers,
        body: Bytes::from(body),
    })
}

#[cfg(feature = "gzip")]
fn gzip(body: &[u8]) -> Result<Vec<u8>> {
    use std::io::Write;
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder
        .write_all(body)
        .and_then(|()| encoder.finish())
        .map_err(|e| Error::InvalidArgument(format!("gzip failed: {e}")))
}

#[cfg(not(feature = "gzip"))]
fn gzip(_body: &[u8]) -> Result<Vec<u8>> {
    unreachable!("compress is only settable with the gzip feature")
}

/// Encodes files as `multipart/form-data`, one part named `file` per file.
fn encode_multipart(files: &[(String, Vec<u8>)], boundary: &str) -> Vec<u8> {
    let size: usize = files
        .iter()
        .map(|(n, d)| n.len() + d.len() + boundary.len() + 128)
        .sum();
    let mut body = Vec::with_capacity(size + boundary.len() + 8);
    for (index, (name, data)) in files.iter().enumerate() {
        let filename = if name.is_empty() {
            format!("label-{}.zpl", index + 1)
        } else {
            escape_filename(name)
        };
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n")
                .as_bytes(),
        );
        body.extend_from_slice(b"Content-Type: text/plain; charset=utf-8\r\n\r\n");
        body.extend_from_slice(data);
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    body
}

/// Escapes a filename for a quoted `Content-Disposition` parameter, as browsers do.
fn escape_filename(name: &str) -> String {
    name.replace('"', "%22")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

/// Builds the error for a non-success response.
pub(crate) fn error_response(status: u16, headers: &HeaderMap, body: &[u8]) -> Error {
    let retry_after = headers
        .get(RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(Duration::from_secs);
    let message = String::from_utf8_lossy(body).trim().to_owned();
    Error::from_status(status, message, retry_after)
}

/// How long to wait before retry number `attempt + 1`, or `None` when `error` is final.
pub(crate) fn retry_delay(error: &Error, attempt: u32, max_retries: u32) -> Option<Duration> {
    if attempt >= max_retries || !error.is_retryable() {
        return None;
    }
    Some(error.retry_after().unwrap_or_else(|| backoff(attempt)))
}

/// Exponential backoff: 0.5 s, 1 s, 2 s, 4 s, then 8 s.
fn backoff(attempt: u32) -> Duration {
    BACKOFF_START
        .saturating_mul(1u32.checked_shl(attempt).unwrap_or(u32::MAX))
        .min(BACKOFF_CAP)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        BuilderState {
            api_key: Some(String::new()),
            ..Default::default()
        }
        .into_config()
        .unwrap()
    }

    #[test]
    fn backoff_doubles_and_caps() {
        let delays: Vec<_> = (0..7).map(backoff).collect();
        let ms: Vec<_> = delays.iter().map(Duration::as_millis).collect();
        assert_eq!(ms, [500, 1000, 2000, 4000, 8000, 8000, 8000]);
        assert_eq!(backoff(100), BACKOFF_CAP);
    }

    #[test]
    fn retries_only_retryable_errors() {
        let limited = Error::from_status(429, String::new(), Some(Duration::from_secs(3)));
        assert_eq!(retry_delay(&limited, 0, 2), Some(Duration::from_secs(3)));
        assert_eq!(retry_delay(&limited, 2, 2), None);
        let overloaded = Error::from_status(503, String::new(), None);
        assert_eq!(retry_delay(&overloaded, 1, 2), Some(Duration::from_secs(1)));
        let server = Error::from_status(500, String::new(), None);
        assert_eq!(retry_delay(&server, 0, 2), None);
    }

    #[test]
    fn base_url_is_trimmed_and_checked() {
        let state = |url: &str| BuilderState {
            base_url: Some(url.into()),
            api_key: Some(String::new()),
            ..Default::default()
        };
        assert_eq!(
            state("http://localhost:8080/")
                .into_config()
                .unwrap()
                .base_url,
            "http://localhost:8080"
        );
        assert!(state("ftp://example.com").into_config().is_err());
        assert!(state("not a url").into_config().is_err());
    }

    #[test]
    fn empty_key_sends_no_authorization() {
        assert!(config().default_headers.get(AUTHORIZATION).is_none());
        let keyed = BuilderState {
            api_key: Some("zpl_test_x".into()),
            ..Default::default()
        }
        .into_config()
        .unwrap();
        assert_eq!(keyed.default_headers[AUTHORIZATION], "Bearer zpl_test_x");
    }

    #[test]
    fn generates_idempotency_key_only_with_retries() {
        let label = || Payload::Label(b"^XA^XZ".to_vec());
        let prepared = prepare(&config(), label(), &RenderOptions::new()).unwrap();
        assert_eq!(prepared.headers["idempotency-key"].len(), 36);
        let mut no_retries = config();
        no_retries.max_retries = 0;
        let prepared = prepare(&no_retries, label(), &RenderOptions::new()).unwrap();
        assert!(prepared.headers.get("idempotency-key").is_none());
        let prepared = prepare(
            &config(),
            label(),
            &RenderOptions::new().idempotency_key("order-1"),
        )
        .unwrap();
        assert_eq!(prepared.headers["idempotency-key"], "order-1");
    }

    #[test]
    fn multipart_shape() {
        let body = encode_multipart(
            &[
                ("a\".zpl".into(), b"^XA^XZ".to_vec()),
                (String::new(), b"x".to_vec()),
            ],
            "B",
        );
        let text = String::from_utf8(body).unwrap();
        assert_eq!(
            text,
            "--B\r\nContent-Disposition: form-data; name=\"file\"; filename=\"a%22.zpl\"\r\n\
             Content-Type: text/plain; charset=utf-8\r\n\r\n^XA^XZ\r\n\
             --B\r\nContent-Disposition: form-data; name=\"file\"; filename=\"label-2.zpl\"\r\n\
             Content-Type: text/plain; charset=utf-8\r\n\r\nx\r\n--B--\r\n"
        );
    }

    #[test]
    fn rejects_empty_file_list() {
        let none: Vec<(String, Vec<u8>)> = Vec::new();
        assert!(matches!(
            Payload::files(none),
            Err(Error::InvalidArgument(_))
        ));
    }
}
