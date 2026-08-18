package io.github.fherbreteau.vodozemac;

/**
 * Abstract base class for all objects backed by a native handle (raw pointer).
 * <p>
 * Each subclass holds a native pointer to a Rust object allocated on the heap.
 * The pointer is released when {@link #close()} is called, which delegates to
 * the subclass-specific {@link #nativeFree(long)} implementation.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 * The {@link #close()} method is idempotent — calling it more than once has no
 * effect. Any method that accesses the native pointer after {@code close()} has
 * been called will throw an {@link IllegalStateException}.
 *
 * @author François HERBRETEAU
 */
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

    /**
     * Indicates whether this native handle has been closed and its native
     * resource released.
     *
     * @return {@code true} if the native resource has been released,
     *         {@code false} otherwise
     */
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
