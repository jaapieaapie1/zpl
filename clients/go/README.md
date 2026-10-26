# caretly (Go)

Go client for the [Caretly](https://caretly.dev) API: render ZPL labels to PDF or PNG.

- Standard library only, Go 1.22 or newer.
- Retries rate limits (429), overload (503) and connection errors, without double billing.
- Typed errors that work with `errors.Is` and `errors.As`.

## Install

```sh
go get github.com/jaapieaapie1/zpl/clients/go@latest
```

```go
import caretly "github.com/jaapieaapie1/zpl/clients/go"
```

## Quickstart

```go
package main

import (
	"context"
	"log"

	caretly "github.com/jaapieaapie1/zpl/clients/go"
)

func main() {
	// Reads CARETLY_API_KEY when WithAPIKey is not given.
	client := caretly.NewClient(caretly.WithAPIKey("zpl_test_..."))

	res, err := client.Render(context.Background(), "^XA^FO50,50^A0N,50,50^FDHello^FS^XZ", nil)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("%d page(s), %s", res.PageCount, res.ContentType)
	if err := res.Save("label.pdf"); err != nil {
		log.Fatal(err)
	}
}
```

Live keys (`zpl_live_…`) are billed per label. Test keys (`zpl_test_…`) are free and
watermark every label with TEST. Create keys at <https://caretly.dev/console/keys>.

### Several files

Each file starts from a fresh printer state; pages follow upload order.

```go
res, err := client.RenderFiles(ctx, []caretly.File{
	{Name: "order-1.zpl", Content: []byte(zpl1)},
	{Name: "order-2.zpl", Content: zpl2Bytes},
}, nil)
// res.FilePageCounts holds the pages of each file, for example [2 1].
```

### PNG

```go
res, err := client.Render(ctx, zpl, &caretly.RenderOptions{
	Format: caretly.FormatPNG,
	Page:   2,  // pages start at 1
	DPMM:   12, // 300 dpi
})
// or: client.RenderPNG(ctx, zpl, 2)
```

## Client options

| Option                 | Default                       | Meaning                                         |
|------------------------|-------------------------------|-------------------------------------------------|
| `WithAPIKey(key)`      | `$CARETLY_API_KEY`            | Sent as `Authorization: Bearer <key>`. Empty sends none (self-hosted). |
| `WithBaseURL(url)`     | `https://caretly.dev`         | API base URL, for example a self-hosted server. |
| `WithHTTPClient(hc)`   | new `http.Client`             | Custom transport, proxies, …                    |
| `WithTimeout(d)`       | 30 s                          | Limit per HTTP attempt; `0` disables it.        |
| `WithMaxRetries(n)`    | 2                             | Retries after 429, 503 or a connection error.   |

Every method takes a `context.Context`; its deadline and cancellation cover the whole call,
retry waits included.

## Render options

`RenderOptions` (pass `nil` for the server defaults). Unset fields are not sent, so the
server defaults apply.

| Field            | Values                      | Default | Meaning                                         |
|------------------|-----------------------------|---------|-------------------------------------------------|
| `Format`         | `FormatPDF`, `FormatPNG`    | PDF     | A PDF holds every page; a PNG is one page.      |
| `Page`           | `>= 1`                      | 1       | Page shown in a PNG, counted across all files.  |
| `DPMM`           | `6`, `8`, `12`, `24`        | 8       | Printer resolution in dots per millimetre.      |
| `Timings`        | `bool`                      | false   | Ask for a `Server-Timing` header.               |
| `IdempotencyKey` | string, max 255 characters  | auto    | A retry with the same key and content is billed once. |
| `Compress`       | `bool`                      | false   | Gzip the request body.                          |

Invalid values fail before any request with an error matching `caretly.ErrInvalidOptions`.

## Result

`*RenderResult`:

| Field            | Type         | Meaning                                                    |
|------------------|--------------|------------------------------------------------------------|
| `Content`        | `[]byte`     | The PDF or PNG.                                            |
| `ContentType`    | `string`     | `application/pdf` or `image/png`.                          |
| `PageCount`      | `int`        | Total pages across the input.                              |
| `FilePageCounts` | `[]int`      | Pages per file for `RenderFiles`; empty otherwise.         |
| `LabelsBilled`   | `*int`       | Labels billed (0 for test keys); nil without API keys.     |
| `RateLimit`      | `*RateLimit` | `Limit` and `Remaining` of the key's burst; nil if absent. |
| `ServerTiming`   | `string`     | Raw `Server-Timing` header with `Timings`; empty otherwise.|

`res.Save(path)` writes `Content` to a file.

`client.Health(ctx)` returns `true` when `GET /health` answers `ok`.

## Errors

A response other than 200 is an `*caretly.APIError` with `StatusCode`, `Message` (the
server's plain-text message) and `RetryAfter` (zero when absent).

| Status | Sentinel                         |
|--------|----------------------------------|
| 400    | `caretly.ErrBadRequest`          |
| 401    | `caretly.ErrUnauthorized`        |
| 402    | `caretly.ErrPaymentRequired`     |
| 413    | `caretly.ErrPayloadTooLarge`     |
| 415    | `caretly.ErrUnsupportedEncoding` |
| 429    | `caretly.ErrRateLimited`         |
| 503    | `caretly.ErrOverloaded`          |
| any    | `caretly.ErrAPI`                 |

Transport failures are a `*caretly.TransportError` matching `caretly.ErrConnection`, or
`caretly.ErrTimeout` when an attempt ran over the client timeout. When the context is
cancelled or its deadline passes, the context's error is returned.

```go
res, err := client.Render(ctx, zpl, nil)
switch {
case errors.Is(err, caretly.ErrBadRequest):
	var apiErr *caretly.APIError
	errors.As(err, &apiErr)
	log.Printf("bad ZPL: %s", apiErr.Message)
case errors.Is(err, caretly.ErrUnauthorized):
	log.Print("check your API key")
case err != nil:
	log.Print(err)
}
```

## Retries

429, 503 and connection errors are retried up to `WithMaxRetries` times (default 2). The
client waits `Retry-After` when the server sends it, else 0.5 s, 1 s, 2 s, … up to 8 s.
Other statuses and timeouts are not retried. When retries are enabled and you set no
`IdempotencyKey`, the client generates a UUID once per call and sends it on every attempt,
so a retried request is never billed twice.

## Development

```sh
gofmt -l .
go vet ./...
go test -race ./...
```

## Publishing

The module lives in the `clients/go` subdirectory of
<https://github.com/jaapieaapie1/zpl>, so Go expects its version tags to carry that
prefix: `clients/go/vX.Y.Z`, not `vX.Y.Z`.

1. Update `Version` in `client.go` and add an entry to `CHANGELOG.md`.
2. Check the module: `gofmt -l . && go vet ./... && go test -race ./...` (from `clients/go`).
3. Commit, then tag and push from the repository root:

   ```sh
   git tag clients/go/v0.1.0
   git push origin clients/go/v0.1.0
   ```

4. Ask the module proxy to index the version (optional; it happens on first `go get`):

   ```sh
   GOPROXY=proxy.golang.org go list -m github.com/jaapieaapie1/zpl/clients/go@v0.1.0
   ```

5. The documentation appears at
   <https://pkg.go.dev/github.com/jaapieaapie1/zpl/clients/go>.

A major version 2 or later needs a `/v2` suffix in the module path (`.../clients/go/v2`)
and tags like `clients/go/v2.0.0`.

## Links

- Caretly: <https://caretly.dev>
- Source: <https://github.com/jaapieaapie1/zpl>

## License

MIT, see [LICENSE](LICENSE).
