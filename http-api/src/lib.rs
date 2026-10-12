//! HTTP API for rendering ZPL labels to PDF or PNG.
//!
//! - `POST /render?format=pdf|png` with the raw ZPL as the request body. `format` defaults
//!   to `pdf`. Responds with the rendered file (`application/pdf` or `image/png`). Every
//!   label (^XA...^XZ, times its ^PQ copies) is a page: a PDF holds them all, a PNG shows
//!   page `?page=N` (1-based, default 1). The `X-Page-Count` header gives the page count.
//!   `?dpmm=6|8|12|24` sets the printer resolution in dots per millimetre (default 8): it
//!   sizes ^MU units, the default 4 x 6 inch label, the PDF page and the PNG in dots.
//! - Add `timings` (`?timings` or `?format=png&timings=1`) to get a `Server-Timing` header
//!   with the duration of each stage, which browser devtools display as well.
//! - `GET /health` returns `ok`.
//! - `GET /playground/` serves the live ZPL editor (`zpl-wasm/www`), which renders in the
//!   browser with WebAssembly. Files come from `ZPL_PLAYGROUND_DIR`; a `.gz` next to a
//!   file is sent instead when the client accepts gzip.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use axum::body::{to_bytes, Body};
use axum::extract::Query;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::Router;
use pdf_renderer::PdfRenderer;
use png_renderer::PngRenderer;
use tower_http::services::ServeDir;
use zpl_parser::Parser;
use zpl_state_manager::{StateManager, DEFAULT_DPMM, DPMM_VALUES};

/// Largest accepted ZPL body. Labels with big ^GF graphics run to a few megabytes.
pub const MAX_BODY_BYTES: usize = 16 * 1024 * 1024;

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
    Router::new()
        .route("/render", post(render))
        .route("/health", get(|| async { "ok" }))
        // The page loads its scripts relative to itself, so it needs the trailing slash.
        .route("/playground", get(|| async { Redirect::permanent("/playground/") }))
        .nest_service("/playground/", ServeDir::new(dir).precompressed_gzip())
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
    pub timings: Vec<(&'static str, Duration)>,
}

#[derive(Debug)]
pub enum RenderError {
    Parse(String),
    Encode(String),
    /// The requested PNG page (1-based) does not exist.
    NoSuchPage { page: usize, page_count: usize },
}

/// Parses, lays out and renders `zpl` for a `dpmm` dots/mm printer, timing each stage.
/// Synchronous and CPU-bound. A PDF holds every page; a PNG shows `page` (1-based).
pub fn render_label(zpl: &str, format: Format, page: usize, dpmm: u32) -> Result<Rendered, RenderError> {
    let mut timings = Vec::new();
    let mut last = Instant::now();
    let mut mark = |stage: &'static str| {
        let now = Instant::now();
        timings.push((stage, now - last));
        last = now;
    };

    let commands = Parser::with_dpmm(zpl, dpmm).parse().map_err(|e| RenderError::Parse(format!("{e:?}")))?;
    mark("parse");

    let mut state = StateManager::new();
    state.set_dpmm(dpmm);
    let pages = state.process_pages(commands);
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

    Ok(Rendered { bytes, page_count, timings })
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

async fn render(Query(params): Query<HashMap<String, String>>, body: Body) -> Response {
    let start = Instant::now();

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
    let timings_enabled = params.get("timings").is_some_and(|v| !matches!(v.as_str(), "0" | "false"));

    let body = match to_bytes(body, MAX_BODY_BYTES).await {
        Ok(body) => body,
        Err(e) if is_length_limit(&e) => {
            return error(StatusCode::PAYLOAD_TOO_LARGE, format!("body exceeds {MAX_BODY_BYTES} bytes"))
        }
        Err(e) => return error(StatusCode::BAD_REQUEST, format!("failed to read body: {e}")),
    };
    let read = start.elapsed();

    let zpl = zpl_text(body.into());

    // Rendering is CPU-bound (PNG takes milliseconds), so keep it off the async workers.
    let queued = Instant::now();
    let result = tokio::task::spawn_blocking(move || {
        let queue = queued.elapsed();
        render_label(&zpl, format, page, dpmm).map(|rendered| (queue, rendered))
    })
    .await;

    let (queue, rendered) = match result {
        Ok(Ok(ok)) => ok,
        Ok(Err(RenderError::Parse(e))) => return error(StatusCode::BAD_REQUEST, format!("parse error: {e}")),
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
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    const LABEL: &str = "^XA^FO50,50^A0N,40,40^FDhello^FS^FO50,120^BCN,60,Y,N,N^FD12345^FS^XZ";

    async fn post(uri: &str, body: impl Into<Body>) -> Response {
        let request = Request::post(uri).body(body.into()).unwrap();
        router().oneshot(request).await.unwrap()
    }

    async fn body_bytes(response: Response) -> Vec<u8> {
        response.into_body().collect().await.unwrap().to_bytes().to_vec()
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

        assert_eq!(get("/playground/missing.js").await.unwrap().status(), StatusCode::NOT_FOUND);
        assert_eq!(get("/playground/../Cargo.toml").await.unwrap().status(), StatusCode::NOT_FOUND);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn non_utf8_is_latin1() {
        assert_eq!(zpl_text(b"^FD\xe9^FS".to_vec()), "^FD\u{e9}^FS");
        assert_eq!(zpl_text("é".as_bytes().to_vec()), "é");
    }

    #[test]
    fn formats_server_timing() {
        let value = server_timing(&[("parse", Duration::from_micros(138)), ("render", Duration::from_micros(1500))]);
        assert_eq!(value, "parse;dur=0.138, render;dur=1.500");
    }
}
