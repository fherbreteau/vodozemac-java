package io.github.fherbreteau.vodozemac;

import static org.assertj.core.api.Assertions.assertThatCode;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import io.github.fherbreteau.vodozemac.exception.KeyException;
import org.junit.jupiter.api.Test;

class KeyValidatorTest {

    @Test
    void testValidateValidKey() {
        byte[] key = new byte[32];
        assertThatCode(() -> KeyValidator.validateEncryptionKey(key))
                .doesNotThrowAnyException();
    }

    @Test
    void testValidateNullKey() {
        assertThatThrownBy(() -> KeyValidator.validateEncryptionKey(null))
                .isInstanceOf(KeyException.class)
                .hasMessage("Encrypted Key must be 256-bit (32-byte)");
    }

    @Test
    void testValidateKeyTooShort() {
        byte[] key = new byte[16];
        assertThatThrownBy(() -> KeyValidator.validateEncryptionKey(key))
                .isInstanceOf(KeyException.class)
                .hasMessage("Encrypted Key must be 256-bit (32-byte)");
    }

    @Test
    void testValidateKeyTooLong() {
        byte[] key = new byte[64];
        assertThatThrownBy(() -> KeyValidator.validateEncryptionKey(key))
                .isInstanceOf(KeyException.class)
                .hasMessage("Encrypted Key must be 256-bit (32-byte)");
    }

    @Test
    void testValidateEmptyKey() {
        byte[] key = new byte[0];
        assertThatThrownBy(() -> KeyValidator.validateEncryptionKey(key))
                .isInstanceOf(KeyException.class)
                .hasMessage("Encrypted Key must be 256-bit (32-byte)");
    }
}
