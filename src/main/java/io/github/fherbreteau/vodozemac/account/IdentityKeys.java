package io.github.fherbreteau.vodozemac.account;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.types.Curve25519PublicKey;
import io.github.fherbreteau.vodozemac.types.Ed25519PublicKey;

/**
 * Holds the two public identity keys of an {@link Account}.
 * <p>
 * An Olm account has two identity keys:
 * <ul>
 *   <li>An Ed25519 key, used for signing (fingerprint key)</li>
 *   <li>A Curve25519 key, used to establish shared secrets (identity key)</li>
 * </ul>
 * Both keys are represented as base64-encoded strings.
 *
 * @author François HERBRETEAU
 */
public class IdentityKeys {
    private final Ed25519PublicKey ed25519;
    private final Curve25519PublicKey curve25519;

    /**
     * Constructs a new {@code IdentityKeys} object.
     *
     * @param ed25519    the base64-encoded Ed25519 public key
     * @param curve25519 the base64-encoded Curve25519 public key
     */
    IdentityKeys(Ed25519PublicKey ed25519, Curve25519PublicKey curve25519) {
        this.ed25519 = ed25519;
        this.curve25519 = curve25519;
    }

    /**
     * Returns the Ed25519 fingerprint key, used for signing.
     *
     * @return the base64-encoded Ed25519 public key
     */
    public Ed25519PublicKey fingerprintKey() {
        return ed25519;
    }

    /**
     * Returns the Curve25519 identity key, used to establish shared secrets.
     *
     * @return the base64-encoded Curve25519 public key
     */
    public Curve25519PublicKey identityKey() {
        return curve25519;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof IdentityKeys identityKeys)) {
            return false;
        }
        return Objects.equals(ed25519, identityKeys.ed25519)
                && Objects.equals(curve25519, identityKeys.curve25519);
    }

    @Override
    public int hashCode() {
        return Objects.hash(ed25519, curve25519);
    }

    @Override
    public String toString() {
        return "{" +
            " ed25519='" + fingerprintKey() + "'" +
            ", curve25519='" + identityKey() + "'" +
            "}";
    }
}
