package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when decoding or validating a cryptographic key fails.
 * <p>
 * This includes failures due to:
 * <ul>
 *   <li>Invalid base64 encoding of a key</li>
 *   <li>Incorrect key length (keys must be 32 bytes for encryption operations)</li>
 *   <li>Invalid Curve25519 or Ed25519 public key data</li>
 *   <li>Session key or exported session key decoding failures</li>
 * </ul>
 *
 * @author François HERBRETEAU
 */
public class KeyException extends VodozemacException {

    /**
     * Constructs a new {@code KeyException} with the specified detail message.
     *
     * @param message the detail message
     */
    public KeyException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code KeyException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public KeyException(String message, Throwable cause) {
        super(message, cause);
    }
}
