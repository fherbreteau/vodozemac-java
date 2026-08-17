package io.github.fherbreteau.vodozemac.ecies;

import java.util.Objects;

/**
 * The result of an outbound ECIES channel establishment.
 * <p>
 * This is returned by {@link Ecies#establishOutboundChannel(String, byte[])}
 * and contains both the established channel and the initial message to send
 * to the recipient.
 *
 * @author François HERBRETEAU
 * @see Ecies#establishOutboundChannel(String, byte[])
 */
public class OutboundCreationResult implements AutoCloseable {

    private final EstablishedEcies ecies;

    private final String initialMessage;

    OutboundCreationResult(long nativePtr, String initialMessage) {
        this.ecies = new EstablishedEcies(nativePtr);
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
    public EstablishedEcies establishedEcies() {
        return ecies;
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
    public String initialMessage() {
        return initialMessage;
    }

    /**
     * Closes this resource by releasing its associated native resources.
     * <p>
     * This method is idempotent: calling it more than once has no effect.
     *
     * {@inheritDoc}
     */
    @Override
    public void close() {
        ecies.close();
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof OutboundCreationResult that)) {
            return false;
        }
        return Objects.equals(initialMessage, that.initialMessage);
    }

    @Override
    public int hashCode() {
        return Objects.hash(initialMessage);
    }

    @Override
    public String toString() {
        return "{" +
            " initialMessage='" + initialMessage + "'" +
            "}";
    }
}
