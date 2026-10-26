# caretly

[![crates.io](https://img.shields.io/crates/v/caretly.svg)](https://crates.io/crates/caretly)
[![docs.rs](https://docs.rs/caretly/badge.svg)](https://docs.rs/caretly)

Rust client for the [Caretly](https://caretly.dev) API: render ZPL labels to vector PDF
or PNG. It has an async client for tokio and a blocking client, sharing the same
options, result and error types.

Source: <https://github.com/jaapieaapie1/zpl> (folder `clients/rust`).

## Install

```sh
cargo add caretly
```

Or in `Cargo.toml`:

```toml
[dependencies]
caretly = "0.1"
```

### Features

| Feature      | Default | What it adds |
|--------------|---------|--------------|
| `async`      | yes     | `caretly::Client`, the async client for tokio. |
| `rustls-tls` | yes     | HTTPS through rustls. |
| `blocking`   | no      | `caretly::blocking::Client`, no async runtime needed. |
| `native-tls` | no      | HTTPS through the platform's TLS library instead of rustls. |
| `gzip`       | no      | The `compress` option: gzip the request body. |

Blocking only, without tokio in your code:

```toml
caretly = { version = "0.1", default-features = false, features = ["blocking", "rustls-tls"] }
```

Keep one of `rustls-tls` or `native-tls` on to reach `https://` URLs; without them only
`http://` (for example a self-hosted server on localhost) works.

Minimum supported Rust version: 1.85. On toolchains older than the newest dependency
releases need, let Cargo pick compatible versions with `resolver = "3"` (or
`CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback`).

## Quickstart

Create an API key in the [console](https://caretly.dev/console/keys). Test keys
(`zpl_test_…`) are free and watermark every label; live keys (`zpl_live_…`) are billed
per label. The client reads the key from `CARETLY_API_KEY` unless you pass one.

### Async

```rust,no_run
use caretly::{Client, Dpmm, Format, RenderOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?; // reads CARETLY_API_KEY

    let pdf = client.render_pdf("^XA^FO50,50^A0N,40,40^FDHello^FS^XZ").await?;
    pdf.save("label.pdf")?;
    println!("{} page(s)", pdf.page_count);

    let options = RenderOptions::new().format(Format::Png).page(1).dpmm(Dpmm::D12);
    let png = client.render("^XA^FO50,50^A0N,40,40^FDHello^FS^XZ", options).await?;
    png.save("label.png")?;
    Ok(())
}
```

### Blocking

```rust,no_run
use caretly::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().api_key("zpl_test_...").build()?;
    let pdf = client.render_pdf("^XA^FO50,50^A0N,40,40^FDHello^FS^XZ")?;
    pdf.save("label.pdf")?;
    Ok(())
}
```

Do not call the blocking client from inside an async runtime; use the async client there.

### Several files

Each file starts from a fresh printer state; pages follow the order of the files.

```rust,no_run
# async fn run(client: caretly::Client) -> Result<(), caretly::Error> {
let result = client
    .render_files([("a.zpl", "^XA^FDa^FS^XZ"), ("b.zpl", "^XA^FDb^FS^XZ")], Default::default())
    .await?;
println!("pages per file: {:?}", result.file_page_counts);
# Ok(()) }
```

File contents can be `&str`, `String`, `&[u8]` or `Vec<u8>`.

## Client settings

```rust,no_run
use std::time::Duration;

let client = caretly::Client::builder()
    .api_key("zpl_live_...")              // default: CARETLY_API_KEY
    .base_url("http://localhost:3000")    // default: https://caretly.dev
    .timeout(Duration::from_secs(10))     // per attempt, default: 30 s
    .max_retries(3)                       // default: 2
    .build()?;
# Ok::<(), caretly::Error>(())
```

An empty API key sends no `Authorization` header, for self-hosted servers without keys.

## Render options

| Method | Query / header | Meaning |
|--------|----------------|---------|
| `format(Format::Pdf \| Format::Png)` | `format` | Output format. Default PDF (every page). |
| `page(n)` | `page` | For PNG: the page to render, from 1. |
| `dpmm(Dpmm::D6 \| D8 \| D12 \| D24)` | `dpmm` | Resolution in dots/mm. Default 8. |
| `timings(true)` | `timings=1` | Ask for a `Server-Timing` header. |
| `idempotency_key("order-1234")` | `Idempotency-Key` | A retry with the same key and content is billed once. Max 255 characters. |
| `compress(true)` (feature `gzip`) | `Content-Encoding: gzip` | Gzip the request body. |

Unset options are left out of the request, so the server defaults apply. `page(0)`, an
empty or too long idempotency key, and an empty file list fail before anything is sent
with `Error::InvalidArgument`. `Dpmm::try_from(n)` checks a numeric resolution.

## Result

`RenderResult` holds:

| Field | From | Meaning |
|-------|------|---------|
| `bytes` | body | The PDF or PNG. `save(path)` writes it to a file. |
| `content_type` | `Content-Type` | `application/pdf` or `image/png`. |
| `page_count` | `X-Page-Count` | Total pages. |
| `file_page_counts` | `X-File-Page-Counts` | Pages per file (multipart only, else empty). |
| `labels_billed` | `X-Labels-Billed` | Labels billed; 0 for a test key; `None` without keys. |
| `rate_limit` | `RateLimit-Limit`, `RateLimit-Remaining` | Burst size and what is left. |
| `server_timing` | `Server-Timing` | Raw header, with `timings(true)`. |

## Errors

Every call returns `Result<_, caretly::Error>`:

| Variant | Cause |
|---------|-------|
| `BadRequest` | 400: ZPL parse error, invalid parameter, page out of range. |
| `Authentication` | 401: key missing, invalid, expired or revoked. |
| `PaymentRequired` | 402: billing inactive, live keys paused. |
| `PayloadTooLarge` | 413: body over 100 MiB decompressed. |
| `UnsupportedEncoding` | 415: unsupported `Content-Encoding`. |
| `RateLimited` | 429: too many requests (retried). |
| `Overloaded` | 503: server busy, not billed (retried). |
| `Api` | Any other status, including other 5xx. |
| `Connection` | Network failure (retried). |
| `Timeout` | The timeout elapsed (not retried). |
| `InvalidArgument` | Client-side validation failed; nothing was sent. |

The API variants carry an `ApiError` with `status`, `message` (the response body) and
`retry_after`. `Error::status()`, `Error::retry_after()` and `Error::api_error()` reach
them without matching.

```rust,no_run
# async fn run(client: caretly::Client) {
match client.render_pdf("^XA^FO50,50^FDoops").await {
    Ok(pdf) => println!("{} bytes", pdf.bytes.len()),
    Err(caretly::Error::BadRequest(e)) => eprintln!("invalid ZPL: {}", e.message),
    Err(e) => eprintln!("render failed: {e}"),
}
# }
```

## Retries

A 429, a 503 or a connection error is retried up to `max_retries` times (default 2).
The client waits `Retry-After` seconds when the server sends it, and otherwise backs
off 0.5 s, 1 s, 2 s, 4 s, then 8 s. Other statuses and timeouts are not retried.

When retries are on and you did not set an idempotency key, the client generates one
(a UUID v4) per call and sends it on every attempt, so a retried request is never
billed twice. Set `max_retries(0)` to turn retries off.

## Examples

```sh
CARETLY_API_KEY=zpl_test_... cargo run --example render_pdf
CARETLY_API_KEY=zpl_test_... cargo run --example render_files -- a.zpl b.zpl
CARETLY_API_KEY=zpl_test_... cargo run --example blocking --features blocking
```

## Development

```sh
cargo test --all-features
cargo test --no-default-features --features blocking
cargo clippy --all-features --all-targets -- -D warnings
cargo fmt --check
```

The tests run against a local mock server; they need no network or API key.

## Publishing

This crate lives in `clients/rust` and is not part of the repository's root workspace
(its `Cargo.toml` has its own `[workspace]` table). From `clients/rust`:

1. Update `version` in `Cargo.toml` and add an entry to `CHANGELOG.md`.
2. Run the checks above, then `cargo package --list` to review the packaged files.
3. `cargo publish --dry-run`
4. Log in once with `cargo login <token>` (a token from <https://crates.io/settings/tokens>
   with the `publish-new` / `publish-update` scope).
5. `cargo publish`
6. Tag the release: `git tag clients/rust/v0.1.0 && git push origin clients/rust/v0.1.0`.

docs.rs builds the documentation with all features after publishing.

## License

MIT, see [LICENSE](LICENSE).
