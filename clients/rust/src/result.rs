//! The result of a successful render.

use std::io;
use std::path::Path;

use reqwest::header::HeaderMap;

/// A rendered PDF or PNG with the metadata from the response headers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RenderResult {
    /// The file contents.
    pub bytes: Vec<u8>,
    /// `Content-Type`: `application/pdf` or `image/png`.
    pub content_type: String,
    /// `X-Page-Count`: total pages across the input (0 if the header was missing).
    pub page_count: u32,
    /// `X-File-Page-Counts`: pages per uploaded file, in upload order. Empty unless the
    /// request was multipart.
    pub file_page_counts: Vec<u32>,
    /// `X-Labels-Billed`: labels billed for this request (0 for a test key). `None` when
    /// the server runs without API keys.
    pub labels_billed: Option<u32>,
    /// `RateLimit-Limit` and `RateLimit-Remaining`, when the server sent both.
    pub rate_limit: Option<RateLimit>,
    /// The raw `Server-Timing` header, sent when [`RenderOptions::timings`](crate::RenderOptions::timings) is set.
    pub server_timing: Option<String>,
}

/// The API key's rate limit state after this request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct RateLimit {
    /// Burst size: how many requests the key can make at once.
    pub limit: u32,
    /// Requests left in the burst.
    pub remaining: u32,
}

impl RenderResult {
    /// Writes the file contents to `path`, replacing any existing file.
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        std::fs::write(path, &self.bytes)
    }

    /// The file contents, consuming the result.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub(crate) fn from_response(headers: &HeaderMap, bytes: Vec<u8>) -> Self {
        let text = |name: &str| {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::trim)
        };
        let number = |name: &str| text(name).and_then(|v| v.parse::<u32>().ok());
        let rate_limit = match (number("ratelimit-limit"), number("ratelimit-remaining")) {
            (Some(limit), Some(remaining)) => Some(RateLimit { limit, remaining }),
            _ => None,
        };
        RenderResult {
            bytes,
            content_type: text("content-type").unwrap_or_default().to_owned(),
            page_count: number("x-page-count").unwrap_or(0),
            file_page_counts: text("x-file-page-counts")
                .map(|v| v.split(',').filter_map(|n| n.trim().parse().ok()).collect())
                .unwrap_or_default(),
            labels_billed: number("x-labels-billed"),
            rate_limit,
            server_timing: text("server-timing").map(str::to_owned),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn parses_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/pdf"));
        headers.insert("x-page-count", HeaderValue::from_static("3"));
        headers.insert("x-file-page-counts", HeaderValue::from_static("2, 1"));
        headers.insert("x-labels-billed", HeaderValue::from_static("3"));
        headers.insert("ratelimit-limit", HeaderValue::from_static("200"));
        headers.insert("ratelimit-remaining", HeaderValue::from_static("199"));
        headers.insert("server-timing", HeaderValue::from_static("read;dur=0.1"));
        let result = RenderResult::from_response(&headers, b"%PDF".to_vec());
        assert_eq!(result.page_count, 3);
        assert_eq!(result.file_page_counts, vec![2, 1]);
        assert_eq!(result.labels_billed, Some(3));
        assert_eq!(
            result.rate_limit,
            Some(RateLimit {
                limit: 200,
                remaining: 199
            })
        );
        assert_eq!(result.server_timing.as_deref(), Some("read;dur=0.1"));
    }

    #[test]
    fn optional_headers_absent() {
        let result = RenderResult::from_response(&HeaderMap::new(), Vec::new());
        assert_eq!(result.page_count, 0);
        assert!(result.file_page_counts.is_empty());
        assert_eq!(result.labels_billed, None);
        assert_eq!(result.rate_limit, None);
        assert_eq!(result.server_timing, None);
    }
}
