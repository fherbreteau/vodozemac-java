package io.github.fherbreteau.vodozemac.olm;

import io.github.fherbreteau.vodozemac.SessionVersion;

/**
 * Represents the type of an Olm message.
 * <p>
 * Olm uses two message types. The underlying transport protocol must provide
 * a means for recipients to distinguish between them:
 * <ul>
 *   <li>{@link #PRE_KEY} — a pre-key message, which contains the metadata
 *       necessary to establish a session as well as the encrypted message.
 *       This is the first message sent in a new session.</li>
 *   <li>{@link #NORMAL} — a normal message, which contains only the
 *       ciphertext and the metadata required to decrypt it. These are sent
 *       once the session is fully established.</li>
 * </ul>
 *
 * @author François HERBRETEAU
 * @see OlmMessage
 */
public enum MessageType implements SessionVersion {
    /** A pre-key message (type 0), used to establish a new Olm session. */
    PRE_KEY(0),
    /** A normal message (type 1), sent over an already-established Olm session. */
    NORMAL(1);

    private final int value;

    MessageType(int value) {
        this.value = value;
    }

    /**
     * Returns the numeric value of this message type.
     *
     * @return the type number (0 for {@link #PRE_KEY}, 1 for {@link #NORMAL})
     */
    @Override
    public int getValue() {
        return value;
    }

    /**
     * Returns the message type corresponding to the given numeric value.
     *
     * @param type the numeric message type (0 for pre-key, 1 for normal)
     * @return the associated {@code MessageType}
     * @throws io.github.fherbreteau.vodozemac.exception.VodozemacException if no message type matches the given value
     */
    public static MessageType fromValue(int type) {
        return SessionVersion.fromVersion(values(), type, "message type");
    }
}
