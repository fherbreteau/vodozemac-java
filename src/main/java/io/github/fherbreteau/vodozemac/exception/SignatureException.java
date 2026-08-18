package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when signature verification fails.
 * <p>
 * This occurs when a Megolm message's Ed25519 signature is invalid, which can
 * happen if the message was tampered with, or if the message is decrypted
 * with an inbound group session that does not match the outbound group
 * session that created it.
 */
public class SignatureException extends VodozemacException {

    /**
     * Constructs a new {@code SignatureException} with the specified detail message.
     *
     * @param message the detail message
     */
    public SignatureException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code SignatureException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public SignatureException(String message, Throwable cause) {
        super(message, cause);
    }
}
