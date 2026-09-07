package io.github.fherbreteau.vodozemac.examples;

import static java.nio.charset.StandardCharsets.UTF_8;

import io.github.fherbreteau.vodozemac.megolm.DecryptedMessage;
import io.github.fherbreteau.vodozemac.megolm.InboundGroupSession;
import io.github.fherbreteau.vodozemac.megolm.MegolmMessage;
import io.github.fherbreteau.vodozemac.megolm.OutboundGroupSession;

public final class SampleMegolm {

    private SampleMegolm() {
    }

    @SuppressWarnings("java:S106")
    public static void main(String[] args) {
        String message = "This is a message";
        try (OutboundGroupSession outbound = new OutboundGroupSession()) {
            String sessionId = outbound.sessionId();
            System.out.println("Outbound: Session id: " + sessionId);

            String sessionKey = outbound.sessionKey();
            System.out.println("Outbound: Session key: " + sessionKey);

            MegolmMessage encrypted = outbound.encrypt(message.getBytes(UTF_8));
            System.out.println("Outbound: Encrypted message: " + encrypted);
            System.out.println("Inbound: Message index: " + outbound.messageIndex());
            System.out.println("Message: Message index: " + encrypted.messageIndex());
            System.out.println("Message: Ciphertext:    " + encrypted.ciphertext());
            System.out.println("Message: Mac:           " + encrypted.mac());
            System.out.println("Message: Signature:     " + encrypted.signature());

            outbound.encrypt(message.getBytes(UTF_8));
            outbound.encrypt(message.getBytes(UTF_8));
            MegolmMessage encrypted4 = outbound.encrypt(message.getBytes(UTF_8));

            try (InboundGroupSession inbound = new InboundGroupSession(sessionKey)) {
                sessionId = inbound.sessionId();
                System.out.println("Inbound: Session id: " + sessionId);

                DecryptedMessage decrypted = inbound.decrypt(encrypted);

                System.out.println("Inbound: Decrypted content: " + new String(decrypted.plaintext(), UTF_8));
                System.out.println("Inbound: Message index: " + decrypted.messageIndex());

                System.out.println("Inbound: First known index: " + inbound.firstKnownIndex());

                DecryptedMessage decrypted2 = inbound.decrypt(encrypted4);

                System.out.println("Inbound: Decrypted content: " + new String(decrypted2.plaintext(), UTF_8));
                System.out.println("Inbound: Message index: " + decrypted2.messageIndex());
            }
        }
    }
}
