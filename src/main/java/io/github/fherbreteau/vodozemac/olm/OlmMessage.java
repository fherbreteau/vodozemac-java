package io.github.fherbreteau.vodozemac.olm;
import java.util.Objects;

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
 * the Matrix Olm message format ({@code {"body":"<base64>","type":<int>}}),
 * which is what the native vodozemac layer expects for serialisation and
 * deserialisation.
 *
 * @author François HERBRETEAU
 * @see OlmSession#encrypt(byte[])
 * @see OlmSession#decrypt(OlmMessage)
 * @see MessageType
 */
public class OlmMessage {
    private final String body;
    private final MessageType type;

    OlmMessage(int messageType, String body) {
        this.body = body;
        this.type = MessageType.fromValue(messageType);
    }

    /**
     * Returns the base64-encoded ciphertext body of this message.
     *
     * @return the ciphertext body as a base64 string
     */
    public String body() {
        return body;
    }

    /**
     * Returns the type of this message.
     *
     * @return the {@link MessageType} (pre-key or normal)
     */
    public MessageType type() {
        return type;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof OlmMessage olmMessage)) {
            return false;
        }
        return Objects.equals(body, olmMessage.body)
                && Objects.equals(type, olmMessage.type);
    }

    @Override
    public int hashCode() {
        return Objects.hash(body, type);
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
        return String.format("{\"body\":\"%s\",\"type\":%d}", body, type.value());
    }
}
