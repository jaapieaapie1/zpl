package dev.caretly;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpConnectTimeoutException;
import java.net.http.HttpHeaders;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.http.HttpTimeoutException;
import java.nio.charset.StandardCharsets;
import java.time.Duration;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.OptionalInt;
import java.util.UUID;
import java.util.concurrent.CancellationException;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionException;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.function.Function;
import java.util.zip.GZIPOutputStream;

/**
 * Client for the Caretly API: render ZPL labels to PDF or PNG.
 *
 * <pre>{@code
 * CaretlyClient client = CaretlyClient.create(); // reads CARETLY_API_KEY
 * RenderResult pdf = client.render("^XA^FO50,50^A0N,50,50^FDHello^FS^XZ");
 * pdf.saveTo(Path.of("label.pdf"));
 * }</pre>
 *
 * <p>Every operation has a blocking form and an {@code ...Async} form that returns a
 * {@link CompletableFuture}. Blocking calls throw {@link CaretlyException} subclasses; async
 * calls complete exceptionally with them.
 *
 * <p>Responses with status 429 or 503 and connection failures are retried up to
 * {@link Builder#maxRetries(int)} times, waiting {@code Retry-After} when the server sends it and
 * otherwise backing off exponentially (0.5 s, 1 s, 2 s, ... up to 8 s). When retries are enabled
 * and no {@link RenderOptions.Builder#idempotencyKey(String) idempotency key} is given, the
 * client generates one per call so a retried render is never billed twice.
 *
 * <p>Instances are immutable and thread safe; share one across your application.
 */
public final class CaretlyClient {
    /** The version of this library. */
    public static final String VERSION = "0.1.0";
    /** The default base URL, {@value}. */
    public static final String DEFAULT_BASE_URL = "https://caretly.dev";
    /** The default request timeout: 30 seconds. */
    public static final Duration DEFAULT_TIMEOUT = Duration.ofSeconds(30);
    /** The default number of retries: {@value}. */
    public static final int DEFAULT_MAX_RETRIES = 2;
    /** The environment variable the API key is read from when none is given. */
    public static final String API_KEY_ENV = "CARETLY_API_KEY";

    static final String USER_AGENT = "caretly-java/" + VERSION;
    private static final Duration INITIAL_BACKOFF = Duration.ofMillis(500);
    private static final Duration MAX_BACKOFF = Duration.ofSeconds(8);

    private final String apiKey;
    private final String baseUrl;
    private final Duration timeout;
    private final int maxRetries;
    private final HttpClient http;
    private final Sleeper sleeper;

    private CaretlyClient(Builder b) {
        String key = b.apiKey;
        if (key == null || key.isBlank()) {
            key = b.env.apply(API_KEY_ENV);
        }
        this.apiKey = key == null || key.isBlank() ? null : key.trim();
        this.baseUrl = b.baseUrl;
        this.timeout = b.timeout;
        this.maxRetries = b.maxRetries;
        this.http = b.httpClient != null
                ? b.httpClient
                : HttpClient.newBuilder()
                        .connectTimeout(b.timeout)
                        .followRedirects(HttpClient.Redirect.NORMAL)
                        .build();
        this.sleeper = b.sleeper;
    }

    /**
     * Creates a client with the defaults, reading the API key from {@code CARETLY_API_KEY}.
     * Without that variable the client sends no key, which suits self-hosted servers.
     *
     * @return a client
     */
    public static CaretlyClient create() {
        return builder().build();
    }

    /**
     * Returns a builder for a client.
     *
     * @return a new builder
     */
    public static Builder builder() {
        return new Builder();
    }

    /**
     * Returns the base URL requests are sent to, without a trailing slash.
     *
     * @return the base URL
     */
    public String baseUrl() {
        return baseUrl;
    }

    /**
     * Returns whether the client sends an API key.
     *
     * @return {@code true} when a key was given or read from the environment
     */
    public boolean hasApiKey() {
        return apiKey != null;
    }

    // ---------------------------------------------------------------------------------------
    // render

    /**
     * Renders ZPL with the server defaults (a PDF at 8 dpmm).
     *
     * @param zpl the ZPL; may contain several labels ({@code ^XA...^XZ}), each a page
     * @return the rendered file
     * @throws CaretlyException if the request fails
     */
    public RenderResult render(String zpl) {
        return await(renderAsync(zpl, RenderOptions.defaults()));
    }

    /**
     * Renders ZPL.
     *
     * @param zpl the ZPL; may contain several labels ({@code ^XA...^XZ}), each a page
     * @param options the render options
     * @return the rendered file
     * @throws CaretlyException if the request fails
     */
    public RenderResult render(String zpl, RenderOptions options) {
        return await(renderAsync(zpl, options));
    }

    /**
     * Renders ZPL with the server defaults, asynchronously.
     *
     * @param zpl the ZPL
     * @return a future of the rendered file, completed exceptionally with a
     *     {@link CaretlyException} if the request fails
     */
    public CompletableFuture<RenderResult> renderAsync(String zpl) {
        return renderAsync(zpl, RenderOptions.defaults());
    }

    /**
     * Renders ZPL asynchronously.
     *
     * @param zpl the ZPL
     * @param options the render options
     * @return a future of the rendered file, completed exceptionally with a
     *     {@link CaretlyException} if the request fails
     */
    public CompletableFuture<RenderResult> renderAsync(String zpl, RenderOptions options) {
        Objects.requireNonNull(zpl, "zpl");
        Objects.requireNonNull(options, "options");
        byte[] body = zpl.getBytes(StandardCharsets.UTF_8);
        return send(body, "text/plain; charset=utf-8", options);
    }

    /**
     * Renders ZPL to a PDF with every page.
     *
     * @param zpl the ZPL
     * @return the PDF
     * @throws CaretlyException if the request fails
     */
    public RenderResult renderPdf(String zpl) {
        return render(zpl, RenderOptions.builder().format(Format.PDF).build());
    }

    /**
     * Renders the first page of ZPL to a PNG.
     *
     * @param zpl the ZPL
     * @return the PNG
     * @throws CaretlyException if the request fails
     */
    public RenderResult renderPng(String zpl) {
        return renderPng(zpl, 1);
    }

    /**
     * Renders one page of ZPL to a PNG.
     *
     * @param zpl the ZPL
     * @param page the 1-based page
     * @return the PNG
     * @throws IllegalArgumentException if {@code page} is less than 1
     * @throws CaretlyException if the request fails
     */
    public RenderResult renderPng(String zpl, int page) {
        return render(zpl, RenderOptions.builder().format(Format.PNG).page(page).build());
    }

    // ---------------------------------------------------------------------------------------
    // renderFiles

    /**
     * Renders several ZPL files with the server defaults. Each file starts from a fresh printer
     * state, and pages follow upload order.
     *
     * @param files the files, at least one
     * @return the rendered file, with {@link RenderResult#filePageCounts()} set
     * @throws IllegalArgumentException if {@code files} is empty
     * @throws CaretlyException if the request fails
     */
    public RenderResult renderFiles(List<ZplFile> files) {
        return await(renderFilesAsync(files, RenderOptions.defaults()));
    }

    /**
     * Renders several ZPL files. Each file starts from a fresh printer state, and pages follow
     * upload order; {@link RenderOptions.Builder#page(int)} counts across all files.
     *
     * @param files the files, at least one
     * @param options the render options
     * @return the rendered file, with {@link RenderResult#filePageCounts()} set
     * @throws IllegalArgumentException if {@code files} is empty
     * @throws CaretlyException if the request fails
     */
    public RenderResult renderFiles(List<ZplFile> files, RenderOptions options) {
        return await(renderFilesAsync(files, options));
    }

    /**
     * Renders several ZPL files with the server defaults, asynchronously.
     *
     * @param files the files, at least one
     * @return a future of the rendered file
     * @throws IllegalArgumentException if {@code files} is empty
     */
    public CompletableFuture<RenderResult> renderFilesAsync(List<ZplFile> files) {
        return renderFilesAsync(files, RenderOptions.defaults());
    }

    /**
     * Renders several ZPL files asynchronously.
     *
     * @param files the files, at least one
     * @param options the render options
     * @return a future of the rendered file
     * @throws IllegalArgumentException if {@code files} is empty
     */
    public CompletableFuture<RenderResult> renderFilesAsync(List<ZplFile> files, RenderOptions options) {
        Objects.requireNonNull(files, "files");
        Objects.requireNonNull(options, "options");
        if (files.isEmpty()) {
            throw new IllegalArgumentException("files must not be empty");
        }
        String boundary = "caretly-" + UUID.randomUUID().toString().replace("-", "");
        byte[] body = multipart(files, boundary);
        return send(body, "multipart/form-data; boundary=" + boundary, options);
    }

    // ---------------------------------------------------------------------------------------
    // health

    /**
     * Checks that the server is up by calling {@code GET /health}. Not retried.
     *
     * @return {@code true} when the server answers {@code ok}, {@code false} on any other
     *     response
     * @throws ConnectionException if the server cannot be reached
     * @throws RequestTimeoutException if the server does not answer in time
     */
    public boolean health() {
        return await(healthAsync());
    }

    /**
     * Checks that the server is up, asynchronously. See {@link #health()}.
     *
     * @return a future that is {@code true} when the server answers {@code ok}
     */
    public CompletableFuture<Boolean> healthAsync() {
        HttpRequest request = HttpRequest.newBuilder(URI.create(baseUrl + "/health"))
                .timeout(timeout)
                .header("User-Agent", USER_AGENT)
                .GET()
                .build();
        return http.sendAsync(request, HttpResponse.BodyHandlers.ofString())
                .handle((response, error) -> {
                    if (error != null) {
                        throw transportError(unwrap(error));
                    }
                    return response.statusCode() == 200 && "ok".equals(response.body().trim());
                });
    }

    // ---------------------------------------------------------------------------------------
    // internals

    private CompletableFuture<RenderResult> send(byte[] body, String contentType, RenderOptions options) {
        String idempotencyKey = options.idempotencyKey()
                .orElse(maxRetries > 0 ? UUID.randomUUID().toString() : null);
        byte[] payload = options.compress() ? gzip(body) : body;
        URI uri = URI.create(baseUrl + "/render" + query(options));
        Format format = options.format().orElse(Format.PDF);
        return attempt(0, uri, payload, contentType, options.compress(), idempotencyKey, format);
    }

    private CompletableFuture<RenderResult> attempt(int attempt, URI uri, byte[] payload, String contentType,
            boolean compressed, String idempotencyKey, Format format) {
        HttpRequest.Builder request = HttpRequest.newBuilder(uri)
                .timeout(timeout)
                .header("User-Agent", USER_AGENT)
                .header("Content-Type", contentType)
                .POST(HttpRequest.BodyPublishers.ofByteArray(payload));
        if (apiKey != null) {
            request.header("Authorization", "Bearer " + apiKey);
        }
        if (idempotencyKey != null) {
            request.header("Idempotency-Key", idempotencyKey);
        }
        if (compressed) {
            request.header("Content-Encoding", "gzip");
        }

        CompletableFuture<HttpResponse<byte[]>> sent;
        try {
            sent = http.sendAsync(request.build(), HttpResponse.BodyHandlers.ofByteArray());
        } catch (RuntimeException e) {
            sent = CompletableFuture.failedFuture(e);
        }

        return sent.handle((response, error) -> {
            CaretlyException failure;
            boolean retryable;
            Duration retryAfter = null;
            if (error != null) {
                failure = transportError(unwrap(error));
                retryable = failure instanceof ConnectionException;
            } else if (response.statusCode() / 100 == 2) {
                return CompletableFuture.completedFuture(result(response, format));
            } else {
                int status = response.statusCode();
                retryAfter = parseRetryAfter(response.headers()).orElse(null);
                String text = new String(response.body(), StandardCharsets.UTF_8).trim();
                failure = ApiException.of(status, text.isEmpty() ? "HTTP " + status : text, retryAfter);
                retryable = status == 429 || status == 503;
            }
            if (!retryable || attempt >= maxRetries) {
                return CompletableFuture.<RenderResult>failedFuture(failure);
            }
            Duration delay = retryAfter != null ? retryAfter : backoff(attempt);
            return sleeper.sleep(delay).thenCompose(ignored ->
                    attempt(attempt + 1, uri, payload, contentType, compressed, idempotencyKey, format));
        }).thenCompose(Function.identity());
    }

    static Duration backoff(int attempt) {
        long millis = INITIAL_BACKOFF.toMillis() << Math.min(attempt, 10);
        return Duration.ofMillis(Math.min(millis, MAX_BACKOFF.toMillis()));
    }

    private static String query(RenderOptions options) {
        List<String> params = new ArrayList<>();
        options.format().ifPresent(f -> params.add("format=" + f.value()));
        options.page().ifPresent(p -> params.add("page=" + p));
        options.dpmm().ifPresent(d -> params.add("dpmm=" + d));
        if (options.timings()) {
            params.add("timings=1");
        }
        return params.isEmpty() ? "" : "?" + String.join("&", params);
    }

    private static RenderResult result(HttpResponse<byte[]> response, Format format) {
        HttpHeaders headers = response.headers();
        String contentType = headers.firstValue("Content-Type").orElse(format.contentType());
        int pageCount = intHeader(headers, "X-Page-Count").orElse(0);
        List<Integer> filePageCounts = headers.firstValue("X-File-Page-Counts")
                .map(CaretlyClient::parseIntList)
                .orElse(Collections.emptyList());
        OptionalInt billed = intHeader(headers, "X-Labels-Billed");
        OptionalInt limit = intHeader(headers, "RateLimit-Limit");
        OptionalInt remaining = intHeader(headers, "RateLimit-Remaining");
        RateLimit rateLimit = limit.isPresent() && remaining.isPresent()
                ? new RateLimit(limit.getAsInt(), remaining.getAsInt())
                : null;
        return new RenderResult(response.body(), contentType, pageCount, filePageCounts,
                billed.isPresent() ? billed.getAsInt() : null, rateLimit,
                headers.firstValue("Server-Timing").orElse(null));
    }

    private static OptionalInt intHeader(HttpHeaders headers, String name) {
        Optional<String> value = headers.firstValue(name);
        if (value.isEmpty()) {
            return OptionalInt.empty();
        }
        try {
            return OptionalInt.of(Integer.parseInt(value.get().trim()));
        } catch (NumberFormatException e) {
            return OptionalInt.empty();
        }
    }

    private static List<Integer> parseIntList(String value) {
        List<Integer> counts = new ArrayList<>();
        for (String part : value.split(",")) {
            String trimmed = part.trim();
            if (trimmed.isEmpty()) {
                continue;
            }
            try {
                counts.add(Integer.parseInt(trimmed));
            } catch (NumberFormatException e) {
                return Collections.emptyList();
            }
        }
        return counts;
    }

    static Optional<Duration> parseRetryAfter(HttpHeaders headers) {
        Optional<String> value = headers.firstValue("Retry-After").map(String::trim);
        if (value.isEmpty() || value.get().isEmpty()) {
            return Optional.empty();
        }
        String v = value.get();
        try {
            long seconds = Long.parseLong(v);
            return Optional.of(Duration.ofSeconds(Math.max(0, seconds)));
        } catch (NumberFormatException ignored) {
            // Not seconds; try an HTTP date.
        }
        try {
            ZonedDateTime at = ZonedDateTime.parse(v, DateTimeFormatter.RFC_1123_DATE_TIME);
            Duration d = Duration.between(ZonedDateTime.now(at.getZone()), at);
            return Optional.of(d.isNegative() ? Duration.ZERO : d);
        } catch (DateTimeParseException e) {
            return Optional.empty();
        }
    }

    private static Throwable unwrap(Throwable error) {
        Throwable e = error;
        while ((e instanceof CompletionException || e instanceof ExecutionException) && e.getCause() != null) {
            e = e.getCause();
        }
        return e;
    }

    private static CaretlyException transportError(Throwable e) {
        if (e instanceof CaretlyException) {
            return (CaretlyException) e;
        }
        if (e instanceof HttpConnectTimeoutException) {
            return new ConnectionException("Could not connect to the Caretly API: " + e.getMessage(), e);
        }
        if (e instanceof HttpTimeoutException) {
            return new RequestTimeoutException("The Caretly API did not answer in time: " + e.getMessage(), e);
        }
        if (e instanceof IOException) {
            return new ConnectionException("Could not reach the Caretly API: " + e.getMessage(), e);
        }
        return new CaretlyException("Request to the Caretly API failed: " + e, e);
    }

    private static <T> T await(CompletableFuture<T> future) {
        try {
            return future.get();
        } catch (ExecutionException e) {
            Throwable cause = unwrap(e);
            if (cause instanceof RuntimeException) {
                throw (RuntimeException) cause;
            }
            if (cause instanceof Error) {
                throw (Error) cause;
            }
            throw new CaretlyException("Request to the Caretly API failed: " + cause, cause);
        } catch (InterruptedException e) {
            future.cancel(true);
            Thread.currentThread().interrupt();
            throw new CaretlyException("Interrupted while waiting for the Caretly API", e);
        } catch (CancellationException e) {
            throw new CaretlyException("The request to the Caretly API was cancelled", e);
        }
    }

    static byte[] multipart(List<ZplFile> files, String boundary) {
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        for (ZplFile file : files) {
            Objects.requireNonNull(file, "files must not contain null");
            String head = "--" + boundary + "\r\n"
                    + "Content-Disposition: form-data; name=\"file\"; filename=\"" + escapeFilename(file.name()) + "\"\r\n"
                    + "Content-Type: application/octet-stream\r\n"
                    + "\r\n";
            out.writeBytes(head.getBytes(StandardCharsets.UTF_8));
            out.writeBytes(file.contentUnsafe());
            out.writeBytes("\r\n".getBytes(StandardCharsets.US_ASCII));
        }
        out.writeBytes(("--" + boundary + "--\r\n").getBytes(StandardCharsets.US_ASCII));
        return out.toByteArray();
    }

    /** Escapes a filename for a quoted Content-Disposition parameter, as browsers do. */
    private static String escapeFilename(String name) {
        return name.replace("\"", "%22").replace("\r", "%0D").replace("\n", "%0A");
    }

    private static byte[] gzip(byte[] body) {
        ByteArrayOutputStream out = new ByteArrayOutputStream(Math.max(64, body.length / 4));
        try (GZIPOutputStream gz = new GZIPOutputStream(out)) {
            gz.write(body);
        } catch (IOException e) {
            throw new UncheckedIOException(e);
        }
        return out.toByteArray();
    }

    /** Waits between retries. Replaced in tests so they do not sleep. */
    interface Sleeper {
        CompletableFuture<Void> sleep(Duration delay);

        Sleeper DEFAULT = delay -> CompletableFuture.runAsync(() -> { },
                CompletableFuture.delayedExecutor(delay.toMillis(), TimeUnit.MILLISECONDS));
    }

    /** Builds a {@link CaretlyClient}. */
    public static final class Builder {
        private String apiKey;
        private String baseUrl = DEFAULT_BASE_URL;
        private Duration timeout = DEFAULT_TIMEOUT;
        private int maxRetries = DEFAULT_MAX_RETRIES;
        private HttpClient httpClient;
        private Sleeper sleeper = Sleeper.DEFAULT;
        private Function<String, String> env = System::getenv;

        private Builder() {
        }

        /**
         * Sets the API key, sent as {@code Authorization: Bearer <key>}. When not set, the key is
         * read from {@code CARETLY_API_KEY}; without either, no key is sent (for self-hosted
         * servers). Live keys start with {@code zpl_live_}, test keys with {@code zpl_test_}.
         *
         * @param apiKey the key, or {@code null} to use the environment
         * @return this builder
         */
        public Builder apiKey(String apiKey) {
            this.apiKey = apiKey;
            return this;
        }

        /**
         * Sets the base URL, for example of a self-hosted server. Defaults to
         * {@value CaretlyClient#DEFAULT_BASE_URL}.
         *
         * @param baseUrl an {@code http} or {@code https} URL
         * @return this builder
         * @throws IllegalArgumentException if the URL is not an absolute http(s) URL
         */
        public Builder baseUrl(String baseUrl) {
            Objects.requireNonNull(baseUrl, "baseUrl");
            String trimmed = baseUrl.trim();
            while (trimmed.endsWith("/")) {
                trimmed = trimmed.substring(0, trimmed.length() - 1);
            }
            URI uri;
            try {
                uri = URI.create(trimmed);
            } catch (IllegalArgumentException e) {
                throw new IllegalArgumentException("baseUrl is not a valid URL: " + baseUrl, e);
            }
            String scheme = uri.getScheme();
            if (scheme == null || !(scheme.equalsIgnoreCase("http") || scheme.equalsIgnoreCase("https"))
                    || uri.getHost() == null) {
                throw new IllegalArgumentException("baseUrl must be an absolute http(s) URL: " + baseUrl);
            }
            if (uri.getQuery() != null || uri.getFragment() != null) {
                throw new IllegalArgumentException("baseUrl must not have a query or fragment: " + baseUrl);
            }
            this.baseUrl = trimmed;
            return this;
        }

        /**
         * Sets the base URL. See {@link #baseUrl(String)}.
         *
         * @param baseUrl an {@code http} or {@code https} URL
         * @return this builder
         */
        public Builder baseUrl(URI baseUrl) {
            return baseUrl(Objects.requireNonNull(baseUrl, "baseUrl").toString());
        }

        /**
         * Sets the timeout of each request attempt. Defaults to 30 seconds. Also used as the
         * connect timeout of the default HTTP client.
         *
         * @param timeout a positive duration
         * @return this builder
         * @throws IllegalArgumentException if {@code timeout} is zero or negative
         */
        public Builder timeout(Duration timeout) {
            Objects.requireNonNull(timeout, "timeout");
            if (timeout.isZero() || timeout.isNegative()) {
                throw new IllegalArgumentException("timeout must be positive");
            }
            this.timeout = timeout;
            return this;
        }

        /**
         * Sets how often a request is retried after a 429, a 503 or a connection failure.
         * Defaults to 2. Zero disables retries (and the generated idempotency key).
         *
         * @param maxRetries zero or more
         * @return this builder
         * @throws IllegalArgumentException if {@code maxRetries} is negative
         */
        public Builder maxRetries(int maxRetries) {
            if (maxRetries < 0) {
                throw new IllegalArgumentException("maxRetries must not be negative");
            }
            this.maxRetries = maxRetries;
            return this;
        }

        /**
         * Sets the HTTP client to send requests with, for example to configure a proxy, an
         * executor or TLS. Defaults to a new client that follows redirects.
         *
         * @param httpClient the HTTP client
         * @return this builder
         */
        public Builder httpClient(HttpClient httpClient) {
            this.httpClient = Objects.requireNonNull(httpClient, "httpClient");
            return this;
        }

        Builder sleeper(Sleeper sleeper) {
            this.sleeper = Objects.requireNonNull(sleeper, "sleeper");
            return this;
        }

        Builder env(Function<String, String> env) {
            this.env = Objects.requireNonNull(env, "env");
            return this;
        }

        /**
         * Builds the client.
         *
         * @return the client
         */
        public CaretlyClient build() {
            return new CaretlyClient(this);
        }
    }
}
