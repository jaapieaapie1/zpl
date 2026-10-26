# Changelog

All notable changes to this package are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the package uses
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-10-27

### Added

- `Caretly` client with `render`, `renderFiles`, `renderPdf`, `renderPng` and `health`.
- `RenderResult` with the file bytes, page counts, labels billed, rate limit, server
  timings and `saveTo(path)`.
- Typed errors: `CaretlyError`, `APIError` and one subclass per documented status,
  `ConnectionError` and `TimeoutError`.
- Automatic retries of 429, 503 and connection errors, honouring `Retry-After`, with an
  `Idempotency-Key` per call so retries are never billed twice.
- Optional gzip request bodies (`compress`).
- ESM and CommonJS builds with type declarations; no runtime dependencies.
