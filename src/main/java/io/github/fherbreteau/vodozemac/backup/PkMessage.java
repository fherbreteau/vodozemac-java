package io.github.fherbreteau.vodozemac.backup;

import java.util.Objects;

/**
 * A message that was encrypted using a {@link PkEncryption} object.
 * <p>
 * A {@code PkMessage} consists of three base64-encoded components:
 * <ul>
 *   <li>The ciphertext, produced by AES-CBC encryption of the plaintext</li>
 *   <li>The MAC (message authentication code), intended to authenticate the
 *       message</li>
 *   <li>The ephemeral Curve25519 public key used to derive the message key
 *       via X25519 ECDH</li>
 * </ul>
 * <p>
 * <strong>Warning:</strong> As stated in the {@link PkEncryption} class
 * documentation, the MAC does <em>not</em> authenticate the ciphertext.
 * The MAC is computed over an empty message rather than the actual
 * ciphertext, meaning tampering with the ciphertext will not be detected.
 *
 * @author François HERBRETEAU
 * @see PkEncryption#encrypt(byte[])
 * @see PkDecryption#decrypt(PkMessage)
 */
public class PkMessage {

    private final String ciphertext;
    private final String mac;
    private final String ephemeralKey;

    PkMessage(String ciphertext, String mac, String ephemeralKey) {
        this.ciphertext = ciphertext;
        this.mac = mac;
        this.ephemeralKey = ephemeralKey;
    }

    /**
     * Returns the base64-encoded ciphertext of the message.
     *
     * @return the ciphertext as a base64 string
     */
    public String ciphertext() {
        return ciphertext;
    }

    /**
     * Returns the base64-encoded MAC (message authentication code) of the
     * message.
     * <p>
     * <strong>Warning:</strong> The MAC does not authenticate the ciphertext.
     * See the {@link PkEncryption} class documentation for details.
     *
     * @return the MAC as a base64 string
     */
    public String mac() {
        return mac;
    }

    /**
     * Returns the base64-encoded ephemeral Curve25519 public key used to
     * derive the message key for this message.
     *
     * @return the ephemeral key as a base64 string
     */
    public String ephemeralKey() {
        return ephemeralKey;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof PkMessage pkMessage)) {
            return false;
        }
        return Objects.equals(ciphertext, pkMessage.ciphertext)
                && Objects.equals(mac, pkMessage.mac)
                && Objects.equals(ephemeralKey, pkMessage.ephemeralKey);
    }

    @Override
    public int hashCode() {
        return Objects.hash(ciphertext, mac, ephemeralKey);
    }

    @Override
    public String toString() {
        return "{" +
            " ciphertext='" + ciphertext + "'" +
            ", mac='" + mac + "'" +
            ", ephemeralKey='" + ephemeralKey + "'" +
            "}";
    }
}
