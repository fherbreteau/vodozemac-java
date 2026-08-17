package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when decrypting an Olm or Megolm message fails.
 * <p>
 * This includes failures due to invalid MAC, invalid padding, or attempting
 * to decrypt a message whose index is before the session's first known index
 * (the message key has been discarded).
 *
 * @author François HERBRETEAU
 */
public class DecryptionException extends VodozemacException {

    /**
     * Constructs a new {@code DecryptionException} with the specified detail message.
     *
     * @param message the detail message
     */
    public DecryptionException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code DecryptionException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public DecryptionException(String message, Throwable cause) {
        super(message, cause);
    }
}
