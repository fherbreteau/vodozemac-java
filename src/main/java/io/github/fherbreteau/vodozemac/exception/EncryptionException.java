package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when encrypting a message fails.
 * <p>
 * This includes failures due to non-contributory keys in the Diffie-Hellman
 * operation, where the shared secret does not provide sufficient security
 * guarantees.
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
}
