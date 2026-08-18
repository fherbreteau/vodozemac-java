package io.github.fherbreteau.vodozemac.backup;

import io.github.fherbreteau.vodozemac.NativeHandle;
import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.exception.DecryptionException;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.PickleException;

/**
 * The decryption component of the PK Encryption module.
 * <p>
 * This implements the {@code m.megolm_backup.v1.curve25519-aes-sha2} algorithm
 * described in the
 * <a href="https://spec.matrix.org/v1.11/client-server-api/#backup-algorithm-mmegolm_backupv1curve25519-aes-sha2">Matrix specification</a>.
 * It is a hybrid encryption scheme utilizing Curve25519 and AES-CBC. X25519
 * ECDH is performed between an ephemeral key pair and a long-lived backup key
 * pair to establish a shared secret, from which symmetric encryption and
 * message authentication (MAC) keys are derived.
 * <p>
 * A {@code PkDecryption} holds a Curve25519 secret key that serves as the
 * long-term key to derive individual message keys. The corresponding public
 * key can be shared with others, allowing them to encrypt messages which can
 * only be decrypted using this object.
 * <p>
 * <strong>Warning:</strong> The algorithm contains a critical flaw and does
 * not provide authentication of the ciphertext. The MAC is computed over an
 * empty message rather than the actual ciphertext, meaning tampering with the
 * ciphertext will not be detected.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 *
 * @author François HERBRETEAU
 * @see PkEncryption
 * @see PkMessage
 */
public final class PkDecryption extends NativeHandle {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    /**
     * Creates a new {@code PkDecryption} with a fresh random Curve25519 key pair.
     */
    public PkDecryption() {
        super(nativeNew());
    }

    private PkDecryption(long nativePtr) {
        super(nativePtr);
    }

    /**
     * Creates a {@code PkDecryption} from a base64-encoded Curve25519 secret key.
     * <p>
     * The secret key is used as the long-term key to derive individual message
     * keys. The corresponding public key can be obtained via
     * {@link #publicKey()} after creation.
     *
     * @param key the base64-encoded Curve25519 secret key (32 bytes)
     * @return a new {@code PkDecryption} instance
     * @throws io.github.fherbreteau.vodozemac.exception.VodozemacException if the key is not a valid 32-byte base64-encoded key
     */
    public static PkDecryption fromKey(String key) {
        long nativePtr = nativeFromKey(key);
        return new PkDecryption(nativePtr);
    }

    /**
     * Returns the Curve25519 secret key of this {@code PkDecryption}.
     * <p>
     * If persistence is required, securely store this key. It can be used to
     * reconstruct the {@code PkDecryption} object for decrypting associated
     * messages via {@link #fromKey(String)}.
     *
     * @return the base64-encoded Curve25519 secret key
     * @throws IllegalStateException if this {@code PkDecryption} has been closed
     */
    public String secretKey() {
        checkNotClosed();
        return nativeSecretKey(nativePtr);
    }

    /**
     * Returns the Curve25519 public key associated with this
     * {@code PkDecryption}.
     * <p>
     * This key can be shared with others to allow them to create a
     * {@link PkEncryption} object and encrypt messages that only this
     * {@code PkDecryption} can decrypt.
     *
     * @return the base64-encoded Curve25519 public key
     * @throws IllegalStateException if this {@code PkDecryption} has been closed
     */
    public String publicKey() {
        checkNotClosed();
        return nativePublicKey(nativePtr);
    }

    /**
     * Decrypts a {@link PkMessage} that was encrypted for this
     * {@code PkDecryption}.
     * <p>
     * The decryption performs X25519 ECDH between the secret key and the
     * message's ephemeral key to derive the symmetric keys, then verifies
     * the MAC and decrypts the AES-CBC ciphertext.
     *
     * @param message the {@link PkMessage} to decrypt
     * @return the decrypted plaintext bytes
     * @throws IllegalStateException if this {@code PkDecryption} has been closed
     * @throws KeyException         if the ephemeral key in the message is invalid
     * @throws DecryptionException  if the MAC verification fails or the padding is invalid
     */
    public byte[] decrypt(PkMessage message) {
        checkNotClosed();
        return nativeDecrypt(nativePtr, message.getCiphertext(), message.getMac(), message.getEphemeralKey());
    }

    /**
     * Restores a {@code PkDecryption} from a legacy libolm pickle format.
     * The pickle must be encrypted with the provided key.
     *
     * @param pickleData the libolm pickle data
     * @param pickleKey  the key used to encrypt the pickle data
     * @return a restored {@code PkDecryption}
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static PkDecryption unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new PkDecryption(nativePtr);
    }

    private static native long nativeNew();

    private static native long nativeFromKey(String key);

    private static native long nativeUnpickleLegacy(String pickleData, byte[] pickleKey);

    private native String nativeSecretKey(long ptr);

    private native String nativePublicKey(long ptr);

    private native byte[] nativeDecrypt(long ptr, String ciphertext, String mac, String ephemeralKey);

    protected native void nativeFree(long ptr);

}
