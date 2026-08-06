package io.github.fherbreteau.vodozemac.olm;

import io.github.fherbreteau.vodozemac.VodozemacException;

public class OlmSession implements AutoCloseable {
    private long nativePtr;

    public OlmSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    public String sessionId() {
        checkNotClosed();
        return nativeSessionId(nativePtr);
    }

    public boolean hasReceivedMessage() {
        checkNotClosed();
        return nativeHasReceivedMessage(nativePtr);
    }

    public String encrypt(byte[] plaintext) {
        checkNotClosed();
        return nativeEncrypt(nativePtr, plaintext);
    }

    public byte[] decrypt(String message) {
        checkNotClosed();
        return nativeDecrypt(nativePtr, message);
    }

    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    public String pickle(byte[] key) {
        checkNotClosed();
        if (key.length != 32) {
            throw new VodozemacException("Encrypted Key must be 256-bit (32-byte)");
        }
        return nativeEncryptedPickle(nativePtr, key);
    }

    public static OlmSession unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new OlmSession(nativePtr);
    }

    public static OlmSession unpickle(String pickleData, byte[] key) {
        if (key.length != 32) {
            throw new VodozemacException("Encrypted Key must be 256-bit (32-byte)");
        }
        long nativePtr = nativeEncryptedUnpickle(pickleData, key);
        return new OlmSession(nativePtr);
    }

    public static OlmSession unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new OlmSession(nativePtr);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("Account has been closed");
        }
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
