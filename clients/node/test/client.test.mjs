import { test, describe, before, after, beforeEach } from "node:test";
import assert from "node:assert/strict";
import { readFile, mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createRequire } from "node:module";
import {
  Caretly,
  RenderResult,
  CaretlyError,
  APIError,
  BadRequestError,
  AuthenticationError,
  PaymentRequiredError,
  PayloadTooLargeError,
  UnsupportedEncodingError,
  RateLimitError,
  OverloadedError,
  ConnectionError,
  TimeoutError,
  VERSION,
} from "../dist/index.js";
import { startServer, ok, fail, parseForm } from "./mock-server.mjs";

const LABEL = "^XA^FO50,50^A0N,40^FDhi^FS^XZ";

let server;
let sleeps;
const client = (options = {}) =>
  new Caretly({
    apiKey: "zpl_test_abc",
    baseUrl: server.baseUrl,
    sleep: async (ms) => void sleeps.push(ms),
    ...options,
  });

before(async () => {
  server = await startServer();
});
after(() => server.close());
beforeEach(() => {
  server.requests.length = 0;
  sleeps = [];
});

describe("render", () => {
  test("sends raw ZPL with auth, user agent and content type", async () => {
    await client().render(LABEL);
    const [req] = server.requests;
    assert.equal(req.method, "POST");
    assert.equal(req.url.pathname, "/render");
    assert.equal(req.url.search, "", "no query parameters unless set");
    assert.equal(req.headers.authorization, "Bearer zpl_test_abc");
    assert.equal(req.headers["user-agent"], `caretly-node/${VERSION}`);
    assert.equal(req.headers["content-type"], "text/plain; charset=utf-8");
    assert.equal(req.headers["content-encoding"], undefined);
    assert.equal(req.body.toString(), LABEL);
  });

  test("sends options as query parameters", async () => {
    await client().render(LABEL, { format: "png", page: 2, dpmm: 12, timings: true });
    const params = Object.fromEntries(server.requests[0].url.searchParams);
    assert.deepEqual(params, { format: "png", page: "2", dpmm: "12", timings: "1" });
  });

  test("parses the success headers", async () => {
    server.reply(
      ok(
        {
          "content-type": "image/png",
          "x-page-count": "3",
          "x-labels-billed": "1",
          "ratelimit-limit": "200",
          "ratelimit-remaining": "199",
          "server-timing": "read;dur=0.1, parse;dur=0.2",
        },
        "PNGDATA",
      ),
    );
    const result = await client().render(LABEL, { format: "png" });
    assert.ok(result instanceof RenderResult);
    assert.ok(result.bytes instanceof Uint8Array);
    assert.equal(new TextDecoder().decode(result.bytes), "PNGDATA");
    assert.equal(result.contentType, "image/png");
    assert.equal(result.pageCount, 3);
    assert.deepEqual(result.filePageCounts, []);
    assert.equal(result.labelsBilled, 1);
    assert.deepEqual(result.rateLimit, { limit: 200, remaining: 199 });
    assert.equal(result.serverTiming, "read;dur=0.1, parse;dur=0.2");
  });

  test("optional headers are null when absent", async () => {
    const result = await client().render(LABEL);
    assert.equal(result.labelsBilled, null);
    assert.equal(result.rateLimit, null);
    assert.equal(result.serverTiming, null);
  });

  test("gzips the body with compress", async () => {
    await client().render(LABEL, { compress: true });
    const [req] = server.requests;
    assert.equal(req.headers["content-encoding"], "gzip");
    assert.equal(req.raw[0], 0x1f);
    assert.equal(req.raw[1], 0x8b);
    assert.equal(req.body.toString(), LABEL);
  });

  test("renderPdf and renderPng set the format", async () => {
    const c = client();
    await c.renderPdf(LABEL);
    await c.renderPng(LABEL, 4);
    await c.renderPng(LABEL);
    assert.equal(server.requests[0].url.search, "?format=pdf");
    assert.equal(server.requests[1].url.search, "?format=png&page=4");
    assert.equal(server.requests[2].url.search, "?format=png");
  });

  test("saveTo writes the bytes", async () => {
    const dir = await mkdtemp(join(tmpdir(), "caretly-"));
    try {
      const result = await client().render(LABEL);
      const path = join(dir, "label.pdf");
      await result.saveTo(path);
      assert.equal((await readFile(path)).toString(), "%PDF-1.7 fake");
    } finally {
      await rm(dir, { recursive: true });
    }
  });
});

describe("renderFiles", () => {
  test("sends multipart with one `file` part per file", async () => {
    server.reply(ok({ "x-page-count": "3", "x-file-page-counts": "2, 1" }));
    const result = await client().renderFiles([
      { name: "a.zpl", content: "^XA^XZ^XA^XZ" },
      { name: "b.zpl", content: new TextEncoder().encode("^XA^FDb^FS^XZ") },
      { name: "c.zpl", content: new Blob(["^XA^FDc^FS^XZ"]) },
    ]);
    const [req] = server.requests;
    assert.match(req.headers["content-type"], /^multipart\/form-data; boundary=/);
    const form = await parseForm(req);
    const files = form.getAll("file");
    assert.equal(files.length, 3);
    assert.deepEqual(
      files.map((f) => f.name),
      ["a.zpl", "b.zpl", "c.zpl"],
    );
    assert.deepEqual(await Promise.all(files.map((f) => f.text())), ["^XA^XZ^XA^XZ", "^XA^FDb^FS^XZ", "^XA^FDc^FS^XZ"]);
    assert.deepEqual([...form.keys()].filter((k) => k !== "file"), []);
    assert.equal(result.pageCount, 3);
    assert.deepEqual(result.filePageCounts, [2, 1]);
  });

  test("gzips the multipart body with compress", async () => {
    await client().renderFiles([{ name: "a.zpl", content: LABEL }], { compress: true, format: "png" });
    const [req] = server.requests;
    assert.equal(req.headers["content-encoding"], "gzip");
    assert.equal(req.url.search, "?format=png");
    const form = await parseForm(req);
    assert.equal(await form.get("file").text(), LABEL);
  });

  test("rejects an empty list and unnamed files", async () => {
    await assert.rejects(client().renderFiles([]), TypeError);
    await assert.rejects(client().renderFiles([{ name: "", content: LABEL }]), TypeError);
    assert.equal(server.requests.length, 0);
  });
});

describe("errors", () => {
  const cases = [
    [400, BadRequestError],
    [401, AuthenticationError],
    [402, PaymentRequiredError],
    [413, PayloadTooLargeError],
    [415, UnsupportedEncodingError],
    [429, RateLimitError],
    [503, OverloadedError],
    [500, APIError],
    [404, APIError],
  ];
  for (const [status, Class] of cases) {
    test(`${status} throws ${Class.name}`, async () => {
      server.reply(fail(status, `message ${status}`, { "retry-after": "7" }));
      const error = await client({ maxRetries: 0 }).render(LABEL).catch((e) => e);
      assert.ok(error instanceof Class, `${error?.name} is a ${Class.name}`);
      assert.ok(error instanceof APIError);
      assert.ok(error instanceof CaretlyError);
      assert.ok(error instanceof Error);
      assert.equal(error.name, Class.name);
      assert.equal(error.status, status);
      assert.equal(error.message, `message ${status}`);
      assert.equal(error.retryAfter, 7);
      assert.equal(server.requests.length, 1, "not retried with maxRetries 0");
    });
  }

  test("sibling classes do not match", async () => {
    server.reply(fail(400, "parse error"));
    const error = await client().render(LABEL).catch((e) => e);
    assert.ok(!(error instanceof AuthenticationError));
    assert.ok(!(error instanceof ConnectionError));
    assert.ok(!(new Error("x") instanceof CaretlyError));
    assert.equal(error.retryAfter, null);
  });

  test("does not retry 400, 401, 402 or 500", async () => {
    for (const status of [400, 401, 402, 500]) {
      server.requests.length = 0;
      server.reply(fail(status, "no"));
      await assert.rejects(client().render(LABEL), APIError);
      assert.equal(server.requests.length, 1, `status ${status}`);
    }
    assert.deepEqual(sleeps, []);
  });

  test("unreachable server throws ConnectionError", async () => {
    const closed = await startServer();
    const baseUrl = closed.baseUrl;
    await closed.close();
    const error = await client({ baseUrl }).render(LABEL).catch((e) => e);
    assert.ok(error instanceof ConnectionError, String(error));
    assert.ok(error instanceof CaretlyError);
    assert.deepEqual(sleeps, [500, 1000], "retried with backoff");
  });

  test("slow server throws TimeoutError", async () => {
    server.reply((_req, res) => setTimeout(() => ok()(_req, res), 500));
    const error = await client({ timeoutMs: 50 }).render(LABEL).catch((e) => e);
    assert.ok(error instanceof TimeoutError, String(error));
    assert.ok(error instanceof CaretlyError);
  });

  test("an aborted signal rejects with its reason", async () => {
    const controller = new AbortController();
    server.reply((_req, res) => setTimeout(() => ok()(_req, res), 500));
    const promise = client().render(LABEL, { signal: controller.signal });
    setTimeout(() => controller.abort(), 20);
    const error = await promise.catch((e) => e);
    assert.equal(error.name, "AbortError");
  });
});

describe("retries", () => {
  test("retries 429 honouring Retry-After, with one idempotency key", async () => {
    server.reply(fail(429, "slow down", { "retry-after": "3" }), ok());
    const result = await client().render(LABEL);
    assert.equal(result.pageCount, 1);
    assert.equal(server.requests.length, 2);
    assert.deepEqual(sleeps, [3000]);
    const [first, second] = server.requests.map((r) => r.headers["idempotency-key"]);
    assert.match(first, /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/);
    assert.equal(first, second);
  });

  test("each call gets its own idempotency key", async () => {
    const c = client();
    await c.render(LABEL);
    await c.render(LABEL);
    const keys = server.requests.map((r) => r.headers["idempotency-key"]);
    assert.notEqual(keys[0], keys[1]);
  });

  test("retries 503, then gives up after maxRetries", async () => {
    server.reply(...Array.from({ length: 3 }, () => fail(503, "busy", { "retry-after": "1" })));
    const error = await client().render(LABEL).catch((e) => e);
    assert.ok(error instanceof OverloadedError);
    assert.equal(server.requests.length, 3);
    assert.deepEqual(sleeps, [1000, 1000]);
  });

  test("backs off exponentially without Retry-After, capped at 8 s", async () => {
    server.reply(...Array.from({ length: 7 }, () => fail(429, "slow down")));
    await assert.rejects(client({ maxRetries: 6 }).render(LABEL), RateLimitError);
    assert.deepEqual(sleeps, [500, 1000, 2000, 4000, 8000, 8000]);
  });

  test("keeps a user idempotency key", async () => {
    server.reply(fail(429, "slow down"), ok());
    await client().render(LABEL, { idempotencyKey: "order-42" });
    assert.deepEqual(
      server.requests.map((r) => r.headers["idempotency-key"]),
      ["order-42", "order-42"],
    );
  });

  test("no idempotency key without retries", async () => {
    await client({ maxRetries: 0 }).render(LABEL);
    assert.equal(server.requests[0].headers["idempotency-key"], undefined);
  });

  test("retries a multipart body unchanged", async () => {
    server.reply(fail(503, "busy"), ok());
    await client().renderFiles([{ name: "a.zpl", content: LABEL }], { compress: true });
    assert.equal(server.requests.length, 2);
    assert.equal(await (await parseForm(server.requests[1])).get("file").text(), LABEL);
  });
});

describe("validation", () => {
  for (const [options, Class] of [
    [{ page: 0 }, RangeError],
    [{ page: 1.5 }, RangeError],
    [{ dpmm: 10 }, RangeError],
    [{ format: "gif" }, RangeError],
    [{ idempotencyKey: "x".repeat(256) }, RangeError],
  ]) {
    test(`rejects ${JSON.stringify(options)}`, async () => {
      await assert.rejects(client().render(LABEL, options), Class);
      assert.equal(server.requests.length, 0);
    });
  }

  test("rejects non-string zpl", async () => {
    await assert.rejects(client().render(42), TypeError);
  });

  test("rejects bad client options", () => {
    assert.throws(() => new Caretly({ maxRetries: -1 }), RangeError);
    assert.throws(() => new Caretly({ timeoutMs: 0 }), RangeError);
  });
});

describe("configuration", () => {
  test("falls back to CARETLY_API_KEY", async () => {
    process.env.CARETLY_API_KEY = "zpl_live_env";
    try {
      await new Caretly({ baseUrl: server.baseUrl }).render(LABEL);
    } finally {
      delete process.env.CARETLY_API_KEY;
    }
    assert.equal(server.requests[0].headers.authorization, "Bearer zpl_live_env");
  });

  test("sends no auth header without a key", async () => {
    await new Caretly({ baseUrl: `${server.baseUrl}/` }).render(LABEL);
    assert.equal(server.requests[0].headers.authorization, undefined);
    assert.equal(server.requests[0].url.pathname, "/render", "trailing slash trimmed");
  });

  test("uses an injected fetch", async () => {
    const calls = [];
    const fetch = async (url, init) => {
      calls.push([url, init]);
      return new Response("PDF", { headers: { "content-type": "application/pdf", "x-page-count": "2" } });
    };
    const result = await new Caretly({ fetch }).render(LABEL);
    assert.equal(calls[0][0], "https://caretly.dev/render");
    assert.equal(result.pageCount, 2);
  });

  test("VERSION matches package.json", async () => {
    const pkg = createRequire(import.meta.url)("../package.json");
    assert.equal(VERSION, pkg.version);
  });
});

describe("health", () => {
  test("true for 200 ok", async () => {
    server.reply((_req, res) => res.end("ok"));
    assert.equal(await client().health(), true);
    assert.equal(server.requests[0].method, "GET");
    assert.equal(server.requests[0].url.pathname, "/health");
  });

  test("false for an error status", async () => {
    server.reply(fail(500, "down"));
    assert.equal(await client().health(), false);
  });
});
