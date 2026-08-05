package io.github.fherbreteau.vodozemac.olm;

/**
 * Result of a creation of inbound {@code OlmSession}.
 */
public class InboundCreationResult {
    private final OlmSession session;
    private final byte[] plaintext;

    public InboundCreationResult(long sessionPtr, byte[] plaintext) {
        session = new OlmSession(sessionPtr);
        this.plaintext = plaintext;
    }

    /**
     * Returns The {@code OlmSession} that was created from a pre-key message.
     * @return an {@code OlmSession} object
     */
    public OlmSession getSession() {
        return session;
    }

    /**
     * Returns The plaintext of the pre-key message.
     * @return the text of the message
     */
    public byte[] getPlaintext() {
        return plaintext;
    }
}
