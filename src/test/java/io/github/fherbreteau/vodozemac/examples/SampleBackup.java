package io.github.fherbreteau.vodozemac.examples;

import java.nio.charset.StandardCharsets;
import java.util.Objects;

import io.github.fherbreteau.vodozemac.backup.PkDecryption;
import io.github.fherbreteau.vodozemac.backup.PkEncryption;
import io.github.fherbreteau.vodozemac.backup.PkMessage;

public final class SampleBackup {

    private SampleBackup() {
    }

    @SuppressWarnings("java:S106")
    public static void main(String[] args) {
        byte[] plaintext = "It's a secret to everybody".getBytes(StandardCharsets.UTF_8);

        try (PkDecryption decryption = new PkDecryption();
                PkEncryption encryption = PkEncryption.fromKey(decryption.publicKey())) {
            PkMessage message = encryption.encrypt(plaintext);

            System.out.println("Encrypted Backup: " + message.ciphertext());
            System.out.println("Ephemeral key   : " + message.ephemeralKey());
            System.out.println("Mac             : " + message.mac());

            byte[] decrypted = decryption.decrypt(message);
            assert Objects.deepEquals(plaintext, decrypted);
            System.out.println("Backup Roundtrip successful");
        }
    }
}
