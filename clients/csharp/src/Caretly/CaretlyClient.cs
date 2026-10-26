using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Caretly
{
    /// <summary>
    /// Client for the Caretly API: renders ZPL labels to PDF or PNG. Thread-safe; create one and
    /// reuse it.
    /// </summary>
    public sealed class CaretlyClient : IDisposable
    {
        /// <summary>The version of this library, sent in the <c>User-Agent</c> header.</summary>
        public const string Version = "0.1.0";

        private const string UserAgent = "caretly-csharp/" + Version;
        private const int MaxIdempotencyKeyLength = 255;
        private static readonly int[] DpmmValues = { 6, 8, 12, 24 };
        private static readonly TimeSpan MaxBackoff = TimeSpan.FromSeconds(8);

        private readonly HttpClient _http;
        private readonly bool _ownsHttpClient;
        private readonly string? _apiKey;
        private readonly string _baseUrl;
        private readonly TimeSpan _timeout;
        private readonly int _maxRetries;
        private bool _disposed;

        /// <summary>
        /// Creates a client for <c>https://caretly.dev</c> using the <c>CARETLY_API_KEY</c>
        /// environment variable, if set.
        /// </summary>
        public CaretlyClient()
            : this(new CaretlyClientOptions())
        {
        }

        /// <summary>Creates a client for <c>https://caretly.dev</c> with the given API key.</summary>
        /// <param name="apiKey">The API key (<c>zpl_live_…</c> or <c>zpl_test_…</c>).</param>
        public CaretlyClient(string apiKey)
            : this(new CaretlyClientOptions { ApiKey = apiKey })
        {
        }

        /// <summary>Creates a client with its own <see cref="HttpClient"/>.</summary>
        /// <param name="options">The client settings.</param>
        public CaretlyClient(CaretlyClientOptions options)
            : this(new HttpClient { Timeout = System.Threading.Timeout.InfiniteTimeSpan }, true, options)
        {
        }

        /// <summary>
        /// Creates a client that sends its requests through <paramref name="httpClient"/>, for
        /// example one from <c>IHttpClientFactory</c>. The client does not dispose it.
        /// </summary>
        /// <param name="httpClient">The HTTP client to use. Its own <c>Timeout</c> still applies.</param>
        /// <param name="options">The client settings, or <see langword="null"/> for the defaults.</param>
        public CaretlyClient(HttpClient httpClient, CaretlyClientOptions? options = null)
            : this(httpClient ?? throw new ArgumentNullException(nameof(httpClient)), false, options ?? new CaretlyClientOptions())
        {
        }

        private CaretlyClient(HttpClient httpClient, bool ownsHttpClient, CaretlyClientOptions options)
        {
            if (options == null)
            {
                if (ownsHttpClient)
                {
                    httpClient.Dispose();
                }

                throw new ArgumentNullException(nameof(options));
            }

            if (options.MaxRetries < 0)
            {
                throw new ArgumentOutOfRangeException(nameof(options), "MaxRetries must be 0 or more.");
            }

            if (options.Timeout <= TimeSpan.Zero && options.Timeout != System.Threading.Timeout.InfiniteTimeSpan)
            {
                throw new ArgumentOutOfRangeException(nameof(options), "Timeout must be positive or Timeout.InfiniteTimeSpan.");
            }

            _http = httpClient;
            _ownsHttpClient = ownsHttpClient;
            _timeout = options.Timeout;
            _maxRetries = options.MaxRetries;

            var apiKey = options.ApiKey ?? Environment.GetEnvironmentVariable(CaretlyClientOptions.ApiKeyEnvironmentVariable);
            _apiKey = string.IsNullOrWhiteSpace(apiKey) ? null : apiKey!.Trim();

            var baseUrl = options.BaseUrl ?? httpClient.BaseAddress?.ToString() ?? CaretlyClientOptions.DefaultBaseUrl;
            if (!Uri.TryCreate(baseUrl, UriKind.Absolute, out var parsed) || (parsed.Scheme != Uri.UriSchemeHttp && parsed.Scheme != Uri.UriSchemeHttps))
            {
                throw new ArgumentException($"BaseUrl must be an absolute http or https URL, got \"{baseUrl}\".", nameof(options));
            }

            _baseUrl = baseUrl.TrimEnd('/');
        }

        /// <summary>The base URL requests are sent to, without a trailing slash.</summary>
        public string BaseUrl => _baseUrl;

        /// <summary>Waits between retries. Replaced in tests so they do not sleep.</summary>
        internal Func<TimeSpan, CancellationToken, Task> Delay { get; set; } = (delay, ct) => Task.Delay(delay, ct);

        /// <summary>Renders one ZPL document, which may hold several labels.</summary>
        /// <param name="zpl">The ZPL to render.</param>
        /// <param name="options">Output format, page, resolution and request options.</param>
        /// <param name="cancellationToken">Cancels the request, including retries.</param>
        /// <returns>The rendered file and its metadata.</returns>
        /// <exception cref="CaretlyApiException">The server answered with an error status.</exception>
        /// <exception cref="CaretlyConnectionException">The server could not be reached.</exception>
        /// <exception cref="CaretlyTimeoutException">An attempt took longer than the timeout.</exception>
        public Task<RenderResult> RenderAsync(string zpl, RenderOptions? options = null, CancellationToken cancellationToken = default)
        {
            if (zpl == null)
            {
                throw new ArgumentNullException(nameof(zpl));
            }

            var body = new RequestBody(Encoding.UTF8.GetBytes(zpl), "text/plain; charset=utf-8");
            return RenderCoreAsync(body, options, cancellationToken);
        }

        /// <summary>
        /// Renders several ZPL files in one request, sent as <c>multipart/form-data</c>. Each file
        /// starts from a fresh printer state; pages follow in the given order.
        /// </summary>
        /// <param name="files">The files to render, at least one.</param>
        /// <param name="options">Output format, page, resolution and request options.</param>
        /// <param name="cancellationToken">Cancels the request, including retries.</param>
        /// <returns>The rendered file and its metadata, with <see cref="RenderResult.FilePageCounts"/>.</returns>
        /// <exception cref="CaretlyApiException">The server answered with an error status.</exception>
        /// <exception cref="CaretlyConnectionException">The server could not be reached.</exception>
        /// <exception cref="CaretlyTimeoutException">An attempt took longer than the timeout.</exception>
        public Task<RenderResult> RenderFilesAsync(IEnumerable<ZplFile> files, RenderOptions? options = null, CancellationToken cancellationToken = default)
        {
            if (files == null)
            {
                throw new ArgumentNullException(nameof(files));
            }

            var list = files.ToList();
            if (list.Count == 0)
            {
                throw new ArgumentException("At least one file is required.", nameof(files));
            }

            if (list.Any(f => f == null))
            {
                throw new ArgumentException("Files must not be null.", nameof(files));
            }

            return RenderCoreAsync(BuildMultipart(list), options, cancellationToken);
        }

        /// <summary>Renders ZPL to a PDF holding every page.</summary>
        /// <param name="zpl">The ZPL to render.</param>
        /// <param name="cancellationToken">Cancels the request, including retries.</param>
        /// <returns>The rendered PDF and its metadata.</returns>
        public Task<RenderResult> RenderPdfAsync(string zpl, CancellationToken cancellationToken = default)
            => RenderAsync(zpl, new RenderOptions { Format = RenderFormat.Pdf }, cancellationToken);

        /// <summary>Renders one page of ZPL to a PNG.</summary>
        /// <param name="zpl">The ZPL to render.</param>
        /// <param name="page">The page to render, starting at 1.</param>
        /// <param name="cancellationToken">Cancels the request, including retries.</param>
        /// <returns>The rendered PNG and its metadata.</returns>
        public Task<RenderResult> RenderPngAsync(string zpl, int page = 1, CancellationToken cancellationToken = default)
            => RenderAsync(zpl, new RenderOptions { Format = RenderFormat.Png, Page = page }, cancellationToken);

        /// <summary>
        /// Checks that the server is up (<c>GET /health</c>). Not retried.
        /// </summary>
        /// <param name="cancellationToken">Cancels the request.</param>
        /// <returns><see langword="true"/> when the server answers 200 <c>ok</c>, else <see langword="false"/>.</returns>
        /// <exception cref="CaretlyConnectionException">The server could not be reached.</exception>
        /// <exception cref="CaretlyTimeoutException">The request took longer than the timeout.</exception>
        public async Task<bool> HealthAsync(CancellationToken cancellationToken = default)
        {
            ThrowIfDisposed();
            using (var timeout = CreateTimeoutSource(cancellationToken))
            {
                try
                {
                    using (var request = new HttpRequestMessage(HttpMethod.Get, _baseUrl + "/health"))
                    {
                        request.Headers.TryAddWithoutValidation("User-Agent", UserAgent);
                        using (var response = await _http.SendAsync(request, timeout.Token).ConfigureAwait(false))
                        {
                            if (response.StatusCode != HttpStatusCode.OK)
                            {
                                return false;
                            }

                            var body = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
                            return string.Equals(body.Trim(), "ok", StringComparison.OrdinalIgnoreCase);
                        }
                    }
                }
                catch (OperationCanceledException) when (cancellationToken.IsCancellationRequested)
                {
                    throw;
                }
                catch (OperationCanceledException e)
                {
                    throw TimeoutError(e);
                }
                catch (HttpRequestException e)
                {
                    throw ConnectionError(e);
                }
            }
        }

        /// <summary>Disposes the underlying <see cref="HttpClient"/> if this client created it.</summary>
        public void Dispose()
        {
            if (_disposed)
            {
                return;
            }

            _disposed = true;
            if (_ownsHttpClient)
            {
                _http.Dispose();
            }
        }

        private async Task<RenderResult> RenderCoreAsync(RequestBody body, RenderOptions? options, CancellationToken cancellationToken)
        {
            ThrowIfDisposed();
            options ??= new RenderOptions();
            var url = _baseUrl + "/render" + BuildQuery(options);

            var idempotencyKey = options.IdempotencyKey;
            if (idempotencyKey != null)
            {
                ValidateIdempotencyKey(idempotencyKey);
            }
            else if (_maxRetries > 0)
            {
                // One key for every attempt of this call, so a retry is never billed twice.
                idempotencyKey = Guid.NewGuid().ToString();
            }

            if (options.Compress)
            {
                body = new RequestBody(Gzip(body.Content), body.ContentType, "gzip");
            }

            for (var attempt = 0; ; attempt++)
            {
                TimeSpan retryDelay;
                using (var timeout = CreateTimeoutSource(cancellationToken))
                {
                    try
                    {
                        using (var request = BuildRenderRequest(url, body, idempotencyKey))
                        using (var response = await _http.SendAsync(request, timeout.Token).ConfigureAwait(false))
                        {
                            if (response.IsSuccessStatusCode)
                            {
                                return await ParseResultAsync(response, options).ConfigureAwait(false);
                            }

                            var error = await ParseErrorAsync(response).ConfigureAwait(false);
                            if (!IsRetryable(error.StatusCode) || attempt >= _maxRetries)
                            {
                                throw error;
                            }

                            retryDelay = error.RetryAfter ?? Backoff(attempt);
                        }
                    }
                    catch (OperationCanceledException) when (cancellationToken.IsCancellationRequested)
                    {
                        throw;
                    }
                    catch (OperationCanceledException e)
                    {
                        throw TimeoutError(e);
                    }
                    catch (HttpRequestException e)
                    {
                        if (attempt >= _maxRetries)
                        {
                            throw ConnectionError(e);
                        }

                        retryDelay = Backoff(attempt);
                    }
                }

                await Delay(retryDelay, cancellationToken).ConfigureAwait(false);
            }
        }

        private HttpRequestMessage BuildRenderRequest(string url, RequestBody body, string? idempotencyKey)
        {
            var content = new ByteArrayContent(body.Content);
            content.Headers.TryAddWithoutValidation("Content-Type", body.ContentType);
            if (body.ContentEncoding != null)
            {
                content.Headers.ContentEncoding.Add(body.ContentEncoding);
            }

            var request = new HttpRequestMessage(HttpMethod.Post, url) { Content = content };
            request.Headers.TryAddWithoutValidation("User-Agent", UserAgent);
            if (_apiKey != null)
            {
                request.Headers.Authorization = new AuthenticationHeaderValue("Bearer", _apiKey);
            }

            if (idempotencyKey != null)
            {
                request.Headers.TryAddWithoutValidation("Idempotency-Key", idempotencyKey);
            }

            return request;
        }

        private static string BuildQuery(RenderOptions options)
        {
            var query = new List<string>();
            if (options.Format.HasValue)
            {
                query.Add("format=" + (options.Format.Value == RenderFormat.Png ? "png" : "pdf"));
            }

            if (options.Page.HasValue)
            {
                if (options.Page.Value < 1)
                {
                    throw new ArgumentOutOfRangeException(nameof(options), options.Page.Value, "Page must be 1 or more.");
                }

                query.Add("page=" + options.Page.Value.ToString(CultureInfo.InvariantCulture));
            }

            if (options.Dpmm.HasValue)
            {
                if (Array.IndexOf(DpmmValues, options.Dpmm.Value) < 0)
                {
                    throw new ArgumentOutOfRangeException(nameof(options), options.Dpmm.Value, "Dpmm must be 6, 8, 12 or 24.");
                }

                query.Add("dpmm=" + options.Dpmm.Value.ToString(CultureInfo.InvariantCulture));
            }

            if (options.Timings)
            {
                query.Add("timings=1");
            }

            return query.Count == 0 ? string.Empty : "?" + string.Join("&", query);
        }

        private static void ValidateIdempotencyKey(string key)
        {
            if (key.Length == 0 || key.Length > MaxIdempotencyKeyLength)
            {
                throw new ArgumentException($"IdempotencyKey must be 1 to {MaxIdempotencyKeyLength} characters.", nameof(RenderOptions.IdempotencyKey));
            }

            if (key.Any(c => c < 0x20 || c > 0x7e))
            {
                throw new ArgumentException("IdempotencyKey must be printable ASCII.", nameof(RenderOptions.IdempotencyKey));
            }
        }

        private static RequestBody BuildMultipart(IReadOnlyList<ZplFile> files)
        {
            var boundary = "caretly-" + Guid.NewGuid().ToString("N");
            using (var stream = new MemoryStream())
            {
                foreach (var file in files)
                {
                    var name = file.Name.Replace("\r", " ").Replace("\n", " ").Replace("\"", "%22");
                    var header = "--" + boundary + "\r\n"
                        + "Content-Disposition: form-data; name=\"file\"; filename=\"" + name + "\"\r\n"
                        + "Content-Type: application/octet-stream\r\n\r\n";
                    Write(stream, Encoding.UTF8.GetBytes(header));
                    Write(stream, file.Content);
                    Write(stream, Encoding.ASCII.GetBytes("\r\n"));
                }

                Write(stream, Encoding.ASCII.GetBytes("--" + boundary + "--\r\n"));
                return new RequestBody(stream.ToArray(), "multipart/form-data; boundary=" + boundary);
            }
        }

        private static void Write(Stream stream, byte[] bytes) => stream.Write(bytes, 0, bytes.Length);

        private static byte[] Gzip(byte[] data)
        {
            using (var output = new MemoryStream())
            {
                using (var gzip = new GZipStream(output, CompressionLevel.Optimal, leaveOpen: true))
                {
                    gzip.Write(data, 0, data.Length);
                }

                return output.ToArray();
            }
        }

        private static async Task<RenderResult> ParseResultAsync(HttpResponseMessage response, RenderOptions options)
        {
            var content = await response.Content.ReadAsByteArrayAsync().ConfigureAwait(false);
            var contentType = response.Content.Headers.ContentType?.MediaType
                ?? (options.Format == RenderFormat.Png ? "image/png" : "application/pdf");

            var filePageCounts = new List<int>();
            var fileCounts = Header(response, "X-File-Page-Counts");
            if (fileCounts != null)
            {
                foreach (var part in fileCounts.Split(','))
                {
                    if (int.TryParse(part.Trim(), NumberStyles.Integer, CultureInfo.InvariantCulture, out var count))
                    {
                        filePageCounts.Add(count);
                    }
                }
            }

            var limit = IntHeader(response, "RateLimit-Limit");
            var remaining = IntHeader(response, "RateLimit-Remaining");
            var rateLimit = limit.HasValue && remaining.HasValue ? new RateLimitInfo(limit.Value, remaining.Value) : null;

            return new RenderResult(
                content,
                contentType,
                IntHeader(response, "X-Page-Count") ?? 0,
                filePageCounts.AsReadOnly(),
                IntHeader(response, "X-Labels-Billed"),
                rateLimit,
                Header(response, "Server-Timing"));
        }

        private static async Task<CaretlyApiException> ParseErrorAsync(HttpResponseMessage response)
        {
            string body;
            try
            {
                body = (await response.Content.ReadAsStringAsync().ConfigureAwait(false)).Trim();
            }
            catch (HttpRequestException)
            {
                body = string.Empty;
            }

            if (body.Length == 0)
            {
                body = response.ReasonPhrase ?? string.Empty;
            }

            var status = (int)response.StatusCode;
            var retryAfter = RetryAfter(response);
            switch (status)
            {
                case 400: return new BadRequestException(status, body, retryAfter);
                case 401: return new AuthenticationException(status, body, retryAfter);
                case 402: return new PaymentRequiredException(status, body, retryAfter);
                case 413: return new PayloadTooLargeException(status, body, retryAfter);
                case 415: return new UnsupportedEncodingException(status, body, retryAfter);
                case 429: return new RateLimitedException(status, body, retryAfter);
                case 503: return new OverloadedException(status, body, retryAfter);
                default: return new CaretlyApiException(status, body, retryAfter);
            }
        }

        private static TimeSpan? RetryAfter(HttpResponseMessage response)
        {
            var value = response.Headers.RetryAfter;
            if (value == null)
            {
                return null;
            }

            TimeSpan? delay = value.Delta;
            if (delay == null && value.Date.HasValue)
            {
                delay = value.Date.Value - DateTimeOffset.UtcNow;
            }

            return delay.HasValue && delay.Value < TimeSpan.Zero ? TimeSpan.Zero : delay;
        }

        private static string? Header(HttpResponseMessage response, string name)
        {
            if (response.Headers.TryGetValues(name, out var values) || response.Content.Headers.TryGetValues(name, out values))
            {
                return string.Join(", ", values);
            }

            return null;
        }

        private static int? IntHeader(HttpResponseMessage response, string name)
        {
            var value = Header(response, name);
            return value != null && int.TryParse(value.Trim(), NumberStyles.Integer, CultureInfo.InvariantCulture, out var result)
                ? result
                : (int?)null;
        }

        private static bool IsRetryable(int status) => status == 429 || status == 503;

        /// <summary>Exponential backoff: 0.5 s, 1 s, 2 s, …, capped at 8 s.</summary>
        internal static TimeSpan Backoff(int attempt)
        {
            var seconds = 0.5 * Math.Pow(2, Math.Min(attempt, 10));
            var delay = TimeSpan.FromSeconds(seconds);
            return delay > MaxBackoff ? MaxBackoff : delay;
        }

        private CancellationTokenSource CreateTimeoutSource(CancellationToken cancellationToken)
        {
            var source = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
            if (_timeout != System.Threading.Timeout.InfiniteTimeSpan)
            {
                source.CancelAfter(_timeout);
            }

            return source;
        }

        private CaretlyTimeoutException TimeoutError(Exception inner)
            => new CaretlyTimeoutException($"The request to {_baseUrl} timed out after {_timeout.TotalSeconds:0.###} s.", inner);

        private CaretlyConnectionException ConnectionError(Exception inner)
            => new CaretlyConnectionException($"Could not reach {_baseUrl}: {inner.Message}", inner);

        private void ThrowIfDisposed()
        {
            if (_disposed)
            {
                throw new ObjectDisposedException(nameof(CaretlyClient));
            }
        }

        private sealed class RequestBody
        {
            public RequestBody(byte[] content, string contentType, string? contentEncoding = null)
            {
                Content = content;
                ContentType = contentType;
                ContentEncoding = contentEncoding;
            }

            public byte[] Content { get; }

            public string ContentType { get; }

            public string? ContentEncoding { get; }
        }
    }
}
