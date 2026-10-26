using System.Net;
using System.Net.Http.Headers;

namespace Caretly.Tests;

/// <summary>A recorded request, with its body read at send time.</summary>
internal sealed record Recorded(HttpMethod Method, Uri Uri, HttpRequestHeaders Headers, HttpContentHeaders? ContentHeaders, byte[] Body);

/// <summary>Answers requests from a queue of responders and records every request.</summary>
internal sealed class FakeHandler : HttpMessageHandler
{
    private readonly Queue<Func<HttpRequestMessage, CancellationToken, Task<HttpResponseMessage>>> _responders = new();

    public List<Recorded> Requests { get; } = new();

    public bool Disposed { get; private set; }

    public FakeHandler Respond(HttpResponseMessage response)
    {
        _responders.Enqueue((_, _) => Task.FromResult(response));
        return this;
    }

    public FakeHandler Respond(HttpStatusCode status, string body = "", Action<HttpResponseMessage>? configure = null)
    {
        var response = new HttpResponseMessage(status) { Content = new StringContent(body) };
        configure?.Invoke(response);
        return Respond(response);
    }

    public FakeHandler Throw(Exception exception)
    {
        _responders.Enqueue((_, _) => Task.FromException<HttpResponseMessage>(exception));
        return this;
    }

    public FakeHandler Hang()
    {
        _responders.Enqueue(async (_, ct) =>
        {
            await Task.Delay(Timeout.Infinite, ct);
            throw new InvalidOperationException("unreachable");
        });
        return this;
    }

    protected override async Task<HttpResponseMessage> SendAsync(HttpRequestMessage request, CancellationToken cancellationToken)
    {
        var body = request.Content == null ? Array.Empty<byte>() : await request.Content.ReadAsByteArrayAsync(cancellationToken);
        Requests.Add(new Recorded(request.Method, request.RequestUri!, request.Headers, request.Content?.Headers, body));
        if (_responders.Count == 0)
        {
            throw new InvalidOperationException("No response queued for " + request.RequestUri);
        }

        return await _responders.Dequeue()(request, cancellationToken);
    }

    protected override void Dispose(bool disposing)
    {
        Disposed = true;
        base.Dispose(disposing);
    }

    public static HttpResponseMessage Pdf(byte[]? bytes = null)
    {
        var content = new ByteArrayContent(bytes ?? "%PDF-1.7"u8.ToArray());
        content.Headers.ContentType = new MediaTypeHeaderValue("application/pdf");
        return new HttpResponseMessage(HttpStatusCode.OK) { Content = content };
    }
}
