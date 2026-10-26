//! Client for the [Caretly](https://caretly.dev) API: render ZPL labels to PDF or PNG.
//!
//! The crate has an async client, [`Client`] (feature `async`, on by default, for tokio),
//! and a blocking client, [`blocking::Client`] (feature `blocking`). Both take the same
//! [`RenderOptions`], return the same [`RenderResult`] and fail with the same
//! [`enum@Error`].
//!
//! # Features
//!
//! | Feature      | Default | What it adds |
//! |--------------|---------|--------------|
//! | `async`      | yes     | [`Client`], the async client for tokio. |
//! | `rustls-tls` | yes     | HTTPS through rustls. |
//! | `blocking`   | no      | [`blocking::Client`], usable without an async runtime. |
//! | `native-tls` | no      | HTTPS through the platform's TLS library. |
//! | `gzip`       | no      | [`RenderOptions::compress`]: gzip the request body. |
//!
//! Without `rustls-tls` or `native-tls` the client can only reach `http://` URLs.
//!
//! # API key
//!
//! The key is read from the `CARETLY_API_KEY` environment variable unless you pass one to
//! the builder. Test keys (`zpl_test_…`) are free and watermark every label. Self-hosted
//! servers run without keys.
//!
#![cfg_attr(
    feature = "async",
    doc = r#"
# Async example

```no_run
use caretly::{Client, Format, RenderOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().api_key("zpl_test_...").build()?;

    let pdf = client.render_pdf("^XA^FO50,50^A0N,40^FDHello^FS^XZ").await?;
    println!("{} page(s), {} label(s) billed", pdf.page_count, pdf.labels_billed.unwrap_or(0));
    pdf.save("label.pdf")?;

    let png = client
        .render("^XA^FO50,50^A0N,40^FDHello^FS^XZ", RenderOptions::new().format(Format::Png).page(1))
        .await?;
    png.save("label.png")?;
    Ok(())
}
```
"#
)]
#![cfg_attr(
    feature = "blocking",
    doc = r#"
# Blocking example

```no_run
use caretly::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?; // reads CARETLY_API_KEY
    let pdf = client.render_pdf("^XA^FO50,50^A0N,40^FDHello^FS^XZ")?;
    pdf.save("label.pdf")?;
    Ok(())
}
```
"#
)]
//!
//! # Errors and retries
//!
//! Error statuses map to variants of [`enum@Error`] (`BadRequest`, `Authentication`,
//! `RateLimited`, …), each with the status, the server's message and `Retry-After`.
//!
//! A 429, a 503 or a connection error is retried up to
//! `max_retries` times (default 2), waiting `Retry-After` when the server sends it and
//! otherwise 0.5 s, 1 s, 2 s, … up to 8 s. When retries are on and you did not set an
//! [`idempotency key`](RenderOptions::idempotency_key), the client generates one per
//! call, so a retried request is never billed twice.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(
    not(any(feature = "async", feature = "blocking")),
    allow(dead_code, unused_imports, unused_macros)
)]

mod error;
mod options;
mod result;
mod shared;

#[cfg(feature = "async")]
mod client;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use client::{Client, ClientBuilder};
pub use error::{ApiError, BoxError, Error, Result};
pub use options::{Dpmm, Format, RenderOptions, MAX_IDEMPOTENCY_KEY_LEN};
pub use result::{RateLimit, RenderResult};

/// Compiles the README's examples as doctests.
#[cfg(all(doctest, feature = "async", feature = "blocking"))]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;
