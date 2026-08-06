package io.github.fherbreteau.vodozemac.megolm;

public class OutboundGroupSession implements AutoCloseable {

    private long nativePtr;

    public OutboundGroupSession(MegolmSessionVersion version) {
        nativePtr = nativeNew(version.getValue());
    }

    private OutboundGroupSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    public String sessionId() {
        checkNotClosed();
        return nativeSessionId(nativePtr);
    }

    public int messageIndex() {
        checkNotClosed();
        return nativeMessageIndex(nativePtr);
    }

    public String sessionKey() {
        checkNotClosed();
        return nativeSessionKey(nativePtr);
    }

    public String encrypt(byte[] plainText) {
        checkNotClosed();
        return nativeEncrypt(nativePtr, plainText);
    }

    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    public String pickle(byte[] key) {
        checkNotClosed();
        return nativeEncryptedPickle(nativePtr, key);
    }

    public static OutboundGroupSession unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new OutboundGroupSession(nativePtr);
    }

    public static OutboundGroupSession unpickle(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeEncryptedUnpickle(pickleData, pickleKey);
        return new OutboundGroupSession(nativePtr);
    }

    public static OutboundGroupSession unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new OutboundGroupSession(nativePtr);
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
