"""Result types shared by the sync and async clients."""

from __future__ import annotations

import os
import pathlib
from dataclasses import dataclass, field
from typing import Literal, Mapping, Optional, Sequence, Tuple, Union

__all__ = ["FileContent", "Files", "Format", "RateLimit", "RenderResult"]

Format = Literal["pdf", "png"]
"""Output format: ``"pdf"`` holds every page, ``"png"`` a single page."""

FileContent = Union[str, bytes]
"""A ZPL file's content: text (sent as UTF-8) or raw bytes."""

Files = Union[Mapping[str, FileContent], Sequence[Tuple[str, FileContent]]]
"""Files for :meth:`Client.render_files`: a mapping or a sequence of ``(name, content)``."""


@dataclass(frozen=True)
class RateLimit:
    """The API key's rate limit, from the ``RateLimit-*`` response headers."""

    limit: int
    """Burst size: how many requests the key can make at once."""
    remaining: int
    """Requests the key can still make at once before it is rate limited."""


@dataclass(frozen=True)
class RenderResult:
    """A successful render."""

    content: bytes
    """The rendered file: a PDF or a PNG."""
    content_type: str
    """``application/pdf`` or ``image/png``."""
    page_count: int
    """Total pages across the input (``X-Page-Count``)."""
    file_page_counts: Tuple[int, ...] = ()
    """Multipart requests only: pages per file in upload order; empty otherwise."""
    labels_billed: Optional[int] = None
    """Labels billed for this request (0 for test keys); ``None`` when the server has no keys."""
    rate_limit: Optional[RateLimit] = None
    """The key's rate limit, when the server reports it."""
    server_timing: Optional[str] = field(default=None)
    """The raw ``Server-Timing`` header, when requested with ``timings=True``."""

    def save(self, path: Union[str, "os.PathLike[str]"]) -> pathlib.Path:
        """Write :attr:`content` to ``path`` and return it as a :class:`pathlib.Path`."""
        target = pathlib.Path(path)
        target.write_bytes(self.content)
        return target

    def __repr__(self) -> str:
        return (
            f"RenderResult(content=<{len(self.content)} bytes>, "
            f"content_type={self.content_type!r}, page_count={self.page_count!r}, "
            f"file_page_counts={self.file_page_counts!r}, "
            f"labels_billed={self.labels_billed!r}, rate_limit={self.rate_limit!r}, "
            f"server_timing={self.server_timing!r})"
        )
