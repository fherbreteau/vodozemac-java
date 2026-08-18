package io.github.fherbreteau.vodozemac.olm;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import static org.assertj.core.api.InstanceOfAssertFactories.STRING;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;

import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.account.OneTimeKeyGenerationResult;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.SessionCreationException;
import io.github.fherbreteau.vodozemac.exception.VodozemacException;
import org.junit.jupiter.api.Test;

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
            try (OlmSession outboundSession = aliceAccount.createOutboundSession(
                    OlmSessionVersion.V2, bobCurve25519Key, bobOneTimeKey)) {

                String sessionId = outboundSession.sessionId();
                assertThat(sessionId)
                        .as("Session ID should not be null")
                        .isNotNull()
                        .isNotEmpty();

                SessionKeys sessionKeys = outboundSession.sessionKeys();
                assertThat(sessionKeys)
                        .extracting(SessionKeys::sessionId)
                        .isEqualTo(sessionId);
                assertThat(sessionKeys)
                        .extracting(SessionKeys::identityKey)
                        .isEqualTo(aliceAccount.curve25519Key());
                assertThat(sessionKeys)
                        .extracting(SessionKeys::oneTimeKey)
                        .isEqualTo(bobOneTimeKey);
                assertThat(sessionKeys)
                        .extracting(SessionKeys::baseKey, STRING)
                        .isNotEmpty();

                assertThat(outboundSession.sessionConfig())
                        .isEqualTo(OlmSessionVersion.V2);

                assertThat(outboundSession.hasReceivedMessage())
                        .as("Outbound session should not have received a message")
                        .isFalse();

                // Alice encrypts a message
                String plaintext = "Hello Bob";
                OlmMessage encrypted = outboundSession.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));

                assertThat(encrypted)
                        .as("Encrypted message should be a Pre-Key Message")
                        .isNotNull()
                        .extracting(OlmMessage::getType)
                        .isEqualTo(MessageType.PRE_KEY);
                assertThat(encrypted)
                        .as("Encrypted message' body should not be null nor empty")
                        .extracting(OlmMessage::getBody, STRING)
                        .isNotEmpty()
                        .isBase64();

                // Bob creates an inbound session from the pre-key message
                InboundCreationResult inboundResult = bobAccount.createInboundSession(
                        OlmSessionVersion.V2, aliceAccount.curve25519Key(), encrypted);

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
                    OlmMessage encryptedReply = inboundSession.encrypt(reply.getBytes(StandardCharsets.UTF_8));
                    assertThat(encryptedReply)
                            .as("Encrypted message should be a normal OlmMessage")
                            .isNotNull()
                            .extracting(OlmMessage::getType)
                            .isEqualTo(MessageType.NORMAL);
                    assertThat(encryptedReply)
                            .as("Encrypted message' body should not be null nor empty")
                            .extracting(OlmMessage::getBody, STRING)
                            .isNotEmpty()
                            .isBase64();

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
            try (OlmSession session = aliceAccount.createOutboundSession(
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
            try (OlmSession session = aliceAccount.createOutboundSession(
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

            try (OlmSession session = aliceAccount.createOutboundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey)) {

                byte[] invalidKey = new byte[16];
                assertThatThrownBy(() -> session.pickle(invalidKey))
                        .as("Pickle with invalid key size should throw KeyException")
                        .isInstanceOf(KeyException.class)
                        .hasMessageContaining("256-bit (32-byte)");
            }
        }
    }

    @Test
    void testUnpickleWithInvalidKeyThrowsException() {
        byte[] invalidKey = new byte[16];
        assertThatThrownBy(() -> OlmSession.unpickle("invalid", invalidKey))
                .as("Unpickle with invalid key size should throw KeyException")
                .isInstanceOf(KeyException.class)
                .hasMessageContaining("256-bit (32-byte)");
    }

    @Test
    void testSessionCheckNotClosedThrowsAfterClose() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            String bobOneTimeKey = result.getCreated().iterator().next();
            bobAccount.markKeysAsPublished();

            OlmSession session = aliceAccount.createOutboundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey);

            session.close();

            assertThatThrownBy(session::sessionId)
                    .as("Using closed session should throw IllegalStateException")
                    .isInstanceOf(IllegalStateException.class)
                    .hasMessage("OlmSession has been closed");

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

    @Test
    void testSessionCreationExceptionOnVersionMismatch() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            String bobOneTimeKey = result.getCreated().iterator().next();
            bobAccount.markKeysAsPublished();

            try (OlmSession outboundSession = aliceAccount.createOutboundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey)) {
                OlmMessage encrypted = outboundSession.encrypt("Hello Bob".getBytes(StandardCharsets.UTF_8));
                String aliceIdentityKey = aliceAccount.curve25519Key();

                assertThatThrownBy(() -> bobAccount.createInboundSession(OlmSessionVersion.V1, aliceIdentityKey, encrypted))
                        .as("Creating inbound session with mismatched version should throw SessionCreationException")
                        .isInstanceOf(SessionCreationException.class)
                        .hasMessage("The session config doesn't match the one used for the pre-key message: expected SessionConfig { version: V1 }, got Some(SessionConfig { version: V2 })");
            }
        }
    }

    @Test
    void testInvalidMessageTypeThrowsVodozemacException() {
        assertThatThrownBy(() -> MessageType.fromValue(-1))
                .isInstanceOf(VodozemacException.class)
                .hasMessage("unknown message type -1");
    }
}
