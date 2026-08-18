package io.github.fherbreteau.vodozemac.ecies;

import java.util.Arrays;
import java.util.Objects;

/**
 * The result of an inbound ECIES channel establishment.
 * <p>
 * This is returned by {@link Ecies#establishInboundChannel(String)} and
 * contains both the established channel and the decrypted plaintext of the
 * initial message sent by the initiator.
 *
 * @author François HERBRETEAU
 * @see Ecies#establishInboundChannel(String)
 */
public class InboundCreationResult implements AutoCloseable {

    private final EstablishedEcies ecies;

    private final byte[] plaintext;

    InboundCreationResult(long nativePtr, byte[] plaintext) {
        this.ecies = new EstablishedEcies(nativePtr);
        this.plaintext = plaintext;
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
     * Returns the decrypted plaintext of the initial message.
     * <p>
     * This is the plaintext that was encrypted by the initiator and sent as
     * part of the initial message.
     *
     * @return the decrypted plaintext bytes
     */
    public byte[] plaintext() {
        return plaintext.clone();
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
        if (!(o instanceof InboundCreationResult that)) {
            return false;
        }
        return Objects.deepEquals(plaintext, that.plaintext);
    }

    @Override
    public int hashCode() {
        return Arrays.hashCode(plaintext);
    }

    @Override
    public String toString() {
        return "{" +
            " plaintext='" + Arrays.toString(plaintext) + "'" +
            "}";
    }
}
