package io.github.fherbreteau.vodozemac.sas;

import java.util.Arrays;
import java.util.Objects;

/**
 * Bytes generated from a shared secret that can be used as the short
 * authentication string (SAS).
 * <p>
 * The bytes can be converted into emoji indices or decimal numbers for
 * visual key verification, as described in the
 * <a href="https://spec.matrix.org/unstable/client-server-api/#sas-method-emoji">Matrix spec</a>.
 *
 * @see EstablishedSas#bytes(String)
 */
public class SasBytes {

    private final byte[] rawBytes;

    private final int[] emojiIndices;

    private final String[] decimals;

    SasBytes(byte[] rawBytes, int[] emojiIndices, String[] decimals) {
        this.rawBytes = rawBytes;
        this.emojiIndices = emojiIndices;
        this.decimals = decimals;
    }

    /**
     * Returns the indices of 7 emojis that can be presented to users to
     * perform the key verification.
     * <p>
     * The table that maps each index to an emoji can be found in the
     * <a href="https://spec.matrix.org/unstable/client-server-api/#sas-method-emoji">Matrix spec</a>.
     *
     * @return an array of 7 emoji indices
     */
    public int[] emojiIndices() {
        return emojiIndices.clone();
    }

    /**
     * Returns the three decimal numbers that can be presented to users to
     * perform the key verification.
     *
     * @return an array of three decimal strings
     */
    public String[] decimals() {
        return decimals.clone();
    }

    /**
     * Returns the raw 6 bytes of the short authentication string that are
     * converted into the emoji and decimal representations.
     *
     * @return an array of 6 raw bytes
     */
    public byte[] bytes() {
        return rawBytes.clone();
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof SasBytes sasBytes)) {
            return false;
        }
        return Objects.deepEquals(rawBytes, sasBytes.rawBytes);
    }

    @Override
    public int hashCode() {
        return Objects.hash(Arrays.hashCode(rawBytes));
    }
}
