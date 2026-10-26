package dev.caretly;

import java.time.Duration;

/**
 * 402 Payment Required: billing for the account is not active, so live keys are paused.
 * Test keys keep working.
 */
public class PaymentRequiredException extends ApiException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the response body text
     * @param retryAfter the {@code Retry-After} delay, or {@code null} when absent
     */
    public PaymentRequiredException(String message, Duration retryAfter) {
        super(402, message, retryAfter);
    }
}
