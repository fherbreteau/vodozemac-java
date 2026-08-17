package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when serializing or deserializing a vodozemac pickle fails.
 * <p>
 * This includes failures in:
 * <ul>
 *   <li>Plain JSON pickle/unpickle operations</li>
 *   <li>Encrypted pickle/unpickle operations (decryption failures)</li>
 *   <li>Legacy libolm pickle format decoding</li>
 *   <li>Dehydrated device creation or restoration</li>
 * </ul>
 */
public class PickleException extends VodozemacException {

    /**
     * Constructs a new {@code PickleException} with the specified detail message.
     *
     * @param message the detail message
     */
    public PickleException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code PickleException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public PickleException(String message, Throwable cause) {
        super(message, cause);
    }
}
