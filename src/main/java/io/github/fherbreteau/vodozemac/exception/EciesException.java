package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when an ECIES operation fails.
 * <p>
 * This can occur during:
 * <ul>
 *   <li>Outbound channel establishment, e.g. due to a non-contributory key
 *       where at least one of the keys did not have contributory behaviour
 *       and the resulting shared secret would have been insecure</li>
 *   <li>Inbound channel establishment, e.g. due to a non-contributory key or
 *       a malformed initial message</li>
 *   <li>Message decryption, e.g. due to a corrupted message, a replayed
 *       message, or a key mismatch</li>
 * </ul>
 *
 * @author François HERBRETEAU
 * @see io.github.fherbreteau.vodozemac.ecies.Ecies
 * @see io.github.fherbreteau.vodozemac.ecies.EstablishedEcies
 */
public class EciesException extends VodozemacException {

    /**
     * Constructs a new {@code EciesException} with the specified detail message.
     *
     * @param message the detail message
     */
    public EciesException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code EciesException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public EciesException(String message, Throwable cause) {
        super(message, cause);
    }
}
