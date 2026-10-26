from __future__ import annotations

import asyncio
from dataclasses import dataclass, field
from typing import Any, Callable, Union

import httpx
import pytest

import caretly

Handler = Callable[[httpx.Request], httpx.Response]


@dataclass
class Harness:
    """Drives the sync or the async client the same way, so each test covers both."""

    mode: str
    requests: list[httpx.Request] = field(default_factory=list)
    sleeps: list[float] = field(default_factory=list)

    def client(self, handler: Handler, **kwargs: Any) -> Union[caretly.Client, caretly.AsyncClient]:
        def recording(request: httpx.Request) -> httpx.Response:
            request.read()
            self.requests.append(request)
            return handler(request)

        transport = httpx.MockTransport(recording)
        kwargs.setdefault("api_key", "zpl_test_abc")
        if self.mode == "sync":
            client = caretly.Client(http_client=httpx.Client(transport=transport), **kwargs)
            client._sleep = self.sleeps.append
            return client

        async def sleep(seconds: float) -> None:
            self.sleeps.append(seconds)

        async_client = caretly.AsyncClient(http_client=httpx.AsyncClient(transport=transport), **kwargs)
        async_client._sleep = sleep
        return async_client

    def call(self, client: Union[caretly.Client, caretly.AsyncClient], method: str, *args: Any, **kwargs: Any) -> Any:
        result = getattr(client, method)(*args, **kwargs)
        if self.mode == "async":
            return asyncio.run(result)
        return result


@pytest.fixture(params=["sync", "async"])
def harness(request: pytest.FixtureRequest) -> Harness:
    return Harness(request.param)


@pytest.fixture(autouse=True)
def no_env_key(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv("CARETLY_API_KEY", raising=False)


@pytest.fixture
def anyio_backend() -> str:
    return "asyncio"
