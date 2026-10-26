package dev.caretly;

import java.time.Duration;

/**
 * 429 Too Many Requests: the API key's rate limit is exhausted. Retried automatically;
 * thrown once retries run out. {@link #retryAfter()} says how long to wait.
 */
public class RateLimitedException extends ApiException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public RateLimitedException(String message, Duration retryAfter) {
        super(429, message, retryAfter);
    }
}
