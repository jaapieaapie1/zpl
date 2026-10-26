package dev.caretly;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import dev.caretly.MockServer.Recorded;
import dev.caretly.MockServer.Reply;
import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.net.ServerSocket;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionException;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.zip.GZIPInputStream;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class CaretlyClientTest {
    private static final String ZPL = "^XA^FO50,50^A0N,50,50^FDHello^FS^XZ";
    private static final byte[] PDF = "%PDF-1.7 fake".getBytes(StandardCharsets.US_ASCII);
    private static final Pattern UUID_V4 =
            Pattern.compile("[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}");

    private MockServer server;
    private final List<Duration> sleeps = Collections.synchronizedList(new ArrayList<>());

    @BeforeEach
    void start() throws IOException {
        server = new MockServer();
    }

    @AfterEach
    void stop() {
        server.close();
    }

    private CaretlyClient.Builder client() {
        return CaretlyClient.builder()
                .baseUrl(server.url())
                .apiKey("zpl_test_abc")
                .env(name -> null)
                .sleeper(delay -> {
                    sleeps.add(delay);
                    return CompletableFuture.completedFuture(null);
                });
    }

    // --- success --------------------------------------------------------------------------

    @Test
    void parsesSuccessHeaders() {
        server.enqueue(Reply.pdf(PDF)
                .header("X-Page-Count", "3")
                .header("X-Labels-Billed", "3")
                .header("RateLimit-Limit", "200")
                .header("RateLimit-Remaining", "199")
                .header("Server-Timing", "read;dur=0.1, parse;dur=0.4"));

        RenderResult result = client().build().render(ZPL);

        assertArrayEquals(PDF, result.bytes());
        assertEquals("application/pdf", result.contentType());
        assertEquals(3, result.pageCount());
        assertEquals(List.of(), result.filePageCounts());
        assertEquals(3, result.labelsBilled().getAsInt());
        assertEquals(new RateLimit(200, 199), result.rateLimit().orElseThrow());
        assertEquals("read;dur=0.1, parse;dur=0.4", result.serverTiming().orElseThrow());
    }

    @Test
    void optionalHeadersAbsent() {
        server.enqueue(Reply.pdf(PDF).header("X-Page-Count", "1"));
        RenderResult result = client().build().render(ZPL);
        assertFalse(result.labelsBilled().isPresent());
        assertFalse(result.rateLimit().isPresent());
        assertFalse(result.serverTiming().isPresent());
    }

    @Test
    void sendsRawZplWithDefaultsOmitted() {
        server.enqueue(Reply.pdf(PDF).header("X-Page-Count", "1"));
        client().build().render(ZPL);

        Recorded request = server.last();
        assertEquals("POST", request.method);
        assertEquals("/render", request.path);
        assertNull(request.query);
        assertEquals("text/plain; charset=utf-8", request.header("Content-Type"));
        assertEquals("caretly-java/" + CaretlyClient.VERSION, request.header("User-Agent"));
        assertEquals(ZPL, request.bodyText());
        assertNull(request.header("Content-Encoding"));
    }

    @Test
    void sendsQueryParameters() {
        server.enqueue(new Reply(200, new byte[] {(byte) 0x89, 'P', 'N', 'G'})
                .header("Content-Type", "image/png").header("X-Page-Count", "4"));
        RenderOptions options = RenderOptions.builder()
                .format(Format.PNG).page(2).dpmm(12).timings(true).build();

        RenderResult result = client().build().render(ZPL, options);

        assertEquals("format=png&page=2&dpmm=12&timings=1", server.last().query);
        assertEquals("image/png", result.contentType());
    }

    @Test
    void renderPngSugar() {
        server.enqueue(new Reply(200, new byte[] {1}).header("Content-Type", "image/png"));
        client().build().renderPng(ZPL, 3);
        assertEquals("format=png&page=3", server.last().query);
    }

    @Test
    void basePathIsKept() {
        server.enqueue(Reply.pdf(PDF));
        client().baseUrl(server.url() + "/api/").build().render(ZPL);
        assertEquals("/api/render", server.last().path);
    }

    @Test
    void asyncRender() {
        server.enqueue(Reply.pdf(PDF).header("X-Page-Count", "1"));
        RenderResult result = client().build().renderAsync(ZPL).join();
        assertEquals(1, result.pageCount());
    }

    @Test
    void asyncFailureCompletesExceptionally() {
        server.enqueue(Reply.of(400, "bad zpl"));
        CompletableFuture<RenderResult> future = client().maxRetries(0).build().renderAsync(ZPL);
        CompletionException e = assertThrows(CompletionException.class, future::join);
        assertInstanceOf(BadRequestException.class, e.getCause());
    }

    @Test
    void saveTo(@TempDir Path dir) throws IOException {
        server.enqueue(Reply.pdf(PDF));
        Path file = client().build().render(ZPL).saveTo(dir.resolve("label.pdf"));
        assertArrayEquals(PDF, Files.readAllBytes(file));
    }

    // --- auth -----------------------------------------------------------------------------

    @Test
    void sendsBearerKey() {
        server.enqueue(Reply.pdf(PDF));
        client().build().render(ZPL);
        assertEquals("Bearer zpl_test_abc", server.last().header("Authorization"));
    }

    @Test
    void readsKeyFromEnvironment() {
        server.enqueue(Reply.pdf(PDF));
        client().apiKey(null).env(Map.of("CARETLY_API_KEY", "zpl_live_env")::get).build().render(ZPL);
        assertEquals("Bearer zpl_live_env", server.last().header("Authorization"));
    }

    @Test
    void sendsNoKeyWithoutOne() {
        server.enqueue(Reply.pdf(PDF));
        CaretlyClient client = client().apiKey(null).build();
        assertFalse(client.hasApiKey());
        client.render(ZPL);
        assertNull(server.last().header("Authorization"));
    }

    // --- errors ---------------------------------------------------------------------------

    @ParameterizedTest
    @CsvSource({
        "400, dev.caretly.BadRequestException",
        "401, dev.caretly.AuthenticationException",
        "402, dev.caretly.PaymentRequiredException",
        "413, dev.caretly.PayloadTooLargeException",
        "415, dev.caretly.UnsupportedContentEncodingException",
        "429, dev.caretly.RateLimitedException",
        "503, dev.caretly.OverloadedException",
        "500, dev.caretly.ApiException",
        "404, dev.caretly.ApiException",
    })
    void mapsErrorStatus(int status, String type) {
        server.enqueue(Reply.of(status, "message " + status + "\n").header("Retry-After", "7"));
        CaretlyClient client = client().maxRetries(0).build();

        ApiException e = assertThrows(ApiException.class, () -> client.render(ZPL));

        assertEquals(type, e.getClass().getName());
        assertEquals(status, e.statusCode());
        assertEquals("message " + status, e.getMessage());
        assertEquals(Optional.of(Duration.ofSeconds(7)), e.retryAfter());
    }

    @Test
    void errorWithoutBodyOrRetryAfter() {
        server.enqueue(new Reply(500, new byte[0]));
        ApiException e = assertThrows(ApiException.class, () -> client().maxRetries(0).build().render(ZPL));
        assertEquals("HTTP 500", e.getMessage());
        assertFalse(e.retryAfter().isPresent());
    }

    // --- retries --------------------------------------------------------------------------

    @Test
    void retries429HonoringRetryAfter() {
        server.enqueue(Reply.of(429, "slow down").header("Retry-After", "3"));
        server.enqueue(Reply.pdf(PDF).header("X-Page-Count", "1"));

        RenderResult result = client().build().render(ZPL);

        assertEquals(1, result.pageCount());
        assertEquals(List.of(Duration.ofSeconds(3)), sleeps);
        List<Recorded> requests = server.requests();
        assertEquals(2, requests.size());
        String key = requests.get(0).header("Idempotency-Key");
        assertNotNull(key);
        assertTrue(UUID_V4.matcher(key).matches(), key);
        assertEquals(key, requests.get(1).header("Idempotency-Key"));
    }

    @Test
    void retries503WithBackoffThenGivesUp() {
        for (int i = 0; i < 3; i++) {
            server.enqueue(Reply.of(503, "overloaded"));
        }
        OverloadedException e = assertThrows(OverloadedException.class, () -> client().build().render(ZPL));
        assertEquals(503, e.statusCode());
        assertEquals(3, server.requests().size());
        assertEquals(List.of(Duration.ofMillis(500), Duration.ofSeconds(1)), sleeps);
    }

    @Test
    void doesNotRetryOtherStatuses() {
        server.enqueue(Reply.of(500, "boom"));
        assertThrows(ApiException.class, () -> client().build().render(ZPL));
        assertEquals(1, server.requests().size());
        assertTrue(sleeps.isEmpty());
    }

    @Test
    void keepsCallerIdempotencyKey() {
        server.enqueue(Reply.of(429, "slow down").header("Retry-After", "1"));
        server.enqueue(Reply.pdf(PDF));
        client().build().render(ZPL, RenderOptions.builder().idempotencyKey("order-42").build());
        for (Recorded r : server.requests()) {
            assertEquals("order-42", r.header("Idempotency-Key"));
        }
    }

    @Test
    void noGeneratedKeyWithoutRetries() {
        server.enqueue(Reply.pdf(PDF));
        client().maxRetries(0).build().render(ZPL);
        assertNull(server.last().header("Idempotency-Key"));
    }

    @Test
    void backoffIsCapped() {
        assertEquals(Duration.ofMillis(500), CaretlyClient.backoff(0));
        assertEquals(Duration.ofSeconds(1), CaretlyClient.backoff(1));
        assertEquals(Duration.ofSeconds(2), CaretlyClient.backoff(2));
        assertEquals(Duration.ofSeconds(4), CaretlyClient.backoff(3));
        assertEquals(Duration.ofSeconds(8), CaretlyClient.backoff(4));
        assertEquals(Duration.ofSeconds(8), CaretlyClient.backoff(30));
    }

    @Test
    void retriesConnectionErrors() throws IOException {
        int port;
        try (ServerSocket socket = new ServerSocket(0)) {
            port = socket.getLocalPort();
        }
        CaretlyClient client = client().baseUrl("http://127.0.0.1:" + port).build();
        ConnectionException e = assertThrows(ConnectionException.class, () -> client.render(ZPL));
        assertNotNull(e.getCause());
        assertEquals(2, sleeps.size());
    }

    @Test
    void timeoutIsNotRetried() {
        server.enqueue(Reply.pdf(PDF).delay(2000));
        CaretlyClient client = client().timeout(Duration.ofMillis(200)).build();
        assertThrows(RequestTimeoutException.class, () -> client.render(ZPL));
        assertTrue(sleeps.isEmpty());
    }

    // --- multipart ------------------------------------------------------------------------

    @Test
    void multipartShape() {
        server.enqueue(Reply.pdf(PDF).header("X-Page-Count", "3").header("X-File-Page-Counts", "2, 1"));
        List<ZplFile> files = List.of(
                ZplFile.of("first.zpl", "^XA^FDone^FS^XZ^XA^FDtwo^FS^XZ"),
                ZplFile.of("se\"cond.zpl", "^XA^FDthree^FS^XZ".getBytes(StandardCharsets.UTF_8)));

        RenderResult result = client().build().renderFiles(files, RenderOptions.builder().dpmm(8).build());

        assertEquals(List.of(2, 1), result.filePageCounts());
        assertEquals(3, result.pageCount());
        Recorded request = server.last();
        assertEquals("dpmm=8", request.query);
        Matcher m = Pattern.compile("multipart/form-data; boundary=(\\S+)").matcher(request.header("Content-Type"));
        assertTrue(m.matches(), request.header("Content-Type"));
        String boundary = m.group(1);
        String expected = "--" + boundary + "\r\n"
                + "Content-Disposition: form-data; name=\"file\"; filename=\"first.zpl\"\r\n"
                + "Content-Type: application/octet-stream\r\n\r\n"
                + "^XA^FDone^FS^XZ^XA^FDtwo^FS^XZ\r\n"
                + "--" + boundary + "\r\n"
                + "Content-Disposition: form-data; name=\"file\"; filename=\"se%22cond.zpl\"\r\n"
                + "Content-Type: application/octet-stream\r\n\r\n"
                + "^XA^FDthree^FS^XZ\r\n"
                + "--" + boundary + "--\r\n";
        assertEquals(expected, request.bodyText());
    }

    @Test
    void fileFromPath(@TempDir Path dir) throws IOException {
        Path path = Files.writeString(dir.resolve("order.zpl"), ZPL);
        ZplFile file = ZplFile.fromPath(path);
        assertEquals("order.zpl", file.name());
        assertEquals(ZPL, new String(file.content(), StandardCharsets.UTF_8));
    }

    @Test
    void emptyFileListRejected() {
        CaretlyClient client = client().build();
        assertThrows(IllegalArgumentException.class, () -> client.renderFiles(List.of()));
        assertTrue(server.requests().isEmpty());
    }

    // --- gzip -----------------------------------------------------------------------------

    @Test
    void gzipCompressesBody() throws IOException {
        server.enqueue(Reply.pdf(PDF));
        client().build().render(ZPL, RenderOptions.builder().compress(true).build());

        Recorded request = server.last();
        assertEquals("gzip", request.header("Content-Encoding"));
        assertEquals("text/plain; charset=utf-8", request.header("Content-Type"));
        try (GZIPInputStream in = new GZIPInputStream(new ByteArrayInputStream(request.body))) {
            assertEquals(ZPL, new String(in.readAllBytes(), StandardCharsets.UTF_8));
        }
    }

    // --- validation -----------------------------------------------------------------------

    @Test
    void validatesOptions() {
        RenderOptions.Builder b = RenderOptions.builder();
        assertThrows(IllegalArgumentException.class, () -> b.page(0));
        assertThrows(IllegalArgumentException.class, () -> b.dpmm(7));
        assertThrows(IllegalArgumentException.class, () -> b.idempotencyKey(""));
        assertThrows(IllegalArgumentException.class, () -> b.idempotencyKey("x".repeat(256)));
        assertThrows(IllegalArgumentException.class, () -> b.idempotencyKey("a\nb"));
        b.dpmm(6).dpmm(8).dpmm(12).dpmm(24).page(1).idempotencyKey("x".repeat(255));
    }

    @Test
    void validatesBuilder() {
        assertThrows(IllegalArgumentException.class, () -> CaretlyClient.builder().baseUrl("ftp://x"));
        assertThrows(IllegalArgumentException.class, () -> CaretlyClient.builder().baseUrl("not a url"));
        assertThrows(IllegalArgumentException.class, () -> CaretlyClient.builder().maxRetries(-1));
        assertThrows(IllegalArgumentException.class, () -> CaretlyClient.builder().timeout(Duration.ZERO));
        assertEquals("https://caretly.dev", CaretlyClient.builder().env(n -> null).build().baseUrl());
    }

    // --- health ---------------------------------------------------------------------------

    @Test
    void healthOk() {
        server.enqueue(Reply.of(200, "ok"));
        assertTrue(client().build().health());
        assertEquals("GET", server.last().method);
        assertEquals("/health", server.last().path);
    }

    @Test
    void healthNotOk() {
        server.enqueue(Reply.of(502, "bad gateway"));
        assertFalse(client().build().health());
    }

    @Test
    void healthAsync() {
        server.enqueue(Reply.of(200, "ok\n"));
        assertSame(Boolean.TRUE, client().build().healthAsync().join());
    }
}
