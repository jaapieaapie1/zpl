# caretly

Client for the [Caretly](https://caretly.dev) API: render ZPL labels to PDF or PNG.

- No runtime dependencies; built on the standard `fetch`.
- ESM and CommonJS, with TypeScript types.
- Node.js 18 or newer. The core also runs in Deno, Bun and edge runtimes that have
  `fetch`; only `saveTo` needs `node:fs`.

## Install

```sh
npm install @caretly/caretly
```

## Quickstart

Create an API key in the [console](https://caretly.dev/console/keys). Test keys
(`zpl_test_…`) are free and watermark every label; live keys (`zpl_live_…`) are billed
per label.

```ts
import { Caretly } from "@caretly/caretly";

const caretly = new Caretly({ apiKey: process.env.CARETLY_API_KEY });

const pdf = await caretly.render("^XA^FO50,50^A0N,40^FDHello^FS^XZ");
await pdf.saveTo("label.pdf");
console.log(pdf.pageCount, pdf.labelsBilled);

const png = await caretly.render(zpl, { format: "png", page: 2, dpmm: 12 });
const bytes: Uint8Array = png.bytes;
```

CommonJS works too:

```js
const { Caretly } = require("@caretly/caretly");
```

### Several files in one request

Each file starts from a fresh printer state, and pages follow upload order.

```ts
const result = await caretly.renderFiles([
  { name: "order-1.zpl", content: zplText },
  { name: "order-2.zpl", content: await readFile("order-2.zpl") }, // Uint8Array
  { name: "order-3.zpl", content: blob },                           // Blob
]);
result.filePageCounts; // [2, 1, 1]
```

### Health

```ts
await caretly.health(); // true when the server answers `ok`
```

## Client options

| Option       | Default                         | Meaning                                                   |
|--------------|---------------------------------|-----------------------------------------------------------|
| `apiKey`     | `process.env.CARETLY_API_KEY`   | Sent as `Authorization: Bearer`. Optional for self-hosted servers. |
| `baseUrl`    | `https://caretly.dev`           | Point at a self-hosted server.                            |
| `timeoutMs`  | `30000`                         | Timeout per attempt, including reading the response.      |
| `maxRetries` | `2`                             | Retries after 429, 503 or a connection error.             |
| `fetch`      | global `fetch`                  | Custom `fetch`, for example for a proxy or for tests.     |
| `sleep`      | timer                           | Waits between retries; replace it in tests.               |

## Render options

`render(zpl, options)` and `renderFiles(files, options)` take:

| Option           | Values                  | Default | Meaning                                         |
|------------------|-------------------------|---------|-------------------------------------------------|
| `format`         | `"pdf"`, `"png"`        | `"pdf"` | A PDF holds every page; a PNG is one page.      |
| `page`           | integer, 1 or more      | `1`     | The page shown in a PNG.                        |
| `dpmm`           | `6`, `8`, `12`, `24`    | `8`     | Printer resolution in dots per millimetre.      |
| `timings`        | boolean                 | off     | Ask for server timings (`result.serverTiming`). |
| `idempotencyKey` | string, up to 255 chars | auto    | A retry with the same key and content is billed once. |
| `compress`       | boolean                 | off     | Gzip the request body.                          |
| `signal`         | `AbortSignal`           |         | Cancel the call, including waits between retries. |

Options left out are not sent, so the server defaults apply. An invalid `format`, `page`,
`dpmm` or `idempotencyKey` throws a `RangeError` before any request is made.

`renderPdf(zpl, options)` and `renderPng(zpl, page?, options)` are shorthands.

## Result

| Field            | Type                  | Meaning                                                  |
|------------------|-----------------------|----------------------------------------------------------|
| `bytes`          | `Uint8Array`          | The PDF or PNG.                                          |
| `contentType`    | `string`              | `application/pdf` or `image/png`.                        |
| `pageCount`      | `number`              | Total pages across the input.                            |
| `filePageCounts` | `number[]`            | Pages per file for `renderFiles`; empty otherwise.       |
| `labelsBilled`   | `number \| null`      | Labels billed; 0 for a test key; `null` without keys.    |
| `rateLimit`      | `{ limit, remaining } \| null` | The key's burst size and what is left of it.    |
| `serverTiming`   | `string \| null`      | Raw `Server-Timing` header with `timings`.               |
| `headers`        | `Headers`             | All response headers.                                    |
| `saveTo(path)`   | `Promise<void>`       | Write `bytes` to a file (needs `node:fs`).               |

## Errors

Every error from the API or the network extends `CaretlyError`:

| Class                      | When                                                        |
|----------------------------|-------------------------------------------------------------|
| `BadRequestError`          | 400: ZPL parse error, invalid parameter, page out of range. |
| `AuthenticationError`      | 401: API key missing, not valid, expired or revoked.        |
| `PaymentRequiredError`     | 402: billing inactive; live keys are paused.                |
| `PayloadTooLargeError`     | 413: body over 100 MiB after decompression.                 |
| `UnsupportedEncodingError` | 415: unsupported `Content-Encoding`.                        |
| `RateLimitError`           | 429: too many requests; see `retryAfter`.                   |
| `OverloadedError`          | 503: server overloaded; not billed; see `retryAfter`.       |
| `APIError`                 | Base of the above, and any other status (such as 500).      |
| `ConnectionError`          | The request did not reach the server or the connection broke. |
| `TimeoutError`             | An attempt took longer than `timeoutMs`.                    |

`APIError` has `status`, `message` (the server's text), `retryAfter` (seconds or `null`)
and `headers`. Aborting with your own `signal` rejects with the signal's reason (an
`AbortError` by default), not a `CaretlyError`.

```ts
import { BadRequestError, RateLimitError } from "@caretly/caretly";

try {
  await caretly.render(zpl);
} catch (error) {
  if (error instanceof BadRequestError) console.error("Invalid label:", error.message);
  else if (error instanceof RateLimitError) console.error(`Retry in ${error.retryAfter} s`);
  else throw error;
}
```

`instanceof` works even when an application loads both the ESM and the CommonJS build.

## Retries

429, 503 and connection errors are retried up to `maxRetries` times. The client waits
`Retry-After` seconds when the server sends it, and otherwise backs off 0.5 s, 1 s, 2 s, …
up to 8 s. Other statuses and timeouts are not retried. When retries are enabled and you
pass no `idempotencyKey`, the client generates a UUID per call and sends it on every
attempt, so a retried request is never billed twice.

## Development

```sh
npm install
npm run typecheck
npm test            # builds, then runs the tests against a local mock server
npm run lint:package  # publint and @arethetypeswrong/cli
```

## Publishing

1. Update `version` in `package.json` and `VERSION` in `src/version.ts` (a test checks
   they match), and add a section to `CHANGELOG.md`.
2. Check the package:

   ```sh
   npm ci
   npm run typecheck && npm test && npm run lint:package
   npm pack --dry-run
   ```

3. Publish (`prepublishOnly` runs the checks again):

   ```sh
   npm login
   npm publish --access public
   ```

4. Tag the release: `git tag clients/node/v0.1.0 && git push origin clients/node/v0.1.0`.

## Links

- Caretly: https://caretly.dev
- Source: https://github.com/jaapieaapie1/zpl/tree/main/clients/node

## License

MIT
