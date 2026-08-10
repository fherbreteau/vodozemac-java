package io.github.fherbreteau;

import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Base64;

import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.account.OneTimeKeyGenerationResult;
import io.github.fherbreteau.vodozemac.olm.InboundCreationResult;
import io.github.fherbreteau.vodozemac.olm.OlmSession;

public final class Sample {

    private Sample() {
    }

    @SuppressWarnings("java:S106")
    public static void main(String[] args) {
        try (Account aliceAccount = new Account(); Account bobAccount = new Account()) {
            // Extract Alice Identity keys
            System.out.println("Alice: Fingerprint key: " + aliceAccount.identityKeys().fingerprintKey());
            System.out.println("Alice: Identity Key: " + aliceAccount.identityKeys().identityKey());
            System.out.println("Alice: Signature: " + aliceAccount.sign("Hello Matrix!"));

            // Extract Bob Identity keys
            System.out.println("Bob : Fingerprint key: " + bobAccount.identityKeys().fingerprintKey());
            System.out.println("Bob : Ed25519: " + bobAccount.identityKeys().identityKey());
            System.out.println("Bob : Signature: " + bobAccount.sign("Hello Matrix!"));

            // Generate 1 one-time key for Bob
            OneTimeKeyGenerationResult bobOneTimeKeys = bobAccount.generateOneTimeKeys(1L);
            String bobOneTimeKey = bobOneTimeKeys.getCreated().iterator().next();

            bobAccount.markKeysAsPublished();

            // Create a Olm session between Alice and Bob
            String encrypted;
            String alicePickleSession;
            try (OlmSession outboundOlmSession = aliceAccount.createOutbpundSession(bobAccount.curve25519Key(), bobOneTimeKey)) {
                System.out.println("Alice: Outbound olm session's session id: " + outboundOlmSession.sessionId());

                String message = "Hello Bob";
                encrypted = outboundOlmSession.encrypt(message.getBytes());
                System.out.println("Alice: Olm encrypted message : " + encrypted);

                alicePickleSession = outboundOlmSession.pickle();
            }

            InboundCreationResult result = bobAccount.createInboundSession(aliceAccount.curve25519Key(), encrypted);
            try (OlmSession inboundOlmSession = result.getSession()) {
                System.out.println("Bob: Inbound olm session's session id: " + inboundOlmSession.sessionId());

                System.out.println("Bob: Received message: " + new String(result.getPlaintext(), StandardCharsets.UTF_8));

                String message = "Hello Alice";

                encrypted = inboundOlmSession.encrypt(message.getBytes());
                System.out.println("Bob: Olm encrypted message : " + encrypted);
            }

            try (OlmSession outboundOlmSession = OlmSession.unpickle(alicePickleSession)) {
                byte[] message = outboundOlmSession.decrypt(encrypted);
                System.out.println("Alice: Received message: " + new String(message, StandardCharsets.UTF_8));
            }
        }

        byte[] key = new byte[32];
        Arrays.fill(key, (byte) 0);
        String result = Base64.getEncoder().encodeToString(key);
        System.out.println("result: " +  result);
    }
}
