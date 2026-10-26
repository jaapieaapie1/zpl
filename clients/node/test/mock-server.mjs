// A local HTTP server that records requests and answers from a queue of handlers.
import { createServer } from "node:http";
import { gunzipSync } from "node:zlib";

export async function startServer() {
  const requests = [];
  const handlers = [];
  const server = createServer(async (req, res) => {
    const chunks = [];
    for await (const chunk of req) chunks.push(chunk);
    let body = Buffer.concat(chunks);
    const raw = body;
    if (req.headers["content-encoding"] === "gzip") body = gunzipSync(body);
    const recorded = { method: req.method, url: new URL(req.url, "http://localhost"), headers: req.headers, body, raw };
    requests.push(recorded);
    const handler = handlers.shift() ?? ok();
    await handler(recorded, res);
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const { port } = server.address();
  return {
    baseUrl: `http://127.0.0.1:${port}`,
    requests,
    /** Queues handlers, answered in order; when empty, requests get `ok()`. */
    reply(...next) {
      handlers.push(...next);
    },
    close() {
      server.closeAllConnections?.();
      return new Promise((resolve) => server.close(resolve));
    },
  };
}

export function ok(headers = {}, body = "%PDF-1.7 fake") {
  return (_req, res) => {
    res.writeHead(200, { "content-type": "application/pdf", "x-page-count": "1", ...headers });
    res.end(body);
  };
}

export function fail(status, message, headers = {}) {
  return (_req, res) => {
    res.writeHead(status, { "content-type": "text/plain; charset=utf-8", ...headers });
    res.end(message);
  };
}

/** Parses a recorded multipart body with the platform's own parser. */
export async function parseForm(recorded) {
  const request = new Request("http://localhost/", {
    method: "POST",
    headers: { "content-type": recorded.headers["content-type"] },
    body: recorded.body,
  });
  return request.formData();
}
