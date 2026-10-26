"""Errors raised by the Caretly client."""

from __future__ import annotations

from typing import Optional

import httpx

__all__ = [
    "APIConnectionError",
    "APIError",
    "APITimeoutError",
    "AuthenticationError",
    "BadRequestError",
    "CaretlyError",
    "OverloadedError",
    "PayloadTooLargeError",
    "PaymentRequiredError",
    "RateLimitError",
    "UnsupportedEncodingError",
]


class CaretlyError(Exception):
    """Base class for every error raised by this library."""


class APIError(CaretlyError):
    """The API answered with an error status.

    Attributes:
        status_code: The HTTP status code.
        message: The response body: a plain-text explanation from the server.
        retry_after: Seconds to wait before retrying, from ``Retry-After``, or ``None``.
        response: The underlying ``httpx.Response``, or ``None``.
    """

    status_code: int
    message: str
    retry_after: Optional[float]
    response: Optional[httpx.Response]

    def __init__(
        self,
        status_code: int,
        message: str,
        *,
        retry_after: Optional[float] = None,
        response: Optional[httpx.Response] = None,
    ) -> None:
        super().__init__(f"{status_code}: {message}" if message else str(status_code))
        self.status_code = status_code
        self.message = message
        self.retry_after = retry_after
        self.response = response


class BadRequestError(APIError):
    """400: a parse error, an invalid parameter or a page out of range."""


class AuthenticationError(APIError):
    """401: the API key is missing, not valid, expired or revoked."""


class PaymentRequiredError(APIError):
    """402: billing for the account is not active, so live keys are paused."""


class PayloadTooLargeError(APIError):
    """413: the body is larger than 100 MiB after decompression."""


class UnsupportedEncodingError(APIError):
    """415: the server does not support the request's ``Content-Encoding``."""


class RateLimitError(APIError):
    """429: too many requests for this API key. See ``retry_after``."""


class OverloadedError(APIError):
    """503: the server is shedding load. Not billed. See ``retry_after``."""


class APIConnectionError(CaretlyError):
    """The request did not get a response: a network or protocol failure."""


class APITimeoutError(APIConnectionError):
    """The request timed out."""
