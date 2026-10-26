package dev.caretly;

import java.time.Duration;

/**
 * 413 Payload Too Large: the body is larger than 100 MiB after decompression.
 */
public class PayloadTooLargeException extends ApiException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public PayloadTooLargeException(String message, Duration retryAfter) {
        super(413, message, retryAfter);
    }
}
