//! The blocking client, for programs without an async runtime.
//!
//! [`Client`] has the same methods as the async `caretly::Client`, but each call blocks
//! the current thread until the response arrives. Do not use it inside an async runtime
//! (it panics there); use the async client instead.

use std::fmt;
use std::sync::Arc;

use crate::error::{Error, Result};
use crate::options::{Format, RenderOptions};
use crate::result::RenderResult;
use crate::shared::{self, BuilderState, Config, Payload, Prepared};

/// Blocking client for the Caretly API.
///
/// Cloning is cheap and shares the connection pool, so create one client and reuse it.
///
/// ```no_run
/// # fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = caretly::blocking::Client::new()?; // reads CARETLY_API_KEY
/// let pdf = client.render_pdf("^XA^FO50,50^A0N,40^FDHello^FS^XZ")?;
/// pdf.save("label.pdf")?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct Client {
    http: reqwest::blocking::Client,
    config: Arc<Config>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.config.fmt(f)
    }
}

impl Client {
    /// A client with the default settings and the API key from `CARETLY_API_KEY`
    /// (no key when it is unset).
    ///
    /// Fails when the key holds characters not allowed in an HTTP header, or when the
    /// TLS backend cannot be initialised.
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// A builder to configure the API key, base URL, timeout and retries.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Renders one ZPL document (which may hold several `^XA…^XZ` labels).
    ///
    /// `zpl` can be a `&str`, `String`, `&[u8]` or `Vec<u8>`.
    pub fn render(&self, zpl: impl Into<Vec<u8>>, options: RenderOptions) -> Result<RenderResult> {
        let prepared = shared::prepare(&self.config, Payload::Label(zpl.into()), &options)?;
        self.execute(prepared)
    }

    /// Renders several ZPL files in one multipart request. Each file starts from a fresh
    /// printer state, and pages follow the order of `files`.
    ///
    /// ```no_run
    /// # fn run(client: caretly::blocking::Client) -> Result<(), caretly::Error> {
    /// let result = client
    ///     .render_files([("a.zpl", "^XA^FDa^FS^XZ"), ("b.zpl", "^XA^FDb^FS^XZ")], Default::default())
    ///     ?;
    /// assert_eq!(result.file_page_counts, vec![1, 1]);
    /// # Ok(()) }
    /// ```
    pub fn render_files<I, N, D>(&self, files: I, options: RenderOptions) -> Result<RenderResult>
    where
        I: IntoIterator<Item = (N, D)>,
        N: Into<String>,
        D: Into<Vec<u8>>,
    {
        let prepared = shared::prepare(&self.config, Payload::files(files)?, &options)?;
        self.execute(prepared)
    }

    /// Renders ZPL to a PDF with every page.
    pub fn render_pdf(&self, zpl: impl Into<Vec<u8>>) -> Result<RenderResult> {
        self.render(zpl, RenderOptions::new().format(Format::Pdf))
    }

    /// Renders one page of ZPL (starting at 1) to a PNG.
    pub fn render_png(&self, zpl: impl Into<Vec<u8>>, page: u32) -> Result<RenderResult> {
        self.render(zpl, RenderOptions::new().format(Format::Png).page(page))
    }

    /// Calls `GET /health`. `Ok(true)` when the server answers `ok`, `Ok(false)` for any
    /// other answer, and an error when the server cannot be reached. Not retried.
    pub fn health(&self) -> Result<bool> {
        let url = format!("{}/health", self.config.base_url);
        let response = self.http.get(url).send().map_err(Error::from_transport)?;
        let ok = response.status().as_u16() == 200;
        let body = response.text().map_err(Error::from_transport)?;
        Ok(ok && body.trim() == "ok")
    }

    fn execute(&self, prepared: Prepared) -> Result<RenderResult> {
        let mut attempt = 0;
        loop {
            let error = match self.send(&prepared) {
                Ok(result) => return Ok(result),
                Err(error) => error,
            };
            match shared::retry_delay(&error, attempt, self.config.max_retries) {
                Some(delay) => std::thread::sleep(delay),
                None => return Err(error),
            }
            attempt += 1;
        }
    }

    fn send(&self, prepared: &Prepared) -> Result<RenderResult> {
        let response = self
            .http
            .post(&prepared.url)
            .headers(prepared.headers.clone())
            .body(prepared.body.clone())
            .send()
            .map_err(Error::from_transport)?;
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.bytes().map_err(Error::from_transport)?;
        if status == 200 {
            Ok(RenderResult::from_response(&headers, body.into()))
        } else {
            Err(shared::error_response(status, &headers, &body))
        }
    }
}

/// Builder for [`Client`], created with [`Client::builder`].
///
/// ```
/// use std::time::Duration;
///
/// let client = caretly::blocking::Client::builder()
///     .api_key("zpl_test_...")
///     .base_url("http://localhost:3000")
///     .timeout(Duration::from_secs(10))
///     .max_retries(3)
///     .build()?;
/// # Ok::<(), caretly::Error>(())
/// ```
#[derive(Debug, Default, Clone)]
#[must_use]
pub struct ClientBuilder {
    state: BuilderState,
}

impl ClientBuilder {
    shared::builder_setters!();

    /// Builds the client.
    pub fn build(self) -> Result<Client> {
        let config = self.state.into_config()?;
        let http = reqwest::blocking::Client::builder()
            .user_agent(shared::USER_AGENT)
            .default_headers(config.default_headers.clone())
            .timeout(config.timeout)
            .build()
            .map_err(|e| Error::InvalidArgument(format!("could not build the HTTP client: {e}")))?;
        Ok(Client {
            http,
            config: Arc::new(config),
        })
    }
}
