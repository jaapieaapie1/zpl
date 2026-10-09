//! HTTP API for rendering ZPL labels to PDF or PNG.
//!
//! - `POST /render?format=pdf|png` with the raw ZPL as the request body. `format` defaults
//!   to `pdf`. Responds with the rendered file (`application/pdf` or `image/png`).
//! - Add `timings` (`?timings` or `?format=png&timings=1`) to get a `Server-Timing` header
//!   with the duration of each stage, which browser devtools display as well.
//! - `GET /health` returns `ok`.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use axum::body::{to_bytes, Body};
use axum::extract::Query;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use pdf_renderer::PdfRenderer;
use png_renderer::PngRenderer;
use zpl_parser::Parser;
use zpl_state_manager::StateManager;

/// Largest accepted ZPL body. Labels with big ^GF graphics run to a few megabytes.
pub const MAX_BODY_BYTES: usize = 16 * 1024 * 1024;

pub fn router() -> Router {
    Router::new()
        .route("/render", post(render))
        .route("/health", get(|| async { "ok" }))
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
    pub timings: Vec<(&'static str, Duration)>,
}

#[derive(Debug)]
pub enum RenderError {
    Parse(String),
    Encode(String),
}

/// Parses, lays out and renders `zpl`, timing each stage. Synchronous and CPU-bound.
pub fn render_label(zpl: &str, format: Format) -> Result<Rendered, RenderError> {
    let mut timings = Vec::new();
    let mut last = Instant::now();
    let mut mark = |stage: &'static str| {
        let now = Instant::now();
        timings.push((stage, now - last));
        last = now;
    };

    let commands = Parser::new(zpl).parse().map_err(|e| RenderError::Parse(format!("{e:?}")))?;
    mark("parse");

    let mut state = StateManager::new();
    let instructions = state.process(commands);
    let (width, height) = state.get_label_dimensions();
    mark("state");

    let bytes = match format {
        Format::Pdf => {
            let bytes = PdfRenderer::new(width, height).render(&instructions);
            mark("render");
            bytes
        }
        Format::Png => {
            let image = PngRenderer::new(width, height).render(&instructions);
            mark("render");
            let bytes = png_renderer::encode_png(&image).map_err(|e| RenderError::Encode(e.to_string()))?;
            mark("encode");
            bytes
        }
    };

    Ok(Rendered { bytes, timings })
}

/// Renders a small label touching every font and the QR/barcode/compression code, so the
/// first real request doesn't pay for parsing fonts and paging in code.
pub fn warm_up() {
    const LABEL: &str = "^XA^FO50,50^A0N,40,40^FDwarm up^FS\
                         ^FO50,120^BCN,60,Y,N,N^FD12345^FS\
                         ^FO50,250^BQN,2,4^FDQA,warm up^FS^XZ";
    for format in [Format::Pdf, Format::Png] {
        let _ = render_label(LABEL, format);
    }
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
    let timings_enabled = params.get("timings").is_some_and(|v| !matches!(v.as_str(), "0" | "false"));

    let body = match to_bytes(body, MAX_BODY_BYTES).await {
        Ok(body) => body,
        Err(e) if is_length_limit(&e) => {
            return error(StatusCode::PAYLOAD_TOO_LARGE, format!("body exceeds {MAX_BODY_BYTES} bytes"))
        }
        Err(e) => return error(StatusCode::BAD_REQUEST, format!("failed to read body: {e}")),
    };
    let read = start.elapsed();

    let zpl = match String::from_utf8(body.into()) {
        Ok(zpl) => zpl,
        Err(_) => return error(StatusCode::BAD_REQUEST, "body must be UTF-8 ZPL"),
    };

    // Rendering is CPU-bound (PNG takes milliseconds), so keep it off the async workers.
    let queued = Instant::now();
    let result = tokio::task::spawn_blocking(move || {
        let queue = queued.elapsed();
        render_label(&zpl, format).map(|rendered| (queue, rendered))
    })
    .await;

    let (queue, rendered) = match result {
        Ok(Ok(ok)) => ok,
        Ok(Err(RenderError::Parse(e))) => return error(StatusCode::BAD_REQUEST, format!("parse error: {e}")),
        Ok(Err(RenderError::Encode(e))) => return error(StatusCode::INTERNAL_SERVER_ERROR, format!("encode error: {e}")),
        Err(e) => return error(StatusCode::INTERNAL_SERVER_ERROR, format!("render task failed: {e}")),
    };

    let mut response = (
        [(header::CONTENT_TYPE, HeaderValue::from_static(format.content_type()))],
        rendered.bytes,
    )
        .into_response();

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
        assert_eq!(post("/render", vec![0xff, 0xfe]).await.status(), StatusCode::BAD_REQUEST);
        let huge = vec![b'A'; MAX_BODY_BYTES + 1];
        assert_eq!(post("/render", huge).await.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[test]
    fn formats_server_timing() {
        let value = server_timing(&[("parse", Duration::from_micros(138)), ("render", Duration::from_micros(1500))]);
        assert_eq!(value, "parse;dur=0.138, render;dur=1.500");
    }
}
