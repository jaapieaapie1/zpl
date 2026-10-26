# Changelog

All notable changes to `caretly/caretly` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-10-27

### Added

- `Caretly\Client` with `render()`, `renderFiles()` (multipart), `renderPdf()`, `renderPng()` and `health()`.
- `RenderOptions` for `format`, `page`, `dpmm`, `timings`, `idempotencyKey` and `compress` (gzip),
  validated client side.
- `RenderResult` with the file bytes, content type, page counts, labels billed, rate limit state,
  `Server-Timing`, and `saveTo()`.
- Typed exceptions under `Caretly\Exception\` for every API error status and for transport failures.
- Automatic retries for 429, 503 and connection errors, honouring `Retry-After`, with a generated
  `Idempotency-Key` so retries are never billed twice. Timeouts are not retried: a
  `TimeoutException` is thrown on the first timeout.
- Works with any PSR-18 HTTP client, found with `php-http/discovery` when none is injected.

[0.1.0]: https://github.com/jaapieaapie1/caretly-php/releases/tag/v0.1.0
