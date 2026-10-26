import { apiErrorFor, APIError, ConnectionError, RateLimitError, OverloadedError, TimeoutError } from "./errors.js";
import { RenderResult } from "./result.js";
import { VERSION } from "./version.js";

/** Output format. */
export type Format = "pdf" | "png";

/** Printer resolution in dots per millimetre: 6 (152 dpi), 8 (203 dpi), 12 (300 dpi), 24 (600 dpi). */
export type Dpmm = 6 | 8 | 12 | 24;

export interface ClientOptions {
  /** API key (`zpl_live_…` or `zpl_test_…`). Defaults to the `CARETLY_API_KEY` environment variable. Optional for self-hosted servers. */
  apiKey?: string;
  /** Defaults to `https://caretly.dev`. */
  baseUrl?: string;
  /** Timeout per attempt, in milliseconds. Defaults to 30000. */
  timeoutMs?: number;
  /** Retries after a 429, a 503 or a connection error. Defaults to 2. */
  maxRetries?: number;
  /** A `fetch` implementation. Defaults to the global `fetch`. */
  fetch?: typeof fetch;
  /** Waits between retries. Defaults to a timer; replace it in tests. */
  sleep?: (ms: number, signal?: AbortSignal) => Promise<void>;
}

export interface RequestOptions {
  /** Aborts the call, including any wait between retries. */
  signal?: AbortSignal;
}

export interface RenderOptions extends RequestOptions {
  /** `pdf` (every page) or `png` (one page, chosen with `page`). Server default: `pdf`. */
  format?: Format;
  /** For PNG: the page to render, starting at 1. Server default: 1. */
  page?: number;
  /** Printer resolution. Server default: 8. */
  dpmm?: Dpmm;
  /** Ask for a `Server-Timing` header, exposed as `serverTiming`. */
  timings?: boolean;
  /**
   * Up to 255 characters, for example an order number. A retry with the same key and the
   * same content is billed once. When omitted and retries are enabled, the client makes a
   * UUID per call so its own retries are never billed twice.
   */
  idempotencyKey?: string;
  /** Gzip the request body. */
  compress?: boolean;
}

/** A ZPL file for `renderFiles`. */
export interface RenderFile {
  /** File name, sent as the part's filename. */
  name: string;
  /** ZPL text, or its bytes. */
  content: string | Uint8Array | Blob;
}

const DEFAULT_BASE_URL = "https://caretly.dev";
const DEFAULT_TIMEOUT_MS = 30_000;
const DEFAULT_MAX_RETRIES = 2;
const DPMM_VALUES: readonly number[] = [6, 8, 12, 24];
const MAX_BACKOFF_MS = 8_000;

/** Client for the Caretly API: render ZPL labels to PDF or PNG. */
export class Caretly {
  readonly baseUrl: string;
  readonly timeoutMs: number;
  readonly maxRetries: number;
  readonly #apiKey: string | undefined;
  readonly #fetch: typeof fetch;
  readonly #sleep: (ms: number, signal?: AbortSignal) => Promise<void>;

  constructor(options: ClientOptions = {}) {
    const apiKey = options.apiKey ?? envApiKey();
    this.#apiKey = apiKey === undefined || apiKey === "" ? undefined : apiKey;
    this.baseUrl = (options.baseUrl ?? DEFAULT_BASE_URL).replace(/\/+$/, "");
    this.timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT_MS;
    this.maxRetries = options.maxRetries ?? DEFAULT_MAX_RETRIES;
    if (!(this.timeoutMs > 0)) throw new RangeError("timeoutMs must be a positive number");
    if (!Number.isInteger(this.maxRetries) || this.maxRetries < 0) {
      throw new RangeError("maxRetries must be a whole number, 0 or more");
    }
    const fetchImpl = options.fetch ?? globalThis.fetch;
    if (typeof fetchImpl !== "function") {
      throw new TypeError("no fetch implementation found; pass one as the `fetch` option");
    }
    this.#fetch = fetchImpl;
    this.#sleep = options.sleep ?? sleep;
  }

  /** Renders one ZPL document (it may hold several labels). */
  async render(zpl: string, options: RenderOptions = {}): Promise<RenderResult> {
    if (typeof zpl !== "string") throw new TypeError("zpl must be a string");
    const query = renderQuery(options);
    const prepare = async (): Promise<PreparedBody> => {
      if (!options.compress) return { body: zpl, contentType: "text/plain; charset=utf-8" };
      return {
        body: await gzip(new TextEncoder().encode(zpl)),
        contentType: "text/plain; charset=utf-8",
        contentEncoding: "gzip",
      };
    };
    return this.#render(query, prepare, options);
  }

  /**
   * Renders several ZPL files in one request. Each file starts from a fresh printer state;
   * pages follow upload order, and `filePageCounts` gives the pages of each file.
   */
  async renderFiles(files: readonly RenderFile[], options: RenderOptions = {}): Promise<RenderResult> {
    if (!Array.isArray(files) || files.length === 0) throw new TypeError("files must be a non-empty array");
    const form = new FormData();
    for (const file of files) {
      if (typeof file?.name !== "string" || file.name === "") {
        throw new TypeError("every file needs a non-empty name");
      }
      form.append("file", toBlob(file.content), file.name);
    }
    const query = renderQuery(options);
    const prepare = async (): Promise<PreparedBody> => {
      if (!options.compress) return { body: form };
      // Serialise the form to get its bytes and boundary, then gzip it.
      const request = new Request("http://localhost/", { method: "POST", body: form });
      const contentType = request.headers.get("content-type") ?? undefined;
      const bytes = new Uint8Array(await request.arrayBuffer());
      return { body: await gzip(bytes), contentType, contentEncoding: "gzip" };
    };
    return this.#render(query, prepare, options);
  }

  /** Renders `zpl` to a PDF holding every page. */
  renderPdf(zpl: string, options: Omit<RenderOptions, "format" | "page"> = {}): Promise<RenderResult> {
    return this.render(zpl, { ...options, format: "pdf" });
  }

  /** Renders one page (default 1) of `zpl` to a PNG. */
  renderPng(zpl: string, page?: number, options: Omit<RenderOptions, "format" | "page"> = {}): Promise<RenderResult> {
    return this.render(zpl, page === undefined ? { ...options, format: "png" } : { ...options, format: "png", page });
  }

  /** `true` when `GET /health` answers 200 `ok`. Throws `ConnectionError` / `TimeoutError` when unreachable. */
  async health(options: RequestOptions = {}): Promise<boolean> {
    const response = await this.#send(`${this.baseUrl}/health`, { method: "GET" }, options.signal, async (r) => ({
      ok: r.ok,
      text: await r.text(),
    }));
    return response.ok && response.text.trim() === "ok";
  }

  async #render(
    query: URLSearchParams,
    prepare: () => Promise<PreparedBody>,
    options: RenderOptions,
  ): Promise<RenderResult> {
    const prepared = await prepare();
    const headers: Record<string, string> = {};
    if (prepared.contentType) headers["content-type"] = prepared.contentType;
    if (prepared.contentEncoding) headers["content-encoding"] = prepared.contentEncoding;
    const idempotencyKey = options.idempotencyKey ?? (this.maxRetries > 0 ? await uuid() : undefined);
    if (idempotencyKey !== undefined) headers["idempotency-key"] = idempotencyKey;

    const search = query.toString();
    const url = `${this.baseUrl}/render${search ? `?${search}` : ""}`;
    const { signal } = options;

    for (let attempt = 0; ; attempt++) {
      try {
        return await this.#send(url, { method: "POST", headers, body: prepared.body }, signal, async (response) => {
          const bytes = new Uint8Array(await response.arrayBuffer());
          if (!response.ok) {
            const message = new TextDecoder().decode(bytes).trim();
            throw apiErrorFor(response.status, message, response.headers, parseRetryAfter(response.headers));
          }
          return new RenderResult(bytes, response.headers);
        });
      } catch (error) {
        if (attempt >= this.maxRetries || !isRetryable(error)) throw error;
        await this.#sleep(retryDelay(error, attempt), signal);
      }
    }
  }

  /** Sends one request with the timeout and reads the response inside it. */
  async #send<T>(
    url: string,
    init: RequestInit,
    signal: AbortSignal | undefined,
    read: (response: Response) => Promise<T>,
  ): Promise<T> {
    signal?.throwIfAborted();
    const timeout = AbortSignal.timeout(this.timeoutMs);
    const { signal: combined, dispose } = anySignal(signal ? [signal, timeout] : [timeout]);
    const headers = new Headers(init.headers);
    headers.set("user-agent", `caretly-node/${VERSION}`);
    if (this.#apiKey !== undefined) headers.set("authorization", `Bearer ${this.#apiKey}`);
    try {
      let response: Response;
      try {
        response = await this.#fetch(url, { ...init, headers, signal: combined });
      } catch (error) {
        throw transportError(error, signal, timeout, "request failed");
      }
      try {
        return await read(response);
      } catch (error) {
        if (error instanceof APIError) throw error;
        throw transportError(error, signal, timeout, "reading the response failed");
      }
    } finally {
      dispose();
    }
  }
}

interface PreparedBody {
  body: string | Uint8Array<ArrayBuffer> | FormData;
  contentType?: string | undefined;
  contentEncoding?: string;
}

function renderQuery(options: RenderOptions): URLSearchParams {
  const query = new URLSearchParams();
  const { format, page, dpmm, timings, idempotencyKey } = options;
  if (format !== undefined) {
    if (format !== "pdf" && format !== "png") throw new RangeError(`format must be "pdf" or "png", got ${String(format)}`);
    query.set("format", format);
  }
  if (page !== undefined) {
    if (!Number.isInteger(page) || page < 1) throw new RangeError(`page must be a whole number, 1 or more, got ${String(page)}`);
    query.set("page", String(page));
  }
  if (dpmm !== undefined) {
    if (!DPMM_VALUES.includes(dpmm)) throw new RangeError(`dpmm must be 6, 8, 12 or 24, got ${String(dpmm)}`);
    query.set("dpmm", String(dpmm));
  }
  if (timings) query.set("timings", "1");
  if (idempotencyKey !== undefined) {
    if (typeof idempotencyKey !== "string" || idempotencyKey.length === 0 || idempotencyKey.length > 255) {
      throw new RangeError("idempotencyKey must be a string of 1 to 255 characters");
    }
  }
  return query;
}

function toBlob(content: RenderFile["content"]): Blob {
  if (typeof Blob !== "undefined" && content instanceof Blob) return content;
  if (typeof content === "string") return new Blob([content], { type: "text/plain; charset=utf-8" });
  if (content instanceof Uint8Array) return new Blob([content as Uint8Array<ArrayBuffer>], { type: "application/octet-stream" });
  throw new TypeError("file content must be a string, a Uint8Array or a Blob");
}

function isRetryable(error: unknown): boolean {
  return error instanceof RateLimitError || error instanceof OverloadedError || error instanceof ConnectionError;
}

function retryDelay(error: unknown, attempt: number): number {
  if (error instanceof APIError && error.retryAfter !== null) return error.retryAfter * 1000;
  return Math.min(500 * 2 ** attempt, MAX_BACKOFF_MS);
}

/** `Retry-After` in seconds: either a number of seconds or an HTTP date. */
function parseRetryAfter(headers: Headers): number | null {
  const value = headers.get("retry-after")?.trim();
  if (!value) return null;
  if (/^\d+$/.test(value)) return Number(value);
  const date = Date.parse(value);
  if (Number.isNaN(date)) return null;
  return Math.max(0, Math.ceil((date - Date.now()) / 1000));
}

function transportError(error: unknown, signal: AbortSignal | undefined, timeout: AbortSignal, what: string): unknown {
  if (signal?.aborted) return signal.reason;
  if (timeout.aborted) return new TimeoutError("request timed out", { cause: error });
  const detail = error instanceof Error ? `: ${error.message}` : "";
  return new ConnectionError(`${what}${detail}`, { cause: error });
}

function anySignal(signals: AbortSignal[]): { signal: AbortSignal; dispose: () => void } {
  if (signals.length === 1) return { signal: signals[0]!, dispose: () => {} };
  const controller = new AbortController();
  const listeners: Array<() => void> = [];
  for (const signal of signals) {
    if (signal.aborted) {
      controller.abort(signal.reason);
      break;
    }
    const onAbort = (): void => controller.abort(signal.reason);
    signal.addEventListener("abort", onAbort, { once: true });
    listeners.push(() => signal.removeEventListener("abort", onAbort));
  }
  return { signal: controller.signal, dispose: () => listeners.forEach((remove) => remove()) };
}

function sleep(ms: number, signal?: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    if (signal?.aborted) return reject(signal.reason);
    const onAbort = (): void => {
      clearTimeout(timer);
      reject(signal!.reason);
    };
    const timer = setTimeout(() => {
      signal?.removeEventListener("abort", onAbort);
      resolve();
    }, ms);
    signal?.addEventListener("abort", onAbort, { once: true });
  });
}

async function gzip(data: Uint8Array): Promise<Uint8Array<ArrayBuffer>> {
  if (typeof CompressionStream !== "undefined") {
    const stream = new Blob([data as Uint8Array<ArrayBuffer>]).stream().pipeThrough(new CompressionStream("gzip"));
    return new Uint8Array(await new Response(stream).arrayBuffer());
  }
  const zlib = await import("node:zlib");
  return new Uint8Array(zlib.gzipSync(data));
}

async function uuid(): Promise<string> {
  const webCrypto = (globalThis as { crypto?: { randomUUID?: () => string } }).crypto;
  if (typeof webCrypto?.randomUUID === "function") return webCrypto.randomUUID();
  // Node.js 18 only exposes `crypto` globally behind a flag.
  const nodeCrypto = await import("node:crypto");
  return nodeCrypto.randomUUID();
}

function envApiKey(): string | undefined {
  try {
    const env = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env;
    return env?.CARETLY_API_KEY;
  } catch {
    // Deno without --allow-env throws on access.
    return undefined;
  }
}
