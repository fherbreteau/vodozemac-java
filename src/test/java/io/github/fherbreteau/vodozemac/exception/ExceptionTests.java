package io.github.fherbreteau.vodozemac.exception;

import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class ExceptionTests {

    @Test
    void testConversionException() {
        assertThatThrownBy(() -> {
            throw new ConversionException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testDecryptionException() {
        assertThatThrownBy(() -> {
            throw new DecryptionException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testEciesException() {
        assertThatThrownBy(() -> {

            throw new EciesException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testEncryptionException() {
        assertThatThrownBy(() -> {
            throw new EncryptionException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testKeyException() {
        assertThatThrownBy(() -> {
            throw new KeyException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testPickleException() {
        assertThatThrownBy(() -> {
            throw new PickleException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testSasException() {
        assertThatThrownBy(() -> {
            throw new SasException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testSessionCreationException() {
        assertThatThrownBy(() -> {
            throw new SessionCreationException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }

    @Test
    void testSignatureException() {
        assertThatThrownBy(() -> {
            throw new SignatureException("message", new Exception());
        })
                .hasMessage("message")
                .hasCauseInstanceOf(Exception.class);
    }
}
