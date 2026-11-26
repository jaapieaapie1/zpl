//! Logging to stdout, and OpenTelemetry export when `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
//!
//! - Log lines go to stdout, filtered by `RUST_LOG` (default: `info` for this server,
//!   `warn` for libraries).
//! - With `OTEL_EXPORTER_OTLP_ENDPOINT` (OTLP over HTTP/protobuf, e.g.
//!   `http://collector:4318`), the same log events and a span per request are exported
//!   as well. The standard `OTEL_*` variables apply: `OTEL_SERVICE_NAME`,
//!   `OTEL_RESOURCE_ATTRIBUTES`, `OTEL_TRACES_SAMPLER` / `OTEL_TRACES_SAMPLER_ARG`,
//!   `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_BSP_*` / `OTEL_BLRP_*` and so on. An incoming
//!   `traceparent` header continues the caller's trace.
//!
//! Export never holds up a request: spans and log records go into bounded queues that
//! background threads send in batches, and when the collector is slow or gone the queues
//! fill up and further telemetry is dropped. Request bodies and API keys are never
//! recorded.

use std::time::Duration;

use axum::extract::{MatchedPath, Request};
use axum::middleware::Next;
use axum::response::Response;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_http::{Bytes, HeaderExtractor, HttpClient, HttpError};
use opentelemetry_otlp::{LogExporter, SpanExporter, WithHttpConfig};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::Resource;
use tracing::field::Empty;
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::filter::{filter_fn, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

use crate::Format;

const DEFAULT_FILTER: &str = "warn,zpl_server=info,zpl_http_api=info";
/// How long shutdown waits for the last telemetry to be sent.
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

/// The OpenTelemetry providers, when exporting; [`Telemetry::shutdown`] sends what is left.
pub struct Telemetry {
    providers: Option<(SdkTracerProvider, SdkLoggerProvider)>,
}

impl Telemetry {
    /// Whether requests are traced; add [`trace_request`] to the router if so.
    pub fn enabled(&self) -> bool {
        self.providers.is_some()
    }

    /// Sends the spans and log records still queued, waiting at most a few seconds.
    pub async fn shutdown(self) {
        let Some((tracer, logger)) = self.providers else { return };
        // Shutting down blocks until the export threads are done, and their requests run
        // on this runtime: keep it off the async workers.
        let _ = tokio::task::spawn_blocking(move || {
            let _ = tracer.shutdown_with_timeout(SHUTDOWN_TIMEOUT);
            let _ = logger.shutdown_with_timeout(SHUTDOWN_TIMEOUT);
        })
        .await;
    }
}

/// Installs logging, and OpenTelemetry export when `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
/// Call once, inside the Tokio runtime. Never fails: when export can't be set up, the
/// server logs why and runs without it.
pub fn init() -> Telemetry {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_FILTER));
    // Events only: formatting every request span's fields would cost for nothing.
    let stdout = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_filter(filter_fn(|metadata| metadata.is_event()));

    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap_or_default();
    let providers = if endpoint.trim().is_empty() {
        None
    } else {
        match providers() {
            Ok(providers) => Some(providers),
            Err(e) => {
                // Not logged yet: there is no subscriber until below.
                eprintln!("OpenTelemetry export is off: {e}");
                None
            }
        }
    };
    let otel_layers = providers.as_ref().map(|(tracer, logger)| {
        let spans = tracing_opentelemetry::layer()
            .with_tracer(tracer.tracer("zpl-server"))
            .with_location(false)
            .with_threads(false)
            .with_tracked_inactivity(false);
        // The exporter's own HTTP client and the SDK log through `tracing` too: exporting
        // those would feed back into the exporter.
        let logs = opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(logger).with_filter(
            filter_fn(|metadata| {
                !["opentelemetry", "hyper", "reqwest", "h2", "tower"]
                    .iter()
                    .any(|prefix| metadata.target().starts_with(prefix))
            }),
        );
        spans.and_then(logs)
    });

    let installed = tracing_subscriber::registry().with(filter).with(stdout).with(otel_layers).try_init();
    if let Err(e) = installed {
        eprintln!("logging was set up already: {e}");
    }
    if providers.is_some() {
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
        tracing::info!("Exporting traces and logs to {}", endpoint.trim());
    }
    Telemetry { providers }
}

fn providers() -> Result<(SdkTracerProvider, SdkLoggerProvider), String> {
    let client = ExportClient {
        http: reqwest::Client::builder().timeout(Duration::from_secs(10)).build().map_err(|e| e.to_string())?,
        runtime: tokio::runtime::Handle::try_current().map_err(|e| e.to_string())?,
    };
    // Service name and attributes from OTEL_SERVICE_NAME and OTEL_RESOURCE_ATTRIBUTES.
    let resource = Resource::builder().build();
    let spans = SpanExporter::builder()
        .with_http()
        .with_http_client(client.clone())
        .build()
        .map_err(|e| format!("trace exporter: {e}"))?;
    // The sampler comes from OTEL_TRACES_SAMPLER / OTEL_TRACES_SAMPLER_ARG, the queue
    // size from OTEL_BSP_MAX_QUEUE_SIZE (default 2048).
    let tracer = SdkTracerProvider::builder().with_batch_exporter(spans).with_resource(resource.clone()).build();
    let logs = LogExporter::builder()
        .with_http()
        .with_http_client(client)
        .build()
        .map_err(|e| format!("log exporter: {e}"))?;
    let logger = SdkLoggerProvider::builder().with_batch_exporter(logs).with_resource(resource).build();
    Ok((tracer, logger))
}

/// Sends OTLP requests with reqwest on the server's runtime. The SDK exports from its own
/// threads, outside any runtime, so each request is spawned onto it.
#[derive(Debug, Clone)]
struct ExportClient {
    http: reqwest::Client,
    runtime: tokio::runtime::Handle,
}

#[async_trait::async_trait]
impl HttpClient for ExportClient {
    async fn send_bytes(&self, request: opentelemetry_http::Request<Bytes>) -> Result<opentelemetry_http::Response<Bytes>, HttpError> {
        let request = reqwest::Request::try_from(request)?;
        let http = self.http.clone();
        self.runtime
            .spawn(async move {
                let response = http.execute(request).await?;
                let mut out = opentelemetry_http::Response::builder().status(response.status());
                if let Some(headers) = out.headers_mut() {
                    *headers = response.headers().clone();
                }
                Ok(out.body(response.bytes().await?)?)
            })
            .await?
    }
}

/// Middleware: a server span per request, named after the route (`POST /render`), with
/// the method, route and status. Health checks are not traced.
pub async fn trace_request(request: Request, next: Next) -> Response {
    if request.uri().path() == "/health" {
        return next.run(request).await;
    }
    let method = request.method().clone();
    let route = request.extensions().get::<MatchedPath>().cloned();
    let name = match &route {
        Some(route) => format!("{method} {}", route.as_str()),
        None => method.to_string(),
    };
    let span = tracing::info_span!(
        "request",
        otel.name = name,
        otel.kind = "server",
        otel.status_code = Empty,
        http.request.method = method.as_str(),
        http.route = route.as_ref().map(MatchedPath::as_str),
        http.response.status_code = Empty,
        zpl.format = Empty,
        zpl.dpmm = Empty,
        zpl.files = Empty,
        zpl.labels = Empty,
        zpl.queue_ms = Empty,
        zpl.render_ms = Empty,
    );
    let parent = opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });
    let _ = span.set_parent(parent);

    let response = next.run(request).instrument(span.clone()).await;
    let status = response.status();
    span.record("http.response.status_code", status.as_u16());
    if status.is_server_error() {
        span.record("otel.status_code", "error");
    }
    response
}

/// Adds what was rendered to the request's span (if it is traced).
pub(crate) fn record_render(format: Format, dpmm: u32, files: usize, labels: usize, queue: Duration, render: Duration) {
    let span = tracing::Span::current();
    if span.is_disabled() {
        return;
    }
    span.record(
        "zpl.format",
        match format {
            Format::Pdf => "pdf",
            Format::Png => "png",
        },
    );
    span.record("zpl.dpmm", dpmm);
    span.record("zpl.files", files);
    span.record("zpl.labels", labels);
    span.record("zpl.queue_ms", queue.as_secs_f64() * 1000.0);
    span.record("zpl.render_ms", render.as_secs_f64() * 1000.0);
}
