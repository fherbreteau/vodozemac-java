package io.github.fherbreteau.vodozemac;

import java.lang.System.Logger.Level;
import java.lang.ref.Cleaner;
import java.util.Objects;
import java.util.function.LongConsumer;

/**
 * Abstract base class for all objects backed by a native handle (raw pointer).
 * <p>
 * Each subclass holds a native pointer to a Rust object allocated on the heap.
 * The pointer is released when {@link #close()} is called, which delegates to
 * the subclass-specific native free function supplied to the constructor.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 * The {@link #close()} method is idempotent — calling it more than once has no
 * effect. Any method that accesses the native pointer after {@code close()} has
 * been called will throw an {@link IllegalStateException}.
 * <p>
 * As a best-effort safety net, an instance that becomes garbage without having
 * been closed is cleaned up by a {@link Cleaner}: its native memory is released
 * and a warning is logged. The safety net is not deterministic and should never
 * be relied upon — always close handles explicitly.
 * <p>
 * Instances are safe to use from multiple threads: every method that accesses
 * the native handle is synchronized on the instance, so native calls on the
 * same object are serialized and {@code close()} can never race with an
 * ongoing native call. Constructors and static factory methods that only
 * create fresh native objects require no synchronization.
 *
 * @author François HERBRETEAU
 */
public abstract class NativeHandle implements AutoCloseable {

    private static final Cleaner CLEANER = Cleaner.create();

    private static final System.Logger LOGGER = System.getLogger(NativeHandle.class.getName());

    private final State state;
    private final LongConsumer nativeFreer;
    private final Cleaner.Cleanable cleanable;

    protected NativeHandle(long ptr, LongConsumer nativeFreer) {
        this.nativeFreer = Objects.requireNonNull(nativeFreer, "nativeFreer");
        this.state = new State(ptr);
        this.cleanable = CLEANER.register(this, new LeakGuard(state, nativeFreer, getClass().getSimpleName()));
    }

    protected final long nativePtr() {
        return state.peek();
    }

    protected final void checkNotClosed() {
        if (state.isTaken()) {
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
    final synchronized boolean isClosed() {
        return state.isTaken();
    }

    /**
     * Closes this resource by releasing its associated native resources.
     * <p>
     * This method is idempotent: calling it more than once has no effect.
     *
     * {@inheritDoc}
     */
    @Override
    public final synchronized void close() {
        long ptr = state.take();
        if (ptr != 0) {
            cleanable.clean();
            nativeFreer.accept(ptr);
        }
    }

    /**
     * Invalidates this handle without releasing its native resources,
     * because ownership of the native object has been transferred elsewhere.
     * <p>
     * Subclasses whose native methods consume the native object and hand
     * its ownership to a new Java object (e.g. a {@code Sas} consumed by
     * {@code diffieHellman}) must call this method instead of zeroing
     * {@link #nativePtr} directly, so that the {@link Cleaner} safety net
     * does not release the already-transferred pointer a second time.
     * After this call the handle behaves as if it had been closed.
     */
    protected final void invalidate() {
        state.take();
        cleanable.clean();
    }

    /**
     * Shared holder of the native pointer, guarded by its own monitor so that
     * {@link #close()} and the {@link Cleaner} action can never release the
     * same pointer twice.
     */
    private static final class State {

        private volatile long ptr;

        State(long ptr) {
            this.ptr = ptr;
        }

        synchronized long take() {
            long held = this.ptr;
            this.ptr = 0;
            return held;
        }

        boolean isTaken() {
            return ptr == 0;
        }

        long peek() {
            return ptr;
        }
    }

    private static final class LeakGuard implements Runnable {

        private final State state;
        private final LongConsumer nativeFreer;
        private final String typeName;

        LeakGuard(State state, LongConsumer nativeFreer, String typeName) {
            this.state = state;
            this.nativeFreer = nativeFreer;
            this.typeName = typeName;
        }

        @Override
        public void run() {
            long ptr = state.take();
            if (ptr != 0) {
                LOGGER.log(Level.WARNING,
                        "{0} became unreachable without being closed; its native resources have been released by the Cleaner",
                        typeName);
                nativeFreer.accept(ptr);
            }
        }
    }
}
