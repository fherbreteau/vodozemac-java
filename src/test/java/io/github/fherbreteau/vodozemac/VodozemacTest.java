package io.github.fherbreteau.vodozemac;

import static org.assertj.core.api.Assertions.assertThat;

import java.security.SecureRandom;

import org.junit.jupiter.api.Test;

class VodozemacTest {

    private final SecureRandom random = new SecureRandom();

    @Test
    void testBase64EncodingRoundTrip() {
        byte[] source = new byte[32];
        random.nextBytes(source);

        String encoded = Vodozemac.base64Encode(source);
        assertThat(encoded).isNotNull().isBase64();

        byte[] result = Vodozemac.base64Decode(encoded);

        assertThat(result)
            .isEqualTo(source);
    }

    @Test
    void testVersionExtraction() {
        String version = Vodozemac.getVersion();

        assertThat(version).isNotNull()
            .isEqualTo("0.10.0");
    }
}
