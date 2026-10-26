/**
 * Errors thrown by the Caretly client.
 *
 * Every class carries a brand registered with `Symbol.for`, so `instanceof` keeps working
 * when an application loads both the ESM and the CommonJS build of this package (the
 * "dual package hazard"): an error made by one copy is still an instance of the classes
 * of the other copy.
 */

const BRANDS = Symbol.for("caretly.error.brands");

type Branded = { [BRANDS]?: readonly string[] };

function brandsOf(target: Function): string[] {
  const brands: string[] = [];
  let current: unknown = target;
  while (typeof current === "function" && current !== Error) {
    const own = Object.prototype.hasOwnProperty.call(current, "brand")
      ? (current as { brand?: unknown }).brand
      : undefined;
    if (typeof own === "string") brands.push(own);
    current = Object.getPrototypeOf(current);
  }
  return brands;
}

/** Base class of every error thrown by this package, other than argument errors. */
export class CaretlyError extends Error {
  /** @internal */
  static readonly brand: string = "CaretlyError";

  constructor(message: string, options?: { cause?: unknown }) {
    super(message, options);
    this.name = new.target.name;
    Object.defineProperty(this, BRANDS, { value: brandsOf(new.target), enumerable: false });
  }

  static [Symbol.hasInstance](instance: unknown): boolean {
    if (typeof instance !== "object" || instance === null) return false;
    const brands = (instance as Branded)[BRANDS];
    const brand = Object.prototype.hasOwnProperty.call(this, "brand")
      ? (this as unknown as { brand: string }).brand
      : undefined;
    if (Array.isArray(brands) && brand !== undefined) return brands.includes(brand);
    return Function.prototype[Symbol.hasInstance].call(this, instance);
  }
}

/** The API answered with an error status. `message` is the server's plain-text message. */
export class APIError extends CaretlyError {
  /** @internal */
  static override readonly brand: string = "APIError";

  /** HTTP status code. */
  readonly status: number;
  /** Seconds to wait before retrying, from the `Retry-After` header; `null` when absent. */
  readonly retryAfter: number | null;
  /** The response headers. */
  readonly headers: Headers;

  constructor(status: number, message: string, headers: Headers, retryAfter: number | null) {
    super(message || `HTTP ${status}`);
    this.status = status;
    this.headers = headers;
    this.retryAfter = retryAfter;
  }
}

/** 400: a ZPL parse error, an invalid parameter or a `page` out of range. */
export class BadRequestError extends APIError {
  /** @internal */
  static override readonly brand: string = "BadRequestError";
}

/** 401: the API key is missing, not valid, expired or revoked. */
export class AuthenticationError extends APIError {
  /** @internal */
  static override readonly brand: string = "AuthenticationError";
}

/** 402: billing for the account is not active, so live keys are paused. */
export class PaymentRequiredError extends APIError {
  /** @internal */
  static override readonly brand: string = "PaymentRequiredError";
}

/** 413: the body is larger than 100 MiB after decompression. */
export class PayloadTooLargeError extends APIError {
  /** @internal */
  static override readonly brand: string = "PayloadTooLargeError";
}

/** 415: unsupported `Content-Encoding`. */
export class UnsupportedEncodingError extends APIError {
  /** @internal */
  static override readonly brand: string = "UnsupportedEncodingError";
}

/** 429: too many requests for this API key. See `retryAfter`. */
export class RateLimitError extends APIError {
  /** @internal */
  static override readonly brand: string = "RateLimitError";
}

/** 503: the server is overloaded and turned the request away (not billed). See `retryAfter`. */
export class OverloadedError extends APIError {
  /** @internal */
  static override readonly brand: string = "OverloadedError";
}

/** The request did not reach the server, or the connection broke before a full response. */
export class ConnectionError extends CaretlyError {
  /** @internal */
  static override readonly brand: string = "ConnectionError";
}

/** The request took longer than `timeoutMs`. */
export class TimeoutError extends CaretlyError {
  /** @internal */
  static override readonly brand: string = "TimeoutError";
}

/** @internal */
export function apiErrorFor(
  status: number,
  message: string,
  headers: Headers,
  retryAfter: number | null,
): APIError {
  const Class =
    {
      400: BadRequestError,
      401: AuthenticationError,
      402: PaymentRequiredError,
      413: PayloadTooLargeError,
      415: UnsupportedEncodingError,
      429: RateLimitError,
      503: OverloadedError,
    }[status] ?? APIError;
  return new Class(status, message, headers, retryAfter);
}
