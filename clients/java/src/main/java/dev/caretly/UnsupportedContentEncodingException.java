package dev.caretly;

import java.time.Duration;

/**
 * 415 Unsupported Media Type: the server does not support the request's
 * {@code Content-Encoding}.
 */
public class UnsupportedContentEncodingException extends ApiException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public UnsupportedContentEncodingException(String message, Duration retryAfter) {
        super(415, message, retryAfter);
    }
}
