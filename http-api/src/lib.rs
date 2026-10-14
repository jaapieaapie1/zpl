//! HTTP API for rendering ZPL labels to PDF or PNG.
//!
//! - `POST /render?format=pdf|png` with the raw ZPL as the request body. `format` defaults
//!   to `pdf`. Responds with the rendered file (`application/pdf` or `image/png`). Every
//!   label (^XA...^XZ, times its ^PQ copies) is a page: a PDF holds them all, a PNG shows
//!   page `?page=N` (1-based, default 1). The `X-Page-Count` header gives the page count.
//!   `?dpmm=6|8|12|24` sets the printer resolution in dots per millimetre (default 8): it
//!   sizes ^MU units, the default 4 x 6 inch label, the PDF page and the PNG in dots.
//! - The same endpoint takes several ZPL files as `multipart/form-data`:
//!   `curl -F file=@a.zpl -F file=@b.zpl 'https://…/render?format=pdf'`. Every part with a
//!   filename, or named `file`, `files` or `zpl`, is a ZPL file; other fields are ignored.
//!   Each file starts from a fresh printer (no ^PW, ^LH, ~DG graphics or ^DF formats carry
//!   over) and their pages follow in upload order, so `page`, `X-Page-Count` and the page
//!   cap count across all files. `X-File-Page-Counts` gives each file's pages (`2, 1`).
//!   A parse error names the file; the whole request is capped at 100 MiB.
//! - Request bodies, raw or multipart, may be compressed with `Content-Encoding: gzip` (or
//!   `deflate`, zlib-wrapped as HTTP has it):
//!   `gzip -c label.zpl | curl --data-binary @- -H 'Content-Encoding: gzip' 'https://…/render?format=pdf'`.
//!   The 100 MiB cap applies to the decompressed body, checked while inflating, so a small
//!   gzip that inflates past it gets 413 without being inflated in full. Corrupt gzip is a
//!   400 and any other encoding a 415, whose `Accept-Encoding` lists the supported ones.
//! - Add `timings` (`?timings` or `?format=png&timings=1`) to get a `Server-Timing` header
//!   with the duration of each stage, which browser devtools display as well. `read`
//!   includes decompressing the body.
//! - With API keys enabled (`ZPL_KEYS_PUBLIC_KEYS`, see [`billing`]), `/render` needs
//!   `Authorization: Bearer <key>` from https://caretly.dev/console. Responses carry
//!   `X-Labels-Billed`: the pages billed for a live key, 0 for a test key, whose labels
//!   get a watermark. Without that setting the API is open, as for local use.
//!   Each key is rate limited ([`rate_limit`]; by default 50 requests a second with
//!   bursts of 200, per replica): over it, `/render` answers 429 with `Retry-After`.
//!   `RateLimit-Limit` and `RateLimit-Remaining` give the burst and what is left of it.
//! - Renders run a few at a time, by default one per available CPU, and a limited number
//!   of requests wait for their turn ([`admission`]): when that line is full, or a request
//!   has waited 5 s, `/render` answers 503 with `Retry-After: 1`. Shed requests are not
//!   billed; with `timings`, their `Server-Timing` names the reason (`shed;desc=…`).
//! - `GET /health` returns `ok`. It never waits for rendering, so it stays fast under load.
//! - `GET /playground/` serves the live ZPL editor (`zpl-wasm/www`), which renders in the
//!   browser with WebAssembly. Files come from `ZPL_PLAYGROUND_DIR`; a `.gz` next to a
//!   file is sent instead when the client accepts gzip.

pub mod admission;
pub mod billing;
pub mod console;
mod log_every;
pub mod rate_limit;
pub mod usage;

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use axum::body::{to_bytes, Body};
use axum::extract::multipart::MultipartError;
use axum::extract::{DefaultBodyLimit, FromRequest, Multipart, Query, Request};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use admission::{Admission, Shed};
use axum::{Extension, Router};
use billing::Billing;
use pdf_renderer::PdfRenderer;
use png_renderer::PngRenderer;
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::map_request_body::MapRequestBodyLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use zpl_parser::Parser;
use zpl_state_manager::{StateManager, DEFAULT_DPMM, DPMM_VALUES, MAX_PAGES};

/// Largest accepted ZPL body, after decompression. A label that is one big ~DG bitmap is
/// ~40-90 KB, so this fits a batch of about a thousand of them.
pub const MAX_BODY_BYTES: usize = 100 * 1024 * 1024;

/// The playground's files: `ZPL_PLAYGROUND_DIR`, or `zpl-wasm/www` in this repository.
pub fn playground_dir() -> PathBuf {
    std::env::var_os("ZPL_PLAYGROUND_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../zpl-wasm/www"))
}

pub fn router() -> Router {
    router_with_playground(&playground_dir())
}

pub fn router_with_playground(dir: &Path) -> Router {
    router_with(dir, None)
}

/// The API with the playground in `dir`, requiring API keys when `billing` is given, with
/// the default render concurrency and queue.
pub fn router_with(dir: &Path, billing: Option<Arc<Billing>>) -> Router {
    router_with_admission(dir, billing, Arc::new(Admission::with_defaults()))
}

/// Like [`router_with`], rendering as `admission` allows.
pub fn router_with_admission(dir: &Path, billing: Option<Arc<Billing>>, admission: Arc<Admission>) -> Router {
    Router::new()
        // The raw body has its own limit; this one is for multipart forms. Both count the
        // decompressed bytes, as the body is inflated before the handler reads it. The
        // chained layers need their (never used) error type spelled out.
        .route(
            "/render",
            post(render)
                .layer::<_, Infallible>(DefaultBodyLimit::max(MAX_BODY_BYTES))
                .layer::<_, Infallible>(MapRequestBodyLayer::new(Body::new))
                .layer::<_, Infallible>(RequestDecompressionLayer::new()),
        )
        .route("/health", get(|| async { "ok" }))
        // The page loads its scripts relative to itself, so it needs the trailing slash.
        .route("/playground", get(|| async { Redirect::permanent("/playground/") }))
        // Revalidate on every load: the page, worker and wasm change together on deploy, and
        // a cached worker from the previous release breaks a fresh page.
        .nest_service(
            "/playground/",
            Router::new().fallback_service(ServeDir::new(dir).precompressed_gzip()).layer(
                SetResponseHeaderLayer::overriding(header::CACHE_CONTROL, HeaderValue::from_static("no-cache")),
            ),
        )
        .layer(Extension(billing))
        .layer(Extension(admission))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Pdf,
    Png,
}

impl Format {
    fn content_type(self) -> &'static str {
        match self {
            Format::Pdf => "application/pdf",
            Format::Png => "image/png",
        }
    }
}

/// A rendered label plus the duration of each rendering stage.
pub struct Rendered {
    pub bytes: Vec<u8>,
    /// Labels in the input; a PDF holds all of them, a PNG one.
    pub page_count: usize,
    /// Pages from each input file, in order (one entry for a single label).
    pub file_page_counts: Vec<usize>,
    pub timings: Vec<(&'static str, Duration)>,
}

#[derive(Debug)]
pub enum RenderError {
    /// `file` names the uploaded file the error is in, if there were several.
    Parse { file: Option<String>, message: String },
    Encode(String),
    /// The requested PNG page (1-based) does not exist.
    NoSuchPage { page: usize, page_count: usize },
}

/// Parses, lays out and renders `zpl` for a `dpmm` dots/mm printer, timing each stage.
/// Synchronous and CPU-bound. A PDF holds every page; a PNG shows `page` (1-based).
pub fn render_label(zpl: &str, format: Format, page: usize, dpmm: u32) -> Result<Rendered, RenderError> {
    render_documents(&[(None, zpl)], format, page, dpmm, None)
}

/// Like [`render_label`] for several `(name, zpl)` files, each on a fresh printer, their
/// pages one after the other. `page` counts across all files, as does the page cap.
pub fn render_files(files: &[(String, String)], format: Format, page: usize, dpmm: u32) -> Result<Rendered, RenderError> {
    let documents: Vec<_> = files.iter().map(|(name, zpl)| (Some(name.as_str()), zpl.as_str())).collect();
    render_documents(&documents, format, page, dpmm, None)
}

/// Text stamped on every page rendered with a test key.
pub const TEST_WATERMARK: &str = "TEST";

/// Renders `documents`, stamping `watermark` on every page when given.
fn render_documents(
    documents: &[(Option<&str>, &str)],
    format: Format,
    page: usize,
    dpmm: u32,
    watermark: Option<&str>,
) -> Result<Rendered, RenderError> {
    let mut timings = Vec::new();
    let mut last = Instant::now();
    let mut mark = |stage: &'static str| {
        let now = Instant::now();
        timings.push((stage, now - last));
        last = now;
    };

    // Parse everything first, so a bad file is reported before any layout work.
    let mut parsed = Vec::with_capacity(documents.len());
    for &(name, zpl) in documents {
        let commands = Parser::with_dpmm(zpl, dpmm).parse().map_err(|e| RenderError::Parse {
            file: name.map(str::to_owned),
            message: format!("{e:?}"),
        })?;
        parsed.push(commands);
    }
    mark("parse");

    // A fresh StateManager per file: one file's printer state must not leak into the next.
    let mut pages = Vec::new();
    let mut file_page_counts = Vec::with_capacity(parsed.len());
    for commands in parsed {
        let remaining = MAX_PAGES - pages.len();
        if remaining == 0 {
            file_page_counts.push(0);
            continue;
        }
        let mut state = StateManager::new();
        state.set_dpmm(dpmm);
        state.set_max_pages(remaining);
        let file_pages = state.process_pages(commands);
        file_page_counts.push(file_pages.len());
        pages.extend(file_pages);
    }
    if let Some(text) = watermark {
        pages.iter_mut().for_each(|p| p.add_watermark(text));
    }
    let page_count = pages.len();
    mark("state");

    let bytes = match format {
        Format::Pdf => {
            let pages: Vec<_> = pages.iter().map(|p| (p.setup, p.instructions.as_slice())).collect();
            let bytes = PdfRenderer::for_page(&pages[0].0).render_pages(&pages);
            mark("render");
            bytes
        }
        Format::Png => {
            let Some(page) = page.checked_sub(1).and_then(|i| pages.get(i)) else {
                return Err(RenderError::NoSuchPage { page, page_count });
            };
            let image = PngRenderer::for_page(&page.setup, false).render(&page.instructions);
            mark("render");
            let bytes = png_renderer::encode_png(&image).map_err(|e| RenderError::Encode(e.to_string()))?;
            mark("encode");
            bytes
        }
    };

    Ok(Rendered { bytes, page_count, file_page_counts, timings })
}

/// Renders a small label touching every font and the QR/barcode/compression code, so the
/// first real request doesn't pay for parsing fonts and paging in code.
pub fn warm_up() {
    const LABEL: &str = "^XA^FO50,50^A0N,40,40^FDwarm up^FS\
                         ^FO50,120^BCN,60,Y,N,N^FD12345^FS\
                         ^FO50,250^BQN,2,4^FDQA,warm up^FS^XZ";
    for format in [Format::Pdf, Format::Png] {
        let _ = render_label(LABEL, format, 1, DEFAULT_DPMM);
    }
}

/// ZPL is normally UTF-8 (^CI28); anything else is taken as a single-byte code page, one
/// character per byte, so legacy Latin-1/CP1252 labels still render.
pub fn zpl_text(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap_or_else(|e| e.into_bytes().into_iter().map(char::from).collect())
}

fn error(status: StatusCode, message: impl Into<String>) -> Response {
    (status, message.into()).into_response()
}

/// What a render request carries: one raw ZPL body, or `(name, zpl)` files from a form.
enum Input {
    Label(String),
    Files(Vec<(String, String)>),
}

impl Input {
    /// Memory budget this input takes ([`admission::RENDER_PERMITS`]): one per started MiB.
    fn permits(&self) -> u32 {
        let bytes = match self {
            Input::Label(zpl) => zpl.len(),
            Input::Files(files) => files.iter().map(|(_, zpl)| zpl.len()).sum(),
        };
        (bytes >> 20) as u32 + 1
    }

    /// BLAKE3 of what gets rendered: the labels (each file's name and contents, length-
    /// prefixed so boundaries count) and the output settings. Makes an `Idempotency-Key`
    /// only deduplicate a true retry: different labels, page or format bill again.
    fn fingerprint(&self, format: Format, page: usize, dpmm: u32) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();
        let mut field = |bytes: &[u8]| {
            hasher.update(&(bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        };
        field(format.content_type().as_bytes());
        field(&(page as u64).to_le_bytes());
        field(&dpmm.to_le_bytes());
        match self {
            Input::Label(zpl) => field(zpl.as_bytes()),
            Input::Files(files) => {
                for (name, zpl) in files {
                    field(name.as_bytes());
                    field(zpl.as_bytes());
                }
            }
        }
        hasher.finalize()
    }
}

fn is_multipart(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.trim_start().to_ascii_lowercase().starts_with("multipart/form-data"))
}

/// The ZPL files in a form, in upload order: every part with a filename, or named `file`,
/// `files` or `zpl`. Other fields are skipped. A file is named after its filename, or else
/// its field name and position (`zpl #2`), for error messages.
async fn read_files(mut multipart: Multipart) -> Result<Vec<(String, String)>, MultipartError> {
    let mut files = Vec::new();
    let mut index = 0;
    while let Some(field) = multipart.next_field().await? {
        index += 1;
        let field_name = field.name().unwrap_or("");
        let name = match field.file_name() {
            Some(file_name) => file_name.to_owned(),
            None if matches!(field_name, "file" | "files" | "zpl") => format!("{field_name} #{index}"),
            None => continue,
        };
        let bytes = field.bytes().await?;
        files.push((name, zpl_text(bytes.into())));
    }
    Ok(files)
}

/// 503 for a request turned away under load. With `timings`, `Server-Timing` gives the
/// stages so far and the reason.
fn overloaded(shed: Shed, stages: Option<Vec<(&'static str, Duration)>>, start: Instant) -> Response {
    let (reason, message) = match shed {
        Shed::QueueFull => ("queue full", "The renderer is busy: too many requests are waiting. Retry after 1 s."),
        Shed::WaitedTooLong => ("waited too long", "The renderer is busy: the request waited too long. Retry after 1 s."),
    };
    let mut response = error(StatusCode::SERVICE_UNAVAILABLE, message);
    response.headers_mut().insert(header::RETRY_AFTER, HeaderValue::from_static("1"));
    if let Some(mut stages) = stages {
        stages.push(("total", start.elapsed()));
        let value = format!("{}, shed;desc=\"{reason}\"", server_timing(&stages));
        response
            .headers_mut()
            .insert("server-timing", HeaderValue::from_str(&value).expect("server-timing is ASCII"));
    }
    response
}

async fn render(
    Extension(billing): Extension<Option<Arc<Billing>>>,
    Extension(admission): Extension<Arc<Admission>>,
    Query(params): Query<HashMap<String, String>>,
    request: Request,
) -> Response {
    let start = Instant::now();
    let timings_enabled = params.get("timings").is_some_and(|v| !matches!(v.as_str(), "0" | "false"));

    // Check the key, the load and the rate limit before reading a possibly large body.
    // Shedding comes before the rate limit, so a 503 does not use up the key's tokens.
    let (caller, limit_headers) = match &billing {
        Some(billing) => {
            let caller = match billing.authenticate(request.headers()) {
                Ok(caller) => caller,
                Err(response) => return response,
            };
            if let Err(shed) = admission.check() {
                return overloaded(shed, timings_enabled.then(Vec::new), start);
            }
            match billing.rate_limit(&caller) {
                Ok(headers) => (Some(caller), headers),
                Err(response) => return response,
            }
        }
        None => {
            if let Err(shed) = admission.check() {
                return overloaded(shed, timings_enabled.then(Vec::new), start);
            }
            (None, HeaderMap::new())
        }
    };
    let watermark = caller.as_ref().filter(|c| c.is_test()).map(|_| TEST_WATERMARK);

    let format = match params.get("format").map(String::as_str) {
        None | Some("pdf") => Format::Pdf,
        Some("png") => Format::Png,
        Some(other) => return error(StatusCode::BAD_REQUEST, format!("unsupported format {other:?}; use pdf or png")),
    };
    let page = match params.get("page").map(|p| p.parse::<usize>()) {
        None => 1,
        Some(Ok(page)) if page >= 1 => page,
        Some(_) => return error(StatusCode::BAD_REQUEST, "page must be a page number, starting at 1"),
    };
    let dpmm = match params.get("dpmm").map(|d| d.parse::<u32>()) {
        None => DEFAULT_DPMM,
        Some(Ok(dpmm)) if DPMM_VALUES.contains(&dpmm) => dpmm,
        Some(_) => return error(StatusCode::BAD_REQUEST, "dpmm must be 6, 8, 12 or 24 (dots per millimetre)"),
    };

    let input = if is_multipart(request.headers()) {
        let multipart = match Multipart::from_request(request, &()).await {
            Ok(multipart) => multipart,
            Err(rejection) => return rejection.into_response(),
        };
        match read_files(multipart).await {
            Ok(files) if files.is_empty() => {
                return error(StatusCode::BAD_REQUEST, "no ZPL files in the form; send them as `file` parts")
            }
            Ok(files) => Input::Files(files),
            Err(e) if e.status() == StatusCode::PAYLOAD_TOO_LARGE => {
                return error(StatusCode::PAYLOAD_TOO_LARGE, format!("body exceeds {MAX_BODY_BYTES} bytes"))
            }
            Err(e) if is_decompress_error(&e) => {
                return error(StatusCode::BAD_REQUEST, format!("invalid compressed body: {}", e.body_text()))
            }
            Err(e) => return error(e.status(), format!("failed to read form: {}", e.body_text())),
        }
    } else {
        match to_bytes(request.into_body(), MAX_BODY_BYTES).await {
            Ok(body) => Input::Label(zpl_text(body.into())),
            Err(e) if is_length_limit(&e) => {
                return error(StatusCode::PAYLOAD_TOO_LARGE, format!("body exceeds {MAX_BODY_BYTES} bytes"))
            }
            Err(e) if is_decompress_error(&e) => {
                return error(StatusCode::BAD_REQUEST, format!("invalid compressed body: {e}"))
            }
            Err(e) => return error(StatusCode::BAD_REQUEST, format!("failed to read body: {e}")),
        }
    };
    let read = start.elapsed();

    // Rendering is CPU-bound (PNG takes milliseconds), so keep it off the async workers,
    // a few at a time. `queue` includes waiting for the memory budget.
    let queued = Instant::now();
    let ticket = match admission.admit(input.permits()).await {
        Ok(ticket) => ticket,
        Err(shed) => return overloaded(shed, timings_enabled.then(|| vec![("read", read), ("queue", queued.elapsed())]), start),
    };
    let queue = queued.elapsed();
    let fingerprint_needed = caller.as_ref().is_some_and(|c| c.idempotency_key.is_some());
    // The ticket goes with the render: if the client hangs up, the render still holds its
    // slot until it is done.
    let result = tokio::task::spawn_blocking(move || {
        let _ticket = ticket;
        // Only requests with an Idempotency-Key need it; microseconds for a typical label.
        let fingerprint = fingerprint_needed.then(|| input.fingerprint(format, page, dpmm));
        let rendered = match &input {
            Input::Label(zpl) => render_documents(&[(None, zpl)], format, page, dpmm, watermark),
            Input::Files(files) => {
                let documents: Vec<_> = files.iter().map(|(name, zpl)| (Some(name.as_str()), zpl.as_str())).collect();
                render_documents(&documents, format, page, dpmm, watermark)
            }
        };
        rendered.map(|rendered| (queue, rendered, matches!(input, Input::Files(_)), fingerprint))
    })
    .await;

    let (queue, rendered, multipart, fingerprint) = match result {
        Ok(Ok(ok)) => ok,
        Ok(Err(RenderError::Parse { file: None, message })) => {
            return error(StatusCode::BAD_REQUEST, format!("parse error: {message}"))
        }
        Ok(Err(RenderError::Parse { file: Some(file), message })) => {
            return error(StatusCode::BAD_REQUEST, format!("parse error in {file}: {message}"))
        }
        Ok(Err(RenderError::Encode(e))) => return error(StatusCode::INTERNAL_SERVER_ERROR, format!("encode error: {e}")),
        Ok(Err(RenderError::NoSuchPage { page, page_count })) => {
            return error(StatusCode::BAD_REQUEST, format!("page {page} out of range; the label has {page_count} page(s)"))
        }
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, format!("render task failed: {e}")),
    };

    let mut response = (
        [(header::CONTENT_TYPE, HeaderValue::from_static(format.content_type()))],
        rendered.bytes,
    )
        .into_response();
    response.headers_mut().insert("x-page-count", HeaderValue::from(rendered.page_count));
    response.headers_mut().extend(limit_headers);
    if let (Some(billing), Some(caller)) = (&billing, &caller) {
        // A PNG response is one page, whatever the input holds.
        let (labels, kind) = match format {
            Format::Pdf => (rendered.page_count, "pdf"),
            Format::Png => (1, "png"),
        };
        let billed = billing.record(caller, labels, kind, fingerprint.as_ref());
        response.headers_mut().insert("x-labels-billed", HeaderValue::from(billed));
    }
    if multipart {
        let counts: Vec<_> = rendered.file_page_counts.iter().map(usize::to_string).collect();
        let value = HeaderValue::from_str(&counts.join(", ")).expect("page counts are ASCII");
        response.headers_mut().insert("x-file-page-counts", value);
    }

    if timings_enabled {
        let mut stages = vec![("read", read), ("queue", queue)];
        stages.extend(rendered.timings);
        stages.push(("total", start.elapsed()));
        let value = server_timing(&stages);
        response
            .headers_mut()
            .insert("server-timing", HeaderValue::from_str(&value).expect("server-timing is ASCII"));
    }

    response
}

fn is_length_limit(e: &axum::Error) -> bool {
    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(e);
    while let Some(err) = source {
        if err.is::<http_body_util::LengthLimitError>() {
            return true;
        }
        source = err.source();
    }
    false
}

/// Whether reading the body failed because it does not decompress. The decompressor
/// reports corrupt or truncated input as an invalid-data or unexpected-EOF I/O error.
fn is_decompress_error(e: &(dyn std::error::Error + 'static)) -> bool {
    let mut source = Some(e);
    while let Some(err) = source {
        if let Some(io) = err.downcast_ref::<std::io::Error>() {
            return matches!(io.kind(), std::io::ErrorKind::InvalidData | std::io::ErrorKind::UnexpectedEof);
        }
        source = err.source();
    }
    false
}

/// `Server-Timing` header value; durations are in milliseconds.
fn server_timing(stages: &[(&str, Duration)]) -> String {
    stages
        .iter()
        .map(|(name, d)| format!("{name};dur={:.3}", d.as_secs_f64() * 1000.0))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tower::ServiceExt;

    const LABEL: &str = "^XA^FO50,50^A0N,40,40^FDhello^FS^FO50,120^BCN,60,Y,N,N^FD12345^FS^XZ";

    async fn post(uri: &str, body: impl Into<Body>) -> Response {
        let request = Request::post(uri).body(body.into()).unwrap();
        router().oneshot(request).await.unwrap()
    }

    async fn body_bytes(response: Response) -> Vec<u8> {
        response.into_body().collect().await.unwrap().to_bytes().to_vec()
    }

    mod keys {
        use super::*;
        use crate::console::tests::{EXPIRED, KEYS, LIVE, TEST};
        use crate::console::{KeyEnv, Revocation, RevocationList, Verifier};
        use crate::rate_limit::RateLimiter;
        use crate::usage::Usage;

        fn billing() -> (Arc<Billing>, Arc<Usage>) {
            let usage = Usage::new();
            (Billing::with_verifier(Verifier::from_config(KEYS).unwrap(), Some(usage.clone())), usage)
        }

        /// The one message for the usage recorded since the last cut.
        fn one_message(usage: &Usage) -> crate::console::UsageAggregate {
            let mut messages = usage.cut();
            assert_eq!(messages.len(), 1, "{messages:?}");
            messages.pop().unwrap()
        }

        async fn send(billing: &Arc<Billing>, uri: &str, key: Option<&str>, extra: &[(&str, &str)]) -> Response {
            let mut request = Request::post(uri);
            if let Some(key) = key {
                request = request.header(header::AUTHORIZATION, format!("Bearer {key}"));
            }
            for (name, value) in extra {
                request = request.header(*name, *value);
            }
            let router = router_with(&playground_dir(), Some(billing.clone()));
            router.oneshot(request.body(Body::from("^XA^FO50,50^A0N,40^FDhi^FS^PQ3^XZ")).unwrap()).await.unwrap()
        }

        async fn text(response: Response) -> String {
            String::from_utf8(body_bytes(response).await).unwrap()
        }

        #[tokio::test]
        async fn requires_a_valid_key() {
            let (billing, _rx) = billing();
            let missing = send(&billing, "/render", None, &[]).await;
            assert_eq!(missing.status(), StatusCode::UNAUTHORIZED);
            assert_eq!(missing.headers()[header::WWW_AUTHENTICATE], "Bearer realm=\"zpl\"");
            assert!(text(missing).await.contains("https://console.test/keys"));
            let bad = send(&billing, "/render", Some("zpl_live_nonsense"), &[]).await;
            assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);
            let expired = send(&billing, "/render", Some(EXPIRED), &[]).await;
            assert!(text(expired).await.contains("expired"));
        }

        #[tokio::test]
        async fn bills_live_pages_and_reports_usage() {
            let (billing, usage) = billing();
            let response = send(&billing, "/render?format=pdf", Some(LIVE), &[("idempotency-key", "order-1")]).await;
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers()["x-labels-billed"], "3");
            let message = one_message(&usage);
            assert_eq!((message.env, message.format, message.labels, message.requests), (KeyEnv::Live, "pdf", 0, 0));
            assert_eq!((message.key_id, message.org_id), (uuid::Uuid::from_u128(0xb), uuid::Uuid::from_u128(0xa)));
            let request = &message.idempotent[0];
            assert_eq!((request.labels, request.idempotency_key.as_str()), (3, "order-1"));
            // A retry with the same Idempotency-Key gets the same request id, in the same
            // window or a later one...
            send(&billing, "/render?format=pdf", Some(LIVE), &[("idempotency-key", "order-1")]).await;
            send(&billing, "/render?format=pdf", Some(LIVE), &[("idempotency-key", "order-1")]).await;
            let retried = one_message(&usage);
            assert_eq!(retried.idempotent.len(), 1, "listed once per window");
            assert_eq!(retried.idempotent[0].request_id, request.request_id);
            assert_ne!(retried.id, message.id);
            // ...but reusing it for another page or format is a new, billed request.
            send(&billing, "/render?format=png&page=2", Some(LIVE), &[("idempotency-key", "order-1")]).await;
            assert_ne!(one_message(&usage).idempotent[0].request_id, request.request_id);
            // A PNG is one page, whatever the label holds; requests without a key add up.
            let png = send(&billing, "/render?format=png", Some(LIVE), &[]).await;
            assert_eq!(png.headers()["x-labels-billed"], "1");
            send(&billing, "/render?format=png", Some(LIVE), &[]).await;
            let pngs = one_message(&usage);
            assert_eq!((pngs.format, pngs.labels, pngs.requests, pngs.idempotent.len()), ("png", 2, 2, 0));
        }

        #[tokio::test]
        async fn test_keys_are_free_and_watermarked() {
            let (billing, usage) = billing();
            let test = send(&billing, "/render?format=png", Some(TEST), &[]).await;
            assert_eq!(test.headers()["x-labels-billed"], "0");
            assert_eq!(one_message(&usage).env, KeyEnv::Test);
            let live = send(&billing, "/render?format=png", Some(LIVE), &[]).await;
            let (test, live) = (body_bytes(test).await, body_bytes(live).await);
            let ink = |png: &[u8]| {
                let image = image::load_from_memory(png).unwrap().to_luma8();
                image.pixels().filter(|p| p.0[0] < 128).count()
            };
            assert!(ink(&test) > ink(&live) + 1000, "watermark adds ink");
        }

        #[tokio::test]
        async fn revoked_keys_and_suspended_accounts() {
            let (billing, _rx) = billing();
            billing.set_revocations(&RevocationList { revoked: vec![], suspended_orgs: vec![uuid::Uuid::from_u128(0xa)] });
            let live = send(&billing, "/render", Some(LIVE), &[]).await;
            assert_eq!(live.status(), StatusCode::PAYMENT_REQUIRED);
            assert!(text(live).await.contains("https://console.test/billing"));
            assert_eq!(send(&billing, "/render", Some(TEST), &[]).await.status(), StatusCode::OK);
            billing.set_revocations(&RevocationList {
                revoked: vec![Revocation { key_id: uuid::Uuid::from_u128(0xb) }],
                suspended_orgs: vec![],
            });
            let revoked = send(&billing, "/render", Some(TEST), &[]).await;
            assert_eq!(revoked.status(), StatusCode::UNAUTHORIZED);
            assert!(text(revoked).await.contains("revoked"));
        }

        #[test]
        fn fingerprint_covers_content_and_settings() {
            let label = Input::Label("^XA^FDa^FS^XZ".into());
            let base = label.fingerprint(Format::Pdf, 1, 8);
            assert_eq!(base, Input::Label("^XA^FDa^FS^XZ".into()).fingerprint(Format::Pdf, 1, 8));
            assert_ne!(base, Input::Label("^XA^FDb^FS^XZ".into()).fingerprint(Format::Pdf, 1, 8));
            assert_ne!(base, label.fingerprint(Format::Png, 1, 8));
            assert_ne!(base, label.fingerprint(Format::Pdf, 2, 8));
            assert_ne!(base, label.fingerprint(Format::Pdf, 1, 12));
            // File boundaries count: "ab" + "c" is not "a" + "bc".
            let files = |a: &str, b: &str| Input::Files(vec![("f".into(), a.into()), ("f".into(), b.into())]);
            assert_ne!(files("ab", "c").fingerprint(Format::Pdf, 1, 8), files("a", "bc").fingerprint(Format::Pdf, 1, 8));
        }

        /// A rate limit on a clock that only moves by `advance(ms)`.
        fn limited(per_second: u32, burst: u32) -> (Arc<Billing>, impl Fn(u64)) {
            let start = Instant::now();
            let elapsed = Arc::new(AtomicUsize::new(0));
            let clock = elapsed.clone();
            let limiter = RateLimiter::new(per_second, burst)
                .unwrap()
                .with_clock(move || start + Duration::from_millis(clock.load(Ordering::Relaxed) as u64));
            let billing = Billing::with_rate_limit(Verifier::from_config(KEYS).unwrap(), None, Some(limiter));
            (billing, move |ms: u64| {
                elapsed.fetch_add(ms as usize, Ordering::Relaxed);
            })
        }

        #[tokio::test]
        async fn rate_limit_burst_then_429() {
            let (billing, _advance) = limited(1, 3);
            for remaining in ["2", "1", "0"] {
                let response = send(&billing, "/render?format=png", Some(LIVE), &[]).await;
                assert_eq!(response.status(), StatusCode::OK);
                assert_eq!(response.headers()["ratelimit-limit"], "3");
                assert_eq!(response.headers()["ratelimit-remaining"], remaining);
            }
            let limited = send(&billing, "/render?format=png", Some(LIVE), &[]).await;
            assert_eq!(limited.status(), StatusCode::TOO_MANY_REQUESTS);
            assert_eq!(limited.headers()[header::RETRY_AFTER], "1");
            assert_eq!(limited.headers()["ratelimit-remaining"], "0");
            assert!(text(limited).await.starts_with("Too many requests for this API key"));
            // The test key has the same key id (0xb), so it shares the bucket.
            assert_eq!(send(&billing, "/render", Some(TEST), &[]).await.status(), StatusCode::TOO_MANY_REQUESTS);
            // A bad key is turned away before it counts against anything.
            assert_eq!(send(&billing, "/render", None, &[]).await.status(), StatusCode::UNAUTHORIZED);
        }

        #[tokio::test]
        async fn rate_limit_refills() {
            // 2 a second: a token is back after 500 ms.
            let (billing, advance) = limited(2, 1);
            assert_eq!(send(&billing, "/render", Some(LIVE), &[]).await.status(), StatusCode::OK);
            advance(100);
            let response = send(&billing, "/render", Some(LIVE), &[]).await;
            assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
            assert_eq!(response.headers()[header::RETRY_AFTER], "1", "rounded up to a whole second");
            advance(400);
            let response = send(&billing, "/render", Some(LIVE), &[]).await;
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers()["ratelimit-remaining"], "0");
        }

        #[test]
        fn rate_limit_buckets() {
            use crate::rate_limit::Decision::{Allowed, Limited};
            let limiter = RateLimiter::new(2, 4).unwrap();
            let (a, b) = (uuid::Uuid::from_u128(1), uuid::Uuid::from_u128(2));
            let start = Instant::now();
            let at = |ms: u64| start + Duration::from_millis(ms);
            for remaining in (0..4).rev() {
                assert_eq!(limiter.check_at(a, at(0)), Allowed { remaining });
            }
            assert_eq!(limiter.check_at(a, at(0)), Limited { retry_after: 1 });
            // Other keys are independent.
            assert_eq!(limiter.check_at(b, at(0)), Allowed { remaining: 3 });
            // 2 a second: one token after 500 ms, none spare.
            assert_eq!(limiter.check_at(a, at(400)), Limited { retry_after: 1 });
            assert_eq!(limiter.check_at(a, at(500)), Allowed { remaining: 0 });
            // Refilling stops at the burst.
            assert_eq!(limiter.check_at(a, at(60_000)), Allowed { remaining: 3 });
            // At 1 a second with nothing left, the wait rounds up to whole seconds.
            let slow = RateLimiter::new(1, 1).unwrap();
            slow.check_at(a, at(0));
            assert_eq!(slow.check_at(a, at(100)), Limited { retry_after: 1 });
            assert_eq!(slow.check_at(a, at(1_000)), Allowed { remaining: 0 });
        }

        #[tokio::test]
        async fn rate_limit_disabled() {
            let billing = Billing::with_rate_limit(Verifier::from_config(KEYS).unwrap(), None, RateLimiter::new(0, 1));
            for _ in 0..5 {
                let response = send(&billing, "/render", Some(LIVE), &[]).await;
                assert_eq!(response.status(), StatusCode::OK);
                assert!(response.headers().get("ratelimit-remaining").is_none());
            }
        }

        #[tokio::test]
        async fn overload_is_shed_unbilled() {
            let (billing, usage) = billing();
            let admission = Arc::new(Admission::new(1, 0, Duration::from_secs(5)));
            let router = router_with_admission(&playground_dir(), Some(billing), admission.clone());
            let request = |uri: &str| {
                Request::post(uri)
                    .header(header::AUTHORIZATION, format!("Bearer {LIVE}"))
                    .body(Body::from(LABEL))
                    .unwrap()
            };
            // The only render slot is taken and nobody may wait: turned away, before the
            // body is read, without usage or a rate limit token.
            let busy = admission.admit(1).await.unwrap();
            let response = router.clone().oneshot(request("/render?timings")).await.unwrap();
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(response.headers()[header::RETRY_AFTER], "1");
            assert!(response.headers().get("x-labels-billed").is_none());
            assert!(response.headers().get("ratelimit-remaining").is_none());
            let timing = response.headers()["server-timing"].to_str().unwrap().to_owned();
            assert!(timing.starts_with("total;dur=") && timing.ends_with("shed;desc=\"queue full\""), "{timing}");
            assert!(text(response).await.starts_with("The renderer is busy"));
            assert!(usage.is_empty(), "a shed request is not billed");

            // Health checks never wait for rendering.
            let health = router.clone().oneshot(Request::get("/health").body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(health.status(), StatusCode::OK);

            drop(busy);
            let response = router.oneshot(request("/render")).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(one_message(&usage).labels, 1);
        }

        #[tokio::test]
        async fn waiting_too_long_is_shed() {
            let admission = Arc::new(Admission::new(1, 4, Duration::from_millis(50)));
            let router = router_with_admission(&playground_dir(), None, admission.clone());
            let _busy = admission.admit(1).await.unwrap();
            let response = router.oneshot(Request::post("/render?timings").body(Body::from(LABEL)).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(response.headers()[header::RETRY_AFTER], "1");
            let timing = response.headers()["server-timing"].to_str().unwrap();
            assert!(timing.starts_with("read;dur=") && timing.contains("queue;dur=") && timing.contains("waited too long"));
        }

        #[tokio::test]
        async fn open_without_keys() {
            assert_eq!(post("/render", LABEL).await.status(), StatusCode::OK);
        }
    }

    #[tokio::test]
    async fn renders_pdf_by_default() {
        let response = post("/render", LABEL).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "application/pdf");
        assert!(response.headers().get("server-timing").is_none());
        assert!(body_bytes(response).await.starts_with(b"%PDF"));
    }

    #[tokio::test]
    async fn renders_png() {
        let response = post("/render?format=png", LABEL).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "image/png");
        assert!(body_bytes(response).await.starts_with(b"\x89PNG"));
    }

    #[tokio::test]
    async fn timings_add_server_timing_header() {
        let response = post("/render?timings", LABEL).await;
        let value = response.headers()["server-timing"].to_str().unwrap().to_owned();
        for stage in ["read;dur=", "queue;dur=", "parse;dur=", "state;dur=", "render;dur=", "total;dur="] {
            assert!(value.contains(stage), "{stage} missing from {value}");
        }
        assert!(!value.contains("encode"));

        let response = post("/render?format=png&timings=1", LABEL).await;
        assert!(response.headers()["server-timing"].to_str().unwrap().contains("encode;dur="));

        let response = post("/render?timings=false", LABEL).await;
        assert!(response.headers().get("server-timing").is_none());
    }

    #[tokio::test]
    async fn rejects_bad_input() {
        assert_eq!(post("/render?format=gif", LABEL).await.status(), StatusCode::BAD_REQUEST);
        let huge = vec![b'A'; MAX_BODY_BYTES + 1];
        assert_eq!(post("/render", huge).await.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    fn page_count(response: &Response) -> &str {
        response.headers()["x-page-count"].to_str().unwrap()
    }

    #[tokio::test]
    async fn pdf_has_a_page_per_label() {
        let response = post("/render", LABEL).await;
        assert_eq!(page_count(&response), "1");

        let response = post("/render", format!("{LABEL}{LABEL}^XA^FO1,1^FDthird^FS^XZ")).await;
        assert_eq!(page_count(&response), "3");
        let pdf = body_bytes(response).await;
        assert_eq!(String::from_utf8_lossy(&pdf).matches("/Type /Page\n").count(), 3);

        let response = post("/render", "^XA^PQ3^FO1,1^SN001,1,Y^FS^XZ").await;
        assert_eq!(page_count(&response), "3");
        assert!(String::from_utf8_lossy(&body_bytes(response).await).contains("/Count 3"));
    }

    #[tokio::test]
    async fn png_page_parameter() {
        let two = "^XA^PW100^LL50^FO1,1^GB10,10,10^FS^XZ^XA^PW200^FO1,1^GB10,10,10^FS^XZ";
        let width = |png: &[u8]| u32::from_be_bytes(png[16..20].try_into().unwrap());

        let response = post("/render?format=png", two).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(page_count(&response), "2");
        assert_eq!(width(&body_bytes(response).await), 100);

        let response = post("/render?format=png&page=2", two).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(width(&body_bytes(response).await), 200);

        for bad in ["3", "0", "x"] {
            let response = post(&format!("/render?format=png&page={bad}"), two).await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST, "page={bad}");
        }
    }

    #[tokio::test]
    async fn dpmm_parameter() {
        let width = |png: &[u8]| u32::from_be_bytes(png[16..20].try_into().unwrap());
        let label = "^XA^FO10,10^GB50,50,5^FS^XZ";

        // The default label is 4 x 6 inches at the requested resolution.
        for (dpmm, dots) in [("", 812), ("&dpmm=6", 608), ("&dpmm=8", 812), ("&dpmm=12", 1200), ("&dpmm=24", 2400)] {
            let response = post(&format!("/render?format=png{dpmm}"), label).await;
            assert_eq!(response.status(), StatusCode::OK, "{dpmm}");
            assert_eq!(width(&body_bytes(response).await), dots, "{dpmm}");
        }

        let response = post("/render?dpmm=12", label).await;
        let pdf = body_bytes(response).await;
        assert!(String::from_utf8_lossy(&pdf).contains("/MediaBox [0 0 288 432]"));

        for bad in ["400", "0", "7", "x", ""] {
            let response = post(&format!("/render?dpmm={bad}"), label).await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST, "dpmm={bad}");
        }
    }

    #[tokio::test]
    async fn serves_playground() {
        let dir = std::env::temp_dir().join(format!("zpl-playground-test-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("pkg")).unwrap();
        std::fs::write(dir.join("index.html"), "<!doctype html>").unwrap();
        std::fs::write(dir.join("pkg/zpl_wasm_bg.wasm"), b"\0asm").unwrap();
        let get = |uri: &str| {
            let request = Request::get(uri).body(Body::empty()).unwrap();
            router_with_playground(&dir).oneshot(request)
        };

        let response = get("/playground").await.unwrap();
        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(response.headers()[header::LOCATION], "/playground/");

        let response = get("/playground/").await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_bytes(response).await, b"<!doctype html>");

        let response = get("/playground/pkg/zpl_wasm_bg.wasm").await.unwrap();
        assert_eq!(response.headers()[header::CONTENT_TYPE], "application/wasm");
        // Revalidated, so a deploy never mixes a cached worker with a new page.
        assert_eq!(response.headers()[header::CACHE_CONTROL], "no-cache");

        assert_eq!(get("/playground/missing.js").await.unwrap().status(), StatusCode::NOT_FOUND);
        assert_eq!(get("/playground/../Cargo.toml").await.unwrap().status(), StatusCode::NOT_FOUND);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    const BOUNDARY: &str = "zpl-test-boundary";

    /// A multipart/form-data body of `(field name, filename, contents)` parts.
    fn form(parts: &[(&str, Option<&str>, &[u8])]) -> Vec<u8> {
        let mut body = Vec::new();
        for (name, file_name, contents) in parts {
            body.extend(format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"{name}\"").bytes());
            if let Some(file_name) = file_name {
                body.extend(format!("; filename=\"{file_name}\"").bytes());
            }
            body.extend(b"\r\n\r\n");
            body.extend(*contents);
            body.extend(b"\r\n");
        }
        body.extend(format!("--{BOUNDARY}--\r\n").bytes());
        body
    }

    async fn post_form(uri: &str, parts: &[(&str, Option<&str>, &[u8])]) -> Response {
        let request = Request::post(uri)
            .header(header::CONTENT_TYPE, format!("multipart/form-data; boundary={BOUNDARY}"))
            .body(Body::from(form(parts)))
            .unwrap();
        router().oneshot(request).await.unwrap()
    }

    #[tokio::test]
    async fn multipart_files_follow_each_other() {
        let two = format!("{LABEL}{LABEL}");
        let parts: &[(&str, Option<&str>, &[u8])] = &[
            ("file", Some("a.zpl"), two.as_bytes()),
            ("note", None, b"not a label"),
            ("file", Some("b.zpl"), LABEL.as_bytes()),
        ];
        let response = post_form("/render?format=pdf", parts).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[header::CONTENT_TYPE], "application/pdf");
        assert_eq!(page_count(&response), "3");
        assert_eq!(response.headers()["x-file-page-counts"], "2, 1");
        let pdf = body_bytes(response).await;
        assert_eq!(String::from_utf8_lossy(&pdf).matches("/Type /Page\n").count(), 3);

        // Without a filename the field name decides; non-UTF-8 is Latin-1 as for a raw body.
        let parts: &[(&str, Option<&str>, &[u8])] = &[("zpl", None, LABEL.as_bytes()), ("files", None, b"^XA^FD\xe9^FS^XZ")];
        let response = post_form("/render", parts).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(page_count(&response), "2");

        // A raw body is one file and gets no per-file header.
        assert!(post("/render", LABEL).await.headers().get("x-file-page-counts").is_none());
    }

    #[tokio::test]
    async fn multipart_files_get_a_fresh_printer() {
        // File 1 stores a graphic and sets a 400 dot width; file 2 relies on neither.
        let first = "~DGR:BOX.GRF,8,1,FFFFFFFFFFFFFFFF^XA^PW400^FO0,0^XGR:BOX.GRF,4,4^FS^XZ";
        let second = "^XA^FO0,0^XGR:BOX.GRF,4,4^FS^XZ";
        let width = |png: &[u8]| u32::from_be_bytes(png[16..20].try_into().unwrap());
        let parts: &[(&str, Option<&str>, &[u8])] =
            &[("file", Some("first.zpl"), first.as_bytes()), ("file", Some("second.zpl"), second.as_bytes())];

        let response = post_form("/render?format=png", parts).await;
        assert_eq!(width(&body_bytes(response).await), 400);
        let response = post_form("/render?format=png&page=2", parts).await;
        assert_eq!(response.status(), StatusCode::OK);
        let isolated = body_bytes(response).await;
        assert_eq!(width(&isolated), 812);

        // Page 2 is file 2 on its own, without file 1's graphic...
        let alone = body_bytes(post("/render?format=png", second).await).await;
        assert_eq!(isolated, alone);
        // ...which a single body sharing one printer does draw.
        let shared = body_bytes(post("/render?format=png&page=2", format!("{first}{second}")).await).await;
        assert_ne!(isolated, shared);
    }

    #[tokio::test]
    async fn multipart_png_page_counts_across_files() {
        let a = "^XA^PW100^FO1,1^GB10,10,10^FS^XZ";
        let b = "^XA^PW200^FO1,1^GB10,10,10^FS^XZ^XA^PW300^FO1,1^GB10,10,10^FS^XZ";
        let parts: &[(&str, Option<&str>, &[u8])] = &[("file", Some("a.zpl"), a.as_bytes()), ("file", Some("b.zpl"), b.as_bytes())];
        let width = |png: &[u8]| u32::from_be_bytes(png[16..20].try_into().unwrap());

        for (page, dots) in [(1, 100), (2, 200), (3, 300)] {
            let response = post_form(&format!("/render?format=png&page={page}"), parts).await;
            assert_eq!(response.status(), StatusCode::OK, "page={page}");
            assert_eq!(page_count(&response), "3");
            assert_eq!(width(&body_bytes(response).await), dots, "page={page}");
        }
        let response = post_form("/render?format=png&page=4", parts).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn multipart_page_cap_spans_files() {
        let many = "^XA^PQ999^FO1,1^FDa^FS^XZ";
        let parts: &[(&str, Option<&str>, &[u8])] = &[
            ("file", Some("a.zpl"), many.as_bytes()),
            ("file", Some("b.zpl"), many.as_bytes()),
            ("file", Some("c.zpl"), many.as_bytes()),
        ];
        let response = post_form("/render", parts).await;
        assert_eq!(page_count(&response), MAX_PAGES.to_string());
        assert_eq!(response.headers()["x-file-page-counts"], "999, 1, 0");
    }

    #[tokio::test]
    async fn multipart_rejects_bad_input() {
        let response = post_form("/render", &[("note", None, b"hello")]).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(String::from_utf8_lossy(&body_bytes(response).await).contains("no ZPL files"));
        assert_eq!(post_form("/render", &[]).await.status(), StatusCode::BAD_REQUEST);

        // The parser keeps malformed commands as unknown ones rather than failing, so a
        // "parse error in <file>" cannot be provoked here.

        let huge = vec![b'A'; MAX_BODY_BYTES + 1];
        let response = post_form("/render", &[("file", Some("huge.zpl"), &huge)]).await;
        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);

        // A multipart content type without a boundary.
        let request = Request::post("/render").header(header::CONTENT_TYPE, "multipart/form-data").body(Body::from(LABEL)).unwrap();
        assert_eq!(router().oneshot(request).await.unwrap().status(), StatusCode::BAD_REQUEST);
    }

    fn gzip(data: &[u8]) -> Vec<u8> {
        use std::io::Write;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    async fn post_encoded(uri: &str, encoding: &str, content_type: Option<&str>, body: impl Into<Body>) -> Response {
        let mut request = Request::post(uri).header(header::CONTENT_ENCODING, encoding);
        if let Some(content_type) = content_type {
            request = request.header(header::CONTENT_TYPE, content_type);
        }
        router().oneshot(request.body(body.into()).unwrap()).await.unwrap()
    }

    #[tokio::test]
    async fn gzip_body_renders_like_plain() {
        for uri in ["/render?format=png", "/render?format=pdf"] {
            let plain = post(uri, LABEL).await;
            let gzipped = post_encoded(uri, "gzip", None, gzip(LABEL.as_bytes())).await;
            assert_eq!(gzipped.status(), StatusCode::OK, "{uri}");
            assert_eq!(page_count(&gzipped), page_count(&plain));
            assert_eq!(body_bytes(gzipped).await, body_bytes(plain).await, "{uri}");
        }
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        std::io::Write::write_all(&mut encoder, LABEL.as_bytes()).unwrap();
        let deflated = post_encoded("/render?format=png", "deflate", None, encoder.finish().unwrap()).await;
        assert_eq!(deflated.status(), StatusCode::OK);
        assert_eq!(body_bytes(deflated).await, body_bytes(post("/render?format=png", LABEL).await).await);

        // `identity` is the same as no encoding.
        assert_eq!(post_encoded("/render", "identity", None, LABEL).await.status(), StatusCode::OK);

        // `read` covers inflating the body.
        let response = post_encoded("/render?timings", "gzip", None, gzip(LABEL.as_bytes())).await;
        assert!(response.headers()["server-timing"].to_str().unwrap().starts_with("read;dur="));
    }

    #[tokio::test]
    async fn gzip_multipart_body() {
        let parts: &[(&str, Option<&str>, &[u8])] = &[("file", Some("a.zpl"), LABEL.as_bytes()), ("file", Some("b.zpl"), LABEL.as_bytes())];
        let content_type = format!("multipart/form-data; boundary={BOUNDARY}");
        let response = post_encoded("/render", "gzip", Some(&content_type), gzip(&form(parts))).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["x-file-page-counts"], "1, 1");
    }

    /// An endless gzip stream: `prefix`, then zeros forever. Each piece is a raw deflate
    /// run ended by a full flush, so the pieces concatenate into one valid stream. Counts
    /// the compressed bytes the server pulls into `pulled`.
    fn endless_gzip(prefix: &[u8], pulled: Arc<AtomicUsize>) -> Body {
        let deflate = |data: &[u8]| {
            let mut compress = flate2::Compress::new(flate2::Compression::best(), false);
            let mut out = Vec::with_capacity(data.len() + 1024);
            compress.compress_vec(data, &mut out, flate2::FlushCompress::Full).unwrap();
            out
        };
        let header = [&[0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 0, 0xff][..], &deflate(prefix)].concat();
        let zeros = deflate(&[0; 1 << 20]);
        let chunks = std::iter::once(header).chain(std::iter::repeat(zeros)).map(move |chunk| {
            pulled.fetch_add(chunk.len(), Ordering::Relaxed);
            Ok::<_, Infallible>(chunk)
        });
        Body::from_stream(futures_util::stream::iter(chunks))
    }

    #[tokio::test]
    async fn gzip_limit_counts_decompressed_bytes() {
        // A 1 MiB run of zeros deflates to about 1 KiB, so 100 MiB is about 100 KiB on the wire.
        let start = Instant::now();
        let pulled = Arc::new(AtomicUsize::new(0));
        let response = post_encoded("/render", "gzip", None, endless_gzip(b"^XA", pulled.clone())).await;
        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
        let pulled = pulled.load(Ordering::Relaxed);
        assert!(pulled < 256 * 1024, "read {pulled} compressed bytes");

        let pulled = Arc::new(AtomicUsize::new(0));
        let head = format!("--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"bomb.zpl\"\r\n\r\n");
        let content_type = format!("multipart/form-data; boundary={BOUNDARY}");
        let response = post_encoded("/render", "gzip", Some(&content_type), endless_gzip(head.as_bytes(), pulled.clone())).await;
        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
        let pulled = pulled.load(Ordering::Relaxed);
        assert!(pulled < 256 * 1024, "read {pulled} compressed bytes");
        assert!(start.elapsed() < Duration::from_secs(5), "took {:?}", start.elapsed());

        // Just under the limit is fine, though it compresses to almost nothing.
        let mut label = b"^XA^FO1,1^FDok^FS^XZ".to_vec();
        label.resize(MAX_BODY_BYTES, b' ');
        assert_eq!(post_encoded("/render", "gzip", None, gzip(&label)).await.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn rejects_bad_encodings() {
        let response = post_encoded("/render", "gzip", None, LABEL).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let message = String::from_utf8(body_bytes(response).await).unwrap();
        assert!(message.starts_with("invalid compressed body"), "{message}");

        let compressed = gzip(LABEL.as_bytes());
        let truncated = compressed[..compressed.len() - 10].to_vec();
        let response = post_encoded("/render", "gzip", None, truncated).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let message = String::from_utf8(body_bytes(response).await).unwrap();
        assert!(message.starts_with("invalid compressed body"), "{message}");

        let content_type = format!("multipart/form-data; boundary={BOUNDARY}");
        let response = post_encoded("/render", "gzip", Some(&content_type), form(&[("file", Some("a.zpl"), LABEL.as_bytes())])).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let message = String::from_utf8(body_bytes(response).await).unwrap();
        assert!(message.contains("invalid compressed body"), "{message}");

        for encoding in ["compress", "x-gzip", "gzip, gzip"] {
            let response = post_encoded("/render", encoding, None, gzip(LABEL.as_bytes())).await;
            assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE, "{encoding}");
            assert_eq!(response.headers()[header::ACCEPT_ENCODING], "gzip,deflate");
        }
    }

    #[test]
    fn non_utf8_is_latin1() {
        assert_eq!(zpl_text(b"^FD\xe9^FS".to_vec()), "^FD\u{e9}^FS");
        assert_eq!(zpl_text("é".as_bytes().to_vec()), "é");
    }

    #[test]
    fn render_permits_follow_input_size() {
        assert_eq!(Input::Label(LABEL.to_string()).permits(), 1);
        let files = vec![("a.zpl".to_string(), "x".repeat(3 << 20)), ("b.zpl".to_string(), "x".repeat(1 << 19))];
        assert_eq!(Input::Files(files).permits(), 4);
        assert_eq!(admission::RENDER_PERMITS, 100);
    }

    #[test]
    fn formats_server_timing() {
        let value = server_timing(&[("parse", Duration::from_micros(138)), ("render", Duration::from_micros(1500))]);
        assert_eq!(value, "parse;dur=0.138, render;dur=1.500");
    }
}
