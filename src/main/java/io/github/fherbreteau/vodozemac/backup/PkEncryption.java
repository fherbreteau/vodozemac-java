package io.github.fherbreteau.vodozemac.backup;

import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.exception.EncryptionException;
import io.github.fherbreteau.vodozemac.exception.KeyException;

/**
 * The encryption component of the PK Encryption module.
 * <p>
 * This implements the {@code m.megolm_backup.v1.curve25519-aes-sha2} algorithm
 * described in the
 * <a href="https://spec.matrix.org/v1.11/client-server-api/#backup-algorithm-mmegolm_backupv1curve25519-aes-sha2">Matrix specification</a>.
 * It is a hybrid encryption scheme utilizing Curve25519 and AES-CBC. X25519
 * ECDH is performed between an ephemeral key pair and a long-lived backup key
 * pair to establish a shared secret, from which symmetric encryption and
 * message authentication (MAC) keys are derived.
 * <p>
 * <strong>Warning:</strong> The algorithm contains a critical flaw and does
 * not provide authentication of the ciphertext. The MAC is computed over an
 * empty message rather than the actual ciphertext, meaning tampering with the
 * ciphertext will not be detected.
 * <p>
 * A {@code PkEncryption} instance is created from the Curve25519 public key
 * of a {@link PkDecryption} object and can be used to encrypt messages that
 * only the corresponding {@code PkDecryption} can decrypt.
 * <p>
 * This class does not extend {@link io.github.fherbreteau.vodozemac.NativeHandle}
 * because it holds no persistent native resource — it only stores the public
 * key string and delegates each encryption call to the native layer.
 *
 * @author François HERBRETEAU
 * @see PkDecryption
 * @see PkMessage
 */
public final class PkEncryption {

    static {
        NativeLibraryLoader.loadLibrary();
    }

    private String publicKey;

    PkEncryption(String publicKey) {
        this.publicKey = publicKey;
    }

    /**
     * Creates a {@code PkEncryption} instance from a base64-encoded
     * Curve25519 public key.
     * <p>
     * The public key should be obtained from an existing {@link PkDecryption}
     * object via {@link PkDecryption#publicKey()}.
     *
     * @param key the base64-encoded Curve25519 public key
     * @return a new {@code PkEncryption} instance
     * @throws KeyException if the key is not a valid Curve25519 public key
     */
    public static PkEncryption fromKey(String key) {
        return nativeFromKey(key);
    }

    /**
     * Encrypts the given plaintext and returns the encrypted message.
     * <p>
     * The encryption uses an ephemeral Curve25519 key pair to perform ECDH
     * with the recipient's public key. The resulting {@link PkMessage}
     * contains the ciphertext, a MAC, and the ephemeral public key.
     *
     * @param plaintext the plaintext bytes to encrypt
     * @return a {@link PkMessage} containing the encrypted message
     * @throws EncryptionException if encryption fails (e.g. non-contributory key)
     */
    public PkMessage encrypt(byte[] plaintext) {
        return nativeEncrypt(publicKey, plaintext);
    }

    private static native PkEncryption nativeFromKey(String key);

    private native PkMessage nativeEncrypt(String publicKey, byte[] plaintext);
}
