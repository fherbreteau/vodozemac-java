package io.github.fherbreteau.vodozemac.exception;

/**
 * Base exception class for all errors thrown by the vodozemac Java bindings.
 * <p>
 * All specific exception types ({@link PickleException}, {@link DecryptionException},
 * {@link SessionCreationException}, {@link KeyException}, {@link SignatureException},
 * {@link SasException})
 * extend this class, allowing callers to catch all vodozemac errors with a single
 * catch clause if desired.
 *
 * @author François HERBRETEAU
 */
public class VodozemacException extends RuntimeException {

    /**
     * Constructs a new {@code VodozemacException} with the specified detail message.
     *
     * @param message the detail message
     */
    protected VodozemacException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code VodozemacException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    protected VodozemacException(String message, Throwable cause) {
        super(message, cause);
    }
}
