# Changelog

All notable changes to this package are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-10-27

### Added

- `caretly.Client` (sync) and `caretly.AsyncClient` (asyncio), built on `httpx`.
- `render()` for a single ZPL body and `render_files()` for several files as
  `multipart/form-data`, with `format`, `page`, `dpmm`, `timings`, `idempotency_key` and
  `compress` (gzip) options; `render_pdf()` / `render_png()` shorthands.
- `RenderResult` with the content, content type, page counts, labels billed, rate limit
  and `Server-Timing`, plus `save()`.
- `health()`.
- Typed errors per status (`BadRequestError`, `AuthenticationError`,
  `PaymentRequiredError`, `PayloadTooLargeError`, `UnsupportedEncodingError`,
  `RateLimitError`, `OverloadedError`, `APIError`) and `APIConnectionError` /
  `APITimeoutError`.
- Automatic retries for 429, 503 and connection errors, honouring `Retry-After`, with a
  generated `Idempotency-Key` so retries are not billed twice. Timeouts are not retried.

[0.1.0]: https://github.com/jaapieaapie1/zpl/tree/main/clients/python
