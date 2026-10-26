using System;
using System.Collections.Generic;
using System.IO;
using System.Threading;
using System.Threading.Tasks;

namespace Caretly
{
    /// <summary>A rendered file and the metadata the server returned with it.</summary>
    public sealed class RenderResult
    {
        /// <summary>Creates a result. Normally created by <see cref="CaretlyClient"/>.</summary>
        /// <param name="content">The file bytes.</param>
        /// <param name="contentType">The media type, <c>application/pdf</c> or <c>image/png</c>.</param>
        /// <param name="pageCount">Total pages across the input.</param>
        /// <param name="filePageCounts">Pages per file for a multi-file request; empty otherwise.</param>
        /// <param name="labelsBilled">Labels billed, when the server reports it.</param>
        /// <param name="rateLimit">The rate limit state, when the server reports it.</param>
        /// <param name="serverTiming">The raw <c>Server-Timing</c> header, when requested.</param>
        public RenderResult(
            byte[] content,
            string contentType,
            int pageCount,
            IReadOnlyList<int> filePageCounts,
            int? labelsBilled,
            RateLimitInfo? rateLimit,
            string? serverTiming)
        {
            Content = content ?? throw new ArgumentNullException(nameof(content));
            ContentType = contentType ?? throw new ArgumentNullException(nameof(contentType));
            PageCount = pageCount;
            FilePageCounts = filePageCounts ?? throw new ArgumentNullException(nameof(filePageCounts));
            LabelsBilled = labelsBilled;
            RateLimit = rateLimit;
            ServerTiming = serverTiming;
        }

        /// <summary>The file bytes: a PDF or a PNG.</summary>
        public byte[] Content { get; }

        /// <summary>The media type, <c>application/pdf</c> or <c>image/png</c>.</summary>
        public string ContentType { get; }

        /// <summary>Total pages across the input (<c>X-Page-Count</c>).</summary>
        public int PageCount { get; }

        /// <summary>
        /// Pages per file, in upload order, for a multi-file request (<c>X-File-Page-Counts</c>).
        /// Empty for a single label.
        /// </summary>
        public IReadOnlyList<int> FilePageCounts { get; }

        /// <summary>
        /// Labels billed for this request (<c>X-Labels-Billed</c>): 0 for a test key, and
        /// <see langword="null"/> when the server runs without API keys.
        /// </summary>
        public int? LabelsBilled { get; }

        /// <summary>The rate limit state of the API key, or <see langword="null"/> when not reported.</summary>
        public RateLimitInfo? RateLimit { get; }

        /// <summary>
        /// The raw <c>Server-Timing</c> header, present when <see cref="RenderOptions.Timings"/> was set.
        /// </summary>
        public string? ServerTiming { get; }

        /// <summary>Writes the file bytes to <paramref name="path"/>, replacing any existing file.</summary>
        /// <param name="path">The file to write.</param>
        public void SaveTo(string path)
        {
            if (path == null)
            {
                throw new ArgumentNullException(nameof(path));
            }

            File.WriteAllBytes(path, Content);
        }

        /// <summary>Writes the file bytes to <paramref name="path"/>, replacing any existing file.</summary>
        /// <param name="path">The file to write.</param>
        /// <param name="cancellationToken">Cancels the write.</param>
        /// <returns>A task that completes when the file is written.</returns>
        public async Task SaveToAsync(string path, CancellationToken cancellationToken = default)
        {
            if (path == null)
            {
                throw new ArgumentNullException(nameof(path));
            }

            using (var stream = new FileStream(path, FileMode.Create, FileAccess.Write, FileShare.None, 81920, useAsync: true))
            {
                await stream.WriteAsync(Content, 0, Content.Length, cancellationToken).ConfigureAwait(false);
            }
        }
    }
}
