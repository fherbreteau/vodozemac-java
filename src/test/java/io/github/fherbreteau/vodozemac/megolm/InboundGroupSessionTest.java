package io.github.fherbreteau.vodozemac.megolm;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;
import java.util.Optional;

import io.github.fherbreteau.vodozemac.exception.DecryptionException;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.PickleException;
import io.github.fherbreteau.vodozemac.exception.SignatureException;
import org.junit.jupiter.api.Test;

class InboundGroupSessionTest {

    private final SecureRandom random = new SecureRandom();

    @Test
    void testCreateAndDecryptFromOutboundSession() {
        String plaintext = "Hello Megolm!";
        String sessionKey;
        MegolmMessage encrypted;

        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
            encrypted = outbound.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            assertThat(inbound.sessionId())
                    .as("Inbound session should have a valid session ID")
                    .isNotNull()
                    .isNotEmpty();

            assertThat(inbound.firstKnownIndex())
                    .as("First known index should be 0 for a new session")
                    .isZero();

            DecryptedMessage decrypted = inbound.decrypt(encrypted);

            assertThat(decrypted)
                    .as("Decrypted message should not be null")
                    .isNotNull();

            assertThat(new String(decrypted.plaintext(), StandardCharsets.UTF_8))
                    .as("Decrypted plaintext should match original")
                    .isEqualTo(plaintext);

            assertThat(decrypted.messageIndex())
                    .as("Message index should be 0 for first message")
                    .isZero();
        }
    }

    @Test
    void testSessionIdMatchesOutbound() {
        String sessionKey;
        String outboundSessionId;

        try (OutboundGroupSession outbound = new OutboundGroupSession()) {
            sessionKey = outbound.sessionKey();
            outboundSessionId = outbound.sessionId();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey)) {
            assertThat(inbound.sessionId())
                    .as("Inbound and outbound session IDs should match")
                    .isEqualTo(outboundSessionId);
        }
    }

    @Test
    void testPickleAndUnpickle() {
        String sessionKey;
        String originalSessionId;
        String pickleData;

        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
            originalSessionId = outbound.sessionId();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            pickleData = inbound.pickle();
        }

        assertThat(pickleData)
                .as("Pickle data should be valid JSON")
                .isNotNull()
                .isNotEmpty()
                .startsWith("{")
                .endsWith("}");

        try (InboundGroupSession unpickled = InboundGroupSession.unpickle(pickleData)) {
            assertThat(unpickled.sessionId())
                    .as("Unpickled session should have the same session ID")
                    .isEqualTo(originalSessionId);
        }
    }

    @Test
    void testEncryptedPickleAndUnpickle() {
        byte[] key = new byte[32];
        random.nextBytes(key);

        String sessionKey;
        String originalSessionId;
        String pickleData;

        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
            originalSessionId = outbound.sessionId();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            pickleData = inbound.pickle(key);
        }

        try (InboundGroupSession unpickled = InboundGroupSession.unpickle(pickleData, key)) {
            assertThat(unpickled.sessionId())
                    .as("Unpickled session should have the same session ID")
                    .isEqualTo(originalSessionId);
        }
    }

    @Test
    void testCheckNotClosedThrowsAfterClose() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V1)) {
            sessionKey = outbound.sessionKey();
        }

        InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V1);
        inbound.close();

        assertThatThrownBy(inbound::sessionId)
                .as("Using closed session should throw IllegalStateException")
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("InboundGroupSession has been closed");
    }

    @Test
    void testUnpickleLegacy() {
        String pickleData =
                "lBWLfHkXe/Ysulgp/Qj46nt5GwAABPHFVCqddUi+WV6VTCMSPqpjheXNCFrJaGwXQauhJHNKjRHVJc"
                + "XpKNM6usxC+gDJNeiY9OXTi587VCMGtbSa9HQ0xWEZ0y6c5YFLXeX+eLCRArvNjwUWZBJpbnJODvwc"
                + "Nd2AjMRmwOvB7mTCGqcLIjVsrId/UM2gf0JoUtH0ufnnDDPLmTCOXBEBDsRb4VSThbCkpwQZBxv+ii"
                + "8/nV7z26iUQp7r45683NsnO1V0LKHosihS9+KlR5tusnbi1WyCUtLvc/SO7ebBiCefCqpIB0pXLgMpO"
                + "IiTzsPDpd3Z62OqYtY/KkR8J/4NtJtOqBjF/Vu2Rie9nlel4SmyEb6ydS9kObo2cAfHAD1t7xzXaJWq"
                + "2pEwEXC0yKG1TvDOA23103h7";
        byte[] pickleKey = "DEFAULT_PICKLE_KEY".getBytes(StandardCharsets.UTF_8);

        try (InboundGroupSession session = InboundGroupSession.unpickleLegacy(pickleData, pickleKey)) {
            assertThat(session)
                    .as("Unpickled legacy inbound group session should be created")
                    .isNotNull();

            assertThat(session.sessionId())
                    .as("Legacy session should have a valid session ID")
                    .isNotNull()
                    .isNotEmpty();
        }
    }

    @Test
    void testExportAt() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            assertThat(inbound.exportAt(0))
                    .as("Export at index 0 should return a non-null key")
                    .isNotNull()
                    .isNotEmpty();

            assertThat(inbound.exportAt(10))
                    .as("Export at index 10 should return a non-null key")
                    .isNotNull()
                    .isNotEmpty();

            inbound.advanceTo(5);

            assertThat(inbound.exportAt(3))
                    .as("Export at index below first known index should return null")
                    .isNull();

            assertThat(inbound.exportAt(5))
                    .as("Export at first known index should return a non-null key")
                    .isNotNull()
                    .isNotEmpty();

            assertThat(inbound.exportAt(15))
                    .as("Export at index above first known index should return a non-null key")
                    .isNotNull()
                    .isNotEmpty();
        }
    }

    @Test
    void testExportAtFirstKnownIndex() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            String exported = inbound.exportAtFirstKnownIndex();

            assertThat(exported)
                    .as("Export at first known index should return a non-null key")
                    .isNotNull()
                    .isNotEmpty();

            assertThat(exported)
                    .as("Export at first known index should match exportAt(firstKnownIndex)")
                    .isEqualTo(inbound.exportAt(inbound.firstKnownIndex()));

            inbound.advanceTo(10);

            String exportedAfterAdvance = inbound.exportAtFirstKnownIndex();

            assertThat(exportedAfterAdvance)
                    .as("Export at first known index after advance should return a non-null key")
                    .isNotNull()
                    .isNotEmpty();

            assertThat(exportedAfterAdvance)
                    .as("Export at first known index after advance should match exportAt(firstKnownIndex)")
                    .isEqualTo(inbound.exportAt(inbound.firstKnownIndex()));
        }
    }

    @Test
    void testImportSession() {
        String sessionKey;
        String sessionId;
        try (OutboundGroupSession outbound = new OutboundGroupSession()) {
            sessionKey = outbound.sessionKey();
            sessionId = outbound.sessionId();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey)) {
            String exportedKey = inbound.exportAt(10);

            try (InboundGroupSession imported = InboundGroupSession.importSession(exportedKey)) {
                assertThat(imported)
                        .as("Imported session should be created")
                        .isNotNull();

                assertThat(imported.sessionId())
                        .as("Imported session should have the same session ID as the original")
                        .isEqualTo(sessionId);

                assertThat(imported.firstKnownIndex())
                        .as("Imported session first known index should match the export index")
                        .isEqualTo(10);
            }
        }
    }

    @Test
    void testImportSessionCanDecryptFromExportIndex() {
        String plaintext = "Hello from index 10";
        String sessionKey;
        MegolmMessage encryptedAt10;

        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
            for (int i = 0; i < 10; i++) {
                outbound.encrypt(("Filler " + i).getBytes(StandardCharsets.UTF_8));
            }
            encryptedAt10 = outbound.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            String exportedKey = inbound.exportAt(10);

            try (InboundGroupSession imported = InboundGroupSession.importSession(exportedKey, MegolmSessionVersion.V2)) {
                DecryptedMessage decrypted = imported.decrypt(encryptedAt10);

                assertThat(new String(decrypted.plaintext(), StandardCharsets.UTF_8))
                        .as("Imported session should decrypt message at the export index")
                        .isEqualTo(plaintext);

                assertThat(decrypted.messageIndex())
                        .as("Decrypted message index should be 10")
                        .isEqualTo(10);
            }
        }
    }

    @Test
    void testAdvanceTo() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            assertThat(inbound.firstKnownIndex())
                    .as("New session should start at first known index 0")
                    .isZero();

            assertThat(inbound.advanceTo(10))
                    .as("Advance to 10 should succeed")
                    .isTrue();

            assertThat(inbound.firstKnownIndex())
                    .as("First known index should be 10 after advance")
                    .isEqualTo(10);

            assertThat(inbound.advanceTo(10))
                    .as("Advance to the same index should return false")
                    .isFalse();

            assertThat(inbound.advanceTo(5))
                    .as("Advance to a lower index should return false")
                    .isFalse();

            assertThat(inbound.advanceTo(20))
                    .as("Advance to 20 should succeed")
                    .isTrue();

            assertThat(inbound.firstKnownIndex())
                    .as("First known index should be 20 after advance")
                    .isEqualTo(20);
        }
    }

    @Test
    void testAdvanceToRemovesAbilityToDecryptEarlierMessages() {
        String plaintext = "Early message";
        String sessionKey;
        MegolmMessage encrypted;

        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
            encrypted = outbound.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            DecryptedMessage decrypted = inbound.decrypt(encrypted);
            assertThat(new String(decrypted.plaintext(), StandardCharsets.UTF_8))
                    .as("Should decrypt before advancing")
                    .isEqualTo(plaintext);

            inbound.advanceTo(1);

            assertThatThrownBy(() -> inbound.decrypt(encrypted))
                    .as("Should not decrypt message before first known index after advancing")
                    .isInstanceOf(DecryptionException.class);
        }
    }

    @Test
    void testConnectedWithSameOutbound() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession session1 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2);
                InboundGroupSession session2 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {

            assertThat(session1.connected(session2))
                    .as("Sessions from the same outbound should be connected")
                    .isTrue();

            assertThat(session2.connected(session1))
                    .as("Connected should be symmetric")
                    .isTrue();

            session2.advanceTo(10);

            assertThat(session1.connected(session2))
                    .as("Sessions should still be connected after advancing one")
                    .isTrue();

            assertThat(session2.connected(session1))
                    .as("Connected should be symmetric after advancing")
                    .isTrue();
        }
    }

    @Test
    void testConnectedWithDifferentOutbound() {
        String sessionKey1;
        String sessionKey2;
        try (OutboundGroupSession outbound1 = new OutboundGroupSession(MegolmSessionVersion.V2);
                OutboundGroupSession outbound2 = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey1 = outbound1.sessionKey();
            sessionKey2 = outbound2.sessionKey();
        }

        try (InboundGroupSession session1 = new InboundGroupSession(sessionKey1, MegolmSessionVersion.V2);
                InboundGroupSession session2 = new InboundGroupSession(sessionKey2, MegolmSessionVersion.V2)) {

            assertThat(session1.connected(session2))
                    .as("Sessions from different outbound sessions should not be connected")
                    .isFalse();
        }
    }

    @Test
    void testConnectedWithDifferentVersions() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V1)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession sessionV1 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V1);
                InboundGroupSession sessionV2 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {

            assertThat(sessionV1.connected(sessionV2))
                    .as("Sessions with different versions should not be connected")
                    .isFalse();
        }
    }

    @Test
    void testCompareEqual() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession session1 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2);
                InboundGroupSession session2 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {

            assertThat(session1.compare(session2))
                    .as("Identical sessions should compare as EQUAL")
                    .isEqualTo(SessionOrdering.EQUAL);

            assertThat(session2.compare(session1))
                    .as("Compare should be symmetric for equal sessions")
                    .isEqualTo(SessionOrdering.EQUAL);
        }
    }

    @Test
    void testCompareBetterAndWorse() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession session1 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2);
                InboundGroupSession session2 = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {

            session2.advanceTo(10);

            assertThat(session1.compare(session2))
                    .as("Session with lower index should be BETTER")
                    .isEqualTo(SessionOrdering.BETTER);

            assertThat(session2.compare(session1))
                    .as("Session with higher index should be WORSE")
                    .isEqualTo(SessionOrdering.WORSE);
        }
    }

    @Test
    void testCompareUnconnected() {
        String sessionKey1;
        String sessionKey2;
        try (OutboundGroupSession outbound1 = new OutboundGroupSession(MegolmSessionVersion.V2);
                OutboundGroupSession outbound2 = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey1 = outbound1.sessionKey();
            sessionKey2 = outbound2.sessionKey();
        }

        try (InboundGroupSession session1 = new InboundGroupSession(sessionKey1, MegolmSessionVersion.V2);
                InboundGroupSession session2 = new InboundGroupSession(sessionKey2, MegolmSessionVersion.V2)) {

            assertThat(session1.compare(session2))
                    .as("Sessions from different outbound should compare as UNCONNECTED")
                    .isEqualTo(SessionOrdering.UNCONNECTED);

            assertThat(session2.compare(session1))
                    .as("Unconnected compare should be symmetric")
                    .isEqualTo(SessionOrdering.UNCONNECTED);
        }
    }

    @Test
    void testMergeConnectedSessions() {
        String sessionKey;
        String sessionId;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
            sessionId = outbound.sessionId();
        }

        try (InboundGroupSession firstSession = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            String exportedKey = firstSession.exportAt(10);

            try (InboundGroupSession secondSession = InboundGroupSession.importSession(exportedKey, MegolmSessionVersion.V2)) {
                assertThat(firstSession.compare(secondSession))
                        .as("First session (lower index) should be BETTER")
                        .isEqualTo(SessionOrdering.BETTER);

                Optional<InboundGroupSession> mergedOpt = secondSession.merge(firstSession);

                assertThat(mergedOpt)
                        .as("Merge of connected sessions should return a session")
                        .isPresent();

                try (InboundGroupSession merged = mergedOpt.get()) {
                    assertThat(merged.sessionId())
                            .as("Merged session should have the same session ID")
                            .isEqualTo(sessionId);

                    assertThat(merged.firstKnownIndex())
                            .as("Merged session should have the lower first known index")
                            .isZero();

                    assertThat(merged.compare(secondSession))
                            .as("Merged session should be BETTER than the imported (higher index) session")
                            .isEqualTo(SessionOrdering.BETTER);

                    assertThat(merged.compare(firstSession))
                            .as("Merged session should be EQUAL to the first session")
                            .isEqualTo(SessionOrdering.EQUAL);
                }
            }
        }
    }

    @Test
    void testMergeUnconnectedSessionsReturnsEmpty() {
        String sessionKey1;
        String sessionKey2;
        try (OutboundGroupSession outbound1 = new OutboundGroupSession(MegolmSessionVersion.V2);
                OutboundGroupSession outbound2 = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey1 = outbound1.sessionKey();
            sessionKey2 = outbound2.sessionKey();
        }

        try (InboundGroupSession session1 = new InboundGroupSession(sessionKey1, MegolmSessionVersion.V2);
                InboundGroupSession session2 = new InboundGroupSession(sessionKey2, MegolmSessionVersion.V2)) {

            Optional<InboundGroupSession> merged = session1.merge(session2);

            assertThat(merged)
                    .as("Merge of unconnected sessions should return empty")
                    .isEmpty();
        }
    }

    @Test
    void testClosedSessionMethodsThrowAfterClose() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
        }

        InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2);
        inbound.close();

        assertThatThrownBy(inbound::exportAtFirstKnownIndex)
                .as("Export on closed session should throw IllegalStateException")
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("InboundGroupSession has been closed");

        assertThatThrownBy(() -> inbound.exportAt(0))
                .as("Export at index on closed session should throw IllegalStateException")
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("InboundGroupSession has been closed");

        assertThatThrownBy(() -> inbound.advanceTo(1))
                .as("AdvanceTo on closed session should throw IllegalStateException")
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("InboundGroupSession has been closed");
    }

    @Test
    void testPickleExceptionOnInvalidPickleData() {
        assertThatThrownBy(() -> InboundGroupSession.unpickle("invalid-json"))
                .as("Unpickling invalid JSON should throw PickleException")
                .isInstanceOf(PickleException.class);
    }

    @Test
    void testPickleExceptionOnInvalidEncryptedUnpickle() {
        byte[] key = new byte[32];
        assertThatThrownBy(() -> InboundGroupSession.unpickle("invalid-encrypted-data", key))
                .as("Unpickling invalid encrypted data should throw PickleException")
                .isInstanceOf(PickleException.class);
    }

    @Test
    void testSignatureExceptionOnWrongSessionDecrypt() {
        String plaintext = "Hello Megolm!";
        MegolmMessage encrypted;

        try (OutboundGroupSession outbound1 = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            encrypted = outbound1.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));
        }

        try (OutboundGroupSession outbound2 = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            String sessionKey2 = outbound2.sessionKey();
            try (InboundGroupSession wrongInbound = new InboundGroupSession(sessionKey2, MegolmSessionVersion.V2)) {
                assertThatThrownBy(() -> wrongInbound.decrypt(encrypted))
                        .as("Decrypting with wrong session should throw SignatureException")
                        .isInstanceOf(SignatureException.class);
            }
        }
    }

    @Test
    void testDecryptionExceptionOnUnknownMessageIndex() {
        String plaintext = "Early message";
        String sessionKey;
        MegolmMessage encrypted;

        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            sessionKey = outbound.sessionKey();
            encrypted = outbound.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V2)) {
            inbound.advanceTo(1);

            assertThatThrownBy(() -> inbound.decrypt(encrypted))
                    .as("Decrypting with unknown message index should throw DecryptionException")
                    .isInstanceOf(DecryptionException.class);
        }
    }

    @Test
    void testKeyExceptionOnInvalidSessionKey() {
        assertThatThrownBy(() -> new InboundGroupSession("invalid-base64-key", MegolmSessionVersion.V2))
                .as("Creating session with invalid key should throw VodozemacException")
                .isInstanceOf(KeyException.class);
    }

    @Test
    void testPickleWithInvalidKeyThrowsException() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V1)) {
            sessionKey = outbound.sessionKey();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V1)) {
            assertThatThrownBy(() -> inbound.pickle(new byte[16]))
                    .as("Pickle with invalid key size should throw KeyException")
                    .isInstanceOf(KeyException.class)
                    .hasMessageContaining("256-bit (32-byte)");
        }
    }

    @Test
    void testUnpickleWithInvalidKeyThrowsException() {
        assertThatThrownBy(() -> InboundGroupSession.unpickle("invalid", new byte[16]))
                .as("Unpickle with invalid key size should throw KeyException")
                .isInstanceOf(KeyException.class)
                .hasMessageContaining("256-bit (32-byte)");
    }

    @Test
    void testDecryptedMessageEqualsHashCodeToString() {
        DecryptedMessage msg = new DecryptedMessage(new byte[]{1, 2, 3}, 5);
        DecryptedMessage same = new DecryptedMessage(new byte[]{1, 2, 3}, 5);
        DecryptedMessage different = new DecryptedMessage(new byte[]{1, 2, 3}, 6);
        DecryptedMessage different2 = new DecryptedMessage(new byte[]{4, 5, 6}, 5);

        assertThat(msg).isEqualTo(msg)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo(different2)
                .isNotEqualTo("not a message")
                .isNotEqualTo(null);
        assertThat(msg.toString()).contains("plaintext", "messageIndex");
    }

    @Test
    void testMegolmMessageFieldsAndRoundTrip() {
        String plaintext = "Hello Megolm!";
        MegolmMessage encrypted;

        try (OutboundGroupSession outbound = new OutboundGroupSession()) {
            encrypted = outbound.encrypt(plaintext.getBytes(StandardCharsets.UTF_8));
        }

        assertThat(encrypted.getCiphertext()).as("ciphertext should not be null").isNotNull().isNotEmpty();
        assertThat(encrypted.getMac()).as("mac should not be null").isNotNull().isNotEmpty();
        assertThat(encrypted.getSignature()).as("signature should not be null").isNotNull().isNotEmpty();
        assertThat(encrypted.getMessageIndex()).as("message index should be 0").isZero();
        assertThat(encrypted.toString()).as("toString should be base64").isNotNull().isNotEmpty();

        MegolmMessage reconstructed = MegolmMessage.fromBase64(encrypted.toString());
        assertThat(reconstructed).as("reconstructed message should equal original")
                .isEqualTo(encrypted)
                .hasSameHashCodeAs(encrypted)
                .isNotEqualTo("not a MegolmMessage")
                .isNotEqualTo(null);
    }
}
