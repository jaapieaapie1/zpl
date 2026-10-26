"""The asyncio client."""

from __future__ import annotations

import asyncio
from types import TracebackType
from typing import Awaitable, Callable, Optional, Type

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

__all__ = ["AsyncClient"]


class AsyncClient(BaseClient):
    """Asynchronous (asyncio) client for the Caretly ZPL rendering API.

    Takes the same arguments as :class:`caretly.Client`, with ``http_client`` an
    ``httpx.AsyncClient``. Use it as an async context manager, or await :meth:`aclose`.
    """

    def __init__(
        self,
        api_key: Optional[str] = None,
        base_url: str = DEFAULT_BASE_URL,
        timeout: TimeoutTypes = DEFAULT_TIMEOUT,
        max_retries: int = DEFAULT_MAX_RETRIES,
        *,
        http_client: Optional[httpx.AsyncClient] = None,
    ) -> None:
        super().__init__(api_key, base_url, timeout, max_retries)
        self._owns_http = http_client is None
        self._http = http_client if http_client is not None else httpx.AsyncClient()
        self._sleep: Callable[[float], Awaitable[None]] = asyncio.sleep

    async def aclose(self) -> None:
        """Close the underlying HTTP client, unless it was passed in."""
        if self._owns_http:
            await self._http.aclose()

    async def __aenter__(self) -> AsyncClient:
        return self

    async def __aexit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc: Optional[BaseException],
        tb: Optional[TracebackType],
    ) -> None:
        await self.aclose()

    async def render(
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
        """Render a ZPL label to a PDF or PNG. See :meth:`caretly.Client.render`."""
        request = self._prepare_label(
            zpl,
            format=format,
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )
        return await self._send_render(request)

    async def render_files(
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
        """Render several ZPL files in one request. See :meth:`caretly.Client.render_files`."""
        request = self._prepare_files(
            files,
            format=format,
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )
        return await self._send_render(request)

    async def render_pdf(
        self,
        zpl: FileContent,
        *,
        dpmm: Optional[int] = None,
        timings: bool = False,
        idempotency_key: Optional[str] = None,
        compress: bool = False,
    ) -> RenderResult:
        """Shorthand for ``render(zpl, format="pdf", ...)``."""
        return await self.render(
            zpl,
            format="pdf",
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )

    async def render_png(
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
        return await self.render(
            zpl,
            format="png",
            page=page,
            dpmm=dpmm,
            timings=timings,
            idempotency_key=idempotency_key,
            compress=compress,
        )

    async def health(self) -> bool:
        """Return whether the server answers ``GET /health`` with ``ok``."""
        request = self._prepare_health()
        try:
            response = await self._http.request(
                request.method, request.url, headers=request.headers, timeout=self.timeout
            )
        except httpx.TransportError as exc:
            raise error_from_transport(exc) from exc
        return is_healthy(response)

    async def _send_render(self, request: PreparedRequest) -> RenderResult:
        attempt = 0
        while True:
            error: CaretlyError
            try:
                response = await self._http.request(
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
            await self._sleep(retry_delay(attempt, error))
            attempt += 1
