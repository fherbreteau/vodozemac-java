package io.github.fherbreteau.vodozemac.megolm;

/**
 * A message successfully decrypted by an {@link InboundGroupSession}.
 * <p>
 * Contains the decrypted plaintext and the message index, which indicates
 * the ratchet position at which the message was encrypted. The message
 * index can be used to detect replay attacks — each plaintext message
 * should be encrypted with a unique message index per session.
 *
 * @see InboundGroupSession#decrypt(String)
 */
public class DecryptedMessage {
    private final byte[] plaintext;
    private final int messageIndex;

    /**
     * Constructs a new {@code DecryptedMessage}.
     *
     * @param plaintext     the decrypted plaintext of the message
     * @param messageIndex  the message index at which the message was encrypted
     */
    public DecryptedMessage(byte[] plaintext, int messageIndex) {
        this.plaintext = plaintext;
        this.messageIndex = messageIndex;
    }

    /**
     * Returns the decrypted plaintext of the message.
     *
     * @return the plaintext bytes
     */
    public byte[] plaintext() {
        return plaintext;
    }

    /**
     * Returns the message index, used to detect replay attacks.
     * <p>
     * Each plaintext message should be encrypted with a unique message
     * index per session.
     *
     * @return the message index
     */
    public int messageIndex() {
        return messageIndex;
    }

}
