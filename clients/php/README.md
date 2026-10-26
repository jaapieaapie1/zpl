# Caretly PHP client

Client for the [Caretly](https://caretly.dev) API: render ZPL labels to PDF or PNG.

- PHP 8.1 or later.
- Bring your own HTTP client: any [PSR-18](https://www.php-fig.org/psr/psr-18/) client works
  (Guzzle, Symfony HttpClient, …) and is found automatically.
- Retries rate limits and overload automatically, without ever billing a label twice.

## Install

```bash
composer require caretly/caretly
```

If your project has no PSR-18 client yet, add one, for example:

```bash
composer require guzzlehttp/guzzle
# or
composer require symfony/http-client nyholm/psr7
```

## Quickstart

```php
use Caretly\Client;

$caretly = new Client(apiKey: 'zpl_test_…'); // or set CARETLY_API_KEY

$result = $caretly->render("^XA^FO50,50^A0N,50,50^FDHello^FS^XZ");
$result->saveTo('label.pdf');

echo $result->pageCount;    // 1
echo $result->labelsBilled; // 0 for a test key
```

Get an API key in the [console](https://caretly.dev/console/keys). Keys starting with
`zpl_test_` are free and watermark every label with TEST; `zpl_live_` keys are billed per label.

### PNG

```php
use Caretly\Format;
use Caretly\RenderOptions;

$png = $caretly->render($zpl, new RenderOptions(format: Format::Png, page: 2, dpmm: 12));
$png->saveTo('page-2.png');

// or the shorthand
$png = $caretly->renderPng($zpl, page: 2);
```

### Several files

Each file starts from a fresh printer state; pages follow the order of the array.

```php
$result = $caretly->renderFiles([
    'shipping.zpl' => file_get_contents('shipping.zpl'),
    'returns.zpl'  => file_get_contents('returns.zpl'),
]);

$result->filePageCounts; // [2, 1]
```

## Client options

```php
$caretly = new Client(
    apiKey: null,                     // falls back to getenv('CARETLY_API_KEY'); optional for self-hosted servers
    baseUrl: 'https://caretly.dev',
    maxRetries: 2,                    // retries for 429, 503 and connection errors (not timeouts)
    timeout: 30.0,                    // seconds; see below
    httpClient: null,                 // any Psr\Http\Client\ClientInterface
    requestFactory: null,             // Psr\Http\Message\RequestFactoryInterface
    streamFactory: null,              // Psr\Http\Message\StreamFactoryInterface
    sleep: null,                      // Closure(float $seconds): void, used between retries
);
```

PSR-18 has no standard way to set a timeout. When you do not inject `httpClient`, the client
creates a Guzzle or Symfony client itself (whichever is installed) with `timeout` applied, and
otherwise falls back to `php-http/discovery`. When you inject your own client, configure its
timeout there.

## Render options

| Option           | Values                    | Default (server) | Meaning                                              |
|------------------|---------------------------|------------------|------------------------------------------------------|
| `format`         | `Format::Pdf`, `Format::Png` | PDF           | A PDF holds every page; a PNG is one page.           |
| `page`           | int ≥ 1                   | 1                | The page shown in a PNG.                             |
| `dpmm`           | 6, 8, 12, 24              | 8                | Printer resolution in dots per millimetre.           |
| `timings`        | bool                      | false            | Return a `Server-Timing` header (`$result->serverTiming`). |
| `idempotencyKey` | string, 1–255 chars       | generated        | A retry with the same key and content is billed once. |
| `compress`       | bool                      | false            | Gzip the request body (needs ext-zlib).              |

Invalid `page`, `dpmm` or `idempotencyKey` values throw `\InvalidArgumentException` before any request.

## Result

`RenderResult` is read-only:

| Property          | Type              | Meaning                                                 |
|-------------------|-------------------|---------------------------------------------------------|
| `content`         | `string`          | The PDF or PNG bytes.                                   |
| `contentType`     | `string`          | `application/pdf` or `image/png`.                       |
| `pageCount`       | `int`             | Total pages across the input.                           |
| `filePageCounts`  | `list<int>`       | Pages per file for `renderFiles()`, else empty.         |
| `labelsBilled`    | `?int`            | Labels billed (0 for test keys); null without keys.     |
| `rateLimit`       | `?RateLimit`      | `limit` and `remaining` of the burst; null when absent. |
| `serverTiming`    | `?string`         | Raw `Server-Timing` header, with `timings`.             |

`saveTo(string $path)` writes the bytes to a file.

`$caretly->health()` returns `true` when the server answers `GET /health` with `ok`, and
`false` otherwise (it never throws).

## Errors

Every runtime exception extends `Caretly\Exception\CaretlyException`.

| Exception                       | Status | When                                                   |
|---------------------------------|--------|--------------------------------------------------------|
| `BadRequestException`           | 400    | Parse error, invalid parameter, page out of range.     |
| `AuthenticationException`       | 401    | API key missing, not valid, expired or revoked.        |
| `PaymentRequiredException`      | 402    | Billing inactive; live keys paused.                    |
| `PayloadTooLargeException`      | 413    | Body over 100 MiB after decompression.                 |
| `UnsupportedEncodingException`  | 415    | Unsupported `Content-Encoding`.                        |
| `RateLimitedException`          | 429    | Too many requests; see `retryAfter`.                   |
| `OverloadedException`           | 503    | Server overloaded; not billed; see `retryAfter`.       |
| `ApiException`                  | other  | Base of all the above; any other error status.         |
| `TransportException`            | —      | No response: connection or DNS failure.                |
| `TimeoutException`              | —      | Extends `TransportException`; timed out; not retried.  |

`ApiException` has `status` (int), `retryAfter` (?int, seconds) and the response body as its message.

```php
use Caretly\Exception\ApiException;
use Caretly\Exception\BadRequestException;

try {
    $caretly->render($zpl);
} catch (BadRequestException $e) {
    echo "Invalid ZPL: {$e->getMessage()}";
} catch (ApiException $e) {
    echo "Caretly answered {$e->status}: {$e->getMessage()}";
}
```

### Retries

429 and 503 responses and connection errors are retried up to `maxRetries` times (default 2).
Timeouts are not retried: a `TimeoutException` is thrown on the first timeout, because the server
may still be working on the request.
The client waits `Retry-After` seconds when the server sends it, else 0.5 s, 1 s, 2 s, … up to 8 s.
Other statuses are never retried. Unless you pass `idempotencyKey`, the client generates a UUID
per call and sends it on every attempt, so a retried render is never billed twice. Set
`maxRetries: 0` to turn retries off (no key is generated then).

## Development

```bash
composer install
vendor/bin/phpunit
vendor/bin/phpstan analyse
composer validate --strict
```

Tests use a mock PSR-18 client and never touch the network.

## Publishing

This package lives in `clients/php/` of the [monorepo](https://github.com/jaapieaapie1/zpl), but
Packagist needs a repository whose root holds `composer.json`. The workflow
`.github/workflows/php-split.yml` therefore mirrors `clients/php/` to the read-only repository
[jaapieaapie1/caretly-php](https://github.com/jaapieaapie1/caretly-php) on every push to `main`,
and turns a monorepo tag `clients/php/vX.Y.Z` into the tag `vX.Y.Z` there. Packagist watches the mirror.
Never commit to the mirror directly; changes go through the monorepo.

### One-time setup

1. Create an empty public repository `jaapieaapie1/caretly-php` on GitHub (no README, no license).
2. Create an SSH key pair (`ssh-keygen -t ed25519 -N '' -f split-key`). Add the public key
   to the mirror as a deploy key with write access
   (`gh repo deploy-key add split-key.pub --repo jaapieaapie1/caretly-php --allow-write`), and
   the private key to the monorepo as the Actions secret `PHP_SPLIT_DEPLOY_KEY`
   (`gh secret set PHP_SPLIT_DEPLOY_KEY --repo jaapieaapie1/zpl < split-key`). Then delete
   both files.
3. Run the *Split PHP client* workflow once (or push a change under `clients/php/`) so the mirror
   gets its `main` branch.
4. On [packagist.org](https://packagist.org/packages/submit), submit
   `https://github.com/jaapieaapie1/caretly-php`. The package name `caretly/caretly` comes from
   `composer.json`.
5. Turn on automatic updates: connect your GitHub account in the Packagist profile, or add the
   Packagist webhook to the mirror repository (Settings → Webhooks, payload URL
   `https://packagist.org/api/github?username=<packagist user>`, content type `application/json`,
   secret = your Packagist API token).

### Release

1. Update `CHANGELOG.md` (move entries under the new version and date). `composer.json` has no
   `version` field; the tag sets the version. Also update `Client::VERSION` (used in the
   `User-Agent`).
2. Commit to `main`, then tag and push:

   ```bash
   git tag clients/php/v0.1.0
   git push origin clients/php/v0.1.0
   ```

3. The workflow pushes tag `v0.1.0` to the mirror and Packagist publishes it within a minute.
   Check https://packagist.org/packages/caretly/caretly.

`.gitattributes` keeps tests and dev configuration out of the installed package (`export-ignore`).

## License

MIT, see [LICENSE](LICENSE).
