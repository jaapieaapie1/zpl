# caretly

Python client for the [Caretly](https://caretly.dev) API: render ZPL labels to PDF or PNG.

- Sync (`caretly.Client`) and asyncio (`caretly.AsyncClient`) clients with the same API.
- One dependency: [`httpx`](https://www.python-httpx.org/).
- Typed (`py.typed`), Python 3.9+.
- Retries rate limits (429), overload (503) and connection errors, with an
  `Idempotency-Key` so a retry is never billed twice.

## Install

```sh
pip install caretly
```

## Quickstart

Create an API key in the [console](https://caretly.dev/console/keys). Live keys start
with `zpl_live_`, test keys with `zpl_test_` (test keys are free and watermark every label).

```python
import caretly

client = caretly.Client(api_key="zpl_test_...")  # or set CARETLY_API_KEY

result = client.render("^XA^FO50,50^A0N,50,50^FDHello^FS^XZ")
result.save("label.pdf")
print(result.page_count, result.labels_billed)

png = client.render("^XA^FO50,50^A0N,50,50^FDHello^FS^XZ", format="png", dpmm=12)
png.save("label.png")
```

Several files in one request, each with a fresh printer state, pages in upload order:

```python
result = client.render_files({"order-1.zpl": zpl_1, "order-2.zpl": zpl_2})
print(result.file_page_counts)  # (2, 1)
```

`files` is a mapping of name to content, or a sequence of `(name, content)` pairs.
Content is `str` (sent as UTF-8) or `bytes`.

Async:

```python
import asyncio
import caretly


async def main() -> None:
    async with caretly.AsyncClient() as client:
        result = await client.render("^XA^FDHello^FS^XZ")
        result.save("label.pdf")


asyncio.run(main())
```

Close the client when done (`client.close()` / `await client.aclose()`), or use it as a
(async) context manager.

## Client options

| Argument      | Default                 | Meaning                                                       |
|---------------|-------------------------|---------------------------------------------------------------|
| `api_key`     | `$CARETLY_API_KEY`      | Sent as `Authorization: Bearer`. Optional for self-hosted servers. |
| `base_url`    | `https://caretly.dev`   | Server URL.                                                   |
| `timeout`     | `30.0`                  | Seconds per request, an `httpx.Timeout`, or `None` for none.  |
| `max_retries` | `2`                     | Retries for 429, 503 and connection errors.                   |
| `http_client` | new client              | Your own `httpx.Client` / `httpx.AsyncClient` (proxies, transports, ...). You keep ownership: closing the Caretly client does not close it. |

## Render options

`render(zpl, ...)` and `render_files(files, ...)` take these keyword arguments:

| Option            | Default  | Meaning                                                              |
|-------------------|----------|----------------------------------------------------------------------|
| `format`          | `"pdf"`  | `"pdf"` holds every page; `"png"` is the single page chosen with `page`. |
| `page`            | server: 1 | For PNG, the page to render, starting at 1. Counts across all files. |
| `dpmm`            | server: 8 | Printer resolution in dots per mm: 6, 8, 12 or 24 (152-600 dpi).    |
| `timings`         | `False`  | Ask for a `Server-Timing` header (`result.server_timing`).           |
| `idempotency_key` | generated | Up to 255 characters, e.g. an order number. A retry with the same key and labels is billed once. When unset and `max_retries > 0`, a UUID is generated per call. |
| `compress`        | `False`  | Gzip the request body. Useful for large batches.                     |

`page` and `dpmm` are checked before sending and raise `ValueError` when out of range.

`render_pdf(zpl, ...)` and `render_png(zpl, page=None, ...)` are shorthands.

## Result

`RenderResult` is a frozen dataclass:

| Field              | Type                  | Meaning                                         |
|--------------------|-----------------------|-------------------------------------------------|
| `content`          | `bytes`               | The PDF or PNG.                                 |
| `content_type`     | `str`                 | `application/pdf` or `image/png`.               |
| `page_count`       | `int`                 | Total pages across the input.                   |
| `file_page_counts` | `tuple[int, ...]`     | `render_files` only: pages per file; else `()`. |
| `labels_billed`    | `int \| None`         | Labels billed (0 for test keys); `None` without keys. |
| `rate_limit`       | `RateLimit \| None`   | `limit` (burst size) and `remaining`.           |
| `server_timing`    | `str \| None`         | Raw `Server-Timing` header, with `timings=True`. |

`result.save(path)` writes `content` to a file and returns the `pathlib.Path`.

`client.health()` returns `True` when `GET /health` answers `ok`, `False` for any other
response, and raises `APIConnectionError` when the server cannot be reached.

## Errors

All errors derive from `caretly.CaretlyError`.

| Exception                  | When                                                   |
|----------------------------|--------------------------------------------------------|
| `BadRequestError`          | 400: ZPL parse error, invalid parameter, page out of range. |
| `AuthenticationError`      | 401: API key missing, not valid, expired or revoked.  |
| `PaymentRequiredError`     | 402: billing inactive; live keys paused (test keys keep working). |
| `PayloadTooLargeError`     | 413: body over 100 MiB after decompression.           |
| `UnsupportedEncodingError` | 415: unsupported `Content-Encoding`.                  |
| `RateLimitError`           | 429: too many requests for this key.                  |
| `OverloadedError`          | 503: the server is shedding load (not billed).        |
| `APIError`                 | Base class of the above, and any other status (e.g. 500). |
| `APIConnectionError`       | No response: network or protocol failure.             |
| `APITimeoutError`          | The request timed out (a subclass of `APIConnectionError`). |

`APIError` has `status_code`, `message` (the server's plain-text explanation),
`retry_after` (seconds, or `None`) and `response` (the `httpx.Response`).

```python
try:
    client.render(zpl)
except caretly.BadRequestError as e:
    print("Fix the label:", e.message)
except caretly.APIError as e:
    print(e.status_code, e.message)
```

### Retries

429 and 503 responses and connection errors are retried up to `max_retries` times
(default 2). Timeouts are not retried: `APITimeoutError` is raised on the first one. The client waits `Retry-After` seconds when the server
sends it, else 0.5 s, 1 s, 2 s, ... up to 8 s. Other statuses are not retried. Set
`max_retries=0` to turn retries off.

## Development

```sh
python -m venv .venv
.venv/bin/pip install -e . --group dev   # pip >= 25.1
.venv/bin/pytest
.venv/bin/mypy --strict src
.venv/bin/ruff check . && .venv/bin/ruff format --check .
```

## Publishing

1. Bump `__version__` in `src/caretly/_version.py` and add a `CHANGELOG.md` entry.
2. Build and check:

   ```sh
   rm -rf dist
   .venv/bin/python -m build
   .venv/bin/twine check dist/*
   ```

3. Optionally try TestPyPI first: `.venv/bin/twine upload --repository testpypi dist/*`.
4. Upload: `.venv/bin/twine upload dist/*` (with a PyPI API token as password, user
   `__token__`), or set up [trusted publishing](https://docs.pypi.org/trusted-publishers/).
5. Tag the release, e.g. `git tag clients/python/v0.1.0 && git push origin clients/python/v0.1.0`.

## Links

- Caretly: https://caretly.dev
- Source: https://github.com/jaapieaapie1/zpl (this client lives in `clients/python`)

## License

MIT, see [LICENSE](LICENSE).
