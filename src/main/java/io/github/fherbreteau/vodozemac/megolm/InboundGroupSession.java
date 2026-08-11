package io.github.fherbreteau.vodozemac.megolm;

import java.util.Optional;

import io.github.fherbreteau.vodozemac.exception.DecryptionException;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.PickleException;

/**
 * A Megolm inbound group session represents a single receiving participant
 * in an encrypted group communication involving multiple recipients.
 * <p>
 * The session includes a ratchet for decryption and an Ed25519 public key
 * for ensuring authenticity. An inbound group session is typically created
 * from a session key obtained from the sender's {@link OutboundGroupSession}
 * via {@link OutboundGroupSession#sessionKey()}.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 */
public class InboundGroupSession implements AutoCloseable {

    private long nativePtr;

    /**
     * Creates a new inbound group session from a session key received over
     * an authenticated channel using the default megolm version.
     *
     * @param sessionKey the base64-encoded session key from {@link OutboundGroupSession#sessionKey()}
     * @throws io.github.fherbreteau.vodozemac.exception.VodozemacException if the session key is invalid
     */
    public InboundGroupSession(String sessionKey) {
        this(sessionKey, MegolmSessionVersion.defaultVersion());
    }

    /**
     * Creates a new inbound group session from a session key received over
     * an authenticated channel.
     *
     * @param sessionKey the base64-encoded session key from {@link OutboundGroupSession#sessionKey()}
     * @param version    the Megolm session protocol version to use
     * @throws io.github.fherbreteau.vodozemac.exception.VodozemacException if the session key is invalid
     */
    public InboundGroupSession(String sessionKey, MegolmSessionVersion version) {
        nativePtr = nativeNew(sessionKey, version.getValue());
    }

    private InboundGroupSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    /**
     * Returns the unique ID of this session.
     * <p>
     * The session ID is the Ed25519 public key of the outbound group session
     * that created this session, encoded in base64.
     *
     * @return the session ID as a base64 string
     * @throws IllegalStateException if this session has been closed
     */
    public String sessionId() {
        checkNotClosed();
        return nativeSessionId(nativePtr);
    }

    /**
     * Returns the first known message index for this session.
     * <p>
     * The message index reflects how many times the ratchet has advanced,
     * determining which messages the session can decrypt. For example, if
     * the first known index is zero, the session can decrypt all messages
     * encrypted by the outbound group session. If the index is one, it can
     * decrypt all messages except the first (zeroth) one.
     *
     * @return the first known message index
     * @throws IllegalStateException if this session has been closed
     */
    public int firstKnownIndex() {
        checkNotClosed();
        return nativeFirstKnownIndex(nativePtr);
    }

    /**
     * Decrypts the provided Megolm message.
     *
     * @param message the base64-encoded encrypted message
     * @return a {@link DecryptedMessage} containing the plaintext and message index
     * @throws IllegalStateException    if this session has been closed
     * @throws DecryptionException      if decryption fails (invalid MAC, padding, or unknown message index)
     * @throws SignatureException       if the message signature is invalid
     */
    public DecryptedMessage decrypt(String message) {
        checkNotClosed();
        return nativeDecrypt(nativePtr, message);
    }

    /**
     * Converts the inbound group session into a JSON string representation.
     *
     * @return a JSON string representing the session
     * @throws IllegalStateException if this session has been closed
     */
    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    /**
     * Encrypts the inbound group session using the given 32-byte key and
     * returns the encrypted representation.
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
     * Exports the session at the specified message index.
     * <p>
     * The exported session key can be used to create a new
     * {@link InboundGroupSession} via {@link #importSession(String, MegolmSessionVersion)}
     * that can decrypt messages from the given index onwards.
     *
     * @param index the message index to export at
     * @return the exported session key as a base64 string, or {@code null}
     *         if the session has been ratcheted beyond the given index
     * @throws IllegalStateException if this session has been closed
     */
    public String exportAt(int index) {
        checkNotClosed();
        return nativeExportAt(nativePtr, index);
    }

    /**
     * Exports the session at its first known message index.
     * <p>
     * This is equivalent to calling {@link #exportAt(int)} with
     * {@link #firstKnownIndex()}.
     *
     * @return the exported session key as a base64 string
     * @throws IllegalStateException if this session has been closed
     */
    public String exportAtFirstKnownIndex() {
        checkNotClosed();
        return nativeExportAtFirstKnownIndex(nativePtr);
    }

    /**
     * Permanently advances the session to the specified message index.
     * <p>
     * Advancing the session will remove the ability to decrypt messages
     * encrypted with a lower index than the provided one.
     *
     * @param index the message index to advance to
     * @return {@code true} if the ratchet was successfully advanced,
     *         {@code false} if the ratchet was already advanced beyond the given index
     * @throws IllegalStateException if this session has been closed
     */
    public boolean advanceTo(int index) {
        checkNotClosed();
        return nativeAdvanceTo(nativePtr, index);
    }

    /**
     * Checks if two sessions are connected, i.e. created from the same
     * outbound group session.
     * <p>
     * If the sessions are connected, the session with the lower message
     * index can safely replace the one with the higher message index.
     *
     * @param other the other session to compare with
     * @return {@code true} if the sessions are connected, {@code false} otherwise
     * @throws IllegalStateException if either session has been closed
     */
    boolean connected(InboundGroupSession other) {
        checkNotClosed();
        other.checkNotClosed();
        return nativeConnected(nativePtr, other.nativePtr);
    }

    /**
     * Compares this session with another session.
     * <p>
     * Returns a {@link SessionOrdering} describing how the two sessions
     * relate to each other. Only connected sessions can be compared
     * meaningfully.
     *
     * @param other the other session to compare with
     * @return the ordering relationship between the two sessions
     * @throws IllegalStateException if either session has been closed
     */
    SessionOrdering compare(InboundGroupSession other) {
        checkNotClosed();
        other.checkNotClosed();
        return nativeCompare(nativePtr, other.nativePtr);
    }

    /**
     * Merges this session with the given other session, picking the best
     * parts from each.
     * <p>
     * This is useful when multiple sessions with the same session ID are
     * received with different ratchet indices and trust properties. The
     * merged session will have the lower (better) initial ratchet index
     * and the combined trust of both sessions.
     *
     * @param other the other session to merge with
     * @return an {@link Optional} containing the merged session if the
     *         sessions are connected, or an empty {@link Optional} if they
     *         are not connected
     * @throws IllegalStateException if either session has been closed
     */
    Optional<InboundGroupSession> merge(InboundGroupSession other) {
        checkNotClosed();
        other.checkNotClosed();
        Long result = nativeMerge(nativePtr, other.nativePtr);
        return Optional.ofNullable(result).map(InboundGroupSession::new);
    }

    /**
     * Restores an {@code InboundGroupSession} from a previously saved
     * JSON string.
     *
     * @param pickleData the JSON string from {@link #pickle()}
     * @return a restored {@code InboundGroupSession}
     * @throws PickleException if the data cannot be deserialized
     */
    public static InboundGroupSession unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new InboundGroupSession(nativePtr);
    }

    /**
     * Restores an {@code InboundGroupSession} from an encrypted string
     * using a 32-byte key.
     *
     * @param pickleData the encrypted pickle data from {@link #pickle(byte[])}
     * @param pickleKey  a 256-bit (32-byte) key for decrypting the session
     * @return a restored {@code InboundGroupSession}
     * @throws KeyException   if the key is not 32 bytes
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static InboundGroupSession unpickle(String pickleData, byte[] pickleKey) {
        if (pickleKey.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
        long nativePtr = nativeEncryptedUnpickle(pickleData, pickleKey);
        return new InboundGroupSession(nativePtr);
    }

    /**
     * Restores an {@code InboundGroupSession} from a legacy libolm pickle
     * format. The pickle must be encrypted with the provided key.
     *
     * @param pickleData the libolm pickle data
     * @param pickleKey  the key used to encrypt the pickle data
     * @return a restored {@code InboundGroupSession}
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static InboundGroupSession unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new InboundGroupSession(nativePtr);
    }

    /**
     * Creates a new {@code InboundGroupSession} from an exported session key.
     * <p>
     * An exported session key can be obtained from another recipient's
     * {@link InboundGroupSession} using {@link #exportAt(int)} or
     * {@link #exportAtFirstKnownIndex()}.
     * <p>
     * <b>Warning</b>: Extra care is required to ensure the authenticity of
     * the session, because an exported session key does not include the
     * signature of the original outbound group session creator.
     *
     * @param sessionKey the base64-encoded exported session key
     * @param version     the Megolm session protocol version to use
     * @return a new {@code InboundGroupSession}
     * @throws io.github.fherbreteau.vodozemac.exception.KeyException if the session key is invalid
     */
    public static InboundGroupSession importSession(String sessionKey, MegolmSessionVersion version) {
        long nativePtr = nativeImport(sessionKey, version.getValue());
        return new InboundGroupSession(nativePtr);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("InboundGroupSession has been closed");
        }
    }

    /* For test usage only */
    boolean isClosed() {
        return nativePtr == 0;
    }

    /**
     * Closes the {@code InboundGroupSession} by releasing its associated
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

    private native long nativeNew(String sessionKey, int version);

    private native String nativeSessionId(long ptr);

    private native int nativeFirstKnownIndex(long ptr);

    private native DecryptedMessage nativeDecrypt(long ptr, String message);

    private native String nativePickle(long ptr);

    private native String nativeExportAt(long ptr, int index);

    private native String nativeExportAtFirstKnownIndex(long ptr);

    private native boolean nativeAdvanceTo(long ptr, int index);

    private native boolean nativeConnected(long ptr, long otherPtr);

    private native SessionOrdering nativeCompare(long ptr, long otherPtr);

    private native Long nativeMerge(long ptr, long otherPtr);

    private native void nativeFree(long ptr);

    private native String nativeEncryptedPickle(long ptr, byte[] key);

    private static native long nativeUnpickle(String pickleData);

    private static native long nativeEncryptedUnpickle(String pickleData, byte[] key);

    private static native long nativeUnpickleLegacy(String pickleData, byte[] pickleKey);

    private static native long nativeImport(String sessionKey, long version);
}
