package io.github.fherbreteau.vodozemac;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.function.Function;
import java.util.stream.Stream;

import io.github.fherbreteau.vodozemac.exception.VodozemacException;
import io.github.fherbreteau.vodozemac.megolm.MegolmSessionVersion;
import io.github.fherbreteau.vodozemac.olm.MessageType;
import io.github.fherbreteau.vodozemac.olm.OlmSessionVersion;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

class SessionVersionTest {

    private static Stream<Arguments> testValues() {
        return Stream.of(
            Arguments.of(MegolmSessionVersion.values(), new MegolmSessionVersion[] {MegolmSessionVersion.V1, MegolmSessionVersion.V2}),
            Arguments.of(OlmSessionVersion.values(), new OlmSessionVersion[] {OlmSessionVersion.V1, OlmSessionVersion.V2}),
            Arguments.of(MessageType.values(), new MessageType[] {MessageType.PRE_KEY, MessageType.NORMAL})
        );
    }

    @ParameterizedTest
    @MethodSource("testValues")
    void testVersionValues(SessionVersion[] extractedValues, SessionVersion[] expectedValues) {
        assertThat(extractedValues)
                .as("SessionVersion should have exactly the expected values")
                .containsExactly(expectedValues);
    }

    private static Stream<Arguments> testNumericValues() {
        return Stream.of(
            Arguments.of(MegolmSessionVersion.V1, 1),
            Arguments.of(MegolmSessionVersion.V2, 2),
            Arguments.of(OlmSessionVersion.V1, 1),
            Arguments.of(OlmSessionVersion.V2, 2),
            Arguments.of(MessageType.PRE_KEY, 0),
            Arguments.of(MessageType.NORMAL, 1)
        );
    }

    @ParameterizedTest
    @MethodSource("testNumericValues")
    void testNumericValue(SessionVersion version, int expectedValue) {
        assertThat(version.getValue())
                .isEqualTo(expectedValue);
    }

    private static Stream<Arguments> testDefaultValues() {
        return Stream.of(
            Arguments.of(MegolmSessionVersion.defaultVersion(), MegolmSessionVersion.V1),
            Arguments.of(OlmSessionVersion.defaultVersion(), OlmSessionVersion.V1)
        );
    }

    @ParameterizedTest
    @MethodSource("testDefaultValues")
    void testDefaultValue(SessionVersion defaultValue, SessionVersion expectedValue) {
        assertThat(defaultValue)
                .as("default version should be V1")
                .isEqualTo(expectedValue);
    }

    private static Stream<Arguments> testFromValues() {
        return Stream.of(
            Arguments.of(1, (Function<Integer, SessionVersion>) MegolmSessionVersion::fromVersion, MegolmSessionVersion.V1),
            Arguments.of(2, (Function<Integer, SessionVersion>) MegolmSessionVersion::fromVersion, MegolmSessionVersion.V2),
            Arguments.of(1, (Function<Integer, SessionVersion>) OlmSessionVersion::fromVersion, OlmSessionVersion.V1),
            Arguments.of(2, (Function<Integer, SessionVersion>) OlmSessionVersion::fromVersion, OlmSessionVersion.V2),
            Arguments.of(0, (Function<Integer, SessionVersion>) MessageType::fromValue, MessageType.PRE_KEY),
            Arguments.of(1, (Function<Integer, SessionVersion>) MessageType::fromValue, MessageType.NORMAL)
        );
    }

    @ParameterizedTest
    @MethodSource("testFromValues")
    void testFromVersion(int value, Function<Integer, SessionVersion> parser, SessionVersion expectedVersion) {
        assertThat(parser.apply(value))
                .isEqualTo(expectedVersion);
    }

    private static Stream<Arguments> testFailedValues() {
        return Stream.of(
            Arguments.of(0, (Function<Integer, SessionVersion>) MegolmSessionVersion::fromVersion),
            Arguments.of(3, (Function<Integer, SessionVersion>) MegolmSessionVersion::fromVersion),
            Arguments.of(0, (Function<Integer, SessionVersion>) OlmSessionVersion::fromVersion),
            Arguments.of(3, (Function<Integer, SessionVersion>) OlmSessionVersion::fromVersion),
            Arguments.of(-1, (Function<Integer, SessionVersion>) MessageType::fromValue),
            Arguments.of(2, (Function<Integer, SessionVersion>) MessageType::fromValue)
        );
    }

    @ParameterizedTest
    @MethodSource("testFailedValues")
    void testFailedVersion(int value, Function<Integer, SessionVersion> parser) {
        assertThatThrownBy(() -> parser.apply(value))
                .isInstanceOf(VodozemacException.class)
                .hasMessageContaining("unknown");
    }

}
