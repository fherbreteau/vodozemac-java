package io.github.fherbreteau.vodozemac.ecies;

import io.github.fherbreteau.vodozemac.NativeHandle;
import io.github.fherbreteau.vodozemac.exception.EciesException;

/**
 * An established ECIES channel that can be used to encrypt and decrypt
 * messages between the two sides of the channel.
 * <p>
 * An {@code EstablishedEcies} is obtained by calling
 * {@link Ecies#establishOutboundChannel(String, byte[])} on the initiator side
 * or {@link Ecies#establishInboundChannel(String)} on the recipient side. Once
 * established, it can be used to:
 * <ul>
 *   <li>Encrypt and decrypt messages using ChaCha20-Poly1305 via
 *       {@link #encrypt(byte[])} and {@link #decrypt(String)}</li>
 *   <li>Retrieve the {@link CheckCode} for out-of-band MITM verification via
 *       {@link #checkCode()}</li>
 *   <li>Retrieve the Curve25519 public key via {@link #publicKey()}</li>
 * </ul>
 * <p>
 * The encryption and decryption operations use internal nonce counters that
 * increment with each message. Messages must be decrypted in the same order
 * they were encrypted.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 */
public final class EstablishedEcies extends NativeHandle {

    EstablishedEcies(long nativePtr) {
        super(nativePtr);
    }

    /**
     * Returns our Curve25519 public key that was used to establish the ECIES
     * channel.
     *
     * @return the base64-encoded public key
     * @throws IllegalStateException if this {@code EstablishedEcies} has been closed
     */
    public String publicKey() {
        checkNotClosed();
        return nativePublicKey(nativePtr);
    }

    /**
     * Returns the {@link CheckCode} which uniquely identifies this ECIES
     * session.
     * <p>
     * The check code can be used to verify that both sides of the session are
     * using the same shared secret. It should be shared out-of-band to protect
     * against active man-in-the-middle (MITM) attacks.
     *
     * @return the check code
     * @throws IllegalStateException if this {@code EstablishedEcies} has been closed
     */
    public CheckCode checkCode() {
        checkNotClosed();
        return nativeCheckCode(nativePtr);
    }

    /**
     * Encrypts the given plaintext using this ECIES session.
     * <p>
     * The ciphertext is returned as a base64-encoded string. Each call to this
     * method uses an incrementing nonce, so messages must be decrypted in the
     * same order they were encrypted.
     *
     * @param plaintext the plaintext to encrypt
     * @return the encrypted message as a base64-encoded string
     * @throws IllegalStateException if this {@code EstablishedEcies} has been closed
     */
    public String encrypt(byte[] plaintext) {
        checkNotClosed();
        return nativeEncrypt(nativePtr, plaintext);
    }

    /**
     * Decrypts the given message using this ECIES session.
     * <p>
     * The message must be a base64-encoded string produced by
     * {@link #encrypt(byte[])} on the other side. Each call to this method
     * uses an incrementing nonce, so messages must be decrypted in the same
     * order they were encrypted by the sender.
     *
     * @param message the encrypted message as a base64-encoded string
     * @return the decrypted plaintext
     * @throws IllegalStateException if this {@code EstablishedEcies} has been closed
     * @throws EciesException        if decryption fails, e.g. due to a corrupted
     *         message, a replayed message, or a key mismatch
     */
    public byte[] decrypt(String message) {
        checkNotClosed();
        return nativeDecrypt(nativePtr, message);
    }

    private native String nativePublicKey(long ptr);

    private native CheckCode nativeCheckCode(long ptr);

    private native String nativeEncrypt(long ptr, byte[] plaintext);

    private native byte[] nativeDecrypt(long ptr, String message);

    protected native void nativeFree(long ptr);
}
