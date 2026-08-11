package io.github.fherbreteau.vodozemac.ecies;

/**
 * The result of an outbound ECIES channel establishment.
 * <p>
 * This is returned by {@link Ecies#establishOutboundChannel(String, byte[])}
 * and contains both the established channel and the initial message to send
 * to the recipient.
 *
 * @see Ecies#establishOutboundChannel(String, byte[])
 */
public class OutboundCreationResult {

    private final long nativePtr;

    private final String initialMessage;

    OutboundCreationResult(long nativePtr, String initialMessage) {
        this.nativePtr = nativePtr;
        this.initialMessage = initialMessage;
    }

    /**
     * Returns the established ECIES channel.
     * <p>
     * The returned {@link EstablishedEcies} can be used to encrypt and decrypt
     * further messages after the channel has been established.
     *
     * @return the established ECIES channel
     */
    public EstablishedEcies getEstablishedEcies() {
        return new EstablishedEcies(nativePtr);
    }

    /**
     * Returns the initial message to send to the recipient.
     * <p>
     * The message contains the initiator's ephemeral Curve25519 public key and
     * the ciphertext of the initial plaintext, encoded as a string. The
     * recipient uses this message to establish their side of the channel via
     * {@link Ecies#establishInboundChannel(String)}.
     * <p>
     * The initiator's key embedded in this message is unauthenticated, so
     * authentication must happen out-of-band using the {@link CheckCode} to
     * protect against active man-in-the-middle (MITM) attacks.
     *
     * @return the initial message string
     */
    public String getInitialMessage() {
        return initialMessage;
    }
}
