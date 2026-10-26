package dev.caretly;

/**
 * The request could not be sent or the response could not be read, for example because the
 * server is unreachable or closed the connection. Retried automatically.
 */
public class ConnectionException extends CaretlyException {
    private static final long serialVersionUID = 1L;

    /**
     * Creates the error.
     *
     * @param message the message
     * @param cause the transport failure
     */
    public ConnectionException(String message, Throwable cause) {
        super(message, cause);
    }
}
