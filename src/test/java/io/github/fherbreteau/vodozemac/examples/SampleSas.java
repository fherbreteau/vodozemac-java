package io.github.fherbreteau.vodozemac.examples;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.sas.EstablishedSas;
import io.github.fherbreteau.vodozemac.sas.Sas;
import io.github.fherbreteau.vodozemac.sas.SasBytes;

public final class SampleSas {

    private SampleSas() {

    }

    @SuppressWarnings("java:S106")
    public static void main(String[] args) {

        String agreedInfo = "AGREED_INFO";

        try (Sas alice = new Sas(); Sas bob = new Sas()) {
            String bobPublicKey = bob.publicKey();
            System.out.println("Alice: Sas public Key: " + alice.publicKey());
            System.out.println("Bob: Sas public Key: " + bob.publicKey());

            try (EstablishedSas bobSas = bob.diffieHellman(alice.publicKey());
                EstablishedSas aliceSas = alice.diffieHellman(bobPublicKey)) {

                SasBytes aliceSasBytes = aliceSas.bytes(agreedInfo);
                SasBytes bobSasBytes = bobSas.bytes(agreedInfo);

                System.out.println("Alice: Decimals " + String.join(", ", aliceSasBytes.decimals()));
                System.out.println("Bob: Decimals " + String.join(", ", bobSasBytes.decimals()));

                assert Objects.deepEquals(aliceSasBytes.emojiIndices(), bobSasBytes.emojiIndices());

                String aliceMac = aliceSas.calculateMac("message", agreedInfo);
                String bobMac = bobSas.calculateMac("message", agreedInfo);

                System.out.println("Alice: Calculated MAC:" + aliceMac);
                System.out.println("Bob  : Calculated MAC:" + bobMac);

                assert Objects.equals(aliceMac, bobMac);

                aliceSas.verifyMac("message", agreedInfo, bobMac);
                bobSas.verifyMac("message", agreedInfo, aliceMac);

                System.out.println("SAS Verification is complete");
            }
        }
    }
}
