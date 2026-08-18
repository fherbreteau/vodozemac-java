package io.github.fherbreteau.vodozemac;

public abstract class NativeHandle implements AutoCloseable {

    protected long nativePtr;

    protected NativeHandle(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    protected final void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException(getClass().getSimpleName() + " has been closed");
        }
    }

    final boolean isClosed() {
        return nativePtr == 0;
    }

    /**
     * Closes this resource by releasing its associated native resources.
     * <p>
     * This method is idempotent: calling it more than once has no effect.
     *
     * {@inheritDoc}
     */
    @Override
    public final void close() {
        if (nativePtr != 0) {
            nativeFree(nativePtr);
            nativePtr = 0;
        }
    }

    protected abstract void nativeFree(long ptr);
}
