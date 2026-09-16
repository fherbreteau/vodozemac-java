package io.github.fherbreteau.vodozemac;

import static java.nio.charset.StandardCharsets.UTF_8;
import static org.assertj.core.api.Assertions.assertThat;

import java.lang.ref.WeakReference;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.logging.Handler;
import java.util.logging.Level;
import java.util.logging.LogRecord;

import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.account.OneTimeKeyGenerationResult;
import io.github.fherbreteau.vodozemac.backup.PkDecryption;
import io.github.fherbreteau.vodozemac.ecies.Ecies;
import io.github.fherbreteau.vodozemac.ecies.OutboundCreationResult;
import io.github.fherbreteau.vodozemac.megolm.InboundGroupSession;
import io.github.fherbreteau.vodozemac.megolm.OutboundGroupSession;
import io.github.fherbreteau.vodozemac.olm.OlmSessionVersion;
import io.github.fherbreteau.vodozemac.sas.EstablishedSas;
import io.github.fherbreteau.vodozemac.sas.Sas;
import io.github.fherbreteau.vodozemac.types.Curve25519PublicKey;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class NativeHandleTest {

    private final List<LogRecord> warningRecords = new ArrayList<>();

    private java.util.logging.Logger julLogger;

    private Handler handler;

    private Level previousLevel;

    @BeforeEach
    void captureWarningLogs() {
        julLogger = java.util.logging.Logger.getLogger(NativeHandle.class.getName());
        handler = new Handler() {
            @Override
            public void publish(LogRecord logRecord) {
                if (logRecord.getLevel() == Level.WARNING) {
                    synchronized (warningRecords) {
                        warningRecords.add(logRecord);
                    }
                }
            }

            @Override
            public void flush() {
                // No-op: records are kept in memory for assertions.
            }

            @Override
            public void close() {
                // No-op: there is no backing stream to close.
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
    void testAccountIsClosed() {
        NativeHandle handle = new Account();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testOlmSessionIsClosed() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {
            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            Curve25519PublicKey bobOneTimeKey = result.created().iterator().next();
            bobAccount.markKeysAsPublished();

            NativeHandle handle = aliceAccount.createOutboundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey);
            assertThat(handle.isClosed()).isFalse();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
        }
    }

    @Test
    void testOutboundGroupSessionIsClosed() {
        NativeHandle handle = new OutboundGroupSession();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testInboundGroupSessionIsClosed() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession()) {
            sessionKey = outbound.sessionKey();
        }

        NativeHandle handle = new InboundGroupSession(sessionKey);
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testSasIsClosed() {
        NativeHandle handle = new Sas();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testEstablishedSasIsClosed() {
        try (Sas aliceSas = new Sas(); Sas bobSas = new Sas()) {
            NativeHandle handle = aliceSas.diffieHellman(bobSas.publicKey());
            assertThat(handle.isClosed()).isFalse();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
        }
    }

    @Test
    void testEciesIsClosed() {
        NativeHandle handle = new Ecies();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testEstablishedEciesIsClosed() {
        Ecies alice = new Ecies();
        Ecies bob = new Ecies();
        OutboundCreationResult result = alice.establishOutboundChannel(
                bob.publicKey(), "plaintext".getBytes(UTF_8));
        NativeHandle handle = result.establishedEcies();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        alice.close();
        bob.close();
    }

    @Test
    void testPkDecryptionIsClosed() {
        NativeHandle handle = new PkDecryption();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testNativeLibraryIsLoaded() {
        assertThat(NativeLibraryLoader.isLoaded())
                .as("Native Library is Loaded")
                .isTrue();
    }

    @Test
    void testCleanerReleasesLeakedHandle() throws Exception {
        CountDownLatch freed = new CountDownLatch(1);
        newLeakedHandle(freed);

        for (int i = 0; i < 3 && !freed.await(1, TimeUnit.SECONDS); i++) {
            System.gc();
        }

        assertThat(freed.getCount())
                .as("The Cleaner should have released the leaked handle")
                .isZero();

        synchronized (warningRecords) {
            assertThat(warningRecords)
                    .as("The Cleaner should have logged a warning for the leaked handle")
                    .anyMatch(warning -> warning.getMessage().contains("became unreachable without being closed")
                            && warning.getParameters() != null
                            && warning.getParameters().length == 1
                            && "TestNativeHandle".equals(warning.getParameters()[0]));
        }
    }

    @Test
    void testConsumedHandleDoesNotTriggerCleanerRelease() throws Exception {
        ConsumedSas consumedSas = newConsumedSas();
        WeakReference<Sas> consumed = consumedSas.reference();

        for (int i = 0; i < 3 && consumed.get() != null; i++) {
            System.gc();
        }
        assertThat(consumed.get())
                .as("The consumed handle should be garbage collected")
                .isNull();

        CountDownLatch sentinel = new CountDownLatch(1);
        newLeakedHandle(sentinel);
        for (int i = 0; i < 3 && !sentinel.await(1, TimeUnit.SECONDS); i++) {
            System.gc();
        }
        assertThat(sentinel.getCount())
                .as("The Cleaner thread should have processed the sentinel handle")
                .isZero();

        synchronized (warningRecords) {
            assertThat(warningRecords)
                    .as("The Cleaner must not release the pointer of a consumed handle")
                    .noneMatch(warning -> warning.getMessage().contains("became unreachable without being closed")
                            && "Sas".equals(warning.getParameters()[0]));
        }
        consumedSas.established().close();
    }

    private static void newLeakedHandle(CountDownLatch freed) {
        new TestNativeHandle(freed);
    }

    private record ConsumedSas(WeakReference<Sas> reference, EstablishedSas established) {
    }

    private static ConsumedSas newConsumedSas() {
        Sas sas = new Sas();
        EstablishedSas established = sas.diffieHellman(sas.publicKey());
        return new ConsumedSas(new WeakReference<>(sas), established);
    }

    private static final class TestNativeHandle extends NativeHandle {

        private TestNativeHandle(CountDownLatch freed) {
            super(0x1234L, ptr -> freed.countDown());
        }
    }
}
