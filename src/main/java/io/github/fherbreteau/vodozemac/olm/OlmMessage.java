package io.github.fherbreteau.vodozemac.olm;

/**
 * A structured Olm message, consisting of a {@link MessageType} and a
 * base64-encoded ciphertext body.
 * <p>
 * An {@code OlmMessage} is produced by {@link OlmSession#encrypt(byte[])} and
 * consumed by {@link OlmSession#decrypt(OlmMessage)} and
 * {@link io.github.fherbreau.vodozemac.account.Account#createInboundSession(
   String, OlmMessage)}.
 * <p>
 * The message will either be a {@link MessageType#PRE_KEY pre-key message} or
 * a {@link MessageType#NORMAL normal message}, depending on whether the session
 * is fully established. A session is fully established once at least one
 * message has been received and decrypted from the other side.
 * <p>
 * The {@link #toString()} method produces a JSON representation compatible with
 * the Matrix Olm message format ({@code {"type":<int>,"body":"<base64>"}}),
 * which is what the native vodozemac layer expects for serialisation and
 * deserialisation.
 *
 * @author François HERBRETEAU
 * @see OlmSession#encrypt(byte[])
 * @see OlmSession#decrypt(OlmMessage)
 * @see MessageType
 */
public class OlmMessage {
    private final MessageType type;
    private final String body;

    OlmMessage(int messageType, String body) {
        this.type = MessageType.fromValue(messageType);
        this.body = body;
    }

    /**
     * Returns the type of this message.
     *
     * @return the {@link MessageType} (pre-key or normal)
     */
    public MessageType getType() {
        return type;
    }

    /**
     * Returns the base64-encoded ciphertext body of this message.
     *
     * @return the ciphertext body as a base64 string
     */
    public String getBody() {
        return body;
    }

    /**
     * Returns the JSON representation of this message, in the format
     * {@code {"body":"<base64>","type":<int>}}.
     * <p>
     * This format is compatible with the Matrix Olm message serialisation
     * used by the native vodozemac layer.
     *
     * @return a JSON string representation of this message
     */
    @Override
    public String toString() {
        String pattern = """
                {"body":"%s","type":%d}
                """;
        return String.format(pattern, body, type.getValue());
    }
}
