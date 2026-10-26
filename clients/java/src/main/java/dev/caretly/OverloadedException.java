package dev.caretly;

import java.time.Duration;

/**
 * 503 Service Unavailable: the server is overloaded and shed the request, which is not
 * billed. Retried automatically; thrown once retries run out.
 */
public class OverloadedException extends ApiException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public OverloadedException(String message, Duration retryAfter) {
        super(503, message, retryAfter);
    }
}
