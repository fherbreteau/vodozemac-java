package io.github.fherbreteau.vodozemac;

import java.util.stream.Stream;

import io.github.fherbreteau.vodozemac.exception.ConversionException;

public interface SessionVersion {
    /**
     * Returns the numeric value of this session version.
     *
     * @return the version number (1 for V1, 2 for V2)
     */
    int getValue();

    static <E extends Enum<E> & SessionVersion> E fromVersion(E[] values, int version, String label) {
        return Stream.of(values)
            .filter(v -> version == v.getValue())
            .findFirst()
            .orElseThrow(() -> new ConversionException("unknown " + label + " " + version));
    }
}
