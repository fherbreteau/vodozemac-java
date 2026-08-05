package io.github.fherbreteau.vodozemac.olm;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class OlmSessionVersionTest {

    @Test
    void testVersionValues() {
        assertThat(OlmSessionVersion.values())
                .as("OlmSessionVersion should have exactly two values")
                .containsExactly(OlmSessionVersion.V1, OlmSessionVersion.V2);
    }

    @Test
    void testV1Value() {
        assertThat(OlmSessionVersion.V1.getValue())
                .as("V1 should have value 1")
                .isEqualTo(1);
    }

    @Test
    void testV2Value() {
        assertThat(OlmSessionVersion.V2.getValue())
                .as("V2 should have value 2")
                .isEqualTo(2);
    }

    @Test
    void testValueOf() {
        assertThat(OlmSessionVersion.valueOf("V1"))
                .as("valueOf(\"V1\") should return V1")
                .isEqualTo(OlmSessionVersion.V1);
        assertThat(OlmSessionVersion.valueOf("V2"))
                .as("valueOf(\"V2\") should return V2")
                .isEqualTo(OlmSessionVersion.V2);
    }
}
