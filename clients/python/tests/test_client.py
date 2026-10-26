from __future__ import annotations

import gzip
import re
import uuid
from email.parser import BytesParser
from email.policy import HTTP
from pathlib import Path

import httpx
import pytest

import caretly
from conftest import Harness

PDF = b"%PDF-1.7 fake"
LABEL = "^XA^FO50,50^A0N,50,50^FDHello^FS^XZ"


def ok(content: bytes = PDF, **headers: str) -> httpx.Response:
    base = {"content-type": "application/pdf", "x-page-count": "1"}
    base.update({k.replace("_", "-"): v for k, v in headers.items()})
    return httpx.Response(200, content=content, headers=base)


def test_success_headers_parsed(harness: Harness) -> None:
    response = ok(
        x_page_count="3",
        x_labels_billed="3",
        ratelimit_limit="200",
        ratelimit_remaining="199",
        server_timing="read;dur=0.1, parse;dur=0.2",
    )
    client = harness.client(lambda r: response)
    result = harness.call(client, "render", LABEL, timings=True)
    assert result == caretly.RenderResult(
        content=PDF,
        content_type="application/pdf",
        page_count=3,
        file_page_counts=(),
        labels_billed=3,
        rate_limit=caretly.RateLimit(limit=200, remaining=199),
        server_timing="read;dur=0.1, parse;dur=0.2",
    )


def test_optional_headers_absent(harness: Harness) -> None:
    client = harness.client(lambda r: ok())
    result = harness.call(client, "render", LABEL)
    assert result.labels_billed is None
    assert result.rate_limit is None
    assert result.server_timing is None
    assert result.file_page_counts == ()


def test_request_shape_and_auth_header(harness: Harness) -> None:
    client = harness.client(lambda r: ok(), base_url="https://example.test/", api_key="zpl_live_xyz")
    harness.call(client, "render", LABEL)
    (request,) = harness.requests
    assert request.method == "POST"
    assert request.url.copy_with(query=None) == "https://example.test/render"
    assert dict(request.url.params) == {"format": "pdf"}
    assert request.headers["authorization"] == "Bearer zpl_live_xyz"
    assert request.headers["content-type"] == "text/plain; charset=utf-8"
    assert request.headers["user-agent"] == f"caretly-python/{caretly.__version__}"
    assert "content-encoding" not in request.headers
    assert request.content == LABEL.encode()


def test_query_parameters(harness: Harness) -> None:
    client = harness.client(lambda r: ok())
    harness.call(client, "render", LABEL, format="png", page=2, dpmm=12, timings=True)
    assert dict(harness.requests[0].url.params) == {"format": "png", "page": "2", "dpmm": "12", "timings": "1"}


def test_api_key_from_environment(harness: Harness, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("CARETLY_API_KEY", "zpl_test_env")
    client = harness.client(lambda r: ok(), api_key=None)
    harness.call(client, "render", LABEL)
    assert harness.requests[0].headers["authorization"] == "Bearer zpl_test_env"


def test_no_api_key_sends_no_authorization(harness: Harness) -> None:
    client = harness.client(lambda r: ok(), api_key=None)
    harness.call(client, "render", LABEL)
    assert "authorization" not in harness.requests[0].headers


def test_gzip_option(harness: Harness) -> None:
    client = harness.client(lambda r: ok())
    harness.call(client, "render", LABEL, compress=True)
    request = harness.requests[0]
    assert request.headers["content-encoding"] == "gzip"
    assert gzip.decompress(request.content) == LABEL.encode()


def _parts(request: httpx.Request) -> list[tuple[str, str | None, bytes]]:
    head = f"Content-Type: {request.headers['content-type']}\r\n\r\n".encode()
    message = BytesParser(policy=HTTP).parsebytes(head + request.content)
    assert message.is_multipart()
    return [
        (part.get_param("name", header="content-disposition"), part.get_filename(), part.get_payload(decode=True))
        for part in message.iter_parts()
    ]


def test_multipart_shape(harness: Harness) -> None:
    response = ok(x_page_count="3", x_file_page_counts="2, 1")
    client = harness.client(lambda r: response)
    result = harness.call(client, "render_files", [("a.zpl", LABEL + LABEL), ("b.zpl", LABEL.encode())])
    request = harness.requests[0]
    assert re.fullmatch(r"multipart/form-data; boundary=\S+", request.headers["content-type"])
    assert _parts(request) == [
        ("file", "a.zpl", (LABEL + LABEL).encode()),
        ("file", "b.zpl", LABEL.encode()),
    ]
    assert result.page_count == 3
    assert result.file_page_counts == (2, 1)


def test_multipart_from_mapping_with_gzip(harness: Harness) -> None:
    client = harness.client(lambda r: ok())
    harness.call(client, "render_files", {"one.zpl": LABEL, 'we"ird.zpl': LABEL}, compress=True)
    request = harness.requests[0]
    assert request.headers["content-encoding"] == "gzip"
    raw = httpx.Request("POST", "http://x", headers=request.headers, content=gzip.decompress(request.content))
    assert [(name, filename) for name, filename, _ in _parts(raw)] == [
        ("file", "one.zpl"),
        ("file", "we%22ird.zpl"),
    ]


@pytest.mark.parametrize(
    ("status", "error"),
    [
        (400, caretly.BadRequestError),
        (401, caretly.AuthenticationError),
        (402, caretly.PaymentRequiredError),
        (413, caretly.PayloadTooLargeError),
        (415, caretly.UnsupportedEncodingError),
        (429, caretly.RateLimitError),
        (503, caretly.OverloadedError),
        (500, caretly.APIError),
        (404, caretly.APIError),
    ],
)
def test_error_status_mapped(harness: Harness, status: int, error: type[caretly.APIError]) -> None:
    client = harness.client(
        lambda r: httpx.Response(status, text=f"problem {status}\n", headers={"retry-after": "7"}),
        max_retries=0,
    )
    with pytest.raises(error) as info:
        harness.call(client, "render", LABEL)
    assert type(info.value) is error
    assert isinstance(info.value, caretly.CaretlyError)
    assert info.value.status_code == status
    assert info.value.message == f"problem {status}"
    assert info.value.retry_after == 7
    assert len(harness.requests) == 1


def test_error_without_body_uses_reason(harness: Harness) -> None:
    client = harness.client(lambda r: httpx.Response(500), max_retries=0)
    with pytest.raises(caretly.APIError) as info:
        harness.call(client, "render", LABEL)
    assert info.value.message == "Internal Server Error"
    assert info.value.retry_after is None


def test_retry_on_429_honours_retry_after(harness: Harness) -> None:
    responses = iter([httpx.Response(429, text="slow down", headers={"retry-after": "3"}), ok()])
    client = harness.client(lambda r: next(responses))
    result = harness.call(client, "render", LABEL)
    assert result.content == PDF
    assert harness.sleeps == [3]
    assert len(harness.requests) == 2


def test_retry_503_then_give_up(harness: Harness) -> None:
    client = harness.client(lambda r: httpx.Response(503, text="busy", headers={"retry-after": "1"}))
    with pytest.raises(caretly.OverloadedError):
        harness.call(client, "render", LABEL)
    assert len(harness.requests) == 3
    assert harness.sleeps == [1, 1]


def test_exponential_backoff_without_retry_after(harness: Harness) -> None:
    client = harness.client(lambda r: httpx.Response(429, text="slow down"), max_retries=6)
    with pytest.raises(caretly.RateLimitError):
        harness.call(client, "render", LABEL)
    assert harness.sleeps == [0.5, 1, 2, 4, 8, 8]


def test_retries_reuse_one_generated_idempotency_key(harness: Harness) -> None:
    responses = iter([httpx.Response(503, headers={"retry-after": "0"}), httpx.Response(429)])
    client = harness.client(lambda r: next(responses, None) or ok())
    harness.call(client, "render", LABEL)
    keys = {r.headers["idempotency-key"] for r in harness.requests}
    assert len(harness.requests) == 3
    assert len(keys) == 1
    assert uuid.UUID(keys.pop()).version == 4
    # A second logical call gets a new key.
    harness.call(client, "render", LABEL)
    assert harness.requests[3].headers["idempotency-key"] != harness.requests[0].headers["idempotency-key"]


def test_user_idempotency_key_is_sent(harness: Harness) -> None:
    client = harness.client(lambda r: ok())
    harness.call(client, "render", LABEL, idempotency_key="order-1")
    assert harness.requests[0].headers["idempotency-key"] == "order-1"


def test_no_generated_key_without_retries(harness: Harness) -> None:
    client = harness.client(lambda r: ok(), max_retries=0)
    harness.call(client, "render", LABEL)
    assert "idempotency-key" not in harness.requests[0].headers


@pytest.mark.parametrize("status", [400, 401, 402, 413, 415, 500, 502])
def test_other_statuses_not_retried(harness: Harness, status: int) -> None:
    client = harness.client(lambda r: httpx.Response(status, text="no"))
    with pytest.raises(caretly.APIError):
        harness.call(client, "render", LABEL)
    assert len(harness.requests) == 1
    assert harness.sleeps == []


def test_connection_error_retried_then_raised(harness: Harness) -> None:
    def fail(request: httpx.Request) -> httpx.Response:
        raise httpx.ConnectError("refused", request=request)

    client = harness.client(fail)
    with pytest.raises(caretly.APIConnectionError) as info:
        harness.call(client, "render", LABEL)
    assert not isinstance(info.value, caretly.APITimeoutError)
    assert isinstance(info.value.__cause__, httpx.ConnectError)
    assert len(harness.requests) == 3
    assert harness.sleeps == [0.5, 1]


def test_timeout_not_retried(harness: Harness) -> None:
    def slow(request: httpx.Request) -> httpx.Response:
        raise httpx.ReadTimeout("slow", request=request)

    client = harness.client(slow)
    with pytest.raises(caretly.APITimeoutError) as info:
        harness.call(client, "render", LABEL)
    assert isinstance(info.value.__cause__, httpx.ReadTimeout)
    assert len(harness.requests) == 1
    assert harness.sleeps == []


def test_remote_protocol_error_retried(harness: Harness) -> None:
    attempts = iter([True, False])

    def handler(request: httpx.Request) -> httpx.Response:
        if next(attempts):
            raise httpx.RemoteProtocolError("dropped", request=request)
        return ok()

    client = harness.client(handler)
    assert harness.call(client, "render", LABEL).content == PDF
    assert len(harness.requests) == 2
    assert harness.sleeps == [0.5]


@pytest.mark.parametrize(
    "options",
    [
        {"page": 0},
        {"page": -1},
        {"page": True},
        {"dpmm": 7},
        {"dpmm": 300},
        {"format": "gif"},
        {"idempotency_key": "x" * 256},
        {"idempotency_key": ""},
        {"idempotency_key": "a\nb"},
    ],
)
def test_options_validated_before_sending(harness: Harness, options: dict[str, object]) -> None:
    client = harness.client(lambda r: ok())
    with pytest.raises(ValueError, match=r"^(page|dpmm|format|idempotency_key) "):
        harness.call(client, "render", LABEL, **options)
    assert harness.requests == []


@pytest.mark.parametrize("files", [[], {}, "a.zpl"])
def test_render_files_rejects_bad_input(harness: Harness, files: object) -> None:
    client = harness.client(lambda r: ok())
    with pytest.raises((ValueError, TypeError)):
        harness.call(client, "render_files", files)
    assert harness.requests == []


def test_render_png_sugar(harness: Harness) -> None:
    client = harness.client(lambda r: ok(b"\x89PNG", content_type="image/png"))
    result = harness.call(client, "render_png", LABEL, 2, dpmm=24)
    assert dict(harness.requests[0].url.params) == {"format": "png", "page": "2", "dpmm": "24"}
    assert result.content_type == "image/png"
    harness.call(client, "render_pdf", LABEL)
    assert dict(harness.requests[1].url.params) == {"format": "pdf"}


def test_health(harness: Harness) -> None:
    client = harness.client(lambda r: httpx.Response(200, text="ok"))
    assert harness.call(client, "health") is True
    assert harness.requests[0].method == "GET"
    assert harness.requests[0].url == "https://caretly.dev/health"

    client = harness.client(lambda r: httpx.Response(503, text="down"))
    assert harness.call(client, "health") is False


def test_health_connection_error(harness: Harness) -> None:
    def fail(request: httpx.Request) -> httpx.Response:
        raise httpx.ConnectError("refused", request=request)

    with pytest.raises(caretly.APIConnectionError):
        harness.call(harness.client(fail), "health")


def test_save(tmp_path: Path) -> None:
    result = caretly.RenderResult(content=PDF, content_type="application/pdf", page_count=1)
    path = result.save(tmp_path / "label.pdf")
    assert path.read_bytes() == PDF
    assert "13 bytes" in repr(result)


def test_negative_max_retries_rejected() -> None:
    with pytest.raises(ValueError, match="max_retries"):
        caretly.Client(max_retries=-1)


def test_sync_context_manager_closes_own_client_only() -> None:
    with caretly.Client() as client:
        inner = client._http
    assert inner.is_closed

    injected = httpx.Client()
    with caretly.Client(http_client=injected):
        pass
    assert not injected.is_closed
    injected.close()


@pytest.mark.anyio
async def test_async_context_manager_closes_own_client_only() -> None:
    async with caretly.AsyncClient() as client:
        inner = client._http
    assert inner.is_closed

    injected = httpx.AsyncClient()
    async with caretly.AsyncClient(http_client=injected):
        pass
    assert not injected.is_closed
    await injected.aclose()


@pytest.mark.anyio
async def test_async_render_in_running_loop() -> None:
    transport = httpx.MockTransport(lambda r: ok(x_page_count="2"))
    async with caretly.AsyncClient(api_key="k", http_client=httpx.AsyncClient(transport=transport)) as client:
        result = await client.render(LABEL)
    assert result.page_count == 2


def test_version() -> None:
    assert caretly.__version__ == "0.1.0"
