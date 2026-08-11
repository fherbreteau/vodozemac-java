package io.github.fherbreteau.vodozemac;

import static java.nio.charset.StandardCharsets.UTF_8;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.ecies.Ecies;
import io.github.fherbreteau.vodozemac.ecies.EstablishedEcies;
import io.github.fherbreteau.vodozemac.ecies.InboundCreationResult;
import io.github.fherbreteau.vodozemac.ecies.OutboundCreationResult;
import io.github.fherbreteau.vodozemac.exception.VodozemacException;

public final class SampleEcies {

    private SampleEcies() {
    }

    public static void main() {
        byte[] plaintext = "It's a secret to everybody".getBytes(UTF_8);

        try (Ecies alice = new Ecies();
            Ecies bob = new Ecies()) {
            System.out.println("Alice: Ecies public Key: " + alice.publicKey());
            System.out.println("Bob  : Ecies public Key: " + bob.publicKey());

            OutboundCreationResult aliceResult = alice.establishOutboundChannel(bob.publicKey(), plaintext);
            System.out.println("Alice: Initial message.   : " + aliceResult.getInitialMessage());

            InboundCreationResult bobResult = bob.establishInboundChannel(aliceResult.getInitialMessage());

            assert Objects.deepEquals(plaintext, bobResult.getPlaintext());
            System.out.println("Alice: Sent message.   : " + new String(plaintext));
            System.out.println("Bob. : Recieved message: " + new String(bobResult.getPlaintext()));

            EstablishedEcies aliceEcies = aliceResult.getEstablishedEcies();
            EstablishedEcies bobEcies = bobResult.getEstablishedEcies();

            if (!aliceEcies.checkCode().equals(bobEcies.checkCode())) {
                throw new VodozemacException("The check code must match; possible active MITM attack in progress");
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
