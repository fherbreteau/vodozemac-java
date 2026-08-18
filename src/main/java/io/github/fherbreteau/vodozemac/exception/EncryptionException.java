package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when encrypting a message fails.
 * <p>
 * This includes failures due to non-contributory keys in the Diffie-Hellman
 * operation, where the shared secret does not provide sufficient security
 * guarantees.
 *
 * @author François HERBRETEAU
 */
public class EncryptionException extends VodozemacException {

    /**
     * Constructs a new {@code EncryptionException} with the specified detail message.
     *
     * @param message the detail message
     */
    public EncryptionException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code EncryptionException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public EncryptionException(String message, Throwable cause) {
        super(message, cause);
    }
}
