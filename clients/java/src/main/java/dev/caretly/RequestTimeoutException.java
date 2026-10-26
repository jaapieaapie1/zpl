package dev.caretly;

/**
 * No response arrived within the client's timeout. Not retried, since the server may still be
 * rendering; retry yourself with the same {@link RenderOptions.Builder#idempotencyKey(String)}
 * to avoid double billing.
 */
public class RequestTimeoutException extends CaretlyException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the message
     * @param cause the timeout
     */
    public RequestTimeoutException(String message, Throwable cause) {
        super(message, cause);
    }
}
