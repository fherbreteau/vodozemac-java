package io.github.fherbreteau.vodozemac.account;

import java.util.List;

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
 * @see Account#generateOneTimeKeys(long)
 */
public class OneTimeKeyGenerationResult {
    private final List<String> created;
    private final List<String> removed;

    /**
     * Constructs a new {@code OneTimeKeyGenerationResult}.
     *
     * @param created the public parts of the one-time keys that were created
     * @param removed the public parts of the one-time keys that were discarded
     */
    public OneTimeKeyGenerationResult(List<String> created, List<String> removed) {
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
    public List<String> getCreated() {
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
    public List<String> getRemoved() {
        return removed;
    }
}
