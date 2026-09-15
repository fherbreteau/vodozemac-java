package io.github.fherbreteau.vodozemac;

import static org.assertj.core.api.Assertions.assertThat;

import java.lang.ref.WeakReference;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.BooleanSupplier;
import java.util.logging.Handler;
import java.util.logging.Level;
import java.util.logging.LogRecord;

import io.github.fherbreteau.vodozemac.sas.EstablishedSas;
import io.github.fherbreteau.vodozemac.sas.Sas;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class NativeHandleTest {

    private static final AtomicInteger FREED_COUNT = new AtomicInteger();

    private static final Duration CLEANUP_TIMEOUT = Duration.ofSeconds(10);

    private final List<LogRecord> warningRecords = new ArrayList<>();

    private java.util.logging.Logger julLogger;

    private Handler handler;

    private Level previousLevel;

    @BeforeEach
    void captureWarningLogs() {
        julLogger = java.util.logging.Logger.getLogger(NativeHandle.class.getName());
        handler = new Handler() {
            @Override
            public void publish(LogRecord record) {
                if (record.getLevel() == Level.WARNING) {
                    synchronized (warningRecords) {
                        warningRecords.add(record);
                    }
                }
            }

            @Override
            public void flush() {
            }

            @Override
            public void close() {
            }
        };
        previousLevel = julLogger.getLevel();
        julLogger.addHandler(handler);
        julLogger.setLevel(Level.ALL);
    }

    @AfterEach
    void restoreWarningLogs() {
        julLogger.removeHandler(handler);
        julLogger.setLevel(previousLevel);
    }

    @Test
    void testCloseReleasesNativeResourceExactlyOnce() {
        TestNativeHandle handle = new TestNativeHandle();
        int freedBefore = FREED_COUNT.get();

        assertThat(handle.isClosed()).isFalse();

        handle.close();
        assertThat(handle.isClosed()).isTrue();
        assertThat(FREED_COUNT.get()).isEqualTo(freedBefore + 1);

        handle.close();
        assertThat(FREED_COUNT.get()).isEqualTo(freedBefore + 1);
    }

    @Test
    void testCleanerReleasesLeakedHandle() throws Exception {
        int freedBefore = FREED_COUNT.get();
        WeakReference<TestNativeHandle> leaked = newLeakedHandle();

        gcUntil(() -> FREED_COUNT.get() > freedBefore);

        assertThat(FREED_COUNT.get()).isEqualTo(freedBefore + 1);
        gcUntil(() -> {
            synchronized (warningRecords) {
                return warningRecords.stream()
                        .anyMatch(record -> record.getMessage().contains("TestNativeHandle"));
            }
        });
        synchronized (warningRecords) {
            assertThat(warningRecords)
                    .anyMatch(record -> record.getMessage().contains("became unreachable without being closed"));
        }
    }

    @Test
    void testConsumedHandleDoesNotTriggerCleanerRelease() throws Exception {
        Sas sas = new Sas();
        EstablishedSas established = sas.diffieHellman(sas.publicKey());
        assertThat(established).isNotNull();
        WeakReference<Sas> consumed = new WeakReference<>(sas);

        gcUntil(() -> consumed.get() == null);
        System.gc();
        Thread.sleep(200);

        synchronized (warningRecords) {
            assertThat(warningRecords)
                    .noneMatch(record -> record.getMessage().contains("Sas"));
        }
        established.close();
    }

    private static WeakReference<TestNativeHandle> newLeakedHandle() {
        return new WeakReference<>(new TestNativeHandle());
    }

    private static void gcUntil(BooleanSupplier condition) throws Exception {
        long deadline = System.nanoTime() + CLEANUP_TIMEOUT.toNanos();
        while (!condition.getAsBoolean() && System.nanoTime() < deadline) {
            System.gc();
            Thread.sleep(20);
        }
    }

    private static final class TestNativeHandle extends NativeHandle {

        private TestNativeHandle() {
            super(0x1234L, TestNativeHandle::release);
        }

        private static void release(long ptr) {
            FREED_COUNT.incrementAndGet();
        }
    }
}
