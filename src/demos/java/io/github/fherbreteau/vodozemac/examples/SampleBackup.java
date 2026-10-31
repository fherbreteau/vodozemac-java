package io.github.fherbreteau.vodozemac.examples;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;
import java.util.Objects;

import io.github.fherbreteau.vodozemac.backup.PkDecryption;
import io.github.fherbreteau.vodozemac.backup.PkEncryption;
import io.github.fherbreteau.vodozemac.backup.PkMessage;

public final class SampleBackup {

    private static final SecureRandom RANDOM = new SecureRandom();

    private SampleBackup() {
    }

    @SuppressWarnings("java:S106")
    public static void main(String[] args) {
        byte[] plaintext = "It's a secret to everybody".getBytes(StandardCharsets.UTF_8);
        byte[] pickleKey = new byte[32];
        RANDOM.nextBytes(pickleKey);

        String pickleData = null;
        String[] backup = new String[3];

        try (PkDecryption decryption = new PkDecryption();
                PkEncryption encryption = PkEncryption.fromKey(decryption.publicKey())) {
            System.out.println("Backup Key Initialized");
            PkMessage message = encryption.encrypt(plaintext);
            System.out.println("Backup encrypted");
            backup[0] = message.ciphertext();
            backup[1] = message.mac();
            backup[2] = message.ephemeralKey();

            System.out.println("Encrypted Backup: " + message.ciphertext());
            System.out.println("Ephemeral key   : " + message.ephemeralKey());
            System.out.println("Mac             : " + message.mac());

            byte[] decrypted = decryption.decrypt(message);
            assert Objects.deepEquals(plaintext, decrypted);
            System.out.println("Backup Roundtrip successful");
            pickleData = decryption.pickleLegacy(pickleKey);
            System.out.println("Backup Key serialized for deep storage\n");
        }

        try (PkDecryption decryption = PkDecryption.unpickleLegacy(pickleData, pickleKey)) {
            System.out.println("Backup Key loaded from deep storage");
            PkMessage message = PkMessage.from(backup[0], backup[1], backup[2]);

            System.out.println("Encrypted Backup: " + message.ciphertext());
            System.out.println("Ephemeral key   : " + message.ephemeralKey());
            System.out.println("Mac             : " + message.mac());

            byte[] decrypted = decryption.decrypt(message);
            assert Objects.deepEquals(plaintext, decrypted);
            System.out.println("Backup Roundtrip successful");
        }
    }
}
