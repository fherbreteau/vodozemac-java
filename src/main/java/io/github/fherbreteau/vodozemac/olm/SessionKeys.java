package io.github.fherbreteau.vodozemac.olm;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.types.Curve25519PublicKey;

/**
 * The set of Curve25519 public keys that were used to establish an Olm
 * session.
 * <p>
 * A {@code SessionKeys} value is obtained from
 * {@link OlmSession#sessionKeys()} and contains three public keys:
 * <ul>
 *   <li>the long-term Curve25519 <i>identity</i> key of the session
 *       initiator,</li>
 *   <li>the ephemeral Curve25519 <i>base</i> key created by the initiator to
 *       establish the session, and</li>
 *   <li>the Curve25519 <i>one-time</i> key that the initiator downloaded from
 *       a key server and which was previously published by the recipient.</li>
 * </ul>
 * The session ID is derived as the SHA-256 hash of the concatenation of these
 * three keys, making it (probabilistically) globally unique.
 *
 * @author François HERBRETEAU
 * @see OlmSession#sessionKeys()
 */
public class SessionKeys {
    private final String sessionId;
    private final Curve25519PublicKey identityKey;
    private final Curve25519PublicKey baseKey;
    private final Curve25519PublicKey oneTimeKey;

    SessionKeys(String sessionId, Curve25519PublicKey identityKey, Curve25519PublicKey baseKey, Curve25519PublicKey oneTimeKey) {
        this.sessionId = sessionId;
        this.identityKey = identityKey;
        this.baseKey = baseKey;
        this.oneTimeKey = oneTimeKey;
    }

    /**
     * Returns the globally unique session ID, base64-encoded.
     * <p>
     * The session ID is the SHA-256 hash of the concatenation of the
     * identity key, the base key and the one-time key.
     *
     * @return the session ID as a base64 string
     */
    public String sessionId() {
        return sessionId;
    }

    /**
     * Returns the long-term Curve25519 public key of the session initiator.
     *
     * @return the identity key as a base64 string
     */
    public Curve25519PublicKey identityKey() {
        return identityKey;
    }

    /**
     * Returns the ephemeral Curve25519 public key created by the session
     * initiator to establish the session.
     *
     * @return the base key as a base64 string
     */
    public Curve25519PublicKey baseKey() {
        return baseKey;
    }

    /**
     * Returns the one-time Curve25519 public key that the initiator
     * downloaded from a key server, which was previously created and
     * published by the recipient.
     *
     * @return the one-time key as a base64 string
     */
    public Curve25519PublicKey oneTimeKey() {
        return oneTimeKey;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof SessionKeys sessionKeys)) {
            return false;
        }
        return Objects.equals(sessionId, sessionKeys.sessionId)
                && Objects.equals(identityKey, sessionKeys.identityKey)
                && Objects.equals(baseKey, sessionKeys.baseKey)
                && Objects.equals(oneTimeKey, sessionKeys.oneTimeKey);
    }

    @Override
    public int hashCode() {
        return Objects.hash(sessionId, identityKey, baseKey, oneTimeKey);
    }

    @Override
    public String toString() {
        return "{" +
            " sessionId='" + sessionId + "'" +
            ", identityKey='" + identityKey + "'" +
            ", baseKey='" + baseKey + "'" +
            ", oneTimeKey='" + oneTimeKey + "'" +
            "}";
    }
}
