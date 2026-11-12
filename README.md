# ZPL to PDF

Render ZPL labels to PDF or PNG. This is the engine behind [Caretly](https://caretly.dev),
a hosted API for rendering ZPL. You can use the hosted API, run the server yourself, or
use the CLI and Rust libraries directly.

There is a live editor that renders in the browser with WebAssembly at
[caretly.dev/playground](https://caretly.dev/playground/).

## Install

### Hosted API (caretly.dev)

Get an API key from the [Caretly console](https://caretly.dev/console) and send ZPL to
`/render`:

```sh
curl --data-binary @label.zpl \
  -H "Authorization: Bearer $CARETLY_API_KEY" \
  'https://caretly.dev/render?format=pdf' -o label.pdf
```

Or use one of the client libraries:

| Language | Install                                             |
|----------|-----------------------------------------------------|
| Rust     | `cargo add caretly`                                 |
| Go       | `go get github.com/jaapieaapie1/zpl/clients/go`     |
| Python   | `pip install caretly`                               |
| Node.js  | `npm install @caretly/caretly`                      |
| PHP      | `composer require caretly/caretly`                  |
| C#       | `dotnet add package Caretly`                        |
| Java     | `implementation("dev.caretly:caretly:0.1.0")`       |

See [`clients/README.md`](clients/README.md) for their shared behaviour. Every client
takes a base URL, so they work against a self-hosted server too.

### CLI

Requires a recent stable Rust toolchain (edition 2024).

```sh
git clone https://github.com/jaapieaapie1/zpl.git
cd zpl
cargo install --path cli
```

The CLI reads ZPL from stdin and writes to the given file. The extension picks the
format (`.pdf` or `.png`, default `output.png`):

```sh
zpl-cli label.pdf < label.zpl
zpl-cli label.png --dpmm 12 < label.zpl
```

Flags:

- `--dpmm N` – printer resolution in dots per millimetre: 6, 8, 12 or 24 (default 8).
- `--timings`, `-t` – print the time spent in each pipeline stage.
- `--debug`, `-d` – draw debug overlays in PNG output.

## Self-hosting

The server (`zpl-server`, in [`http-api`](http-api)) serves the same API as caretly.dev,
plus the `/playground` editor. Without API keys configured it is open, no account needed.

### Docker

```sh
docker run --rm -p 3000:3000 ghcr.io/jaapieaapie1/zpl:latest
```

The image is built from `main` (`latest`, and a tag per commit SHA) and from `v*`
release tags. To build it yourself:

```sh
docker build -t zpl .
docker run --rm -p 3000:3000 zpl
```

### From source

```sh
cargo run --release --bin zpl-server
```

This listens on `127.0.0.1:3000`. The playground needs the WebAssembly build of
`zpl-wasm` (see the `wasm` stage in the [`Dockerfile`](Dockerfile)) and
`ZPL_PLAYGROUND_DIR` pointing at it.

### Usage

```sh
# One label to PDF
curl --data-binary @label.zpl 'http://localhost:3000/render?format=pdf' -o label.pdf

# Page 2 as a PNG at 12 dpmm
curl --data-binary @label.zpl 'http://localhost:3000/render?format=png&page=2&dpmm=12' -o label.png

# Several files in one PDF, each from a fresh printer state
curl -F file=@a.zpl -F file=@b.zpl 'http://localhost:3000/render?format=pdf' -o labels.pdf
```

- `format` – `pdf` (default) or `png`. Every label (`^XA…^XZ`, times its `^PQ` copies)
  is a page; a PDF holds them all, a PNG shows one.
- `page` – the page to render as PNG, 1-based (default 1).
- `dpmm` – 6, 8, 12 or 24 (default 8).
- `timings` – add a `Server-Timing` header.
- Bodies may be sent with `Content-Encoding: gzip` or `deflate`; the decompressed request
  is capped at 100 MiB.
- `X-Page-Count` gives the page count, `X-File-Page-Counts` the pages per uploaded file.
- `GET /health` returns `ok`; `GET /playground/` serves the editor.

### Configuration

| Variable                    | Default            | Purpose                                                   |
|-----------------------------|--------------------|-----------------------------------------------------------|
| `ZPL_API_ADDR`              | `127.0.0.1:3000` (`0.0.0.0:3000` in Docker) | Listen address                   |
| `ZPL_PLAYGROUND_DIR`        | unset (`/playground` in Docker) | Directory served at `/playground/`           |
| `ZPL_RENDER_CONCURRENCY`    | CPUs available     | Renders running at once                                   |
| `ZPL_RENDER_QUEUE`          | 4 per render slot  | Requests that may wait; beyond that `/render` returns 503 |
| `ZPL_RENDER_MAX_WAIT_MS`    | `5000`             | Longest wait before a 503                                 |

API keys, rate limiting and usage reporting (`ZPL_KEYS_PUBLIC_KEYS`,
`ZPL_RATE_LIMIT_*`, `ZPL_CONSOLE_*`, `ZPL_AMQP_URL`) are for running alongside
[zpl-console](https://github.com/jaapieaapie1/zpl-console), as caretly.dev does; see
[`http-api/src/billing.rs`](http-api/src/billing.rs). Leave them unset for a private
server.

## Structure

| Path                                     | What                                                            |
|------------------------------------------|-----------------------------------------------------------------|
| [`zpl-tokenizer`](zpl-tokenizer)         | Lossy tokenizer for the ZPL format                              |
| [`zpl-parser`](zpl-parser)               | Turns tokens into command specific structures                   |
| [`zpl-state-manager`](zpl-state-manager) | Tracks printer settings and defaults, emits renderer-agnostic draw operations |
| [`zpl-symbols`](zpl-symbols)             | Renderer-agnostic barcode symbol encoders                       |
| [`pdf-renderer`](pdf-renderer)           | Vector PDF renderer                                             |
| [`png-renderer`](png-renderer)           | Raster PNG renderer                                             |
| [`cli`](cli)                             | `zpl-cli`, ZPL to PDF/PNG from the command line                 |
| [`http-api`](http-api)                   | `zpl-server`, the REST API                                      |
| [`zpl-wasm`](zpl-wasm)                   | WebAssembly bindings and the `/playground` editor               |
| [`zpl-anonymize`](zpl-anonymize)         | Replaces field data with random characters, for bug reports     |
| [`compat`](compat)                       | Compatibility tests against Labelary reference renders          |
| [`clients`](clients)                     | Client libraries for the HTTP API                               |
| [`docs`](docs)                           | ZPL, SGD and ZBI command reference                              |

## Design choices

To keep a project like this maintainable we must have a clear separation of concerns.
To achieve this we split this project into multiple semi independent libraries.
Every library of the core renderer is part of a pipeline:

- tokenizer
- parser
- state manager
- renderer (PDF or PNG)

Each step in this pipeline may depend on the one before it, but must not expose anything
of the previous step to the next step.

## License

[MIT](LICENSE)
