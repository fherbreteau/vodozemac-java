package io.github.fherbreteau.vodozemac.megolm;

public class InboundGroupSession implements AutoCloseable {

    private long nativePtr;

    public InboundGroupSession(String sessionKey, MegolmSessionVersion version) {
        nativePtr = nativeNew(sessionKey, version.getValue());
    }

    private InboundGroupSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    public String sessionId() {
        checkNotClosed();
        return nativeSessionId(nativePtr);
    }

    public int firstKnownIndex() {
        checkNotClosed();
        return nativeFirstKnownIndex(nativePtr);
    }

    public DecryptedMessage decrypt(String message) {
        checkNotClosed();
        return nativeDecrypt(nativePtr, message);
    }

    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    public String pickle(byte[] key) {
        checkNotClosed();
        return nativeEncryptedPickle(nativePtr, key);
    }

    public static InboundGroupSession unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new InboundGroupSession(nativePtr);
    }

    public static InboundGroupSession unpickle(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeEncryptedUnpickle(pickleData, pickleKey);
        return new InboundGroupSession(nativePtr);
    }

    public static InboundGroupSession unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new InboundGroupSession(nativePtr);
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
     * Close the {@code Account} by releasing its associated native resources
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

    private native long nativeNew(String sessionKey, int version);

    private native String nativeSessionId(long ptr);

    private native int nativeFirstKnownIndex(long ptr);

    private native DecryptedMessage nativeDecrypt(long ptr, String message);

    private native String nativePickle(long ptr);

    private native String nativeEncryptedPickle(long ptr, byte[] key);

    private static native long nativeUnpickle(String pickleData);

    private static native long nativeEncryptedUnpickle(String pickleData, byte[] key);

    private static native long nativeUnpickleLegacy(String pickleData, byte[] pickleKey);

    private native void nativeFree(long ptr);
}
