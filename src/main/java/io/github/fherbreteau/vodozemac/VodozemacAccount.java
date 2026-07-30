package io.github.fherbreteau.vodozemac;

public class VodozemacAccount implements AutoCloseable {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    private long nativePtr;

    public VodozemacAccount() {
        this.nativePtr = nativeNew();
    }

    public String curve25519Key() {
        checkNotClosed();
        return nativeCurve25519Key(nativePtr);
    }

    public String ed25519Key() {
        checkNotClosed();
        return nativeEd25519Key(nativePtr);
    }

    public String sign(String message) {
        checkNotClosed();
        return nativeSign(nativePtr, message);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("Account has been closed");
        }
    }

    @Override
    public void close() {
        if (nativePtr != 0) {
            nativeFree(nativePtr);
            nativePtr = 0;
        }
    }

    private native long nativeNew();
    private native String nativeCurve25519Key(long ptr);
    private native String nativeEd25519Key(long ptr);
    private native String nativeSign(long ptr, String message);
    private native void nativeFree(long ptr);
}