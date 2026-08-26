package io.github.fherbreteau.vodozemac.ecies;

import static java.nio.charset.StandardCharsets.UTF_8;
import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatCode;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import io.github.fherbreteau.vodozemac.exception.EciesException;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import org.junit.jupiter.api.Test;

class EciesTest {

    private static final byte[] PLAINTEXT = "It's a secret to everybody".getBytes(UTF_8);

    @Test
    void testEciesCreation() {
        Ecies copy;
        try (Ecies ecies = new Ecies()) {
            String publicKey = ecies.publicKey();
            assertThat(publicKey).isNotNull().isNotEmpty();
            copy = ecies;
        }
        assertThatThrownBy(copy::publicKey)
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("Ecies has been closed");
        copy.close();
    }

    @Test
    void testEciesChannelEstablishment() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            String bobPublicKey = bob.publicKey();

            OutboundCreationResult aliceResult = alice.establishOutboundChannel(bobPublicKey, PLAINTEXT);
            assertThat(aliceResult).isNotNull();
            assertThat(aliceResult.initialMessage()).isNotNull().isNotEmpty();

            InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.initialMessage());
            assertThat(bobResult).isNotNull();
            assertThat(bobResult.plaintext()).isEqualTo(PLAINTEXT);

            EstablishedEcies aliceEcies = aliceResult.establishedEcies();
            EstablishedEcies bobEcies = bobResult.establishedEcies();
            assertThat(aliceEcies).isNotNull();
            assertThat(bobEcies).isNotNull();
        }
    }

    @Test
    void testEstablishedEciesEncryptDecrypt() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            try (OutboundCreationResult aliceResult = alice.establishOutboundChannel(bob.publicKey(), PLAINTEXT);
                InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.initialMessage())) {

                EstablishedEcies aliceEcies = aliceResult.establishedEcies();
                EstablishedEcies bobEcies = bobResult.establishedEcies();

                byte[] message = "Hello from Bob".getBytes(UTF_8);
                String encrypted = bobEcies.encrypt(message);
                assertThat(encrypted).isNotNull().isNotEmpty();

                byte[] decrypted = aliceEcies.decrypt(encrypted);
                assertThat(decrypted).isEqualTo(message);

                byte[] reply = "Hello from Alice".getBytes(UTF_8);
                String encryptedReply = aliceEcies.encrypt(reply);
                byte[] decryptedReply = bobEcies.decrypt(encryptedReply);
                assertThat(decryptedReply).isEqualTo(reply);
            }
        }
    }

    @Test
    void testCheckCode() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            try (OutboundCreationResult aliceResult = alice.establishOutboundChannel(bob.publicKey(), PLAINTEXT);
                InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.initialMessage())) {

                EstablishedEcies aliceEcies = aliceResult.establishedEcies();
                EstablishedEcies bobEcies = bobResult.establishedEcies();

                CheckCode aliceCode = aliceEcies.checkCode();
                CheckCode bobCode = bobEcies.checkCode();

                assertThat(aliceCode).isNotNull();
                assertThat(bobCode).isNotNull();
                assertThat(aliceCode).isEqualTo(bobCode);
                assertThat(aliceCode.asBytes()).isEqualTo(bobCode.asBytes());
                assertThat(aliceCode.toDigit()).isEqualTo(bobCode.toDigit());
                assertThat(aliceCode.toDigit()).isNotNegative();
            }
        }
    }

    @Test
    void testEciesWithInfo() {
        try (Ecies alice = Ecies.withInfo("CUSTOM_INFO_PREFIX"); Ecies bob = Ecies.withInfo("CUSTOM_INFO_PREFIX")) {
            assertThat(alice.publicKey()).isNotNull().isNotEmpty();
            assertThat(bob.publicKey()).isNotNull().isNotEmpty();

            try (OutboundCreationResult aliceResult = alice.establishOutboundChannel(bob.publicKey(), PLAINTEXT);
                InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.initialMessage())) {
                assertThat(bobResult.plaintext()).isEqualTo(PLAINTEXT);

                EstablishedEcies aliceEcies = aliceResult.establishedEcies();
                EstablishedEcies bobEcies = bobResult.establishedEcies();

                CheckCode aliceCode = aliceEcies.checkCode();
                CheckCode bobCode = bobEcies.checkCode();
                assertThat(aliceCode).isEqualTo(bobCode);

                byte[] message = "Custom info test".getBytes(UTF_8);
                String encrypted = aliceEcies.encrypt(message);
                byte[] decrypted = bobEcies.decrypt(encrypted);
                assertThat(decrypted).isEqualTo(message);
            }
        }
    }

    @Test
    void testEstablishedEciesPublicKey() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            String bobPublicKey = bob.publicKey();

            try (OutboundCreationResult aliceResult = alice.establishOutboundChannel(bobPublicKey, PLAINTEXT);
                InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.initialMessage())) {

                EstablishedEcies aliceEcies = aliceResult.establishedEcies();
                EstablishedEcies bobEcies = bobResult.establishedEcies();

                assertThat(aliceEcies.publicKey()).isNotNull().isNotEmpty();
                assertThat(bobEcies.publicKey()).isNotNull().isNotEmpty().isEqualTo(bobPublicKey);
            }
        }
    }

    @Test
    void testEciesCannotBeReusedAfterEstablishment() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            String bobPublicKey = bob.publicKey();

            alice.establishOutboundChannel(bobPublicKey, PLAINTEXT);
            assertThatThrownBy(() -> alice.establishOutboundChannel(bobPublicKey, PLAINTEXT))
                    .isInstanceOf(IllegalStateException.class)
                    .hasMessage("Ecies has been closed");
            assertThatThrownBy(alice::publicKey)
                    .isInstanceOf(IllegalStateException.class)
                    .hasMessage("Ecies has been closed");
        }
    }

    @Test
    void testResultGettersReturnCorrectValues() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            OutboundCreationResult result = alice.establishOutboundChannel(bob.publicKey(), PLAINTEXT);
            assertThat(result.initialMessage()).isNotNull().isNotEmpty();
            assertThat(result.establishedEcies()).isNotNull();

            try (InboundCreationResult inboundResult = bob.establishInboundChannel(result.initialMessage())) {
                assertThat(inboundResult.plaintext()).isEqualTo(PLAINTEXT);
                assertThat(inboundResult.establishedEcies()).isNotNull();
            }
        }
    }

    @Test
    void testCheckCodeEqualsAndHashCode() {
        CheckCode code = new CheckCode(new byte[]{1, 2, 3}, 42);

        assertThat(code)
                .isNotEqualTo(new Object())
                .doesNotHaveSameHashCodeAs(new Object());

        CheckCode same = new CheckCode(new byte[]{1, 2, 3}, 42);
        assertThat(code).isEqualTo(same).hasSameHashCodeAs(same)
                .isNotEqualTo(new CheckCode(new byte[]{4, 5, 6}, 42))
                .isNotEqualTo(new CheckCode(new byte[]{1, 2, 3}, 99))
                .isNotEqualTo("not a CheckCode");
        assertThat(code.equals(null)).isFalse();
        assertThat(code).isEqualTo(code);

        assertThat(code.asBytes()).isEqualTo(new byte[]{1, 2, 3});
        assertThat(code.toDigit()).isEqualTo(42);
    }

    @Test
    void testEciesExceptionConstructor() {
        EciesException exception = new EciesException("test error");
        assertThat(exception).isNotNull();
        assertThat(exception.getMessage()).isEqualTo("test error");
    }

    @Test
    void testEstablishedEciesClose() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            try (OutboundCreationResult aliceResult = alice.establishOutboundChannel(bob.publicKey(), PLAINTEXT);
                InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.initialMessage())) {

                EstablishedEcies aliceEcies = aliceResult.establishedEcies();
                EstablishedEcies bobEcies = bobResult.establishedEcies();

                aliceEcies.close();

                assertThatCode(aliceEcies::publicKey)
                    .isInstanceOf(IllegalStateException.class)
                    .hasMessage("EstablishedEcies has been closed");

                aliceEcies.close(); // EstablishedEcies closure must be indempotent

                bobEcies.close();
            }
        }
    }

    @Test
    void testEstablishedEciesUseAfterClose() {
        try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
            OutboundCreationResult aliceResult = alice.establishOutboundChannel(bob.publicKey(), PLAINTEXT);
            bob.establishInboundChannel(aliceResult.initialMessage());

            EstablishedEcies aliceEcies = aliceResult.establishedEcies();
            aliceEcies.close();
            assertThatThrownBy(aliceEcies::publicKey)
                    .isInstanceOf(IllegalStateException.class)
                    .hasMessage("EstablishedEcies has been closed");
        }
    }

    @Test
    void testInboundCreationResultEqualsHashCodeToString() {
        InboundCreationResult result = new InboundCreationResult(0L, PLAINTEXT);
        InboundCreationResult same = new InboundCreationResult(0L, PLAINTEXT);
        InboundCreationResult different = new InboundCreationResult(0L, "different".getBytes(UTF_8));

        assertThat(result).isEqualTo(result)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo("not a result")
                .isNotEqualTo(null);
        assertThat(result.toString()).contains("plaintext");
    }

    @Test
    void testEciesClosedAfterFailedOutboundChannel() {
        Ecies ecies = new Ecies();
        assertThatThrownBy(() -> ecies.establishOutboundChannel("invalid-key", PLAINTEXT))
                .isInstanceOf(KeyException.class);
        assertThatThrownBy(ecies::publicKey)
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("Ecies has been closed");
        ecies.close();
    }

    @Test
    void testEciesClosedAfterFailedInboundChannel() {
        Ecies ecies = new Ecies();
        assertThatThrownBy(() -> ecies.establishInboundChannel("malformed-message"))
                .isInstanceOf(EciesException.class);
        assertThatThrownBy(ecies::publicKey)
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("Ecies has been closed");
        ecies.close();
    }

    @Test
    void testOutboundCreationResultEqualsHashCodeToString() {
        OutboundCreationResult result = new OutboundCreationResult(0L, "msg");
        OutboundCreationResult same = new OutboundCreationResult(0L, "msg");
        OutboundCreationResult different = new OutboundCreationResult(0L, "other");

        assertThat(result).isEqualTo(result)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo("not a result")
                .isNotEqualTo(null);
        assertThat(result.toString()).contains("initialMessage");
    }
}
