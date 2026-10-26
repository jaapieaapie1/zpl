# Caretly clients

Official client libraries for the [Caretly](https://caretly.dev) API: render ZPL labels to
PDF or PNG. Each folder is an independent package with its own README, tests and
publishing steps. The HTTP API they wrap is described in the OpenAPI spec in the
`zpl-docs` repository (`openapi.json`).

| Language | Folder              | Package                                   | Install                                             |
|----------|---------------------|-------------------------------------------|-----------------------------------------------------|
| Rust     | [`rust`](rust)      | crates.io `caretly`                       | `cargo add caretly` (async, tokio) or `cargo add caretly --no-default-features --features blocking` |
| Go       | [`go`](go)          | `github.com/jaapieaapie1/zpl/clients/go`  | `go get github.com/jaapieaapie1/zpl/clients/go`     |
| Python   | [`python`](python)  | PyPI `caretly`                            | `pip install caretly`                               |
| Node.js  | [`node`](node)      | npm `@caretly/caretly`                    | `npm install @caretly/caretly`                      |
| PHP      | [`php`](php)        | Packagist `caretly/caretly`               | `composer require caretly/caretly`                  |
| C#       | [`csharp`](csharp)  | NuGet `Caretly`                           | `dotnet add package Caretly`                        |
| Java     | [`java`](java)      | Maven `dev.caretly:caretly`               | `implementation("dev.caretly:caretly:0.1.0")`       |

## Shared behaviour

All clients behave the same way:

- Base URL `https://caretly.dev`, configurable for self-hosted servers.
- API key from the constructor, or the `CARETLY_API_KEY` environment variable. Optional,
  as a server without keys enabled is open.
- `render` sends one ZPL document; `renderFiles` sends several files as
  `multipart/form-data`, each rendered from a fresh printer state.
- Options: `format` (`pdf` or `png`), `page`, `dpmm` (6, 8, 12 or 24), `timings`,
  `idempotencyKey` and `compress` (gzip the request body).
- The result holds the file bytes and the response headers: page count, pages per file,
  labels billed, rate limit and server timings.
- 429 and 503 responses and connection errors are retried (2 retries by default), waiting
  for `Retry-After`. Retries share one `Idempotency-Key`, so a retried request is billed
  once.
- Errors are typed per status: bad request (400), authentication (401), payment
  required (402), payload too large (413), unsupported encoding (415), rate limited (429)
  and overloaded (503).

## Releasing

Each client is versioned and released on its own, tagged `clients/<folder>/vX.Y.Z` (for
example `clients/rust/v0.1.0`). Go requires that form for a module in a subdirectory, and
a `clients/php/v*` tag makes the split workflow (`.github/workflows/php-split.yml`)
publish the PHP mirror that Packagist reads. The publishing steps for each registry are in
that client's README.

## License

MIT
