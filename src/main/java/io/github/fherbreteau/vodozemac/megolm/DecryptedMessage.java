package io.github.fherbreteau.vodozemac.megolm;

public class DecryptedMessage {
    private final byte[] plaintext;
    private final int messageIndex;

    public DecryptedMessage(byte[] plaintext, int messageIndex) {
        this.plaintext = plaintext;
        this.messageIndex = messageIndex;
    }

    public byte[] plaintext() {
        return plaintext;
    }

    public int messageIndex() {
        return messageIndex;
    }

}
