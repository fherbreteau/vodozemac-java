package io.github.fherbreteau.vodozemac.olm;

import java.util.Arrays;
import java.util.Objects;

/**
 * Return type for the creation of an inbound {@link OlmSession}.
 * <p>
 * When an inbound session is created from a pre-key message, the first
 * message (the pre-key message itself) is decrypted as part of the session
 * creation process. This result contains both the newly created session
 * and the decrypted plaintext of that first message.
 *
 * @author François HERBRETEAU
 * @see io.github.fherbreteau.vodozemac.account.Account#createInboundSession(io.github.fherbreteau.vodozemac.olm.OlmSessionVersion, String, io.github.fherbreteau.vodozemac.olm.OlmMessage)
 */
public class InboundCreationResult implements AutoCloseable {
    private final OlmSession session;
    private final byte[] plaintext;

    InboundCreationResult(long sessionPtr, byte[] plaintext) {
        session = new OlmSession(sessionPtr);
        this.plaintext = plaintext;
    }

    /**
     * Returns the {@link OlmSession} that was created from the pre-key message.
     *
     * @return the newly created session
     */
    public OlmSession session() {
        return session;
    }

    /**
     * Returns the plaintext of the pre-key message that was decrypted
     * as part of the session creation.
     *
     * @return the decrypted plaintext bytes
     */
    public byte[] plaintext() {
        return plaintext.clone();
    }

    @Override
    public void close() {
        session.close();
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
