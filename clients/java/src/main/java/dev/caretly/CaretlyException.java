package dev.caretly;

/**
 * The base of every error the client throws. Unchecked.
 *
 * <p>{@link ApiException} and its subclasses are errors returned by the API;
 * {@link ConnectionException} and {@link RequestTimeoutException} are transport failures.
 */
public class CaretlyException extends RuntimeException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates an exception.
     *
     * @param message the message
     */
    public CaretlyException(String message) {
        super(message);
    }

    /**
     * Creates an exception with a cause.
     *
     * @param message the message
     * @param cause the cause
     */
    public CaretlyException(String message, Throwable cause) {
        super(message, cause);
    }
}
