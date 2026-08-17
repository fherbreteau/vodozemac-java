package io.github.fherbreteau.vodozemac.megolm;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.NativeLibraryLoader;

/**
 * A structured Megolm message, containing the ciphertext, message index,
 * MAC, and Ed25519 signature.
 * <p>
 * A {@code MegolmMessage} is produced by
 * {@link OutboundGroupSession#encrypt(byte[])} and consumed by
 * {@link InboundGroupSession#decrypt(MegolmMessage)}.
 * <p>
 * The {@link #toString()} method produces the base64-encoded wire
 * representation of the message, which is what the native vodozemac layer
 * expects for serialisation and deserialisation.
 * <p>
 * To reconstruct a {@code MegolmMessage} from a base64 string received over
 * the wire, use {@link #fromBase64(String)}.
 *
 * @author François HERBRETEAU
 * @see OutboundGroupSession#encrypt(byte[])
 * @see InboundGroupSession#decrypt(MegolmMessage)
 */
public class MegolmMessage {

    static {
        NativeLibraryLoader.loadLibrary();
    }

    private final String base64;
    private final String ciphertext;
    private final int messageIndex;
    private final String mac;
    private final String signature;

    MegolmMessage(String base64, String ciphertext, int messageIndex, String mac, String signature) {
        this.base64 = base64;
        this.ciphertext = ciphertext;
        this.messageIndex = messageIndex;
        this.mac = mac;
        this.signature = signature;
    }

    /**
     * Returns the base64-encoded ciphertext of the message.
     *
     * @return the ciphertext as a base64 string
     */
    public String getCiphertext() {
        return ciphertext;
    }

    /**
     * Returns the message index, which indicates the ratchet position at
     * which the message was encrypted.
     *
     * @return the message index
     */
    public int getMessageIndex() {
        return messageIndex;
    }

    /**
     * Returns the base64-encoded MAC (message authentication code) of the
     * message.
     *
     * @return the MAC as a base64 string
     */
    public String getMac() {
        return mac;
    }

    /**
     * Returns the base64-encoded Ed25519 signature of the message.
     *
     * @return the signature as a base64 string
     */
    public String getSignature() {
        return signature;
    }

    /**
     * Returns the base64-encoded wire representation of this message.
     * <p>
     * This format is compatible with the Megolm message serialisation used
     * by the native vodozemac layer and the Matrix protocol.
     *
     * @return the base64-encoded message
     */
    @Override
    public String toString() {
        return base64;
    }

    /**
     * Reconstructs a {@code MegolmMessage} from its base64-encoded wire
     * representation.
     *
     * @param base64 the base64-encoded Megolm message
     * @return a new {@code MegolmMessage} instance
     * @throws io.github.fherbreteau.vodozemac.exception.VodozemacException if the input is not a valid Megolm message
     */
    public static MegolmMessage fromBase64(String base64) {
        return nativeFromBase64(base64);
    }

    private static native MegolmMessage nativeFromBase64(String base64);

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof MegolmMessage megolmMessage)) {
            return false;
        }
        return messageIndex == megolmMessage.messageIndex
                && Objects.equals(ciphertext, megolmMessage.ciphertext)
                && Objects.equals(mac, megolmMessage.mac)
                && Objects.equals(signature, megolmMessage.signature);
    }

    @Override
    public int hashCode() {
        return Objects.hash(ciphertext, messageIndex, mac, signature);
    }
}
