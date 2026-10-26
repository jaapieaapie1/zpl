# Changelog

All notable changes to this crate are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate follows
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-10-27

### Added

- Async client `caretly::Client` (feature `async`, default) and blocking client
  `caretly::blocking::Client` (feature `blocking`) for the Caretly ZPL rendering API.
- `render`, `render_files` (multipart), `render_pdf`, `render_png` and `health`.
- `RenderOptions`: `format`, `page`, `dpmm`, `timings`, `idempotency_key`, and
  `compress` (gzip, feature `gzip`), validated client side.
- `RenderResult` with page counts, billed labels, rate limit and `Server-Timing`, and
  `save` to write the file.
- Typed `Error` per status (400, 401, 402, 413, 415, 429, 503, others), plus
  connection, timeout and invalid-argument errors.
- Automatic retries of 429, 503 and connection errors, honouring `Retry-After`, with a
  generated `Idempotency-Key` so retries are never billed twice.
- TLS through rustls (`rustls-tls`, default) or the platform (`native-tls`).

[0.1.0]: https://github.com/jaapieaapie1/zpl/tree/main/clients/rust
