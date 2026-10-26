package dev.caretly;

import java.time.Duration;
import java.util.Optional;

/**
 * An error response from the API. The message is the response body text.
 *
 * <p>Statuses with their own type are 400 {@link BadRequestException}, 401
 * {@link AuthenticationException}, 402 {@link PaymentRequiredException}, 413
 * {@link PayloadTooLargeException}, 415 {@link UnsupportedContentEncodingException}, 429
 * {@link RateLimitedException} and 503 {@link OverloadedException}. Any other status, including
 * other 5xx errors, is a plain {@code ApiException}.
 */
public class ApiException extends CaretlyException {
    private static final long serialVersionUID = 1L;

    /** The HTTP status code. */
    private final int statusCode;
    /** The {@code Retry-After} delay, or {@code null}. */
    private final Duration retryAfter;

    /**
     * Creates an API error.
     *
     * @param statusCode the HTTP status code
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public ApiException(int statusCode, String message, Duration retryAfter) {
        super(message);
        this.statusCode = statusCode;
        this.retryAfter = retryAfter;
    }

    /**
     * Returns the HTTP status code.
     *
     * @return the status code
     */
    public int statusCode() {
        return statusCode;
    }

    /**
     * Returns how long to wait before retrying, from the {@code Retry-After} header.
     *
     * @return the delay; empty when the server sent none
     */
    public Optional<Duration> retryAfter() {
        return Optional.ofNullable(retryAfter);
    }

    static ApiException of(int status, String message, Duration retryAfter) {
        switch (status) {
            case 400:
                return new BadRequestException(message, retryAfter);
            case 401:
                return new AuthenticationException(message, retryAfter);
            case 402:
                return new PaymentRequiredException(message, retryAfter);
            case 413:
                return new PayloadTooLargeException(message, retryAfter);
            case 415:
                return new UnsupportedContentEncodingException(message, retryAfter);
            case 429:
                return new RateLimitedException(message, retryAfter);
            case 503:
                return new OverloadedException(message, retryAfter);
            default:
                return new ApiException(status, message, retryAfter);
        }
    }
}
