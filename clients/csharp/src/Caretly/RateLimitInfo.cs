namespace Caretly
{
    /// <summary>The rate limit state of the API key, from the <c>RateLimit-*</c> response headers.</summary>
    public sealed class RateLimitInfo
    {
        /// <summary>Creates a rate limit snapshot.</summary>
        /// <param name="limit">The burst size: how many requests the key can make at once.</param>
        /// <param name="remaining">Requests the key can still make at once before it is rate limited.</param>
        public RateLimitInfo(int limit, int remaining)
        {
            Limit = limit;
            Remaining = remaining;
        }

        /// <summary>The burst size: how many requests the key can make at once.</summary>
        public int Limit { get; }

        /// <summary>Requests the key can still make at once before it is rate limited.</summary>
        public int Remaining { get; }

        /// <inheritdoc />
        public override string ToString() => $"{Remaining}/{Limit}";
    }
}
