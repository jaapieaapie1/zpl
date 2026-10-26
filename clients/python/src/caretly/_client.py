"""The synchronous client."""

from __future__ import annotations

import time
from types import TracebackType
from typing import Callable, Optional, Type

import httpx

from ._base import (
    DEFAULT_BASE_URL,
    DEFAULT_MAX_RETRIES,
    DEFAULT_TIMEOUT,
    BaseClient,
    PreparedRequest,
    TimeoutTypes,
    error_from_response,
    error_from_transport,
    is_healthy,
    is_retryable_error,
    is_retryable_transport_error,
    parse_render_response,
    retry_delay,
)
from ._exceptions import CaretlyError
from ._types import FileContent, Files, Format, RenderResult

__all__ = ["Client"]


class Client(BaseClient):
    """Synchronous client for the Caretly ZPL rendering API.

    Args:
        api_key: API key (``zpl_live_…`` or ``zpl_test_…``). Defaults to the
            ``CARETLY_API_KEY`` environment variable. Optional for self-hosted servers.
        base_url: Server URL. Defaults to ``https://caretly.dev``.
        timeout: Per-request timeout in seconds, an ``httpx.Timeout``, or ``None`` for none.
        max_retries: How often to retry 429, 503 and connection errors
            (timeouts are not retried).
        http_client: An ``httpx.Client`` to send requests with. The caller keeps
            ownership: :meth:`close` does not close it.

    Use it as a context manager, or call :meth:`close`, to release connections.
    """

    def __init__(
        self,
        api_key: Optional[str] = None,
        base_url: str = DEFAULT_BASE_URL,
        timeout: TimeoutTypes = DEFAULT_TIMEOUT,
        max_retries: int = DEFAULT_MAX_RETRIES,
        *,
        http_client: Optional[httpx.Client] = None,
    ) -> None:
        super().__init__(api_key, base_url, timeout, max_retries)
        self._owns_http = http_client is None
        self._http = http_client if http_client is not None else httpx.Client()
        self._sleep: Callable[[float], None] = time.sleep

    def close(self) -> None:
        """Close the underlying HTTP client, unless it was passed in."""
        if self._owns_http:
            self._http.close()

    def __enter__(self) -> Client:
        return self

    def __exit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc: Optional[BaseException],
        tb: Optional[TracebackType],
    ) -> None:
        self.close()

    def render(
        self,
        zpl: FileContent,
        *,
        format: Format = "pdf",
        page: Optional[int] = None,
        dpmm: Optional[int] = None,
        timings: bool = False,
        idempotency_key: Optional[str] = None,
        compress: bool = False,
    ) -> RenderResult:
        """Render a ZPL label (or several ``^XA…^XZ`` labels) to a PDF or PNG.

        Args:
            zpl: The ZPL, as text (sent as UTF-8) or bytes.
            format: ``"pdf"`` (every page) or ``"png"`` (the page chosen with ``page``).
            page: For PNG, the page to render, starting at 1.
            dpmm: Printer resolution in dots per millimetre: 6, 8, 12 or 24.
            timings: Ask for a ``Server-Timing`` header (see ``RenderResult.server_timing``).
            idempotency_key: Up to 255 characters; a retry with the same key and
                content is billed once. Generated per call when retries are enabled.
            compress: Gzip the request body.

        Raises:
            ValueError: An option is out of range.
            APIError: The API answered with an error (a subclass per status).
            APIConnectionError: The request got no response.
        """
        request = self._prepare_label(
            zpl,
            format=format,
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )
        return self._send_render(request)

    def render_files(
        self,
        files: Files,
        *,
        format: Format = "pdf",
        page: Optional[int] = None,
        dpmm: Optional[int] = None,
        timings: bool = False,
        idempotency_key: Optional[str] = None,
        compress: bool = False,
    ) -> RenderResult:
        """Render several ZPL files in one request (``multipart/form-data``).

        Each file starts from a fresh printer state and pages follow in the order given.
        ``RenderResult.file_page_counts`` gives the pages per file.

        Args:
            files: A mapping of file name to content, or a sequence of ``(name, content)``.

        The other arguments, and the errors raised, are as for :meth:`render`.
        """
        request = self._prepare_files(
            files,
            format=format,
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )
        return self._send_render(request)

    def render_pdf(
        self,
        zpl: FileContent,
        *,
        dpmm: Optional[int] = None,
        timings: bool = False,
        idempotency_key: Optional[str] = None,
        compress: bool = False,
    ) -> RenderResult:
        """Shorthand for ``render(zpl, format="pdf", ...)``."""
        return self.render(
            zpl,
            format="pdf",
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )

    def render_png(
        self,
        zpl: FileContent,
        page: Optional[int] = None,
        *,
        dpmm: Optional[int] = None,
        timings: bool = False,
        idempotency_key: Optional[str] = None,
        compress: bool = False,
    ) -> RenderResult:
        """Shorthand for ``render(zpl, format="png", page=page, ...)``."""
        return self.render(
            zpl,
            format="png",
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )

    def health(self) -> bool:
        """Return whether the server answers ``GET /health`` with ``ok``.

        Raises:
            APIConnectionError: The server could not be reached.
        """
        request = self._prepare_health()
        try:
            response = self._http.request(request.method, request.url, headers=request.headers, timeout=self.timeout)
        except httpx.TransportError as exc:
            raise error_from_transport(exc) from exc
        return is_healthy(response)

    def _send_render(self, request: PreparedRequest) -> RenderResult:
        attempt = 0
        while True:
            error: CaretlyError
            try:
                response = self._http.request(
                    request.method,
                    request.url,
                    params=request.params,
                    headers=request.headers,
                    content=request.content,
                    timeout=self.timeout,
                )
            except httpx.TransportError as exc:
                error = error_from_transport(exc)
                if not is_retryable_transport_error(exc) or attempt >= self.max_retries:
                    raise error from exc
            else:
                if response.is_success:
                    return parse_render_response(response)
                error = error_from_response(response)
                if not is_retryable_error(error) or attempt >= self.max_retries:
                    raise error
            self._sleep(retry_delay(attempt, error))
            attempt += 1
