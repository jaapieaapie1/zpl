//! gRPC API, `zpl.v1.Renderer` (`proto/zpl/v1/renderer.proto`): the same rendering, API
//! keys, rate limits, usage and load shedding as `POST /render`, sharing one [`Billing`]
//! and one [`Admission`] with the HTTP server. For clients that keep a connection open and
//! send many requests over it.
//!
//! Served in plain text (h2c) on `ZPL_GRPC_ADDR`; TLS is terminated in front of it.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::HeaderMap;
use tonic::codec::CompressionEncoding;
use tonic::metadata::MetadataMap;
use tonic::transport::server::TcpIncoming;
use tonic::{Code, Request, Response, Status};
use zpl_state_manager::{DEFAULT_DPMM, DPMM_VALUES};

use crate::admission::{Admission, Shed};
use crate::billing::{Billing, Denied};
use crate::{run_render, zpl_text, Failed, Format, Input, RenderError, MAX_BODY_BYTES, TEST_WATERMARK};

pub mod proto {
    tonic::include_proto!("zpl.v1");
}

use proto::renderer_server::{Renderer, RendererServer};
use proto::{RenderRequest, RenderResponse};

pub struct RendererService {
    billing: Option<Arc<Billing>>,
    admission: Arc<Admission>,
}

/// The `zpl.v1.Renderer` service, requiring API keys when `billing` is given.
pub fn service(billing: Option<Arc<Billing>>, admission: Arc<Admission>) -> RendererServer<RendererService> {
    RendererServer::new(RendererService { billing, admission })
        .max_decoding_message_size(MAX_BODY_BYTES)
        .max_encoding_message_size(usize::MAX)
        .accept_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Zstd)
}

/// Serves [`service`] on `listener` until `shutdown` resolves, then lets open calls finish.
pub async fn serve(
    listener: tokio::net::TcpListener,
    billing: Option<Arc<Billing>>,
    admission: Arc<Admission>,
    shutdown: impl Future<Output = ()>,
) -> Result<(), tonic::transport::Error> {
    // Pings keep idle connections alive through the proxy and find dead ones.
    tonic::transport::Server::builder()
        .tcp_nodelay(true)
        .http2_keepalive_interval(Some(Duration::from_secs(30)))
        .http2_keepalive_timeout(Some(Duration::from_secs(10)))
        .add_service(service(billing, admission))
        .serve_with_incoming_shutdown(TcpIncoming::from(listener).with_nodelay(Some(true)), shutdown)
        .await
}

/// `ZPL_GRPC_ADDR`, when set: where to serve gRPC.
pub fn addr_from_env() -> Result<Option<SocketAddr>, String> {
    match std::env::var("ZPL_GRPC_ADDR").ok().filter(|v| !v.trim().is_empty()) {
        None => Ok(None),
        Some(v) => v.trim().parse().map(Some).map_err(|e| format!("ZPL_GRPC_ADDR: {e} ({v:?})")),
    }
}

fn metadata(headers: HeaderMap) -> MetadataMap {
    MetadataMap::from_headers(headers)
}

fn denied(denied: Denied) -> Status {
    match denied {
        Denied::Unauthorized(message) => Status::unauthenticated(message),
        Denied::PaymentRequired(message) => Status::permission_denied(message),
        Denied::RateLimited { message, retry_after, limit } => {
            let mut headers = crate::billing::Quota { limit, remaining: 0 }.headers();
            headers.insert("retry-after", retry_after.into());
            Status::with_metadata(Code::ResourceExhausted, message, metadata(headers))
        }
    }
}

fn overloaded(shed: Shed) -> Status {
    let message = match shed {
        Shed::QueueFull => "The renderer is busy: too many requests are waiting. Retry after 1 s.",
        Shed::WaitedTooLong => "The renderer is busy: the request waited too long. Retry after 1 s.",
    };
    let mut headers = HeaderMap::new();
    headers.insert("retry-after", 1.into());
    Status::with_metadata(Code::Unavailable, message, metadata(headers))
}

/// The settings and input of `request`, checked as `POST /render` checks its query.
fn read_request(request: RenderRequest) -> Result<(Input, Format, usize, u32), Status> {
    let format = match proto::Format::try_from(request.format) {
        Ok(proto::Format::Unspecified | proto::Format::Pdf) => Format::Pdf,
        Ok(proto::Format::Png) => Format::Png,
        Err(_) => return Err(Status::invalid_argument(format!("unsupported format {}; use PDF or PNG", request.format))),
    };
    let page = request.page.max(1) as usize;
    let dpmm = match request.dpmm {
        0 => DEFAULT_DPMM,
        dpmm if DPMM_VALUES.contains(&dpmm) => dpmm,
        _ => return Err(Status::invalid_argument("dpmm must be 6, 8, 12 or 24 (dots per millimetre)")),
    };
    let input = match (request.zpl.is_empty(), request.files.is_empty()) {
        (_, true) => Input::Label(zpl_text(request.zpl)),
        (true, false) => Input::Files(
            request
                .files
                .into_iter()
                .enumerate()
                .map(|(i, file)| {
                    let name = if file.name.is_empty() { format!("file #{}", i + 1) } else { file.name };
                    (name, zpl_text(file.zpl))
                })
                .collect(),
        ),
        (false, false) => return Err(Status::invalid_argument("set either zpl or files, not both")),
    };
    Ok((input, format, page, dpmm))
}

#[tonic::async_trait]
impl Renderer for RendererService {
    async fn render(&self, request: Request<RenderRequest>) -> Result<Response<RenderResponse>, Status> {
        let (metadata_in, _, request) = request.into_parts();
        let headers = metadata_in.into_headers();

        // Same order as HTTP: shedding before the rate limit, so UNAVAILABLE costs no token.
        let (caller, quota) = match &self.billing {
            Some(billing) => {
                let caller = billing.check_key(&headers).map_err(denied)?;
                self.admission.check().map_err(overloaded)?;
                let quota = billing.take_token(&caller).map_err(denied)?;
                (Some(caller), quota)
            }
            None => {
                self.admission.check().map_err(overloaded)?;
                (None, None)
            }
        };
        let watermark = caller.as_ref().filter(|c| c.is_test()).map(|_| TEST_WATERMARK);

        let (input, format, page, dpmm) = read_request(request)?;
        let fingerprint_needed = caller.as_ref().is_some_and(|c| c.idempotency_key.is_some());
        let (_, rendered, fingerprint) =
            match run_render(&self.admission, input, format, page, dpmm, watermark, fingerprint_needed).await {
                Ok(ok) => ok,
                Err(Failed::Shed { shed, .. }) => return Err(overloaded(shed)),
                Err(Failed::Render(RenderError::Parse { file: None, message })) => {
                    return Err(Status::invalid_argument(format!("parse error: {message}")))
                }
                Err(Failed::Render(RenderError::Parse { file: Some(file), message })) => {
                    return Err(Status::invalid_argument(format!("parse error in {file}: {message}")))
                }
                Err(Failed::Render(RenderError::NoSuchPage { page, page_count })) => {
                    return Err(Status::invalid_argument(format!(
                        "page {page} out of range; the label has {page_count} page(s)"
                    )))
                }
                Err(Failed::Render(RenderError::Encode(e))) => return Err(Status::internal(format!("encode error: {e}"))),
                Err(Failed::Task(e)) => return Err(Status::internal(format!("render task failed: {e}"))),
            };

        let labels_billed = match (&self.billing, &caller) {
            (Some(billing), Some(caller)) => {
                billing.record(caller, format.billed_labels(rendered.page_count), format.usage_kind(), fingerprint.as_ref())
            }
            _ => 0,
        };
        let mut response = Response::new(RenderResponse {
            data: rendered.bytes,
            content_type: format.content_type().to_owned(),
            page_count: rendered.page_count as u32,
            file_page_counts: rendered.file_page_counts.iter().map(|&n| n as u32).collect(),
            labels_billed: labels_billed as u32,
        });
        if let Some(quota) = quota {
            *response.metadata_mut() = metadata(quota.headers());
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::proto::renderer_client::RendererClient;
    use super::proto::{Format as ProtoFormat, RenderRequest, ZplFile};
    use super::*;
    use crate::console::tests::{KEYS, LIVE, TEST};
    use crate::console::{RevocationList, Verifier};
    use crate::rate_limit::RateLimiter;
    use crate::usage::Usage;
    use tonic::transport::Channel;

    const LABEL: &[u8] = b"^XA^FO50,50^A0N,40^FDhi^FS^PQ3^XZ";

    /// A server on a free port and a client channel to it.
    async fn start(billing: Option<Arc<Billing>>, admission: Arc<Admission>) -> RendererClient<Channel> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(serve(listener, billing, admission, std::future::pending()));
        let channel = Channel::from_shared(format!("http://{addr}")).unwrap().connect().await.unwrap();
        RendererClient::new(channel).max_decoding_message_size(usize::MAX)
    }

    fn open() -> Arc<Admission> {
        Arc::new(Admission::with_defaults())
    }

    fn with_key(key: &str, body: RenderRequest) -> Request<RenderRequest> {
        let mut request = Request::new(body);
        request.metadata_mut().insert("authorization", format!("Bearer {key}").parse().unwrap());
        request
    }

    fn label(format: ProtoFormat) -> RenderRequest {
        RenderRequest { zpl: LABEL.to_vec(), format: format as i32, ..Default::default() }
    }

    #[tokio::test]
    async fn renders_pdf_and_png() {
        let mut client = start(None, open()).await;
        let pdf = client.render(label(ProtoFormat::Unspecified)).await.unwrap().into_inner();
        assert!(pdf.data.starts_with(b"%PDF"));
        assert_eq!((pdf.content_type.as_str(), pdf.page_count, pdf.file_page_counts), ("application/pdf", 3, vec![3]));
        assert_eq!(pdf.labels_billed, 0);

        let png = client.render(label(ProtoFormat::Png)).await.unwrap().into_inner();
        assert!(png.data.starts_with(b"\x89PNG"));
        assert_eq!(png.content_type, "image/png");

        // The same bytes as the HTTP API's renderer.
        let direct = crate::render_label(std::str::from_utf8(LABEL).unwrap(), Format::Png, 1, DEFAULT_DPMM).unwrap();
        assert_eq!(png.data, direct.bytes);
    }

    #[tokio::test]
    async fn settings_and_files() {
        let mut client = start(None, open()).await;
        let width = |png: &[u8]| u32::from_be_bytes(png[16..20].try_into().unwrap());
        let request = RenderRequest { zpl: b"^XA^FO10,10^GB50,50,5^FS^XZ".to_vec(), format: ProtoFormat::Png as i32, dpmm: 12, ..Default::default() };
        assert_eq!(width(&client.render(request).await.unwrap().into_inner().data), 1200);

        let files = vec![
            ZplFile { name: "a.zpl".into(), zpl: b"^XA^PW100^FO1,1^GB10,10,10^FS^XZ".to_vec() },
            ZplFile { name: String::new(), zpl: b"^XA^PW200^FO1,1^GB10,10,10^FS^XZ^XA^PW300^FO1,1^GB10,10,10^FS^XZ".to_vec() },
        ];
        let request = RenderRequest { files: files.clone(), format: ProtoFormat::Png as i32, page: 3, ..Default::default() };
        let response = client.render(request).await.unwrap().into_inner();
        assert_eq!((response.page_count, response.file_page_counts), (3, vec![1, 2]));
        assert_eq!(width(&response.data), 300);

        for bad in [
            RenderRequest { dpmm: 7, ..label(ProtoFormat::Pdf) },
            RenderRequest { format: 9, ..label(ProtoFormat::Pdf) },
            RenderRequest { page: 4, ..label(ProtoFormat::Png) },
            RenderRequest { files, ..label(ProtoFormat::Pdf) },
        ] {
            assert_eq!(client.render(bad).await.unwrap_err().code(), Code::InvalidArgument);
        }
    }

    #[tokio::test]
    async fn accepts_compressed_requests() {
        let mut client = start(None, open()).await.send_compressed(CompressionEncoding::Gzip);
        assert_eq!(client.render(label(ProtoFormat::Pdf)).await.unwrap().into_inner().page_count, 3);
        let mut client = client.send_compressed(CompressionEncoding::Zstd);
        assert_eq!(client.render(label(ProtoFormat::Pdf)).await.unwrap().into_inner().page_count, 3);
    }

    #[tokio::test]
    async fn keys_bill_and_limit() {
        let usage = Usage::new();
        let limiter = RateLimiter::new(1, 3);
        let billing = Billing::with_rate_limit(Verifier::from_config(KEYS).unwrap(), Some(usage.clone()), limiter);
        let mut client = start(Some(billing.clone()), open()).await;

        let missing = client.render(label(ProtoFormat::Pdf)).await.unwrap_err();
        assert_eq!(missing.code(), Code::Unauthenticated);
        assert!(missing.message().contains("https://console.test/keys"));
        let bad = client.render(with_key("zpl_live_nonsense", label(ProtoFormat::Pdf))).await.unwrap_err();
        assert_eq!(bad.code(), Code::Unauthenticated);

        let response = client.render(with_key(LIVE, label(ProtoFormat::Pdf))).await.unwrap();
        assert_eq!(response.metadata().get("ratelimit-limit").unwrap(), "3");
        assert_eq!(response.metadata().get("ratelimit-remaining").unwrap(), "2");
        assert_eq!(response.into_inner().labels_billed, 3);

        let mut request = Request::new(label(ProtoFormat::Png));
        request.metadata_mut().insert("x-api-key", TEST.parse().unwrap());
        assert_eq!(client.render(request).await.unwrap().into_inner().labels_billed, 0);

        // The test key shares the live key's bucket, now empty.
        client.render(with_key(LIVE, label(ProtoFormat::Png))).await.unwrap();
        let limited = client.render(with_key(LIVE, label(ProtoFormat::Png))).await.unwrap_err();
        assert_eq!(limited.code(), Code::ResourceExhausted);
        assert_eq!(limited.metadata().get("retry-after").unwrap(), "1");
        assert_eq!(limited.metadata().get("ratelimit-remaining").unwrap(), "0");

        let billed: u32 = usage.cut().iter().map(|m| m.labels + m.idempotent.iter().map(|r| r.labels).sum::<u32>()).sum();
        assert_eq!(billed, 3 + 1 + 1, "live PDF, test PNG, live PNG");

    }

    #[tokio::test]
    async fn suspended_accounts() {
        let suspended = Billing::with_verifier(Verifier::from_config(KEYS).unwrap(), None);
        suspended.set_revocations(&RevocationList { revoked: vec![], suspended_orgs: vec![uuid::Uuid::from_u128(0xa)] });
        let mut client = start(Some(suspended), open()).await;
        let paused = client.render(with_key(LIVE, label(ProtoFormat::Pdf))).await.unwrap_err();
        assert_eq!(paused.code(), Code::PermissionDenied);
        assert!(client.render(with_key(TEST, label(ProtoFormat::Pdf))).await.is_ok());
    }

    #[tokio::test]
    async fn idempotency_key_metadata() {
        let usage = Usage::new();
        let billing = Billing::with_verifier(Verifier::from_config(KEYS).unwrap(), Some(usage.clone()));
        let mut client = start(Some(billing), open()).await;
        for _ in 0..2 {
            let mut request = with_key(LIVE, label(ProtoFormat::Pdf));
            request.metadata_mut().insert("idempotency-key", "order-1".parse().unwrap());
            client.render(request).await.unwrap();
        }
        let messages = usage.cut();
        let ids: Vec<_> = messages.iter().flat_map(|m| m.idempotent.iter().map(|r| (r.request_id, r.idempotency_key.clone()))).collect();
        assert_eq!(ids.len(), 1, "a retry is listed once: {ids:?}");
        assert_eq!(ids[0].1, "order-1");
    }

    #[tokio::test]
    async fn overload_is_unavailable() {
        let admission = Arc::new(Admission::new(1, 0, Duration::from_secs(5)));
        let mut client = start(None, admission.clone()).await;
        let busy = admission.admit(1).await.unwrap();
        let shed = client.render(label(ProtoFormat::Pdf)).await.unwrap_err();
        assert_eq!(shed.code(), Code::Unavailable);
        assert_eq!(shed.metadata().get("retry-after").unwrap(), "1");
        drop(busy);
        assert!(client.render(label(ProtoFormat::Pdf)).await.is_ok());
    }

    #[tokio::test]
    async fn many_calls_share_one_connection() {
        let client = start(None, open()).await;
        let calls = (0..32).map(|_| {
            let mut client = client.clone();
            tokio::spawn(async move { client.render(label(ProtoFormat::Png)).await.map(|r| r.into_inner().page_count) })
        });
        for call in calls {
            assert_eq!(call.await.unwrap().unwrap(), 3);
        }
    }
}
