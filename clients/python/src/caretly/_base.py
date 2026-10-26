"""Request building, response parsing and retry policy shared by both clients.

Nothing here does I/O, so :class:`caretly.Client` and :class:`caretly.AsyncClient`
behave the same and differ only in how they send requests and sleep.
"""

from __future__ import annotations

import email.utils
import gzip
import os
import time
import uuid
from dataclasses import dataclass
from typing import Dict, List, Mapping, Optional, Tuple, Type, Union

import httpx

from ._exceptions import (
    APIConnectionError,
    APIError,
    APITimeoutError,
    AuthenticationError,
    BadRequestError,
    CaretlyError,
    OverloadedError,
    PayloadTooLargeError,
    PaymentRequiredError,
    RateLimitError,
    UnsupportedEncodingError,
)
from ._types import FileContent, Files, RateLimit, RenderResult
from ._version import __version__

DEFAULT_BASE_URL = "https://caretly.dev"
DEFAULT_TIMEOUT = 30.0
DEFAULT_MAX_RETRIES = 2
API_KEY_ENV = "CARETLY_API_KEY"
USER_AGENT = f"caretly-python/{__version__}"

FORMATS = ("pdf", "png")
DPMM_VALUES = (6, 8, 12, 24)
MAX_IDEMPOTENCY_KEY_LENGTH = 255

INITIAL_BACKOFF = 0.5
MAX_BACKOFF = 8.0
RETRY_STATUSES = frozenset({429, 503})

TimeoutTypes = Union[float, httpx.Timeout, None]

_ERRORS: Dict[int, Type[APIError]] = {
    400: BadRequestError,
    401: AuthenticationError,
    402: PaymentRequiredError,
    413: PayloadTooLargeError,
    415: UnsupportedEncodingError,
    429: RateLimitError,
    503: OverloadedError,
}


@dataclass(frozen=True)
class PreparedRequest:
    """Everything needed to send one logical request, possibly more than once."""

    method: str
    url: str
    params: Dict[str, str]
    headers: Dict[str, str]
    content: Optional[bytes] = None


class BaseClient:
    """Configuration and request building shared by the sync and async clients."""

    api_key: Optional[str]
    base_url: str
    timeout: TimeoutTypes
    max_retries: int

    def __init__(
        self,
        api_key: Optional[str],
        base_url: str,
        timeout: TimeoutTypes,
        max_retries: int,
    ) -> None:
        if max_retries < 0:
            raise ValueError("max_retries must be 0 or more")
        self.api_key = api_key or os.environ.get(API_KEY_ENV) or None
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self.max_retries = max_retries

    def __repr__(self) -> str:
        return f"{type(self).__name__}(base_url={self.base_url!r})"

    def _headers(self) -> Dict[str, str]:
        headers = {"User-Agent": USER_AGENT}
        if self.api_key:
            headers["Authorization"] = f"Bearer {self.api_key}"
        return headers

    def _prepare_health(self) -> PreparedRequest:
        return PreparedRequest("GET", f"{self.base_url}/health", {}, self._headers())

    def _prepare_render(
        self,
        body: bytes,
        content_type: str,
        *,
        format: str,
        page: Optional[int],
        dpmm: Optional[int],
        timings: bool,
        idempotency_key: Optional[str],
        compress: bool,
    ) -> PreparedRequest:
        params = _query(format=format, page=page, dpmm=dpmm, timings=timings)
        headers = self._headers()
        headers["Content-Type"] = content_type
        if idempotency_key is not None:
            _check_idempotency_key(idempotency_key)
        elif self.max_retries > 0:
            # One key per logical call, so a retry after a lost response is not billed twice.
            idempotency_key = str(uuid.uuid4())
        if idempotency_key is not None:
            headers["Idempotency-Key"] = idempotency_key
        if compress:
            body = gzip.compress(body, mtime=0)
            headers["Content-Encoding"] = "gzip"
        return PreparedRequest("POST", f"{self.base_url}/render", params, headers, body)

    def _prepare_label(
        self,
        zpl: FileContent,
        *,
        format: str,
        page: Optional[int],
        dpmm: Optional[int],
        timings: bool,
        idempotency_key: Optional[str],
        compress: bool,
    ) -> PreparedRequest:
        return self._prepare_render(
            _to_bytes(zpl, "zpl"),
            "text/plain; charset=utf-8",
            format=format,
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )

    def _prepare_files(
        self,
        files: Files,
        *,
        format: str,
        page: Optional[int],
        dpmm: Optional[int],
        timings: bool,
        idempotency_key: Optional[str],
        compress: bool,
    ) -> PreparedRequest:
        body, content_type = encode_multipart(files)
        return self._prepare_render(
            body,
            content_type,
            format=format,
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )


def _query(*, format: str, page: Optional[int], dpmm: Optional[int], timings: bool) -> Dict[str, str]:
    if format not in FORMATS:
        raise ValueError(f"format must be 'pdf' or 'png', not {format!r}")
    params = {"format": format}
    if page is not None:
        if isinstance(page, bool) or not isinstance(page, int) or page < 1:
            raise ValueError(f"page must be an integer of 1 or more, not {page!r}")
        params["page"] = str(page)
    if dpmm is not None:
        if isinstance(dpmm, bool) or dpmm not in DPMM_VALUES:
            raise ValueError(f"dpmm must be 6, 8, 12 or 24, not {dpmm!r}")
        params["dpmm"] = str(dpmm)
    if timings:
        params["timings"] = "1"
    return params


def _check_idempotency_key(key: str) -> None:
    if not key:
        raise ValueError("idempotency_key must not be empty")
    if len(key) > MAX_IDEMPOTENCY_KEY_LENGTH:
        raise ValueError(f"idempotency_key must be at most {MAX_IDEMPOTENCY_KEY_LENGTH} characters")
    if not key.isascii() or not key.isprintable():
        raise ValueError("idempotency_key must be printable ASCII")


def _to_bytes(content: FileContent, what: str) -> bytes:
    if isinstance(content, str):
        return content.encode("utf-8")
    if isinstance(content, (bytes, bytearray, memoryview)):
        return bytes(content)
    raise TypeError(f"{what} must be str or bytes, not {type(content).__name__}")


def _quote_filename(name: str) -> str:
    # As browsers do (WHATWG multipart/form-data encoding).
    return name.replace("\r", "%0D").replace("\n", "%0A").replace('"', "%22")


def encode_multipart(files: Files) -> Tuple[bytes, str]:
    """Encode ``files`` as ``multipart/form-data``, each as a ``file`` part with a filename."""
    if isinstance(files, (str, bytes)):
        raise TypeError("files must be a mapping or a sequence of (name, content) pairs")
    items: List[Tuple[str, FileContent]] = list(files.items()) if isinstance(files, Mapping) else list(files)
    if not items:
        raise ValueError("files must contain at least one file")
    boundary = f"caretly-{uuid.uuid4().hex}"
    parts: List[bytes] = []
    for item in items:
        if not isinstance(item, tuple) or len(item) != 2:
            raise TypeError("each file must be a (name, content) pair")
        name, content = item
        if not isinstance(name, str):
            raise TypeError(f"file name must be str, not {type(name).__name__}")
        data = _to_bytes(content, f"content of {name!r}")
        head = (
            f"--{boundary}\r\n"
            f'Content-Disposition: form-data; name="file"; filename="{_quote_filename(name)}"\r\n'
            "Content-Type: text/plain; charset=utf-8\r\n"
            "\r\n"
        )
        parts.append(head.encode("utf-8") + data + b"\r\n")
    parts.append(f"--{boundary}--\r\n".encode("ascii"))
    return b"".join(parts), f"multipart/form-data; boundary={boundary}"


def _int_header(headers: httpx.Headers, name: str) -> Optional[int]:
    value = headers.get(name)
    if value is None:
        return None
    try:
        return int(value.strip())
    except ValueError:
        return None


def parse_retry_after(headers: httpx.Headers) -> Optional[float]:
    """Seconds from ``Retry-After``, given as seconds or as an HTTP date."""
    value = headers.get("retry-after")
    if value is None:
        return None
    value = value.strip()
    try:
        return max(0.0, float(value))
    except ValueError:
        pass
    try:
        when = email.utils.parsedate_to_datetime(value)
    except (TypeError, ValueError):
        return None
    return max(0.0, when.timestamp() - time.time())


def parse_render_response(response: httpx.Response) -> RenderResult:
    headers = response.headers
    counts = headers.get("x-file-page-counts", "")
    file_page_counts = tuple(int(c) for c in counts.split(",") if c.strip())
    limit = _int_header(headers, "ratelimit-limit")
    remaining = _int_header(headers, "ratelimit-remaining")
    return RenderResult(
        content=response.content,
        content_type=headers.get("content-type", ""),
        page_count=_int_header(headers, "x-page-count") or 0,
        file_page_counts=file_page_counts,
        labels_billed=_int_header(headers, "x-labels-billed"),
        rate_limit=RateLimit(limit, remaining) if limit is not None and remaining is not None else None,
        server_timing=headers.get("server-timing"),
    )


def error_from_response(response: httpx.Response) -> APIError:
    message = response.text.strip() or response.reason_phrase
    cls = _ERRORS.get(response.status_code, APIError)
    return cls(
        response.status_code,
        message,
        retry_after=parse_retry_after(response.headers),
        response=response,
    )


def error_from_transport(exc: httpx.TransportError) -> APIConnectionError:
    if isinstance(exc, httpx.TimeoutException):
        return APITimeoutError(f"request timed out: {exc}")
    return APIConnectionError(f"connection error: {exc}")


def is_retryable_transport_error(exc: httpx.TransportError) -> bool:
    """Network and protocol errors are retried; timeouts are not, as in the other clients."""
    if isinstance(exc, httpx.TimeoutException):
        return False
    return isinstance(exc, (httpx.NetworkError, httpx.RemoteProtocolError))


def retry_delay(attempt: int, error: CaretlyError) -> float:
    """Seconds to wait before retry number ``attempt + 1`` (``attempt`` counts from 0)."""
    if isinstance(error, APIError) and error.retry_after is not None:
        return error.retry_after
    return float(min(INITIAL_BACKOFF * (2**attempt), MAX_BACKOFF))


def is_retryable_error(error: CaretlyError) -> bool:
    if isinstance(error, APIError):
        return error.status_code in RETRY_STATUSES
    return isinstance(error, APIConnectionError) and not isinstance(error, APITimeoutError)


def is_healthy(response: httpx.Response) -> bool:
    return response.status_code == 200 and response.text.strip() == "ok"
