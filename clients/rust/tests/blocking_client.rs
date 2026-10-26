#![cfg(feature = "blocking")]

//! The same scenarios as `async_client.rs`, against the blocking client.

mod common;

use std::time::{Duration, Instant};

use caretly::blocking::Client;
use caretly::{Dpmm, Error, Format, RenderOptions};
use common::{closed_url, MockServer, Reply, LABEL};

fn client(server: &MockServer) -> Client {
    Client::builder()
        .api_key("zpl_test_abc")
        .base_url(server.url())
        .max_retries(0)
        .build()
        .unwrap()
}

#[test]
fn parses_success_headers() {
    let server = MockServer::start(vec![Reply::pdf()]);
    let result = client(&server).render(LABEL, RenderOptions::new()).unwrap();
    assert_eq!(result.bytes, b"%PDF-1.7 fake");
    assert_eq!(result.content_type, "application/pdf");
    assert_eq!(result.page_count, 3);
    assert_eq!(result.file_page_counts, vec![2, 1]);
    assert_eq!(result.labels_billed, Some(3));
    let rate_limit = result.rate_limit.unwrap();
    assert_eq!((rate_limit.limit, rate_limit.remaining), (200, 199));
    assert_eq!(
        result.server_timing.as_deref(),
        Some("read;dur=0.1, parse;dur=0.2, total;dur=1.0")
    );
}

#[test]
fn sends_label_auth_and_query() {
    let server = MockServer::start(vec![Reply::pdf()]);
    let options = RenderOptions::new()
        .format(Format::Png)
        .page(2)
        .dpmm(Dpmm::D12)
        .timings(true);
    client(&server).render(LABEL, options).unwrap();

    let requests = server.requests();
    assert_eq!(requests.len(), 1);
    let request = &requests[0];
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.target,
        "/render?format=png&page=2&dpmm=12&timings=1"
    );
    assert_eq!(request.header("authorization"), Some("Bearer zpl_test_abc"));
    assert_eq!(
        request.header("content-type"),
        Some("text/plain; charset=utf-8")
    );
    assert_eq!(
        request.header("user-agent"),
        Some(concat!("caretly-rust/", env!("CARGO_PKG_VERSION")))
    );
    assert_eq!(
        request.header("idempotency-key"),
        None,
        "no retries, no generated key"
    );
    assert_eq!(request.header("content-encoding"), None);
    assert_eq!(request.body, LABEL.as_bytes());
}

#[test]
fn omits_unset_query_and_empty_key() {
    let server = MockServer::start(vec![Reply::pdf()]);
    let client = Client::builder()
        .api_key("")
        .base_url(server.url())
        .build()
        .unwrap();
    client.render(LABEL, RenderOptions::new()).unwrap();
    let request = &server.requests()[0];
    assert_eq!(request.target, "/render");
    assert_eq!(request.header("authorization"), None);
}

#[test]
fn sends_multipart_files() {
    let server = MockServer::start(vec![Reply::pdf()]);
    let files = [
        ("a.zpl", "^XA^FDa^FS^XZ".as_bytes()),
        ("b.zpl", "^XA^FDb^FS^XZ".as_bytes()),
    ];
    let result = client(&server)
        .render_files(files, RenderOptions::new())
        .unwrap();
    assert_eq!(result.file_page_counts, vec![2, 1]);

    let request = &server.requests()[0];
    let content_type = request.header("content-type").unwrap();
    let boundary = content_type
        .strip_prefix("multipart/form-data; boundary=")
        .expect(content_type);
    let body = String::from_utf8(request.body.clone()).unwrap();
    let parts: Vec<&str> = body.split(&format!("--{boundary}")).collect();
    assert_eq!(
        parts.len(),
        4,
        "two parts plus preamble and epilogue: {body}"
    );
    assert!(parts[1].contains("Content-Disposition: form-data; name=\"file\"; filename=\"a.zpl\""));
    assert!(parts[1].ends_with("^XA^FDa^FS^XZ\r\n"));
    assert!(parts[2].contains("filename=\"b.zpl\""));
    assert!(parts[2].ends_with("^XA^FDb^FS^XZ\r\n"));
    assert_eq!(parts[3], "--\r\n");
}

#[cfg(feature = "gzip")]
#[test]
fn gzips_body_when_compress_is_set() {
    let server = MockServer::start(vec![Reply::pdf()]);
    client(&server)
        .render(LABEL, RenderOptions::new().compress(true))
        .unwrap();
    let request = &server.requests()[0];
    assert_eq!(request.header("content-encoding"), Some("gzip"));
    assert_eq!(common::gunzip(&request.body), LABEL.as_bytes());
}

#[test]
fn maps_error_statuses() {
    for status in [400, 401, 402, 413, 415, 429, 503, 500, 404] {
        let server = MockServer::start(vec![
            Reply::new(status, "something went wrong\n").header("Retry-After", "7")
        ]);
        let error = client(&server)
            .render(LABEL, RenderOptions::new())
            .unwrap_err();
        let expected = match status {
            400 => matches!(error, Error::BadRequest(_)),
            401 => matches!(error, Error::Authentication(_)),
            402 => matches!(error, Error::PaymentRequired(_)),
            413 => matches!(error, Error::PayloadTooLarge(_)),
            415 => matches!(error, Error::UnsupportedEncoding(_)),
            429 => matches!(error, Error::RateLimited(_)),
            503 => matches!(error, Error::Overloaded(_)),
            _ => matches!(error, Error::Api(_)),
        };
        assert!(expected, "{status} mapped to {error:?}");
        let api = error.api_error().unwrap();
        assert_eq!(api.status, status);
        assert_eq!(api.message, "something went wrong");
        assert_eq!(api.retry_after, Some(Duration::from_secs(7)));
        assert_eq!(
            server.requests().len(),
            1,
            "{status} must not be retried with max_retries 0"
        );
    }
}

#[test]
fn retries_429_honoring_retry_after() {
    let server = MockServer::start(vec![
        Reply::new(429, "slow down").header("Retry-After", "1"),
        Reply::pdf(),
    ]);
    let client = Client::builder()
        .api_key("k")
        .base_url(server.url())
        .build()
        .unwrap();
    let start = Instant::now();
    let result = client.render(LABEL, RenderOptions::new()).unwrap();
    assert!(
        start.elapsed() >= Duration::from_secs(1),
        "waited {:?}",
        start.elapsed()
    );
    assert_eq!(result.page_count, 3);

    let requests = server.requests();
    assert_eq!(requests.len(), 2);
    let key = requests[0]
        .header("idempotency-key")
        .expect("generated idempotency key");
    assert_eq!(key.len(), 36);
    assert_eq!(
        requests[1].header("idempotency-key"),
        Some(key),
        "same key on the retry"
    );
    assert_eq!(requests[1].body, requests[0].body);
}

#[test]
fn retries_503_then_gives_up() {
    let server = MockServer::start(vec![Reply::new(503, "busy").header("Retry-After", "0")]);
    let client = Client::builder()
        .api_key("k")
        .base_url(server.url())
        .max_retries(2)
        .build()
        .unwrap();
    let options = RenderOptions::new().idempotency_key("order-42");
    let error = client.render(LABEL, options).unwrap_err();
    assert!(matches!(error, Error::Overloaded(_)), "{error:?}");
    let requests = server.requests();
    assert_eq!(requests.len(), 3, "first attempt plus two retries");
    assert!(requests
        .iter()
        .all(|r| r.header("idempotency-key") == Some("order-42")));
}

#[test]
fn does_not_retry_other_statuses() {
    let server = MockServer::start(vec![Reply::new(500, "boom"), Reply::pdf()]);
    let client = Client::builder()
        .api_key("k")
        .base_url(server.url())
        .build()
        .unwrap();
    let error = client.render(LABEL, RenderOptions::new()).unwrap_err();
    assert!(matches!(error, Error::Api(_)));
    assert_eq!(server.requests().len(), 1);
}

#[test]
fn retries_connection_errors() {
    let client = Client::builder()
        .api_key("k")
        .base_url(closed_url())
        .max_retries(1)
        .build()
        .unwrap();
    let start = Instant::now();
    let error = client.render(LABEL, RenderOptions::new()).unwrap_err();
    assert!(matches!(error, Error::Connection(_)), "{error:?}");
    assert!(
        start.elapsed() >= Duration::from_millis(500),
        "backed off once"
    );
}

#[test]
fn times_out() {
    let server = MockServer::start(vec![Reply::pdf().delay(Duration::from_secs(2))]);
    let client = Client::builder()
        .api_key("k")
        .base_url(server.url())
        .timeout(Duration::from_millis(200))
        .build()
        .unwrap();
    let error = client.render(LABEL, RenderOptions::new()).unwrap_err();
    assert!(matches!(error, Error::Timeout(_)), "{error:?}");
    assert_eq!(server.requests().len(), 1, "timeouts are not retried");
}

#[test]
fn validates_before_sending() {
    let server = MockServer::start(vec![Reply::pdf()]);
    let client = client(&server);
    let page_zero = client.render_png(LABEL, 0).unwrap_err();
    assert!(matches!(page_zero, Error::InvalidArgument(_)));
    let long_key = client.render(LABEL, RenderOptions::new().idempotency_key("k".repeat(256)));
    assert!(matches!(long_key, Err(Error::InvalidArgument(_))));
    assert!(matches!(Dpmm::try_from(7), Err(Error::InvalidArgument(_))));
    let no_files: Vec<(String, String)> = Vec::new();
    assert!(matches!(
        client.render_files(no_files, RenderOptions::new()),
        Err(Error::InvalidArgument(_))
    ));
    assert!(server.requests().is_empty());
}

#[test]
fn convenience_methods() {
    let server = MockServer::start(vec![Reply::pdf()]);
    let client = client(&server);
    client.render_pdf(LABEL).unwrap();
    client.render_png(LABEL, 3).unwrap();
    let targets: Vec<_> = server.requests().into_iter().map(|r| r.target).collect();
    assert_eq!(targets, ["/render?format=pdf", "/render?format=png&page=3"]);
}

#[test]
fn health() {
    let server = MockServer::start(vec![Reply::new(200, "ok")]);
    assert!(client(&server).health().unwrap());
    assert_eq!(server.requests()[0].method, "GET");
    assert_eq!(server.requests()[0].target, "/health");

    let unhealthy = MockServer::start(vec![Reply::new(500, "down")]);
    assert!(!client(&unhealthy).health().unwrap());

    let unreachable = Client::builder()
        .api_key("")
        .base_url(closed_url())
        .build()
        .unwrap();
    assert!(matches!(unreachable.health(), Err(Error::Connection(_))));
}

#[test]
fn saves_to_file() {
    let server = MockServer::start(vec![Reply::pdf()]);
    let result = client(&server).render_pdf(LABEL).unwrap();
    let path = std::env::temp_dir().join(format!("caretly-test-{}.pdf", std::process::id()));
    result.save(&path).unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"%PDF-1.7 fake");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn debug_redacts_api_key() {
    let client = Client::builder()
        .api_key("zpl_live_secret")
        .build()
        .unwrap();
    let debug = format!("{client:?}");
    assert!(!debug.contains("secret"), "{debug}");
    assert!(debug.contains("<redacted>"));
}
