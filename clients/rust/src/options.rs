//! Render options: output format, page, resolution and request settings.

use std::fmt;

use crate::error::{Error, Result};

/// The longest `Idempotency-Key` the API accepts.
pub const MAX_IDEMPOTENCY_KEY_LEN: usize = 255;

/// The output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Format {
    /// A vector PDF with every page (the server default).
    #[default]
    Pdf,
    /// A PNG of one page, chosen with [`RenderOptions::page`].
    Png,
}

impl Format {
    /// The value of the `format` query parameter.
    pub fn as_str(self) -> &'static str {
        match self {
            Format::Pdf => "pdf",
            Format::Png => "png",
        }
    }

    /// The MIME type of a response in this format.
    pub fn content_type(self) -> &'static str {
        match self {
            Format::Pdf => "application/pdf",
            Format::Png => "image/png",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Printer resolution in dots per millimetre.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Dpmm {
    /// 6 dots/mm (152 dpi).
    D6,
    /// 8 dots/mm (203 dpi), the server default.
    #[default]
    D8,
    /// 12 dots/mm (300 dpi).
    D12,
    /// 24 dots/mm (600 dpi).
    D24,
}

impl Dpmm {
    /// The resolution as a number of dots per millimetre.
    pub fn get(self) -> u32 {
        match self {
            Dpmm::D6 => 6,
            Dpmm::D8 => 8,
            Dpmm::D12 => 12,
            Dpmm::D24 => 24,
        }
    }
}

impl TryFrom<u32> for Dpmm {
    type Error = Error;

    /// Accepts 6, 8, 12 or 24; anything else is an [`Error::InvalidArgument`].
    fn try_from(value: u32) -> Result<Self> {
        match value {
            6 => Ok(Dpmm::D6),
            8 => Ok(Dpmm::D8),
            12 => Ok(Dpmm::D12),
            24 => Ok(Dpmm::D24),
            other => Err(Error::InvalidArgument(format!(
                "dpmm must be 6, 8, 12 or 24, not {other}"
            ))),
        }
    }
}

impl fmt::Display for Dpmm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

/// Options for a render call. Every option left unset is omitted from the request, so
/// the server's default applies.
///
/// ```
/// use caretly::{Dpmm, Format, RenderOptions};
///
/// let options = RenderOptions::new()
///     .format(Format::Png)
///     .page(2)
///     .dpmm(Dpmm::D12)
///     .idempotency_key("order-1234");
/// assert_eq!(options.page, Some(2));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RenderOptions {
    /// Output format; the server default is PDF.
    pub format: Option<Format>,
    /// For PNG: the page to render, starting at 1. Pages count across all files of a
    /// multipart request.
    pub page: Option<u32>,
    /// Printer resolution; the server default is 8 dots/mm.
    pub dpmm: Option<Dpmm>,
    /// Ask for a `Server-Timing` header, exposed as [`RenderResult::server_timing`](crate::RenderResult::server_timing).
    pub timings: bool,
    /// `Idempotency-Key` header: a retry with the same key and content is billed once.
    /// When unset and retries are enabled, the client generates one per call.
    pub idempotency_key: Option<String>,
    /// Gzip the request body (`Content-Encoding: gzip`).
    #[cfg(feature = "gzip")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gzip")))]
    pub compress: bool,
}

impl RenderOptions {
    /// Options with every value unset (server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the output format.
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    /// Sets the page shown in a PNG, starting at 1.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the printer resolution.
    pub fn dpmm(mut self, dpmm: Dpmm) -> Self {
        self.dpmm = Some(dpmm);
        self
    }

    /// Asks the server for a `Server-Timing` header.
    pub fn timings(mut self, timings: bool) -> Self {
        self.timings = timings;
        self
    }

    /// Sets the `Idempotency-Key` header (at most 255 characters).
    pub fn idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    /// Gzips the request body.
    #[cfg(feature = "gzip")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gzip")))]
    pub fn compress(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    /// Checks the options client side, so invalid values fail before a request is sent.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.page == Some(0) {
            return Err(Error::InvalidArgument("page must be 1 or more".into()));
        }
        if let Some(key) = &self.idempotency_key {
            if key.is_empty() || key.len() > MAX_IDEMPOTENCY_KEY_LEN {
                return Err(Error::InvalidArgument(format!(
                    "idempotency key must be 1 to {MAX_IDEMPOTENCY_KEY_LEN} characters"
                )));
            }
            if !key.bytes().all(|b| (0x20..0x7f).contains(&b)) {
                return Err(Error::InvalidArgument(
                    "idempotency key must be printable ASCII".into(),
                ));
            }
        }
        Ok(())
    }

    /// The query string, starting with `?`, or empty when no parameter is set.
    pub(crate) fn query(&self) -> String {
        let mut params = Vec::new();
        if let Some(format) = self.format {
            params.push(format!("format={format}"));
        }
        if let Some(page) = self.page {
            params.push(format!("page={page}"));
        }
        if let Some(dpmm) = self.dpmm {
            params.push(format!("dpmm={dpmm}"));
        }
        if self.timings {
            params.push("timings=1".to_owned());
        }
        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }

    #[cfg(feature = "gzip")]
    pub(crate) fn compressed(&self) -> bool {
        self.compress
    }

    #[cfg(not(feature = "gzip"))]
    pub(crate) fn compressed(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_omits_unset_values() {
        assert_eq!(RenderOptions::new().query(), "");
        let all = RenderOptions::new()
            .format(Format::Png)
            .page(3)
            .dpmm(Dpmm::D24)
            .timings(true);
        assert_eq!(all.query(), "?format=png&page=3&dpmm=24&timings=1");
    }

    #[test]
    fn validates_page_and_key() {
        assert!(RenderOptions::new().page(0).validate().is_err());
        assert!(RenderOptions::new().page(1).validate().is_ok());
        assert!(RenderOptions::new()
            .idempotency_key("a".repeat(255))
            .validate()
            .is_ok());
        assert!(RenderOptions::new()
            .idempotency_key("a".repeat(256))
            .validate()
            .is_err());
        assert!(RenderOptions::new()
            .idempotency_key("line\nbreak")
            .validate()
            .is_err());
    }

    #[test]
    fn dpmm_from_number() {
        assert_eq!(Dpmm::try_from(12).unwrap(), Dpmm::D12);
        assert!(matches!(Dpmm::try_from(10), Err(Error::InvalidArgument(_))));
    }
}
