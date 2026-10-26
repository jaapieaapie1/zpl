"""Client for the Caretly API: render ZPL labels to PDF or PNG.

>>> import caretly
>>> with caretly.Client() as client:  # reads CARETLY_API_KEY
...     client.render("^XA^FO50,50^A0N,50,50^FDHello^FS^XZ").save("label.pdf")
"""

from ._async_client import AsyncClient
from ._client import Client
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
from ._types import FileContent, Files, Format, RateLimit, RenderResult
from ._version import __version__

__all__ = [
    "APIConnectionError",
    "APIError",
    "APITimeoutError",
    "AsyncClient",
    "AuthenticationError",
    "BadRequestError",
    "CaretlyError",
    "Client",
    "FileContent",
    "Files",
    "Format",
    "OverloadedError",
    "PayloadTooLargeError",
    "PaymentRequiredError",
    "RateLimit",
    "RateLimitError",
    "RenderResult",
    "UnsupportedEncodingError",
    "__version__",
]
