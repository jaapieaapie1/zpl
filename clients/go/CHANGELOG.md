# Changelog

All notable changes to this module are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the module uses
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-10-27

### Added

- `NewClient` with `WithAPIKey`, `WithBaseURL`, `WithHTTPClient`, `WithTimeout` and
  `WithMaxRetries`; the API key falls back to `CARETLY_API_KEY`.
- `Client.Render` for a single ZPL document and `Client.RenderFiles` for several files in
  one multipart request, plus `RenderPDF` and `RenderPNG` shortcuts.
- `RenderOptions`: format, page, dpmm, timings, idempotency key and gzip compression,
  validated before any request.
- `RenderResult` with the file bytes, content type, page count, pages per file, labels
  billed, rate limit and server timing, and `Save` to write it to a file.
- `Client.Health` for `GET /health`.
- `*APIError` and `*TransportError` with sentinel errors for `errors.Is`.
- Automatic retries of 429, 503 and connection errors, honouring `Retry-After`, with an
  `Idempotency-Key` generated once per call so retries are not billed twice.
