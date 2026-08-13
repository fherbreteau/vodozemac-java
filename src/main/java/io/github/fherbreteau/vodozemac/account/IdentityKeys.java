package io.github.fherbreteau.vodozemac.account;

/**
 * Holds the two public identity keys of an {@link Account}.
 * <p>
 * An Olm account has two identity keys:
 * <ul>
 *   <li>An Ed25519 key, used for signing (fingerprint key)</li>
 *   <li>A Curve25519 key, used to establish shared secrets (identity key)</li>
 * </ul>
 * Both keys are represented as base64-encoded strings.
 */
public class IdentityKeys {
    private final String ed25519;
    private final String curve25519;

    /**
     * Constructs a new {@code IdentityKeys} object.
     *
     * @param ed25519    the base64-encoded Ed25519 public key
     * @param curve25519 the base64-encoded Curve25519 public key
     */
    public IdentityKeys(String ed25519, String curve25519) {
        this.ed25519 = ed25519;
        this.curve25519 = curve25519;
    }

    /**
     * Returns the Ed25519 fingerprint key, used for signing.
     *
     * @return the base64-encoded Ed25519 public key
     */
    public String fingerprintKey() {
        return ed25519;
    }

    /**
     * Returns the Curve25519 identity key, used to establish shared secrets.
     *
     * @return the base64-encoded Curve25519 public key
     */
    public String identityKey() {
        return curve25519;
    }
}
