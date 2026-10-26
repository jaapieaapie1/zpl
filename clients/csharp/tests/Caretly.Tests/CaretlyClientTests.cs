using System.IO.Compression;
using System.Net;
using System.Net.Http.Headers;
using System.Text;
using Xunit;

namespace Caretly.Tests;

public class CaretlyClientTests
{
    private const string Zpl = "^XA^FO50,50^A0N,50,50^FDHello^FS^XZ";

    private static (CaretlyClient Client, FakeHandler Handler, List<TimeSpan> Delays) Create(
        CaretlyClientOptions? options = null)
    {
        var handler = new FakeHandler();
        var client = new CaretlyClient(new HttpClient(handler), options ?? new CaretlyClientOptions { ApiKey = "zpl_test_abc", BaseUrl = "http://caretly.test" });
        var delays = new List<TimeSpan>();
        client.Delay = (delay, _) =>
        {
            delays.Add(delay);
            return Task.CompletedTask;
        };
        return (client, handler, delays);
    }

    [Fact]
    public async Task RenderSendsRawZplWithHeaders()
    {
        var (client, handler, _) = Create();
        handler.Respond(FakeHandler.Pdf());

        await client.RenderAsync(Zpl);

        var request = Assert.Single(handler.Requests);
        Assert.Equal(HttpMethod.Post, request.Method);
        Assert.Equal("http://caretly.test/render", request.Uri.ToString());
        Assert.Equal("Bearer zpl_test_abc", request.Headers.Authorization!.ToString());
        Assert.Equal("caretly-csharp/0.1.0", string.Join(" ", request.Headers.GetValues("User-Agent")));
        Assert.Equal("text/plain; charset=utf-8", request.ContentHeaders!.ContentType!.ToString());
        Assert.Equal(Zpl, Encoding.UTF8.GetString(request.Body));
    }

    [Fact]
    public async Task RenderParsesSuccessHeaders()
    {
        var (client, handler, _) = Create();
        var response = FakeHandler.Pdf("%PDF-test"u8.ToArray());
        response.Headers.Add("X-Page-Count", "3");
        response.Headers.Add("X-Labels-Billed", "3");
        response.Headers.Add("RateLimit-Limit", "200");
        response.Headers.Add("RateLimit-Remaining", "199");
        response.Headers.Add("Server-Timing", "read;dur=0.1, parse;dur=0.4");
        handler.Respond(response);

        var result = await client.RenderAsync(Zpl, new RenderOptions { Timings = true });

        Assert.Equal("%PDF-test"u8.ToArray(), result.Content);
        Assert.Equal("application/pdf", result.ContentType);
        Assert.Equal(3, result.PageCount);
        Assert.Empty(result.FilePageCounts);
        Assert.Equal(3, result.LabelsBilled);
        Assert.NotNull(result.RateLimit);
        Assert.Equal(200, result.RateLimit!.Limit);
        Assert.Equal(199, result.RateLimit.Remaining);
        Assert.Equal("read;dur=0.1, parse;dur=0.4", result.ServerTiming);
    }

    [Fact]
    public async Task OptionalHeadersAreNullWhenAbsent()
    {
        var (client, handler, _) = Create();
        handler.Respond(FakeHandler.Pdf());

        var result = await client.RenderAsync(Zpl);

        Assert.Null(result.LabelsBilled);
        Assert.Null(result.RateLimit);
        Assert.Null(result.ServerTiming);
    }

    [Fact]
    public async Task QueryParametersAreSentOnlyWhenSet()
    {
        var (client, handler, _) = Create();
        handler.Respond(FakeHandler.Pdf());

        await client.RenderAsync(Zpl, new RenderOptions { Format = RenderFormat.Png, Page = 2, Dpmm = 12, Timings = true });

        Assert.Equal("http://caretly.test/render?format=png&page=2&dpmm=12&timings=1", handler.Requests[0].Uri.ToString());
    }

    [Fact]
    public async Task ConvenienceMethodsSetFormat()
    {
        var (client, handler, _) = Create();
        handler.Respond(FakeHandler.Pdf()).Respond(FakeHandler.Pdf());

        await client.RenderPdfAsync(Zpl);
        await client.RenderPngAsync(Zpl, 3);

        Assert.Equal("?format=pdf", handler.Requests[0].Uri.Query);
        Assert.Equal("?format=png&page=3", handler.Requests[1].Uri.Query);
    }

    [Theory]
    [InlineData(0)]
    [InlineData(-1)]
    public async Task InvalidPageFailsFast(int page)
    {
        var (client, handler, _) = Create();
        await Assert.ThrowsAsync<ArgumentOutOfRangeException>(() => client.RenderAsync(Zpl, new RenderOptions { Page = page }));
        Assert.Empty(handler.Requests);
    }

    [Theory]
    [InlineData(0)]
    [InlineData(7)]
    [InlineData(300)]
    public async Task InvalidDpmmFailsFast(int dpmm)
    {
        var (client, handler, _) = Create();
        await Assert.ThrowsAsync<ArgumentOutOfRangeException>(() => client.RenderAsync(Zpl, new RenderOptions { Dpmm = dpmm }));
        Assert.Empty(handler.Requests);
    }

    [Fact]
    public async Task TooLongIdempotencyKeyFailsFast()
    {
        var (client, handler, _) = Create();
        await Assert.ThrowsAsync<ArgumentException>(() => client.RenderAsync(Zpl, new RenderOptions { IdempotencyKey = new string('a', 256) }));
        Assert.Empty(handler.Requests);
    }

    [Theory]
    [InlineData(HttpStatusCode.BadRequest, typeof(BadRequestException))]
    [InlineData(HttpStatusCode.Unauthorized, typeof(AuthenticationException))]
    [InlineData(HttpStatusCode.PaymentRequired, typeof(PaymentRequiredException))]
    [InlineData(HttpStatusCode.RequestEntityTooLarge, typeof(PayloadTooLargeException))]
    [InlineData(HttpStatusCode.UnsupportedMediaType, typeof(UnsupportedEncodingException))]
    [InlineData(HttpStatusCode.TooManyRequests, typeof(RateLimitedException))]
    [InlineData(HttpStatusCode.ServiceUnavailable, typeof(OverloadedException))]
    [InlineData(HttpStatusCode.InternalServerError, typeof(CaretlyApiException))]
    [InlineData(HttpStatusCode.NotFound, typeof(CaretlyApiException))]
    public async Task ErrorStatusesMapToTypedExceptions(HttpStatusCode status, Type expected)
    {
        var (client, handler, _) = Create(new CaretlyClientOptions { BaseUrl = "http://caretly.test", MaxRetries = 0 });
        handler.Respond(status, "something went wrong\n");

        var error = await Assert.ThrowsAnyAsync<CaretlyApiException>(() => client.RenderAsync(Zpl));

        Assert.Equal(expected, error.GetType());
        Assert.Equal((int)status, error.StatusCode);
        Assert.Equal("something went wrong", error.ResponseBody);
        Assert.Contains("something went wrong", error.Message);
        Assert.IsAssignableFrom<CaretlyException>(error);
    }

    [Fact]
    public async Task NonRetryableErrorsAreNotRetried()
    {
        var (client, handler, delays) = Create();
        handler.Respond(HttpStatusCode.InternalServerError, "boom");

        await Assert.ThrowsAsync<CaretlyApiException>(() => client.RenderAsync(Zpl));

        Assert.Single(handler.Requests);
        Assert.Empty(delays);
    }

    [Fact]
    public async Task RetriesRateLimitHonoringRetryAfterWithOneIdempotencyKey()
    {
        var (client, handler, delays) = Create();
        handler
            .Respond(HttpStatusCode.TooManyRequests, "slow down", r => r.Headers.RetryAfter = new RetryConditionHeaderValue(TimeSpan.FromSeconds(3)))
            .Respond(FakeHandler.Pdf());

        var result = await client.RenderAsync(Zpl);

        Assert.Equal(2, handler.Requests.Count);
        Assert.Equal(new[] { TimeSpan.FromSeconds(3) }, delays);
        var first = handler.Requests[0].Headers.GetValues("Idempotency-Key").Single();
        var second = handler.Requests[1].Headers.GetValues("Idempotency-Key").Single();
        Assert.Equal(first, second);
        Assert.True(Guid.TryParse(first, out _));
        Assert.Equal(Zpl, Encoding.UTF8.GetString(handler.Requests[1].Body));
        Assert.Equal("%PDF-1.7"u8.ToArray(), result.Content);
    }

    [Fact]
    public async Task UserIdempotencyKeyIsKeptAcrossRetries()
    {
        var (client, handler, _) = Create();
        handler.Respond(HttpStatusCode.ServiceUnavailable, "busy").Respond(FakeHandler.Pdf());

        await client.RenderAsync(Zpl, new RenderOptions { IdempotencyKey = "order-1" });

        Assert.All(handler.Requests, r => Assert.Equal("order-1", r.Headers.GetValues("Idempotency-Key").Single()));
    }

    [Fact]
    public async Task NoIdempotencyKeyWhenRetriesAreDisabled()
    {
        var (client, handler, _) = Create(new CaretlyClientOptions { BaseUrl = "http://caretly.test", MaxRetries = 0 });
        handler.Respond(FakeHandler.Pdf());

        await client.RenderAsync(Zpl);

        Assert.False(handler.Requests[0].Headers.Contains("Idempotency-Key"));
    }

    [Fact]
    public async Task RetriesUseExponentialBackoffAndGiveUp()
    {
        var (client, handler, delays) = Create(new CaretlyClientOptions { BaseUrl = "http://caretly.test", MaxRetries = 3 });
        for (var i = 0; i < 4; i++)
        {
            handler.Respond(HttpStatusCode.ServiceUnavailable, "busy");
        }

        var error = await Assert.ThrowsAsync<OverloadedException>(() => client.RenderAsync(Zpl));

        Assert.Equal(4, handler.Requests.Count);
        Assert.Equal(new[] { 0.5, 1.0, 2.0 }, delays.Select(d => d.TotalSeconds));
        Assert.Equal(503, error.StatusCode);
    }

    [Fact]
    public void BackoffIsCappedAtEightSeconds()
    {
        Assert.Equal(TimeSpan.FromSeconds(8), CaretlyClient.Backoff(4));
        Assert.Equal(TimeSpan.FromSeconds(8), CaretlyClient.Backoff(50));
    }

    [Fact]
    public async Task RetryAfterIsExposedOnTheException()
    {
        var (client, handler, _) = Create(new CaretlyClientOptions { BaseUrl = "http://caretly.test", MaxRetries = 0 });
        handler.Respond(HttpStatusCode.TooManyRequests, "slow down", r => r.Headers.Add("Retry-After", "7"));

        var error = await Assert.ThrowsAsync<RateLimitedException>(() => client.RenderAsync(Zpl));

        Assert.Equal(TimeSpan.FromSeconds(7), error.RetryAfter);
    }

    [Fact]
    public async Task ConnectionErrorsAreRetriedThenWrapped()
    {
        var (client, handler, delays) = Create();
        handler.Throw(new HttpRequestException("refused")).Respond(FakeHandler.Pdf());

        await client.RenderAsync(Zpl);
        Assert.Equal(2, handler.Requests.Count);
        Assert.Single(delays);

        handler.Throw(new HttpRequestException("refused")).Throw(new HttpRequestException("refused")).Throw(new HttpRequestException("refused"));
        var error = await Assert.ThrowsAsync<CaretlyConnectionException>(() => client.RenderAsync(Zpl));
        Assert.IsType<HttpRequestException>(error.InnerException);
    }

    [Fact]
    public async Task TimeoutThrowsCaretlyTimeoutException()
    {
        var (client, handler, _) = Create(new CaretlyClientOptions { BaseUrl = "http://caretly.test", Timeout = TimeSpan.FromMilliseconds(50) });
        handler.Hang();

        await Assert.ThrowsAsync<CaretlyTimeoutException>(() => client.RenderAsync(Zpl));
    }

    [Fact]
    public async Task CallerCancellationIsNotATimeout()
    {
        var (client, handler, _) = Create();
        handler.Hang();
        using var cts = new CancellationTokenSource(TimeSpan.FromMilliseconds(50));

        await Assert.ThrowsAnyAsync<OperationCanceledException>(() => client.RenderAsync(Zpl, cancellationToken: cts.Token));
    }

    [Fact]
    public async Task RenderFilesSendsMultipart()
    {
        var (client, handler, _) = Create();
        var response = FakeHandler.Pdf();
        response.Headers.Add("X-Page-Count", "3");
        response.Headers.Add("X-File-Page-Counts", "2, 1");
        handler.Respond(response);

        var result = await client.RenderFilesAsync(new[]
        {
            new ZplFile("a.zpl", "^XA^FDa^XZ^XA^FDb^XZ"),
            new ZplFile("b.zpl", "^XA^FDc^XZ"u8.ToArray()),
        });

        Assert.Equal(new[] { 2, 1 }, result.FilePageCounts);
        Assert.Equal(3, result.PageCount);

        var request = handler.Requests[0];
        var contentType = request.ContentHeaders!.ContentType!;
        Assert.Equal("multipart/form-data", contentType.MediaType);
        var boundary = contentType.Parameters.Single(p => p.Name == "boundary").Value!;
        var body = Encoding.UTF8.GetString(request.Body);
        Assert.Equal(
            $"--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"a.zpl\"\r\nContent-Type: application/octet-stream\r\n\r\n^XA^FDa^XZ^XA^FDb^XZ\r\n"
            + $"--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"b.zpl\"\r\nContent-Type: application/octet-stream\r\n\r\n^XA^FDc^XZ\r\n"
            + $"--{boundary}--\r\n",
            body);

        // The body parses as a standard multipart form.
        var parsed = new MultipartFormDataContentReader(request.Body, boundary).Parts;
        Assert.Equal(new[] { "a.zpl", "b.zpl" }, parsed.Select(p => p.FileName));
    }

    [Fact]
    public async Task RenderFilesRequiresAFile()
    {
        var (client, _, _) = Create();
        await Assert.ThrowsAsync<ArgumentException>(() => client.RenderFilesAsync(Array.Empty<ZplFile>()));
    }

    [Fact]
    public async Task CompressGzipsTheBody()
    {
        var (client, handler, _) = Create();
        handler.Respond(FakeHandler.Pdf());

        await client.RenderAsync(Zpl, new RenderOptions { Compress = true });

        var request = handler.Requests[0];
        Assert.Equal(new[] { "gzip" }, request.ContentHeaders!.ContentEncoding);
        Assert.Equal("text/plain; charset=utf-8", request.ContentHeaders.ContentType!.ToString());
        using var gzip = new GZipStream(new MemoryStream(request.Body), CompressionMode.Decompress);
        using var reader = new StreamReader(gzip, Encoding.UTF8);
        Assert.Equal(Zpl, reader.ReadToEnd());
    }

    [Fact]
    public async Task CompressGzipsMultipartBodies()
    {
        var (client, handler, _) = Create();
        handler.Respond(FakeHandler.Pdf());

        await client.RenderFilesAsync(new[] { new ZplFile("a.zpl", Zpl) }, new RenderOptions { Compress = true });

        var request = handler.Requests[0];
        Assert.Equal(new[] { "gzip" }, request.ContentHeaders!.ContentEncoding);
        Assert.Equal("multipart/form-data", request.ContentHeaders.ContentType!.MediaType);
        using var gzip = new GZipStream(new MemoryStream(request.Body), CompressionMode.Decompress);
        using var reader = new StreamReader(gzip, Encoding.UTF8);
        Assert.Contains(Zpl, reader.ReadToEnd());
    }

    [Fact]
    public async Task NoAuthorizationWithoutAKey()
    {
        var previous = Environment.GetEnvironmentVariable("CARETLY_API_KEY");
        Environment.SetEnvironmentVariable("CARETLY_API_KEY", null);
        try
        {
            var (client, handler, _) = Create(new CaretlyClientOptions { BaseUrl = "http://caretly.test" });
            handler.Respond(FakeHandler.Pdf());

            await client.RenderAsync(Zpl);

            Assert.Null(handler.Requests[0].Headers.Authorization);
        }
        finally
        {
            Environment.SetEnvironmentVariable("CARETLY_API_KEY", previous);
        }
    }

    [Fact]
    public async Task ApiKeyFallsBackToEnvironment()
    {
        var previous = Environment.GetEnvironmentVariable("CARETLY_API_KEY");
        Environment.SetEnvironmentVariable("CARETLY_API_KEY", "zpl_live_env");
        try
        {
            var (client, handler, _) = Create(new CaretlyClientOptions { BaseUrl = "http://caretly.test" });
            handler.Respond(FakeHandler.Pdf());

            await client.RenderAsync(Zpl);

            Assert.Equal("Bearer zpl_live_env", handler.Requests[0].Headers.Authorization!.ToString());
        }
        finally
        {
            Environment.SetEnvironmentVariable("CARETLY_API_KEY", previous);
        }
    }

    [Fact]
    public void BaseUrlDefaultsAndUsesInjectedBaseAddress()
    {
        using var defaults = new CaretlyClient(new HttpClient(new FakeHandler()));
        Assert.Equal("https://caretly.dev", defaults.BaseUrl);

        using var injected = new CaretlyClient(new HttpClient(new FakeHandler()) { BaseAddress = new Uri("http://self.hosted:8080/") });
        Assert.Equal("http://self.hosted:8080", injected.BaseUrl);

        using var explicitUrl = new CaretlyClient(new CaretlyClientOptions { BaseUrl = "http://localhost:3000/" });
        Assert.Equal("http://localhost:3000", explicitUrl.BaseUrl);

        Assert.Throws<ArgumentException>(() => new CaretlyClient(new CaretlyClientOptions { BaseUrl = "not a url" }));
    }

    [Fact]
    public async Task HealthChecksTheServer()
    {
        var (client, handler, _) = Create();
        handler.Respond(HttpStatusCode.OK, "ok").Respond(HttpStatusCode.InternalServerError, "down");

        Assert.True(await client.HealthAsync());
        Assert.False(await client.HealthAsync());
        Assert.Equal(HttpMethod.Get, handler.Requests[0].Method);
        Assert.Equal("http://caretly.test/health", handler.Requests[0].Uri.ToString());
    }

    [Fact]
    public async Task InjectedHttpClientIsNotDisposed()
    {
        var handler = new FakeHandler();
        var http = new HttpClient(handler);
        var client = new CaretlyClient(http, new CaretlyClientOptions { BaseUrl = "http://caretly.test" });
        client.Dispose();

        Assert.False(handler.Disposed);
        handler.Respond(HttpStatusCode.OK, "ok");
        Assert.Equal(HttpStatusCode.OK, (await http.GetAsync("http://caretly.test/health")).StatusCode);
        await Assert.ThrowsAsync<ObjectDisposedException>(() => client.RenderAsync(Zpl));
    }

    [Fact]
    public async Task SaveToAsyncWritesTheFile()
    {
        var result = new RenderResult("%PDF"u8.ToArray(), "application/pdf", 1, Array.Empty<int>(), null, null, null);
        var path = Path.Combine(Path.GetTempPath(), $"caretly-{Guid.NewGuid():N}.pdf");
        try
        {
            await result.SaveToAsync(path);
            Assert.Equal("%PDF"u8.ToArray(), await File.ReadAllBytesAsync(path));
        }
        finally
        {
            File.Delete(path);
        }
    }
}
