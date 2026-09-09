package io.github.fherbreteau.vodozemac;

import static java.nio.charset.StandardCharsets.UTF_8;
import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.BiConsumer;

import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.megolm.DecryptedMessage;
import io.github.fherbreteau.vodozemac.megolm.InboundGroupSession;
import io.github.fherbreteau.vodozemac.megolm.MegolmMessage;
import io.github.fherbreteau.vodozemac.megolm.OutboundGroupSession;
import org.junit.jupiter.api.Test;

class ConcurrencyTest {

    private static final int THREADS = 8;
    private static final int ITERATIONS = 25;

    @Test
    void sharedAccountSupportsConcurrentOperations() throws Exception {
        try (Account account = new Account()) {
            List<Throwable> failures = runConcurrently(account, (shared, threadIndex) -> {
                for (int i = 0; i < ITERATIONS; i++) {
                    byte[] message = ("message-" + threadIndex + "-" + i).getBytes(UTF_8);
                    assertThat(shared.sign(message)).isNotNull();
                    assertThat(shared.pickle()).isNotBlank();
                }
            });
            assertThat(failures).isEmpty();
        }
    }

    @Test
    void sharedOutboundGroupSessionSupportsConcurrentEncryption() throws Exception {
        try (OutboundGroupSession outbound = new OutboundGroupSession();
                InboundGroupSession inbound = new InboundGroupSession(outbound.sessionKey())) {
            ConcurrentLinkedQueue<MegolmMessage> messages = new ConcurrentLinkedQueue<>();
            List<Throwable> failures = runConcurrently(outbound, (shared, threadIndex) -> {
                for (int i = 0; i < ITERATIONS; i++) {
                    byte[] plaintext = ("secret-" + threadIndex + "-" + i).getBytes(UTF_8);
                    messages.add(shared.encrypt(plaintext));
                }
            });
            assertThat(failures).isEmpty();
            assertThat(messages).hasSize(THREADS * ITERATIONS);

            Set<Integer> indexes = new HashSet<>();
            for (MegolmMessage message : messages) {
                DecryptedMessage decrypted = inbound.decrypt(message);
                assertThat(new String(decrypted.plaintext(), UTF_8)).startsWith("secret-");
                indexes.add(decrypted.messageIndex());
            }
            assertThat(indexes).hasSize(THREADS * ITERATIONS);
        }
    }

    @Test
    void concurrentCloseIsIdempotent() throws Exception {
        Account account = new Account();
        List<Throwable> failures = runConcurrently(account, (shared, threadIndex) -> shared.close());
        assertThat(failures).isEmpty();
        assertThatThrownBy(() -> account.sign("message".getBytes(UTF_8)))
                .isInstanceOf(IllegalStateException.class);
        account.close();
        assertThatThrownBy(() -> account.sign("message".getBytes(UTF_8)))
                .isInstanceOf(IllegalStateException.class);
    }

    @Test
    void closeWhileInUseDoesNotCrashTheJvm() throws Exception {
        Account account = new Account();
        AtomicInteger successes = new AtomicInteger();
        AtomicInteger rejections = new AtomicInteger();
        CountDownLatch start = new CountDownLatch(1);
        ExecutorService executor = Executors.newFixedThreadPool(THREADS + 1);
        List<Future<?>> futures = new ArrayList<>();
        futures.add(executor.submit(() -> {
            start.await();
            account.close();
            return null;
        }));
        for (int threadIndex = 0; threadIndex < THREADS; threadIndex++) {
            futures.add(executor.submit(() -> {
                start.await();
                for (int i = 0; i < ITERATIONS * 20; i++) {
                    try {
                        account.sign(("message-" + i).getBytes(UTF_8));
                        successes.incrementAndGet();
                    } catch (IllegalStateException e) {
                        rejections.incrementAndGet();
                    }
                }
                return null;
            }));
        }
        start.countDown();
        List<Throwable> failures = new ArrayList<>();
        for (Future<?> future : futures) {
            try {
                future.get(60, TimeUnit.SECONDS);
            } catch (ExecutionException e) {
                failures.add(e.getCause());
            }
        }
        executor.shutdownNow();
        assertThat(failures).isEmpty();
        assertThat(rejections.get()).isPositive();
        assertThatThrownBy(() -> account.sign("message".getBytes(UTF_8)))
                .isInstanceOf(IllegalStateException.class);
        account.close();
        assertThat(successes.get() + rejections.get()).isEqualTo(THREADS * ITERATIONS * 20);
    }

    private static <T> List<Throwable> runConcurrently(T shared, BiConsumer<T, Integer> action) throws Exception {
        ExecutorService executor = Executors.newFixedThreadPool(THREADS);
        CountDownLatch start = new CountDownLatch(1);
        List<Future<?>> futures = new ArrayList<>();
        for (int threadIndex = 0; threadIndex < THREADS; threadIndex++) {
            int index = threadIndex;
            futures.add(executor.submit(() -> {
                start.await();
                action.accept(shared, index);
                return null;
            }));
        }
        start.countDown();
        List<Throwable> failures = new ArrayList<>();
        for (Future<?> future : futures) {
            try {
                future.get(60, TimeUnit.SECONDS);
            } catch (ExecutionException e) {
                failures.add(e.getCause());
            }
        }
        executor.shutdownNow();
        return failures;
    }
}
