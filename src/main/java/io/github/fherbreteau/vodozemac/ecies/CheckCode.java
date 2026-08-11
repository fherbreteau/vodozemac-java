package io.github.fherbreteau.vodozemac.ecies;

import java.util.Arrays;
import java.util.Objects;

/**
 * A check code that can be used to confirm that two {@link EstablishedEcies}
 * objects share the same secret.
 * <p>
 * The check code is derived from the shared secret and is intended to be
 * shared out-of-band between the initiator and the recipient to protect
 * against active man-in-the-middle (MITM) attacks.
 * <p>
 * Since the initiator device can always tell whether a MITM attack is in
 * progress after channel establishment, this code technically carries only a
 * single bit of information, representing whether the initiator has determined
 * that the channel is "secure" or "not secure". However, given that this will
 * need to be interactively confirmed by the user, there is a risk that the
 * user would confirm a dialogue without paying attention to its content. By
 * expanding this single bit into a deterministic two-digit check code, the
 * user is forced to pay more attention by having to enter it instead of just
 * clicking through a dialogue.
 */
public class CheckCode {

    private final byte[] bytes;

    private final int digit;

    CheckCode(byte[] bytes, int digit) {
        this.bytes = bytes;
        this.digit = digit;
    }

    /**
     * Returns the raw bytes of the check code.
     *
     * @return a 2-byte array representing the check code
     */
    public byte[] asBytes() {
        return bytes;
    }

    /**
     * Returns the check code as a two-digit base-10 number.
     * <p>
     * The number is in the range 0&ndash;99. It should be displayed with a
     * leading zero if the first digit is 0, e.g. using
     * {@code String.format("%02d", checkCode.toDigit())}.
     *
     * @return the check code as a two-digit number
     */
    public int toDigit() {
        return digit;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof CheckCode checkCode)) {
            return false;
        }
        return digit == checkCode.digit && Objects.deepEquals(bytes, checkCode.bytes);
    }

    @Override
    public int hashCode() {
        return Objects.hash(Arrays.hashCode(bytes), digit);
    }
}
