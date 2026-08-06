package io.github.fherbreteau.vodozemac.olm;

import io.github.fherbreteau.vodozemac.VodozemacException;
import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.account.OneTimeKeyGenerationResult;
import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

class OlmSessionTest {

    private final SecureRandom random = new SecureRandom();

    @Test
    void testFullSessionLifecycle() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            // Bob generates a one-time key
            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            assertThat(result.getCreated())
                    .as("Bob should generate one-time keys")
                    .isNotEmpty();
            String bobOneTimeKey = result.getCreated().iterator().next();
            bobAccount.markKeysAsPublished();

            // Alice creates an outbound session with Bob
            String bobCurve25519Key = bobAccount.curve25519Key();
            try (OlmSession outboundSession = aliceAccount.createOutbpundSession(
                    OlmSessionVersion.V2, bobCurve25519Key, bobOneTimeKey)) {

                assertThat(outboundSession.sessionId())
                        .as("Session ID should not be null")
                        .isNotNull()
                        .isNotEmpty();

                assertThat(outboundSession.hasReceivedMessage())
                        .as("Outbound session should not have received a message")
                        .isFalse();

                // Alice encrypts a message
                String plaintext = "Hello Bob";
                String encrypted = outboundSession.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));

                assertThat(encrypted)
                        .as("Encrypted message should be JSON with type and body")
                        .isNotNull()
                        .startsWith("{")
                        .endsWith("}");

                // Bob creates an inbound session from the pre-key message
                InboundCreationResult inboundResult = bobAccount.createInboundSession(
                        aliceAccount.curve25519Key(), encrypted);

                assertThat(inboundResult)
                        .as("Inbound creation result should not be null")
                        .isNotNull();

                assertThat(new String(inboundResult.getPlaintext(), StandardCharsets.UTF_8))
                        .as("Bob should decrypt the original message")
                        .isEqualTo(plaintext);

                try (OlmSession inboundSession = inboundResult.getSession()) {
                    assertThat(inboundSession.sessionId())
                            .as("Inbound and outbound session IDs should match")
                            .isEqualTo(outboundSession.sessionId());

                    assertThat(inboundSession.hasReceivedMessage())
                            .as("Inbound session should have received a message")
                            .isTrue();

                    // Bob encrypts a reply
                    String reply = "Hello Alice";
                    String encryptedReply = inboundSession.encrypt(reply.getBytes(StandardCharsets.UTF_8));

                    // Alice decrypts the reply
                    byte[] decryptedReply = outboundSession.decrypt(encryptedReply);
                    assertThat(new String(decryptedReply, StandardCharsets.UTF_8))
                            .as("Alice should decrypt Bob's reply")
                            .isEqualTo(reply);

                    assertThat(outboundSession.hasReceivedMessage())
                            .as("Outbound session should now have received a message")
                            .isTrue();
                }
            }
        }
    }

    @Test
    void testSessionPickleAndUnpickle() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            String bobOneTimeKey = result.getCreated().iterator().next();
            bobAccount.markKeysAsPublished();

            String pickleData;
            String originalSessionId;
            try (OlmSession session = aliceAccount.createOutbpundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey)) {
                originalSessionId = session.sessionId();
                pickleData = session.pickle();
            }

            assertThat(pickleData)
                    .as("Pickle data should be valid JSON")
                    .isNotNull()
                    .isNotEmpty()
                    .startsWith("{")
                    .endsWith("}");

            try (OlmSession unpickled = OlmSession.unpickle(pickleData)) {
                assertThat(unpickled.sessionId())
                        .as("Unpickled session should have the same session ID")
                        .isEqualTo(originalSessionId);
            }
        }
    }

    @Test
    void testSessionEncryptedPickleAndUnpickle() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            String bobOneTimeKey = result.getCreated().iterator().next();
            bobAccount.markKeysAsPublished();

            byte[] key = new byte[32];
            random.nextBytes(key);

            String pickleData;
            String originalSessionId;
            try (OlmSession session = aliceAccount.createOutbpundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey)) {
                originalSessionId = session.sessionId();
                pickleData = session.pickle(key);
            }

            assertThat(pickleData)
                    .as("Encrypted pickle data should not be empty")
                    .isNotNull()
                    .isNotEmpty();

            try (OlmSession unpickled = OlmSession.unpickle(pickleData, key)) {
                assertThat(unpickled.sessionId())
                        .as("Unpickled session should have the same session ID")
                        .isEqualTo(originalSessionId);
            }
        }
    }

    @Test
    void testPickleWithInvalidKeyThrowsException() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            String bobOneTimeKey = result.getCreated().iterator().next();
            bobAccount.markKeysAsPublished();

            try (OlmSession session = aliceAccount.createOutbpundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey)) {

                byte[] invalidKey = new byte[16];
                assertThatThrownBy(() -> session.pickle(invalidKey))
                        .as("Pickle with invalid key size should throw VodozemacException")
                        .isInstanceOf(VodozemacException.class)
                        .hasMessageContaining("256-bit (32-byte)");
            }
        }
    }

    @Test
    void testUnpickleWithInvalidKeyThrowsException() {
        byte[] invalidKey = new byte[16];
        assertThatThrownBy(() -> OlmSession.unpickle("invalid", invalidKey))
                .as("Unpickle with invalid key size should throw VodozemacException")
                .isInstanceOf(VodozemacException.class)
                .hasMessageContaining("256-bit (32-byte)");
    }

    @Test
    void testSessionCheckNotClosedThrowsAfterClose() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            String bobOneTimeKey = result.getCreated().iterator().next();
            bobAccount.markKeysAsPublished();

            OlmSession session = aliceAccount.createOutbpundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey);
            assertThat(session.isClosed()).isFalse();

            session.close();

            assertThat(session.isClosed()).isTrue();

            assertThatThrownBy(session::sessionId)
                    .as("Using closed session should throw IllegalStateException")
                    .isInstanceOf(IllegalStateException.class)
                    .hasMessage("Account has been closed");

            session.close();
        }
    }

    @Test
    void testSessionUnpickleLegacy() {
        String pickleData =
                "OTC2YvFEfkLiOCiVcLH6Jk6Akmak7QKT8E9s6YpAsP/WoijQFg+RRODQrnHmSBQp"
                + "DnG7CCD8/qOykPihrs8FCJABqCpXqMoel4awkKZ60/YBCiFt9kGnSTR2RUezupKO"
                + "3vVWTx6nciMl9B23n8Ru2+v1LJSMZ4OBSKbj7LIESKYRQN8LKhIIrYfW6x7hoH5J"
                + "8NvcmrKBe66422tMBf0fTYV4576WBW+68gWi3Rg+J56e1JIzovpJ5NJmMX7D3Igg"
                + "1+CDr64SCyV2WtVC0r+tNPv+VbsxrnhoD3/TYyPWpqGrlWYk6ntM0w";
        byte[] pickleKey = "DEFAULT_PICKLE_KEY".getBytes(StandardCharsets.UTF_8);

        try (OlmSession session = OlmSession.unpickleLegacy(pickleData, pickleKey)) {
            assertThat(session)
                    .as("Unpickled legacy session should be created")
                    .isNotNull();

            assertThat(session.sessionId())
                    .as("Legacy session should have a valid session ID")
                    .isNotNull()
                    .isNotEmpty();
        }
    }
}
