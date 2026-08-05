package io.github.fherbreteau.vodozemac.olm;

public class OlmSession implements AutoCloseable {
    private long nativePtr;

    public OlmSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }



    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("Account has been closed");
        }
    }

    @Override
    public void close() throws Exception {
        if (nativePtr != 0) {
            nativeFree(nativePtr);
            nativePtr = 0;
        }
    }


    private native void nativeFree(long ptr);

}
