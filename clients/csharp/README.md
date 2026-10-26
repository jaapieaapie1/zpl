# Caretly for .NET

The official .NET client for the [Caretly](https://caretly.dev) API: render ZPL labels to
vector PDF or PNG.

- Targets `netstandard2.0` and `net8.0`, so it runs on .NET Framework 4.6.1+, .NET Core and
  .NET 5+.
- No runtime dependencies.
- Async throughout, with typed errors and automatic retries that are never billed twice.

## Install

```sh
dotnet add package Caretly
```

## Quickstart

```csharp
using Caretly;

using var client = new CaretlyClient("zpl_test_..."); // or set CARETLY_API_KEY

var pdf = await client.RenderAsync("^XA^FO50,50^A0N,50,50^FDHello, world^FS^XZ");
await pdf.SaveToAsync("label.pdf");
Console.WriteLine($"{pdf.PageCount} page(s), {pdf.LabelsBilled} billed");

var png = await client.RenderAsync(zpl, new RenderOptions
{
    Format = RenderFormat.Png,
    Page = 2,
    Dpmm = 12, // 300 dpi
});
await png.SaveToAsync("label.png");
```

Get an API key in the [console](https://caretly.dev/console/keys). Keys starting with
`zpl_test_` are free and watermark every label with TEST; `zpl_live_` keys are billed per
label. The key is optional for a self-hosted server that runs without keys.

### Several files in one request

`RenderFilesAsync` sends files as `multipart/form-data`. Each file starts from a fresh
printer state, and the pages follow in the given order.

```csharp
var result = await client.RenderFilesAsync(new[]
{
    new ZplFile("shipping.zpl", shippingZpl),
    ZplFile.FromPath("invoice.zpl"),
});
Console.WriteLine(string.Join(", ", result.FilePageCounts)); // pages per file, e.g. "2, 1"
```

### Health check

```csharp
bool up = await client.HealthAsync(); // GET /health
```

## Configuration

```csharp
using var client = new CaretlyClient(new CaretlyClientOptions
{
    ApiKey = "zpl_live_...",               // default: CARETLY_API_KEY
    BaseUrl = "http://localhost:3000",     // default: https://caretly.dev
    Timeout = TimeSpan.FromSeconds(60),    // per attempt; default 30 s
    MaxRetries = 3,                        // default 2; 0 disables retries
});
```

### With IHttpClientFactory

Pass your own `HttpClient`. The client does not dispose it. When `BaseUrl` is not set, the
`HttpClient.BaseAddress` is used if it has one.

```csharp
services.AddHttpClient("caretly", http => http.BaseAddress = new Uri("https://caretly.dev"));
services.AddSingleton(sp => new CaretlyClient(
    sp.GetRequiredService<IHttpClientFactory>().CreateClient("caretly"),
    new CaretlyClientOptions { ApiKey = configuration["Caretly:ApiKey"] }));
```

## Render options

| Property         | Values                  | Default (server) | Meaning                                                    |
|------------------|-------------------------|------------------|------------------------------------------------------------|
| `Format`         | `Pdf`, `Png`            | `Pdf`            | A PDF holds every page; a PNG is one page.                 |
| `Page`           | 1 or more               | 1                | The page of a PNG, counted across all files.               |
| `Dpmm`           | 6, 8, 12, 24            | 8                | Dots per millimetre (152, 203, 300, 600 dpi).              |
| `Timings`        | `bool`                  | off              | Return per-stage timings in `ServerTiming`.                |
| `IdempotencyKey` | up to 255 ASCII chars   | generated        | A repeat with the same key and labels is billed once.      |
| `Compress`       | `bool`                  | off              | Gzip the request body.                                     |

`Page`, `Dpmm` and `IdempotencyKey` are checked before sending and throw an
`ArgumentException` when invalid.

## Results

`RenderResult` exposes:

| Property         | Type                 | Header                                       |
|------------------|----------------------|----------------------------------------------|
| `Content`        | `byte[]`             | body                                         |
| `ContentType`    | `string`             | `Content-Type`                               |
| `PageCount`      | `int`                | `X-Page-Count`                               |
| `FilePageCounts` | `IReadOnlyList<int>` | `X-File-Page-Counts` (empty for one label)   |
| `LabelsBilled`   | `int?`               | `X-Labels-Billed` (0 for test keys)          |
| `RateLimit`      | `RateLimitInfo?`     | `RateLimit-Limit`, `RateLimit-Remaining`     |
| `ServerTiming`   | `string?`            | `Server-Timing`                              |

`SaveTo(path)` and `SaveToAsync(path)` write `Content` to a file.

## Errors

Every error derives from `CaretlyException`. Error responses throw a `CaretlyApiException`
subclass with `StatusCode`, `ResponseBody` (the server's message) and `RetryAfter`.

| Status | Exception                      | When                                                |
|--------|--------------------------------|-----------------------------------------------------|
| 400    | `BadRequestException`          | ZPL parse error, invalid parameter, page out of range |
| 401    | `AuthenticationException`      | Key missing, invalid, expired or revoked             |
| 402    | `PaymentRequiredException`     | Billing inactive; live keys are paused               |
| 413    | `PayloadTooLargeException`     | Body over 100 MiB after decompression                |
| 415    | `UnsupportedEncodingException` | Unsupported `Content-Encoding`                       |
| 429    | `RateLimitedException`         | Rate limited (retried automatically)                 |
| 503    | `OverloadedException`          | Server busy, not billed (retried automatically)      |
| other  | `CaretlyApiException`          | Any other status, including 5xx                      |
| —      | `CaretlyConnectionException`   | The server could not be reached                      |
| —      | `CaretlyTimeoutException`      | An attempt took longer than `Timeout`                |

Cancelling the `CancellationToken` throws an `OperationCanceledException`, not a
`CaretlyTimeoutException`.

```csharp
try
{
    await client.RenderAsync(zpl);
}
catch (BadRequestException e)
{
    Console.Error.WriteLine($"Invalid label: {e.ResponseBody}");
}
catch (CaretlyApiException e) when (e.StatusCode >= 500)
{
    Console.Error.WriteLine($"Server error {e.StatusCode}");
}
```

## Retries

429 and 503 responses and connection errors are retried up to `MaxRetries` times (2 by
default). The client waits `Retry-After` when the server sends it, and otherwise backs off
exponentially: 0.5 s, 1 s, 2 s, up to 8 s. Other errors and timeouts are not retried.

When retries are on and you did not set `IdempotencyKey`, the client generates a UUID once
per call and sends it with every attempt, so a retried request is never billed twice.

## Development

```sh
dotnet build -c Release
dotnet test
dotnet pack -c Release -o artifacts
```

The tests use a fake `HttpMessageHandler` and need no network.

## Publishing

1. Update `<Version>` in `src/Caretly/Caretly.csproj` and add an entry to `CHANGELOG.md`.
2. Build and pack in CI mode, so Source Link paths are deterministic:

   ```sh
   dotnet pack src/Caretly -c Release -o artifacts -p:ContinuousIntegrationBuild=true
   ```

3. Push the package; the matching `.snupkg` symbol package is pushed with it:

   ```sh
   dotnet nuget push artifacts/Caretly.0.1.0.nupkg --api-key "$NUGET_API_KEY" --source https://api.nuget.org/v3/index.json
   ```

4. Tag the release, for example `csharp-v0.1.0`.

## Links

- Caretly: https://caretly.dev
- Source: https://github.com/jaapieaapie1/zpl/tree/main/clients/csharp

## License

MIT. See [LICENSE](LICENSE).
