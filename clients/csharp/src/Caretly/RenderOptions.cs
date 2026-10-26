namespace Caretly
{
    /// <summary>
    /// Options for a render request. Every property is optional; unset values are left out of
    /// the request so the server defaults apply.
    /// </summary>
    public sealed class RenderOptions
    {
        /// <summary>The output format. The server default is <see cref="RenderFormat.Pdf"/>.</summary>
        public RenderFormat? Format { get; set; }

        /// <summary>
        /// For PNG output: the page to render, starting at 1. Pages are counted across all files
        /// of a multi-file request. The server default is 1.
        /// </summary>
        public int? Page { get; set; }

        /// <summary>
        /// Printer resolution in dots per millimetre: 6 (152 dpi), 8 (203 dpi), 12 (300 dpi) or
        /// 24 (600 dpi). The server default is 8.
        /// </summary>
        public int? Dpmm { get; set; }

        /// <summary>
        /// Ask the server for per-stage timings, returned in <see cref="RenderResult.ServerTiming"/>.
        /// </summary>
        public bool Timings { get; set; }

        /// <summary>
        /// An idempotency key of at most 255 characters, for example an order number. A retry with
        /// the same key, labels and parameters is billed once. When not set and retries are
        /// enabled, the client generates one per call so its own retries are never billed twice.
        /// </summary>
        public string? IdempotencyKey { get; set; }

        /// <summary>Compress the request body with gzip.</summary>
        public bool Compress { get; set; }
    }
}
