package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when SAS (Short Authentication String) verification fails.
 * <p>
 * This includes failures due to:
 * <ul>
 *   <li>MAC validation mismatch during {@link io.github.fherbreteau.vodozemac.sas.EstablishedSas#verifyMac}</li>
 *   <li>An invalid byte count requested during
 *       {@link io.github.fherbreteau.vodozemac.sas.EstablishedSas#bytesRaw}</li>
 * </ul>
 */
public class SasException extends VodozemacException {

    /**
     * Constructs a new {@code SasException} with the specified detail message.
     *
     * @param message the detail message
     */
    public SasException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code SasException} with the specified detail message and cause.
     *
     * @param message the detail message
     * @param cause   the cause of this exception
     */
    public SasException(String message, Throwable cause) {
        super(message, cause);
    }
}
