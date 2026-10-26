//! The error type returned by every client call.

use std::fmt;
use std::time::Duration;

/// A boxed error from the HTTP transport, kept opaque so the public API does not depend
/// on the HTTP library's error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// A `Result` whose error is [`enum@Error`].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Everything that can go wrong when calling the Caretly API.
///
/// HTTP error responses map to one variant per status (see the table below); each carries
/// an [`ApiError`] with the status code, the server's plain-text message and the
/// `Retry-After` delay when the server sent one.
///
/// | Status | Variant |
/// |--------|---------|
/// | 400 | [`Error::BadRequest`] |
/// | 401 | [`Error::Authentication`] |
/// | 402 | [`Error::PaymentRequired`] |
/// | 413 | [`Error::PayloadTooLarge`] |
/// | 415 | [`Error::UnsupportedEncoding`] |
/// | 429 | [`Error::RateLimited`] |
/// | 503 | [`Error::Overloaded`] |
/// | any other | [`Error::Api`] |
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// 400: the ZPL failed to parse, a parameter is invalid, or `page` is out of range.
    #[error("bad request: {0}")]
    BadRequest(ApiError),
    /// 401: the API key is missing, invalid, expired or revoked.
    #[error("authentication failed: {0}")]
    Authentication(ApiError),
    /// 402: billing for the account is not active, so live keys are paused.
    #[error("payment required: {0}")]
    PaymentRequired(ApiError),
    /// 413: the body is larger than 100 MiB after decompression.
    #[error("payload too large: {0}")]
    PayloadTooLarge(ApiError),
    /// 415: the server does not support the request's `Content-Encoding`.
    #[error("unsupported encoding: {0}")]
    UnsupportedEncoding(ApiError),
    /// 429: too many requests for this API key. Retried automatically.
    #[error("rate limited: {0}")]
    RateLimited(ApiError),
    /// 503: the server is overloaded and turned the request away (not billed).
    /// Retried automatically.
    #[error("server overloaded: {0}")]
    Overloaded(ApiError),
    /// Any other non-success status, including other 5xx responses.
    #[error("API error: {0}")]
    Api(ApiError),
    /// The request could not be sent or the response could not be read (DNS, refused
    /// connection, TLS, reset). Retried automatically.
    #[error("connection error: {0}")]
    Connection(#[source] BoxError),
    /// The request took longer than the client's timeout. Not retried.
    #[error("request timed out: {0}")]
    Timeout(#[source] BoxError),
    /// An argument failed client-side validation (for example `page` 0, an
    /// `Idempotency-Key` over 255 characters, or an invalid base URL). Nothing was sent.
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

impl Error {
    /// The API error details, when the server answered with an error status.
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            Error::BadRequest(e)
            | Error::Authentication(e)
            | Error::PaymentRequired(e)
            | Error::PayloadTooLarge(e)
            | Error::UnsupportedEncoding(e)
            | Error::RateLimited(e)
            | Error::Overloaded(e)
            | Error::Api(e) => Some(e),
            _ => None,
        }
    }

    /// The HTTP status code, when the server answered with an error status.
    pub fn status(&self) -> Option<u16> {
        self.api_error().map(|e| e.status)
    }

    /// The server's `Retry-After` delay, when it sent one (429 and 503).
    pub fn retry_after(&self) -> Option<Duration> {
        self.api_error().and_then(|e| e.retry_after)
    }

    /// Whether the client retries this error automatically: 429, 503 and connection
    /// errors.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Error::RateLimited(_) | Error::Overloaded(_) | Error::Connection(_)
        )
    }

    /// Maps an error response to its variant.
    pub(crate) fn from_status(
        status: u16,
        message: String,
        retry_after: Option<Duration>,
    ) -> Error {
        let e = ApiError {
            status,
            message,
            retry_after,
        };
        match status {
            400 => Error::BadRequest(e),
            401 => Error::Authentication(e),
            402 => Error::PaymentRequired(e),
            413 => Error::PayloadTooLarge(e),
            415 => Error::UnsupportedEncoding(e),
            429 => Error::RateLimited(e),
            503 => Error::Overloaded(e),
            _ => Error::Api(e),
        }
    }

    /// Classifies a transport failure from the HTTP library.
    pub(crate) fn from_transport(e: reqwest::Error) -> Error {
        if e.is_timeout() {
            Error::Timeout(Box::new(e))
        } else {
            Error::Connection(Box::new(e))
        }
    }
}

/// Details of an error response from the API.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ApiError {
    /// The HTTP status code.
    pub status: u16,
    /// The plain-text message from the response body.
    pub message: String,
    /// The `Retry-After` delay, when the server sent one.
    pub retry_after: Option<Duration>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {}", self.status)?;
        if !self.message.is_empty() {
            write!(f, ": {}", self.message)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_statuses() {
        type Check = fn(&Error) -> bool;
        let cases: [(u16, Check); 9] = [
            (400, |e| matches!(e, Error::BadRequest(_))),
            (401, |e| matches!(e, Error::Authentication(_))),
            (402, |e| matches!(e, Error::PaymentRequired(_))),
            (413, |e| matches!(e, Error::PayloadTooLarge(_))),
            (415, |e| matches!(e, Error::UnsupportedEncoding(_))),
            (429, |e| matches!(e, Error::RateLimited(_))),
            (503, |e| matches!(e, Error::Overloaded(_))),
            (500, |e| matches!(e, Error::Api(_))),
            (404, |e| matches!(e, Error::Api(_))),
        ];
        for (status, check) in cases {
            let e = Error::from_status(status, "msg".into(), None);
            assert!(check(&e), "{status} mapped to {e:?}");
            assert_eq!(e.status(), Some(status));
        }
    }

    #[test]
    fn display_includes_status_and_message() {
        let e = Error::from_status(400, "parse error: x".into(), None);
        assert_eq!(e.to_string(), "bad request: HTTP 400: parse error: x");
    }
}
