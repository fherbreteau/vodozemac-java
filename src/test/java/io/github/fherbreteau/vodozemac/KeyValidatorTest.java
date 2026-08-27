package io.github.fherbreteau.vodozemac;

import static org.assertj.core.api.Assertions.assertThatCode;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.stream.Stream;

import io.github.fherbreteau.vodozemac.exception.KeyException;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

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

    public static Stream<Arguments> invalidSize() {
        return Stream.of(
            Arguments.of(new byte[16]),
            Arguments.of(new byte[64]),
            Arguments.of(new byte[0])
        );
    }

    @ParameterizedTest
    @MethodSource("invalidSize")
    void testValidateKeyWithInvalidSize(byte[] key) {
        assertThatThrownBy(() -> KeyValidator.validateEncryptionKey(key))
                .isInstanceOf(KeyException.class)
                .hasMessage("Encrypted Key must be 256-bit (32-byte)");
    }
}
