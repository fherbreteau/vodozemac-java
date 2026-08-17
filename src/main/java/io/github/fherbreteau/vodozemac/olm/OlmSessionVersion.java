package io.github.fherbreteau.vodozemac.olm;

import io.github.fherbreteau.vodozemac.SessionVersion;

/**
 * Represents the version of the Olm session protocol to use.
 * <p>
 * The session version determines the cryptographic configuration used
 * for encryption and MAC operations.
 */
public enum OlmSessionVersion implements SessionVersion {
    /** Version 1 — uses truncated MAC, compatible with the original libolm. */
    V1(1),
    /** Version 2 — uses full-length MAC for stronger integrity protection. */
    V2(2);

    private final int value;

    OlmSessionVersion(int value) {
        this.value = value;
    }

    /**
     * Returns the numeric value of this session version.
     *
     * @return the version number (1 for V1, 2 for V2)
     */
    @Override
    public int getValue() {
        return value;
    }

    /**
     * Returns the default Olm session version.
     *
     * @return {@link #V1}
     */
    public static OlmSessionVersion defaultVersion() {
        return V1;
    }

    /**
     * Returns the session version corresponding to the given numeric value.
     *
     * @param version the version number (1 for V1, 2 for V2)
     * @return the associated session version
     * @throws io.github.fherbreteau.vodozemac.exception.VodozemacException if no version matches the given value
     */
    public static OlmSessionVersion fromVersion(int version) {
        return SessionVersion.fromVersion(values(), version, "version");
    }
}
