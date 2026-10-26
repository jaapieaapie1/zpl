package dev.caretly;

import java.time.Duration;

/**
 * 401 Unauthorized: the API key is missing, not valid, expired or revoked.
 */
public class AuthenticationException extends ApiException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public AuthenticationException(String message, Duration retryAfter) {
        super(401, message, retryAfter);
    }
}
