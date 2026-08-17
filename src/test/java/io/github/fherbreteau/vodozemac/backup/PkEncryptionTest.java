package io.github.fherbreteau.vodozemac.backup;


import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import static java.nio.charset.StandardCharsets.UTF_8;
import static org.assertj.core.api.InstanceOfAssertFactories.STRING;

import io.github.fherbreteau.vodozemac.exception.EncryptionException;
import io.github.fherbreteau.vodozemac.exception.PickleException;
import io.github.fherbreteau.vodozemac.exception.VodozemacException;
import org.junit.jupiter.api.Test;

class PkEncryptionTest {

    @Test
    void encryptionRoundTrip() {
        try (PkDecryption decryptor = new PkDecryption()) {
            String publicKey = decryptor.publicKey();
            assertThat(publicKey)
                    .as("Decryptor public key should be defined")
                    .isNotNull().isNotEmpty();
            String secretKey = decryptor.secretKey();
            assertThat(secretKey)
                    .as("Decryptor secret key should be defined")
                    .isNotNull().isNotEmpty();
            PkEncryption encryptor = PkEncryption.fromKey(publicKey);

            String message = "It's a secret to everybody";

            PkMessage encrypted = encryptor.encrypt(message.getBytes(UTF_8));

            assertThat(encrypted).isNotNull();
            assertThat(encrypted).extracting(PkMessage::ciphertext, STRING).isNotEmpty();
            assertThat(encrypted).extracting(PkMessage::mac, STRING).isNotEmpty();
            assertThat(encrypted).extracting(PkMessage::ephemeralKey, STRING).isNotEmpty();

            byte[] plaintext = decryptor.decrypt(encrypted);
            assertThat(plaintext).asString(UTF_8).isEqualTo(message);
        }
    }

    @Test
    void loadDecryptorFromSecretKey() {
        String secretKey = "M9q67lYIG8aLkXQ7SrjX3yAaoO1sDZvGx2J9Yl+DaTY";
        String publicKey = "SCybzXqbfEuzWmcYngO6D60yaMIGLtWjUuAgjgtEXm0";
        try (PkDecryption decryptor = PkDecryption.fromKey(secretKey)) {
            assertThat(decryptor.publicKey()).isEqualTo(publicKey);

        }
    }

    @Test
    void testUnpickleLegacy() {
        String pickleData =
                "qgMW9S5ju0JOugO8+lpT/ajX8EegAPMeie8da9e2JvMH+4fNKWvIQwACpAhMK5mjzHOxQlHv0Srbfcp6W"
                + "J0kcZO+LeIOE0gCq/CW4x3tiRn7mMlwOK3htA";
        byte[] pickleKey = new byte[32];

        try (PkDecryption decryptor = PkDecryption.unpickleLegacy(pickleData, pickleKey)) {
            assertThat(decryptor)
                    .as("Unpickled legacy PkDecryption should be created")
                    .isNotNull();

            assertThat(decryptor.publicKey())
                    .as("Legacy PkDecryption should have a valid public key")
                    .isNotNull()
                    .isNotEmpty()
                    .isEqualTo("SCybzXqbfEuzWmcYngO6D60yaMIGLtWjUuAgjgtEXm0");

            assertThat(decryptor.secretKey())
                    .as("Legacy PkDecryption should have a valid secret key")
                    .isNotNull()
                    .isNotEmpty()
                    .isEqualTo("M9q67lYIG8aLkXQ7SrjX3yAaoO1sDZvGx2J9Yl+DaTY");
        }
    }

    @Test
    void testUnpickleLegacyWithInvalidData() {
        byte[] pickleKey = new byte[32];

        assertThatThrownBy(() -> PkDecryption.unpickleLegacy("invalid-data", pickleKey))
                .as("Unpickling invalid legacy data should throw PickleException")
                .isInstanceOf(PickleException.class);
    }

    @Test
    void testEncryptionExceptionIsVodozemacException() {
        assertThat(new EncryptionException("encryption failed"))
                .as("EncryptionException should be a VodozemacException")
                .isInstanceOf(VodozemacException.class)
                .hasMessage("encryption failed");
    }

    @Test
    void testPkMessageEqualsHashCodeToString() {
        PkMessage msg = new PkMessage("ct", "mac", "ek");
        PkMessage same = new PkMessage("ct", "mac", "ek");
        PkMessage different = new PkMessage("ct2", "mac", "ek");

        assertThat(msg).isEqualTo(msg)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo("not a PkMessage")
                .isNotEqualTo(null);
        assertThat(msg.toString()).contains("ciphertext", "mac", "ephemeralKey");
    }
}
