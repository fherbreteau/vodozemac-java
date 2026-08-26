package io.github.fherbreteau.vodozemac.account;

import java.util.List;
import java.util.Objects;

import io.github.fherbreteau.vodozemac.types.Curve25519PublicKey;

/**
 * Result of generating one-time keys on an {@link Account}.
 * <p>
 * The one-time key store inside an {@link Account} has a limited number of
 * places for one-time keys. If new keys are generated while the store is
 * completely populated, the oldest one-time keys are discarded to make room
 * for new ones.
 * <p>
 * This result contains both the newly created keys and the keys that were
 * discarded to make room.
 *
 * @author François HERBRETEAU
 * @see Account#generateOneTimeKeys(long)
 */
public class OneTimeKeyGenerationResult {
    private final List<Curve25519PublicKey> created;
    private final List<Curve25519PublicKey> removed;

    /**
     * Constructs a new {@code OneTimeKeyGenerationResult}.
     *
     * @param created the public parts of the one-time keys that were created
     * @param removed the public parts of the one-time keys that were discarded
     */
    OneTimeKeyGenerationResult(List<Curve25519PublicKey> created, List<Curve25519PublicKey> removed) {
        this.created = created;
        this.removed = removed;
    }

    /**
     * Returns the public parts of the one-time keys that were created.
     * <p>
     * Each key is a base64-encoded Curve25519 public key.
     *
     * @return a list of newly created one-time key strings
     */
    public List<Curve25519PublicKey> created() {
        return created;
    }

    /**
     * Returns the public parts of the one-time keys that were discarded
     * to make room for the newly created ones.
     * <p>
     * Each key is a base64-encoded Curve25519 public key.
     *
     * @return a list of discarded one-time key strings
     */
    public List<Curve25519PublicKey> removed() {
        return removed;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof OneTimeKeyGenerationResult that)) {
            return false;
        }
        return Objects.equals(created, that.created)
                && Objects.equals(removed, that.removed);
    }

    @Override
    public int hashCode() {
        return Objects.hash(created, removed);
    }

    @Override
    public String toString() {
        return "{" +
            " created='" + created + "'" +
            ", removed='" + removed + "'" +
            "}";
    }
}
