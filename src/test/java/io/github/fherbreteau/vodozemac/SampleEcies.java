package io.github.fherbreteau.vodozemac;

import static java.nio.charset.StandardCharsets.UTF_8;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.ecies.Ecies;
import io.github.fherbreteau.vodozemac.ecies.EstablishedEcies;
import io.github.fherbreteau.vodozemac.ecies.InboundCreationResult;
import io.github.fherbreteau.vodozemac.ecies.OutboundCreationResult;

public final class SampleEcies {

    private SampleEcies() {
    }

    @SuppressWarnings("java:S106")
    public static void main(String[] args) {
        byte[] plaintext = "It's a secret to everybody".getBytes(UTF_8);

        try (Ecies alice = new Ecies();
            Ecies bob = new Ecies()) {
            System.out.println("Alice: Ecies public Key: " + alice.publicKey());
            System.out.println("Bob  : Ecies public Key: " + bob.publicKey());

            OutboundCreationResult aliceResult = alice.establishOutboundChannel(bob.publicKey(), plaintext);
            System.out.println("Alice: Initial message.   : " + aliceResult.initialMessage());

            InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.initialMessage());

            assert Objects.deepEquals(plaintext, bobResult.plaintext());
            System.out.println("Alice: Sent message.   : " + new String(plaintext));
            System.out.println("Bob. : Recieved message: " + new String(bobResult.plaintext()));

            EstablishedEcies aliceEcies = aliceResult.establishedEcies();
            EstablishedEcies bobEcies = bobResult.establishedEcies();

            if (!aliceEcies.checkCode().equals(bobEcies.checkCode())) {
                throw new IllegalStateException("The check code must match; possible active MITM attack in progress");
            }

            plaintext = "Another plaintext".getBytes(UTF_8);
            String message = bobEcies.encrypt(plaintext);
            System.out.println("Alice: Sent message.   : " + message);
            byte[] decrypted = aliceEcies.decrypt(message);

            assert Objects.deepEquals(plaintext, decrypted);
            System.out.println("ECIES Exchange successful");
        }
    }
}
