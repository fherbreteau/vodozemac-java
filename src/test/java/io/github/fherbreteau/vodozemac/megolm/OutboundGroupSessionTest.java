package io.github.fherbreteau.vodozemac.megolm;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;

import io.github.fherbreteau.vodozemac.exception.KeyException;
import org.junit.jupiter.api.Test;

class OutboundGroupSessionTest {

    private final SecureRandom random = new SecureRandom();

    @Test
    void testCreateAndBasicOperations() {
        try (OutboundGroupSession session = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            assertThat(session)
                    .as("Outbound group session should be created")
                    .isNotNull();

            assertThat(session.sessionId())
                    .as("Session ID should not be null or empty")
                    .isNotNull()
                    .isNotEmpty();

            assertThat(session.messageIndex())
                    .as("New session should start at message index 0")
                    .isZero();

            assertThat(session.sessionKey())
                    .as("Session key should not be null or empty")
                    .isNotNull()
                    .isNotEmpty();
            assertThat(session.sessionConfig())
                    .as("Session version should be V2")
                    .isEqualTo(MegolmSessionVersion.V2);

        }
    }

    @Test
    void testEncryptIncrementsMessageIndex() {
        try (OutboundGroupSession session = new OutboundGroupSession(MegolmSessionVersion.V1)) {
            assertThat(session.messageIndex())
                    .as("Initial message index should be 0")
                    .isZero();

            String encrypted = session.encrypt("Hello Megolm".getBytes(StandardCharsets.UTF_8));

            assertThat(encrypted)
                    .as("Encrypted message should be base64")
                    .isNotNull()
                    .isNotEmpty()
                    .matches("[A-Za-z0-9+/=]+");

            assertThat(session.messageIndex())
                    .as("Message index should increment after encrypt")
                    .isEqualTo(1);

            session.encrypt("Second message".getBytes(StandardCharsets.UTF_8));
            assertThat(session.messageIndex())
                    .as("Message index should increment again")
                    .isEqualTo(2);
        }
    }

    @Test
    void testPickleAndUnpickle() {
        String originalSessionId;
        String pickleData;
        try (OutboundGroupSession session = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            originalSessionId = session.sessionId();
            pickleData = session.pickle();
        }

        assertThat(pickleData)
                .as("Pickle data should be valid JSON")
                .isNotNull()
                .isNotEmpty()
                .startsWith("{")
                .endsWith("}");

        try (OutboundGroupSession unpickled = OutboundGroupSession.unpickle(pickleData)) {
            assertThat(unpickled.sessionId())
                    .as("Unpickled session should have the same session ID")
                    .isEqualTo(originalSessionId);
        }
    }

    @Test
    void testEncryptedPickleAndUnpickle() {
        byte[] key = new byte[32];
        random.nextBytes(key);

        String originalSessionId;
        String pickleData;
        try (OutboundGroupSession session = new OutboundGroupSession(MegolmSessionVersion.V2)) {
            originalSessionId = session.sessionId();
            pickleData = session.pickle(key);
        }

        assertThat(pickleData)
                .as("Encrypted pickle data should not be empty")
                .isNotNull()
                .isNotEmpty();

        try (OutboundGroupSession unpickled = OutboundGroupSession.unpickle(pickleData, key)) {
            assertThat(unpickled.sessionId())
                    .as("Unpickled session should have the same session ID")
                    .isEqualTo(originalSessionId);
        }
    }

    @Test
    void testCheckNotClosedThrowsAfterClose() {
        OutboundGroupSession session = new OutboundGroupSession(MegolmSessionVersion.V1);
        session.close();

        assertThatThrownBy(session::sessionId)
                .as("Using closed session should throw IllegalStateException")
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("OutboundGroupSession has been closed");
    }

    @Test
    void testUnpickleLegacy() {
        String pickleData =
                "Diq07H2wlx0KGG5UBzBifXoYYVLUF9s4y1NHbMxAx3v+fcC+4P3OO8N8id1O5dQ5y9udoDXEmLxQiQ"
                + "KLN9WF6AJNbIYN7p27V9tpK2MCjWONpZ9YiPA9vG5bNwYIips/aahtUbrx0ooviZu2Ozso/Wle4FDe"
                + "zHJ0cZ87rJ95YDeEnORu8TihXVLTsdqinAkBB2TAqscqMirpI0RDdreatUK+v00oVxz6QLtKYlC2wBb"
                + "b5taqN7R7UGcYGWd9RN+xRJK9noSXyNH4wxTp4AAyRlJZSXR4qclxNlUvsWQ/qSPaiKvhZvF7qX2ujX+"
                + "6G6TiiNBpwrOUUgM";
        byte[] pickleKey = "DEFAULT_PICKLE_KEY".getBytes(StandardCharsets.UTF_8);

        try (OutboundGroupSession session = OutboundGroupSession.unpickleLegacy(pickleData, pickleKey)) {
            assertThat(session)
                    .as("Unpickled legacy outbound group session should be created")
                    .isNotNull();

            assertThat(session.sessionId())
                    .as("Legacy session should have a valid session ID")
                    .isNotNull()
                    .isNotEmpty();
        }
    }

    @Test
    void testPickleWithInvalidKeyThrowsException() {
        try (OutboundGroupSession session = new OutboundGroupSession(MegolmSessionVersion.V1)) {
            assertThatThrownBy(() -> session.pickle(new byte[16]))
                    .as("Pickle with invalid key size should throw KeyException")
                    .isInstanceOf(KeyException.class)
                    .hasMessageContaining("256-bit (32-byte)");
        }
    }

    @Test
    void testUnpickleWithInvalidKeyThrowsException() {
        assertThatThrownBy(() -> OutboundGroupSession.unpickle("invalid", new byte[16]))
                .as("Unpickle with invalid key size should throw KeyException")
                .isInstanceOf(KeyException.class)
                .hasMessageContaining("256-bit (32-byte)");
    }
}
