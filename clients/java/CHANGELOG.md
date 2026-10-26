# Changelog

All notable changes to this library are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-10-27

### Added

- `CaretlyClient` with blocking and `CompletableFuture` forms of `render`, `renderFiles` and
  `health`, plus `renderPdf` and `renderPng` shortcuts.
- `RenderOptions`: `format`, `page`, `dpmm`, `timings`, `idempotencyKey` and `compress` (gzip),
  validated when set.
- `RenderResult` with the file bytes, content type, page counts, labels billed, rate limit,
  server timings and `saveTo(Path)`.
- Typed exceptions for every API error status, connection failures and timeouts.
- Automatic retries of 429, 503 and connection failures, honouring `Retry-After`, with a
  generated `Idempotency-Key` so retries are never billed twice.
- Java 11+, no runtime dependencies, a named module `dev.caretly`.

[0.1.0]: https://github.com/jaapieaapie1/zpl/tree/main/clients/java
