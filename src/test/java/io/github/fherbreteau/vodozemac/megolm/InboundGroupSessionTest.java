package io.github.fherbreteau.vodozemac.megolm;

import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

class InboundGroupSessionTest {

    private final SecureRandom random = new SecureRandom();

    @Test
    void testCreateAndDecryptFromOutboundSession() {
        String plaintext = "Hello Megolm!";
        String sessionKey;
        String encrypted;

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

        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V1)) {
            sessionKey = outbound.sessionKey();
            outboundSessionId = outbound.sessionId();
        }

        try (InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V1)) {
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
    void testCloseIsIdempotent() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession(MegolmSessionVersion.V1)) {
            sessionKey = outbound.sessionKey();
        }

        InboundGroupSession inbound = new InboundGroupSession(sessionKey, MegolmSessionVersion.V1);

        assertThat(inbound.isClosed()).isFalse();
        inbound.close();
        assertThat(inbound.isClosed()).isTrue();
        inbound.close();
        assertThat(inbound.isClosed()).isTrue();
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
                .hasMessage("Account has been closed");
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
}
