# Caretly for Java

Client for the [Caretly](https://caretly.dev) API: render ZPL labels to PDF or PNG.

- Java 11 or newer, no runtime dependencies (uses `java.net.http.HttpClient`).
- Blocking and asynchronous (`CompletableFuture`) calls.
- Typed errors, automatic retries of rate-limited and overloaded requests, never billed twice.
- Named module `dev.caretly`.

## Install

Gradle (Kotlin DSL):

```kotlin
dependencies {
    implementation("dev.caretly:caretly:0.1.0")
}
```

Maven:

```xml
<dependency>
  <groupId>dev.caretly</groupId>
  <artifactId>caretly</artifactId>
  <version>0.1.0</version>
</dependency>
```

With the module system, add `requires dev.caretly;` to your `module-info.java`.

## Quickstart

Create an API key in the [console](https://caretly.dev/console/keys). Keys start with
`zpl_live_` (billed per label) or `zpl_test_` (free, labels are watermarked TEST).

```java
import dev.caretly.CaretlyClient;
import dev.caretly.RenderResult;
import java.nio.file.Path;

CaretlyClient client = CaretlyClient.create(); // reads CARETLY_API_KEY

RenderResult pdf = client.render("^XA^FO50,50^A0N,50,50^FDHello^FS^XZ");
pdf.saveTo(Path.of("label.pdf"));
System.out.println(pdf.pageCount() + " pages");
```

Configure the client explicitly with the builder. Create one client and share it; it is
thread safe.

```java
CaretlyClient client = CaretlyClient.builder()
        .apiKey("zpl_test_...")             // default: CARETLY_API_KEY, else no key
        .baseUrl("https://caretly.dev")     // default; point it at a self-hosted server
        .timeout(Duration.ofSeconds(30))    // per attempt; default 30 s
        .maxRetries(2)                      // default 2
        .httpClient(HttpClient.newHttpClient()) // optional: proxy, executor, TLS
        .build();
```

### Options

```java
RenderResult png = client.render(zpl, RenderOptions.builder()
        .format(Format.PNG)        // PDF (default, every page) or PNG (one page)
        .page(2)                   // PNG page, 1-based; default 1
        .dpmm(12)                  // 6, 8 (default), 12 or 24 dots per mm
        .timings(true)             // ask for a Server-Timing header
        .idempotencyKey("order-1234") // a retry with the same key and content is billed once
        .compress(true)            // gzip the request body
        .build());
```

Options left unset are not sent, so the server defaults apply. `page` and `dpmm` are validated
when set and throw `IllegalArgumentException` for bad values.

Shortcuts: `client.renderPdf(zpl)`, `client.renderPng(zpl)` and `client.renderPng(zpl, page)`.

### Several files

Each file starts from a fresh printer state; pages follow upload order.

```java
RenderResult result = client.renderFiles(List.of(
        ZplFile.of("order-1.zpl", zpl1),
        ZplFile.fromPath(Path.of("order-2.zpl"))));
result.filePageCounts(); // pages per file, e.g. [2, 1]
```

### Result

| Method             | Meaning                                                        |
|--------------------|----------------------------------------------------------------|
| `bytes()`          | The PDF or PNG bytes.                                          |
| `contentType()`    | `application/pdf` or `image/png`.                              |
| `pageCount()`      | Total pages across the input.                                  |
| `filePageCounts()` | Pages per file for `renderFiles`; empty otherwise.             |
| `labelsBilled()`   | `OptionalInt`: labels billed (0 for test keys); empty without keys. |
| `rateLimit()`      | `Optional<RateLimit>`: burst `limit()` and `remaining()`.      |
| `serverTiming()`   | `Optional<String>`: the raw `Server-Timing` header.            |
| `saveTo(Path)`     | Writes the file.                                               |

### Async

Every call has an `...Async` form:

```java
client.renderAsync(zpl, options)
        .thenAccept(result -> result.saveTo(Path.of("label.pdf")));
client.renderFilesAsync(files, options);
client.healthAsync();
```

Async calls complete exceptionally with the same exceptions as the blocking calls (wrapped in
`CompletionException` when you `join()`).

### Health

```java
boolean up = client.health(); // GET /health answers "ok"
```

## Errors

All exceptions are unchecked and extend `CaretlyException`.

| Exception                             | When                                                    |
|---------------------------------------|---------------------------------------------------------|
| `BadRequestException`                 | 400: ZPL parse error, bad parameter, page out of range. |
| `AuthenticationException`             | 401: key missing, invalid, expired or revoked.          |
| `PaymentRequiredException`            | 402: billing inactive; live keys paused.                |
| `PayloadTooLargeException`            | 413: body over 100 MiB after decompression.             |
| `UnsupportedContentEncodingException` | 415: unsupported `Content-Encoding`.                    |
| `RateLimitedException`                | 429, after retries ran out.                             |
| `OverloadedException`                 | 503, after retries ran out. Not billed.                 |
| `ApiException`                        | Base of the above; also any other status, such as 5xx.  |
| `ConnectionException`                 | The server could not be reached.                        |
| `RequestTimeoutException`             | No response within the timeout.                         |

`ApiException` has `statusCode()`, `getMessage()` (the response body) and `retryAfter()`
(`Optional<Duration>`).

```java
try {
    client.render(zpl);
} catch (BadRequestException e) {
    System.err.println("Bad ZPL: " + e.getMessage());
} catch (ApiException e) {
    System.err.println("HTTP " + e.statusCode() + ": " + e.getMessage());
}
```

## Retries

429, 503 and connection failures are retried up to `maxRetries` times (default 2). The client
waits `Retry-After` when the server sends it, else 0.5 s, 1 s, 2 s, ... up to 8 s. Other
statuses and timeouts are not retried. When retries are enabled and you did not set an
`idempotencyKey`, the client generates one (a UUID) per call, so a retried render is never
billed twice.

## Building

```sh
./gradlew build                 # compile, test, javadoc
./gradlew publishToMavenLocal   # install to ~/.m2 (unsigned)
```

Building needs JDK 17 or newer to run Gradle; the library is compiled for Java 11
(`--release 11`). Signing is only required when publishing to a repository, so local builds
need no keys.

## Publishing to Maven Central

Releases go through the [Central Portal](https://central.sonatype.com) as an uploaded bundle.

### One-time setup

1. Sign in to <https://central.sonatype.com> and add the namespace `dev.caretly`.
2. Verify it: the portal shows a verification key. Add it as a DNS TXT record on
   `caretly.dev`, then press *Verify namespace*:

   ```
   caretly.dev.  TXT  "<verification key>"
   ```

3. Create a user token (*Account* > *Generate User Token*); note its username and password.
4. Create a GPG key and publish it to a key server Central checks:

   ```sh
   gpg --quick-gen-key "Jaap Elst <jaapelst@gmail.com>" ed25519 sign 2y
   gpg --keyserver keyserver.ubuntu.com --send-keys <KEY_ID>
   ```

### Release

1. Set the version in `build.gradle.kts` and add a `CHANGELOG.md` entry.
2. Build the signed bundle. The key is passed as Gradle properties (environment variables work
   too, as below):

   ```sh
   export ORG_GRADLE_PROJECT_signingKey="$(gpg --armor --export-secret-keys <KEY_ID>)"
   export ORG_GRADLE_PROJECT_signingPassword='<passphrase>'
   ./gradlew clean build centralBundle
   ```

   This publishes to `build/staging-deploy` (jar, sources, javadoc, POM, Gradle module
   metadata, `.asc` signatures and checksums) and zips it to
   `build/central/caretly-<version>-bundle.zip`. Alternatively, configure
   `signing.keyId`, `signing.password` and `signing.secretKeyRingFile` in
   `~/.gradle/gradle.properties`.

3. Upload the bundle, either in the portal (*Publish Component*) or with the API:

   ```sh
   TOKEN=$(printf '%s:%s' "$CENTRAL_USERNAME" "$CENTRAL_PASSWORD" | base64)
   curl --fail -H "Authorization: Bearer $TOKEN" \
     -F bundle=@build/central/caretly-0.1.0-bundle.zip \
     "https://central.sonatype.com/api/v1/publisher/upload?publishingType=USER_MANAGED&name=caretly-0.1.0"
   ```

4. In the portal, wait for validation of the deployment, then press *Publish*. Use
   `publishingType=AUTOMATIC` to publish as soon as validation passes. The release appears on
   Maven Central within about 30 minutes.
5. Tag the release: `git tag clients/java/v0.1.0 && git push origin clients/java/v0.1.0`.

## Links

- Website: <https://caretly.dev>
- Source: <https://github.com/jaapieaapie1/zpl/tree/main/clients/java>

## License

MIT. See [LICENSE](LICENSE).
