package io.github.fherbreteau.vodozemac.megolm;

import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.PickleException;

/**
 * A Megolm outbound group session represents a single sending participant in
 * an encrypted group communication context containing multiple receiving
 * parties.
 * <p>
 * A group session consists of a ratchet, used for encryption, and an Ed25519
 * signing key pair, used for authenticity. The session key (obtained via
 * {@link #sessionKey()}) can be shared with other participants so that they
 * can create {@link InboundGroupSession} objects to decrypt messages sent
 * by this session.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 */
public class OutboundGroupSession implements AutoCloseable {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    private long nativePtr;

    /**
     * Constructs a new outbound group session with a random ratchet state
     * and signing key pair.
     */
    public OutboundGroupSession() {
        this(MegolmSessionVersion.defaultVersion());
    }

    /**
     * Constructs a new outbound group session with a random ratchet state
     * and signing key pair.
     *
     * @param version the Megolm session protocol version to use
     */
    public OutboundGroupSession(MegolmSessionVersion version) {
        nativePtr = nativeNew(version.getValue());
    }

    private OutboundGroupSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    /**
     * Returns the globally unique session ID, in base64-encoded form.
     * <p>
     * The session ID is the public part of the Ed25519 key pair associated
     * with the group session. Due to the construction, every session ID is
     * probabilistically globally unique.
     *
     * @return the session ID as a base64 string
     * @throws IllegalStateException if this session has been closed
     */
    public String sessionId() {
        checkNotClosed();
        return nativeSessionId(nativePtr);
    }

    /**
     * Returns the current message index.
     * <p>
     * The message index is incremented each time a message is encrypted
     * with the group session.
     *
     * @return the current message index
     * @throws IllegalStateException if this session has been closed
     */
    public int messageIndex() {
        checkNotClosed();
        return nativeMessageIndex(nativePtr);
    }

    /**
     * Exports the group session into a session key.
     * <p>
     * The session key contains the key version constant, the current message
     * index, the ratchet state, and the public part of the signing key pair.
     * It is signed by the signing key pair for authenticity. The session key
     * is typically sent to other group participants via a secure peer-to-peer
     * channel (e.g. an Olm channel) so that they can reconstruct an
     * {@link InboundGroupSession} to decrypt messages.
     *
     * @return the session key as a base64 string
     * @throws IllegalStateException if this session has been closed
     */
    public String sessionKey() {
        checkNotClosed();
        return nativeSessionKey(nativePtr);
    }

    /**
     * Encrypts the given plaintext with the group session.
     * <p>
     * The resulting ciphertext is MAC-ed, signed with the group session's
     * Ed25519 key pair, and base64-encoded.
     *
     * @param plainText the plaintext to encrypt
     * @return the encrypted message as a base64 string
     * @throws IllegalStateException if this session has been closed
     */
    public String encrypt(byte[] plainText) {
        checkNotClosed();
        return nativeEncrypt(nativePtr, plainText);
    }

    /**
     * Converts the group session into a JSON string representation.
     *
     * @return a JSON string representing the session
     * @throws IllegalStateException if this session has been closed
     */
    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    /**
     * Encrypts the group session using the given 32-byte key and returns
     * the encrypted representation.
     *
     * @param key a 256-bit (32-byte) key for encrypting the session
     * @return an encrypted string representation of the session
     * @throws IllegalStateException if this session has been closed
     * @throws KeyException         if the key is not 32 bytes
     */
    public String pickle(byte[] key) {
        checkNotClosed();
        if (key.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
        return nativeEncryptedPickle(nativePtr, key);
    }

    /**
     * Restores an {@code OutboundGroupSession} from a previously saved
     * JSON string.
     *
     * @param pickleData the JSON string from {@link #pickle()}
     * @return a restored {@code OutboundGroupSession}
     * @throws PickleException if the data cannot be deserialized
     */
    public static OutboundGroupSession unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new OutboundGroupSession(nativePtr);
    }

    /**
     * Restores an {@code OutboundGroupSession} from an encrypted string
     * using a 32-byte key.
     *
     * @param pickleData the encrypted pickle data from {@link #pickle(byte[])}
     * @param pickleKey  a 256-bit (32-byte) key for decrypting the session
     * @return a restored {@code OutboundGroupSession}
     * @throws KeyException   if the key is not 32 bytes
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static OutboundGroupSession unpickle(String pickleData, byte[] pickleKey) {
        if (pickleKey.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
        long nativePtr = nativeEncryptedUnpickle(pickleData, pickleKey);
        return new OutboundGroupSession(nativePtr);
    }

    /**
     * Restores an {@code OutboundGroupSession} from a legacy libolm pickle
     * format. The pickle must be encrypted with the provided key.
     *
     * @param pickleData the libolm pickle data
     * @param pickleKey  the key used to encrypt the pickle data
     * @return a restored {@code OutboundGroupSession}
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static OutboundGroupSession unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new OutboundGroupSession(nativePtr);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("OutboundGroupSession has been closed");
        }
    }

    /* For test usage only */
    boolean isClosed() {
        return nativePtr == 0;
    }

    /**
     * Closes the {@code OutboundGroupSession} by releasing its associated
     * native resources.
     *
     * {@inheritDoc}
     */
    @Override
    public void close() {
        if (nativePtr != 0) {
            nativeFree(nativePtr);
            nativePtr = 0;
        }
    }

    private native long nativeNew(int version);

    private native String nativeSessionId(long ptr);

    private native int nativeMessageIndex(long ptr);

    private native String nativeSessionKey(long ptr);

    private native String nativeEncrypt(long ptr, byte[] plaintext);

    private native String nativePickle(long ptr);

    private native String nativeEncryptedPickle(long ptr, byte[] key);

    private static native long nativeUnpickle(String pickleData);

    private static native long nativeEncryptedUnpickle(String pickleData, byte[] key);

    private static native long nativeUnpickleLegacy(String pickleData, byte[] pickleKey);

    private native void nativeFree(long ptr);

}
