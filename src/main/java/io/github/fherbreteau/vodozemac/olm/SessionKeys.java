package io.github.fherbreteau.vodozemac.olm;

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
    private final String identityKey;
    private final String baseKey;
    private final String oneTimeKey;

    SessionKeys(String sessionId, String identityKey, String baseKey, String oneTimeKey) {
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
    public String identityKey() {
        return identityKey;
    }

    /**
     * Returns the ephemeral Curve25519 public key created by the session
     * initiator to establish the session.
     *
     * @return the base key as a base64 string
     */
    public String baseKey() {
        return baseKey;
    }

    /**
     * Returns the one-time Curve25519 public key that the initiator
     * downloaded from a key server, which was previously created and
     * published by the recipient.
     *
     * @return the one-time key as a base64 string
     */
    public String oneTimeKey() {
        return oneTimeKey;
    }
}
