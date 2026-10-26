package dev.caretly;

import java.time.Duration;

/**
 * 400 Bad Request: the ZPL could not be parsed, a parameter is invalid, the {@code page}
 * is out of range, a multipart request has no ZPL files, or a compressed body is corrupt.
 */
public class BadRequestException extends ApiException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public BadRequestException(String message, Duration retryAfter) {
        super(400, message, retryAfter);
    }
}
