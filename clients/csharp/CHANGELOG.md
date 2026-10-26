# Changelog

All notable changes to the `Caretly` NuGet package are listed here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the package uses
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-10-27

### Added

- `CaretlyClient` for the Caretly ZPL rendering API, targeting `netstandard2.0` and `net8.0`
  with no runtime dependencies.
- `RenderAsync` for a single ZPL document and `RenderFilesAsync` for several files sent as
  `multipart/form-data`, plus `RenderPdfAsync` and `RenderPngAsync` shortcuts.
- `HealthAsync` for `GET /health`.
- `RenderOptions`: format, page, dpmm, timings, idempotency key and gzip compression, with
  client-side validation of page, dpmm and the idempotency key.
- `RenderResult` with the file bytes, content type, page count, pages per file, labels billed,
  rate limit and server timings, and `SaveTo` / `SaveToAsync`.
- Typed exceptions per status: `BadRequestException`, `AuthenticationException`,
  `PaymentRequiredException`, `PayloadTooLargeException`, `UnsupportedEncodingException`,
  `RateLimitedException`, `OverloadedException`, plus `CaretlyConnectionException` and
  `CaretlyTimeoutException`.
- Automatic retries of 429, 503 and connection errors (2 by default), honouring `Retry-After`
  and sharing one generated `Idempotency-Key` so a retry is billed once.
- A constructor that takes an `HttpClient`, for `IHttpClientFactory`.

[0.1.0]: https://github.com/jaapieaapie1/zpl/tree/main/clients/csharp
