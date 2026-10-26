using System;

namespace Caretly
{
    /// <summary>Settings for <see cref="CaretlyClient"/>.</summary>
    public sealed class CaretlyClientOptions
    {
        /// <summary>The default base URL, <c>https://caretly.dev</c>.</summary>
        public const string DefaultBaseUrl = "https://caretly.dev";

        /// <summary>The environment variable read when <see cref="ApiKey"/> is not set.</summary>
        public const string ApiKeyEnvironmentVariable = "CARETLY_API_KEY";

        /// <summary>
        /// The API key (<c>zpl_live_…</c> or <c>zpl_test_…</c>). When <see langword="null"/>, the
        /// <c>CARETLY_API_KEY</c> environment variable is used. Optional: a self-hosted server
        /// without keys accepts requests without one.
        /// </summary>
        public string? ApiKey { get; set; }

        /// <summary>
        /// The server's base URL. When <see langword="null"/>, the injected
        /// <see cref="System.Net.Http.HttpClient.BaseAddress"/> is used if set, else
        /// <see cref="DefaultBaseUrl"/>.
        /// </summary>
        public string? BaseUrl { get; set; }

        /// <summary>The timeout of each attempt. Defaults to 30 seconds.</summary>
        public TimeSpan Timeout { get; set; } = TimeSpan.FromSeconds(30);

        /// <summary>
        /// How often a request is retried after a 429, a 503 or a connection error. Defaults to 2.
        /// Set to 0 to disable retries.
        /// </summary>
        public int MaxRetries { get; set; } = 2;
    }
}
