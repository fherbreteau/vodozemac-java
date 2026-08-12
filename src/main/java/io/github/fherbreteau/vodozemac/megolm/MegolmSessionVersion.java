package io.github.fherbreteau.vodozemac.megolm;

import java.util.stream.Stream;

import io.github.fherbreteau.vodozemac.exception.VodozemacException;

/**
 * Represents the version of the Megolm session protocol to use.
 * <p>
 * The session version determines the cryptographic configuration used
 * for encryption and MAC operations.
 */
public enum MegolmSessionVersion {
    /** Version 1 — uses truncated MAC, compatible with the original libolm. */
    V1(1),
    /** Version 2 — uses full-length MAC for stronger integrity protection. */
    V2(2);

    private final int value;

    MegolmSessionVersion(int value) {
        this.value = value;
    }

    /**
     * Returns the numeric value of this session version.
     *
     * @return the version number (1 for V1, 2 for V2)
     */
    public int getValue() {
        return value;
    }

    /**
     * Returns the default Megolm session version.
     *
     * @return {@link #V1}
     */
    public static MegolmSessionVersion defaultVersion() {
        return V1;
    }

    /**
     * Return the session version based on its version number.
     * @param version the version number
     * @return the associated session version
     */
    public static MegolmSessionVersion fromVersion(int version) {
        return Stream.of(values())
            .filter(v -> version == v.value)
            .findFirst()
            .orElseThrow(() -> new VodozemacException("unknown version " + version));
    }
}
