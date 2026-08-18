package io.github.fherbreteau.vodozemac;

import java.util.stream.Stream;

import io.github.fherbreteau.vodozemac.exception.ConversionException;

/**
 * Common interface for session version enums such as
 * {@link io.github.fherbreteau.vodozemac.olm.OlmSessionVersion} and
 * {@link io.github.fherbreteau.vodozemac.megolm.MegolmSessionVersion}.
 * <p>
 * Provides a shared {@link #fromVersion(Enum[], int, String)} utility that
 * resolves a numeric version to its corresponding enum constant, throwing
 * a {@link VodozemacException} when no match is found.
 *
 * @author François HERBRETEAU
 */
public interface SessionVersion {
    /**
     * Returns the numeric value of this session version.
     *
     * @return the version number (1 for V1, 2 for V2)
     */
    int value();

    /**
     * Resolves the given numeric version to its corresponding enum constant.
     *
     * @param values  the enum constants to search (typically {@code E.values()})
     * @param version the numeric version to match
     * @param label   a human-readable label used in the error message if no
     *                match is found (e.g. {@code "version"} or
     *                {@code "message type"})
     * @param <E>     the enum type, which must also implement
     *                {@link SessionVersion}
     * @return the enum constant whose {@link #value()} matches the given version
     * @throws VodozemacException if no constant matches the given version
     */
    static <E extends Enum<E> & SessionVersion> E fromVersion(E[] values, int version, String label) {
        return Stream.of(values)
            .filter(v -> version == v.value())
            .findFirst()
            .orElseThrow(() -> new ConversionException("unknown " + label + " " + version));
    }
}
