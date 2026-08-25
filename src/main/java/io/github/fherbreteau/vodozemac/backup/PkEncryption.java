package io.github.fherbreteau.vodozemac.backup;

import io.github.fherbreteau.vodozemac.NativeHandle;
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
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 *
 * @author François HERBRETEAU
 * @see PkDecryption
 * @see PkMessage
 */
public final class PkEncryption extends NativeHandle {

    static {
        NativeLibraryLoader.loadLibrary();
    }

    private PkEncryption(long nativePtr) {
        super(nativePtr);
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
        long nativePtr = nativeFromKey(key);
        return new PkEncryption(nativePtr);
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
        return nativeEncrypt(nativePtr, plaintext);
    }

    private static native long nativeFromKey(String key);

    private native PkMessage nativeEncrypt(long ptr, byte[] plaintext);

    protected native void nativeFree(long ptr);

}
