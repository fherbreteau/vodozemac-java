package io.github.fherbreteau.vodozemac.olm;

import io.github.fherbreteau.vodozemac.exception.DecryptionException;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.PickleException;

/**
 * An Olm session represents one end of an encrypted communication channel
 * between two participants.
 * <p>
 * A session enables the session owner to encrypt messages intended for,
 * and decrypt messages sent by, the other participant of the channel.
 * <p>
 * Olm sessions have two important properties:
 * <ol>
 *   <li>They are based on a double ratchet algorithm which continuously
 *       introduces new entropy into the channel as messages are sent and
 *       received. This imbues the channel with <i>self-healing</i>
 *       properties, allowing it to recover from a momentary loss of
 *       confidentiality in the event of a key compromise.</li>
 *   <li>They are <i>asynchronous</i>, allowing the participant to start
 *       sending messages to the other side even if the other participant
 *       is not online at the moment.</li>
 * </ol>
 * <p>
 * An {@code OlmSession} is acquired from an
 * {@link io.github.fherbreteau.vodozemac.account.Account}, by calling
 * {@link io.github.fherbreteau.vodozemac.account.Account#createOutboundSession(OlmSessionVersion, String, String)}
 * if you are the first participant to send a message, or
 * {@link io.github.fherbreteau.vodozemac.account.Account#createInboundSession(OlmSessionVersion, String, String)}
 * if the other participant initiated the channel by sending you a message.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 */
public class OlmSession implements AutoCloseable {
    private long nativePtr;

    OlmSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    /**
     * Returns the globally unique session ID, in base64-encoded form.
     *
     * @return the session ID as a base64 string
     * @throws IllegalStateException if this session has been closed
     */
    public String sessionId() {
        checkNotClosed();
        return nativeSessionId(nativePtr);
    }

    /**
     * Checks whether a message has ever been received and decrypted from
     * the other side.
     * <p>
     * This is used to decide if outgoing messages should be sent as normal
     * or pre-key messages.
     *
     * @return {@code true} if at least one message has been received,
     *         {@code false} otherwise
     * @throws IllegalStateException if this session has been closed
     */
    public boolean hasReceivedMessage() {
        checkNotClosed();
        return nativeHasReceivedMessage(nativePtr);
    }

    /**
     * Encrypts the plaintext and returns a JSON representation of an
     * OlmMessage.
     * <p>
     * The message will either be a pre-key message or a normal message,
     * depending on whether the session is fully established. A session is
     * fully established once you receive (and decrypt) at least one message
     * from the other side.
     *
     * @param plaintext the plaintext to encrypt
     * @return a JSON string representation of the OlmMessage
     * @throws IllegalStateException if this session has been closed
     */
    public String encrypt(byte[] plaintext) {
        checkNotClosed();
        return nativeEncrypt(nativePtr, plaintext);
    }

    /**
     * Decrypts an Olm message and returns the plaintext.
     *
     * @param message a JSON string representation of an OlmMessage
     * @return the decrypted plaintext bytes
     * @throws IllegalStateException   if this session has been closed
     * @throws DecryptionException    if decryption fails
     */
    public byte[] decrypt(String message) {
        checkNotClosed();
        return nativeDecrypt(nativePtr, message);
    }

    /**
     * Converts the session into a JSON string representation.
     *
     * @return a JSON string representing the session
     * @throws IllegalStateException if this session has been closed
     */
    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    /**
     * Encrypts the session using the given 32-byte key and returns the
     * encrypted representation.
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
     * Restores an {@code OlmSession} from a previously saved JSON string.
     *
     * @param pickleData the JSON string from {@link #pickle()}
     * @return a restored {@code OlmSession}
     * @throws PickleException if the data cannot be deserialized
     */
    public static OlmSession unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new OlmSession(nativePtr);
    }

    /**
     * Restores an {@code OlmSession} from an encrypted string using a
     * 32-byte key.
     *
     * @param pickleData the encrypted pickle data from {@link #pickle(byte[])}
     * @param key        a 256-bit (32-byte) key for decrypting the session
     * @return a restored {@code OlmSession}
     * @throws KeyException   if the key is not 32 bytes
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static OlmSession unpickle(String pickleData, byte[] key) {
        if (key.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
        long nativePtr = nativeEncryptedUnpickle(pickleData, key);
        return new OlmSession(nativePtr);
    }

    /**
     * Restores an {@code OlmSession} from a legacy libolm pickle format.
     * The pickle must be encrypted with the provided key.
     *
     * @param pickleData the libolm pickle data
     * @param pickleKey  the key used to encrypt the pickle data
     * @return a restored {@code OlmSession}
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static OlmSession unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new OlmSession(nativePtr);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("OlmSession has been closed");
        }
    }

    /* For test usage only */
    boolean isClosed() {
        return nativePtr == 0;
    }

    /**
     * Closes the {@code OlmSession} by releasing its associated native
     * resources.
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

    private native String nativeSessionId(long ptr);

    private native boolean nativeHasReceivedMessage(long ptr);

    private native String nativeEncrypt(long ptr, byte[] plaintext);

    private native byte[] nativeDecrypt(long ptr, String message);

    private native String nativePickle(long ptr);

    private native String nativeEncryptedPickle(long ptr, byte[] key);

    private static native long nativeUnpickle(String pickleData);

    private static native long nativeEncryptedUnpickle(String pickleData, byte[] key);

    private static native long nativeUnpickleLegacy(String pickleData, byte[] pickleKey);

    private native void nativeFree(long ptr);

}
