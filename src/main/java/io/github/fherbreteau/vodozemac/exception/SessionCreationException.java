package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when creating an inbound Olm session from a pre-key message fails.
 * <p>
 * This includes failures due to:
 * <ul>
 *   <li>The pre-key message containing an unknown one-time key</li>
 *   <li>The identity key not matching the one in the pre-key message</li>
 *   <li>The session config (version) not matching the one used to encrypt the pre-key message</li>
 *   <li>Receiving a normal message instead of a pre-key message</li>
 * </ul>
 *
 * @author François HERBRETEAU
 */
public class SessionCreationException extends VodozemacException {

    /**
     * Constructs a new {@code SessionCreationException} with the specified detail message.
     *
     * @param message the detail message
     */
    public SessionCreationException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code SessionCreationException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public SessionCreationException(String message, Throwable cause) {
        super(message, cause);
    }
}
