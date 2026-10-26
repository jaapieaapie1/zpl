package dev.caretly;

import com.sun.net.httpserver.Headers;
import com.sun.net.httpserver.HttpServer;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Deque;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/** A local HTTP server that answers with queued responses and records every request. */
final class MockServer implements AutoCloseable {
    static final class Recorded {
        final String method;
        final String path;
        final String query;
        final Headers headers;
        final byte[] body;

        Recorded(String method, String path, String query, Headers headers, byte[] body) {
            this.method = method;
            this.path = path;
            this.query = query;
            this.headers = headers;
            this.body = body;
        }

        String header(String name) {
            return headers.getFirst(name);
        }

        String bodyText() {
            return new String(body, StandardCharsets.UTF_8);
        }
    }

    static final class Reply {
        final int status;
        final byte[] body;
        final Map<String, String> headers = new LinkedHashMap<>();
        long delayMillis;

        Reply(int status, byte[] body) {
            this.status = status;
            this.body = body;
        }

        static Reply of(int status, String body) {
            return new Reply(status, body.getBytes(StandardCharsets.UTF_8));
        }

        static Reply pdf(byte[] body) {
            return new Reply(200, body).header("Content-Type", "application/pdf");
        }

        Reply header(String name, String value) {
            headers.put(name, value);
            return this;
        }

        Reply delay(long millis) {
            this.delayMillis = millis;
            return this;
        }
    }

    private final HttpServer server;
    private final Deque<Reply> replies = new ArrayDeque<>();
    private final List<Recorded> requests = Collections.synchronizedList(new ArrayList<>());

    MockServer() throws IOException {
        server = HttpServer.create(new InetSocketAddress(InetAddress.getLoopbackAddress(), 0), 0);
        server.createContext("/", exchange -> {
            byte[] body;
            try (InputStream in = exchange.getRequestBody()) {
                body = in.readAllBytes();
            }
            requests.add(new Recorded(exchange.getRequestMethod(), exchange.getRequestURI().getPath(),
                    exchange.getRequestURI().getRawQuery(), exchange.getRequestHeaders(), body));
            Reply reply;
            synchronized (replies) {
                reply = replies.isEmpty() ? Reply.of(500, "no reply queued") : replies.poll();
            }
            if (reply.delayMillis > 0) {
                try {
                    Thread.sleep(reply.delayMillis);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
            }
            reply.headers.forEach((k, v) -> exchange.getResponseHeaders().add(k, v));
            try {
                exchange.sendResponseHeaders(reply.status, reply.body.length == 0 ? -1 : reply.body.length);
                if (reply.body.length > 0) {
                    try (OutputStream out = exchange.getResponseBody()) {
                        out.write(reply.body);
                    }
                }
            } catch (IOException e) {
                // The client gave up (timeout test).
            } finally {
                exchange.close();
            }
        });
        server.start();
    }

    MockServer enqueue(Reply reply) {
        synchronized (replies) {
            replies.add(reply);
        }
        return this;
    }

    String url() {
        return "http://" + server.getAddress().getHostString() + ":" + server.getAddress().getPort();
    }

    List<Recorded> requests() {
        synchronized (requests) {
            return new ArrayList<>(requests);
        }
    }

    Recorded last() {
        List<Recorded> all = requests();
        return all.get(all.size() - 1);
    }

    @Override
    public void close() {
        server.stop(0);
    }
}
