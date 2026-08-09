package io.github.fherbreteau.vodozemac.olm;

import io.github.fherbreteau.vodozemac.KeyException;

public class OlmSession implements AutoCloseable {
    private long nativePtr;

    public OlmSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    /**
     * Returns the globally unique session ID, in base64-encoded form.
     *
     * @return a base64 string
     */
    public String sessionId() {
        checkNotClosed();
        return nativeSessionId(nativePtr);
    }

    /**
     * Have we ever received and decrypted a message from the other side?
     *
     * @return outgoing messages should be sent as normal or pre-key messages
     */
    public boolean hasReceivedMessage() {
        checkNotClosed();
        return nativeHasReceivedMessage(nativePtr);
    }

    /**
     * Encrypt the plaintext and construct a JSON representation of an OlmMessage.
     *
     * @param plaintext the plaintext to encrypt
     * @return a JSON representation of an OlmMessage
     */
    public String encrypt(byte[] plaintext) {
        checkNotClosed();
        return nativeEncrypt(nativePtr, plaintext);
    }

    /**
     * Try to decrypt an Olm message, which will either return the plaintext.
     *
     * @param message a JSON representation of an OlmMessage
     * @return the plaintext decrypted
     */
    public byte[] decrypt(String message) {
        checkNotClosed();
        return nativeDecrypt(nativePtr, message);
    }

    /**
     * Convert the session into a JSON representation.
     *
     * @return a JSON representation of an {@code OlmSession}
     */
    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    /**
     * Encrypt a session using a 32-byte key.
     *
     * @return an encrypted string of an {@code OlmSession}
     */
    public String pickle(byte[] key) {
        checkNotClosed();
        if (key.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
        return nativeEncryptedPickle(nativePtr, key);
    }

    /**
     * Restore an {@code OlmSession} from a previously saved.
     *
     * @param pickleData a JSON representation of an {@code OlmSession}
     * @return an {@code OlmSession}
     */
    public static OlmSession unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new OlmSession(nativePtr);
    }

    /**
     * Create a {@code OlmSession} object by unpickling a session pickle encrypted with 32-byte key.
     *
     * @param pickleData the pickle data
     * @param key a 256-bit (32-byte) key for encrypting the device.
     * @return an {@code OlmSession} object
     */
    public static OlmSession unpickle(String pickleData, byte[] key) {
        if (key.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
        long nativePtr = nativeEncryptedUnpickle(pickleData, key);
        return new OlmSession(nativePtr);
    }

    /**
     * Create a {@code OlmSession} object by unpickling a session pickle in libolm
     * legacy pickle format.
     *
     * @param pickleData the libolm pickle data
     * @param pickleKey  the key used to encrypt the pickle data
     * @return an {@code OlmSession} object
     */
    public static OlmSession unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new OlmSession(nativePtr);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("Account has been closed");
        }
    }

    /* For test usage only */
    boolean isClosed() {
        return nativePtr == 0;
    }

    /**
     * Close the {@code OlmSession} by releasing its associated native resources
     *
     * {@InheritDoc}
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
