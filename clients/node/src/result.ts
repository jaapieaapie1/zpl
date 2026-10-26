/** The rate limit of the API key, from the `RateLimit-*` headers. */
export interface RateLimit {
  /** Burst size: how many requests the key can make at once. */
  limit: number;
  /** Requests the key can still make at once before it is rate limited. */
  remaining: number;
}

/** A rendered PDF or PNG plus the metadata the server sent with it. */
export class RenderResult {
  /** The file contents. */
  readonly bytes: Uint8Array;
  /** `application/pdf` or `image/png`. */
  readonly contentType: string;
  /** Total pages across the input (`X-Page-Count`). */
  readonly pageCount: number;
  /** Pages per file, in upload order (`X-File-Page-Counts`); empty unless multipart. */
  readonly filePageCounts: number[];
  /** Labels billed (`X-Labels-Billed`); 0 for a test key, `null` when the server has no keys. */
  readonly labelsBilled: number | null;
  /** The key's rate limit; `null` when the server did not send it. */
  readonly rateLimit: RateLimit | null;
  /** Raw `Server-Timing` header, sent when `timings` is set; otherwise `null`. */
  readonly serverTiming: string | null;
  /** All response headers. */
  readonly headers: Headers;

  /** @internal */
  constructor(bytes: Uint8Array, headers: Headers) {
    this.bytes = bytes;
    this.headers = headers;
    this.contentType = headers.get("content-type") ?? "application/octet-stream";
    this.pageCount = parseInteger(headers.get("x-page-count")) ?? 0;
    this.filePageCounts = parseIntegerList(headers.get("x-file-page-counts"));
    this.labelsBilled = parseInteger(headers.get("x-labels-billed"));
    const limit = parseInteger(headers.get("ratelimit-limit"));
    const remaining = parseInteger(headers.get("ratelimit-remaining"));
    this.rateLimit = limit !== null && remaining !== null ? { limit, remaining } : null;
    this.serverTiming = headers.get("server-timing");
  }

  /**
   * Writes the file to `path`, replacing it if it exists. Needs `node:fs` (Node.js, Bun,
   * Deno); in other runtimes use `bytes` directly.
   */
  async saveTo(path: string): Promise<void> {
    const fs = await import("node:fs/promises");
    await fs.writeFile(path, this.bytes);
  }
}

function parseInteger(value: string | null): number | null {
  if (value === null) return null;
  const trimmed = value.trim();
  if (!/^\d+$/.test(trimmed)) return null;
  return Number(trimmed);
}

function parseIntegerList(value: string | null): number[] {
  if (value === null || value.trim() === "") return [];
  return value
    .split(",")
    .map(parseInteger)
    .filter((n): n is number => n !== null);
}
