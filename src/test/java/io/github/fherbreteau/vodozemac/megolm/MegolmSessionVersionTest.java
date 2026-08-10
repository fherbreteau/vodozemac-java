package io.github.fherbreteau.vodozemac.megolm;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MegolmSessionVersionTest {

    @Test
    void testVersionValues() {
        assertThat(MegolmSessionVersion.values())
                .as("MegolmSessionVersion should have exactly two values")
                .containsExactly(MegolmSessionVersion.V1, MegolmSessionVersion.V2);
    }

    @Test
    void testV1Value() {
        assertThat(MegolmSessionVersion.V1.getValue())
                .as("V1 should have value 1")
                .isEqualTo(1);
    }

    @Test
    void testV2Value() {
        assertThat(MegolmSessionVersion.V2.getValue())
                .as("V2 should have value 2")
                .isEqualTo(2);
    }

    @Test
    void testDefaultValue() {
        assertThat(MegolmSessionVersion.defaultVersion())
                .as("default version should be V1")
                .isEqualTo(MegolmSessionVersion.V1);
    }

    @Test
    void testValueOf() {
        assertThat(MegolmSessionVersion.valueOf("V1"))
                .as("valueOf(\"V1\") should return V1")
                .isEqualTo(MegolmSessionVersion.V1);
        assertThat(MegolmSessionVersion.valueOf("V2"))
                .as("valueOf(\"V2\") should return V2")
                .isEqualTo(MegolmSessionVersion.V2);
    }
}
