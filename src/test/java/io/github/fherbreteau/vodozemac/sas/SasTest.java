package io.github.fherbreteau.vodozemac.sas;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatCode;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import io.github.fherbreteau.vodozemac.exception.SasException;
import org.junit.jupiter.api.Test;

class SasTest {

    private static final String ALICE_MXID = "@alice:example.com";
    private static final String ALICE_DEVICE_ID = "AAAAAAAAAA";
    private static final String BOB_MXID = "@bob:example.com";
    private static final String BOB_DEVICE_ID = "BBBBBBBBBB";

    @Test
    void testSasCreation() {
        Sas copy;
        try (Sas sas = new Sas()) {
            String publicKey = sas.publicKey();
            assertThat(publicKey).isNotNull().isNotEmpty();
            copy = sas;
            assertThat(copy.isClosed()).isFalse();
        }
        assertThat(copy.isClosed()).isTrue();
        assertThatThrownBy(copy::publicKey)
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("Sas has been closed");
        copy.close(); // closing a closed Sas doesn't do anything
        assertThat(copy.isClosed()).isTrue();
    }

    @Test
    void testEstablishedSasCreationAndSameByteGeneration() {
        String info = "TEST";
        EstablishedSas copy;
        try (Sas aliceSas = new Sas(); Sas bobSas = new Sas()) {
            String alicePublicKey = aliceSas.publicKey();
            String bobPublicKey = bobSas.publicKey();

            try (EstablishedSas aliceEstablishedSas = aliceSas.diffieHellman(bobPublicKey);
                    EstablishedSas bobEstablishedSas = bobSas.diffieHellman(alicePublicKey)) {

                assertThat(aliceEstablishedSas).isNotNull();
                assertThat(bobEstablishedSas).isNotNull();

                assertThat(aliceEstablishedSas.ourPublicKey())
                        .isEqualTo(alicePublicKey);
                assertThat(aliceEstablishedSas.theirPublicKey())
                        .isEqualTo(bobPublicKey);
                assertThat(bobEstablishedSas.ourPublicKey())
                        .isEqualTo(bobPublicKey);
                assertThat(bobEstablishedSas.theirPublicKey())
                        .isEqualTo(alicePublicKey);

                SasBytes aliceSasBytes = aliceEstablishedSas.bytes(info);
                SasBytes bobSasBytes = bobEstablishedSas.bytes(info);

                assertThat(aliceSasBytes)
                        .as("The two sides calculated different bytes.")
                        .isEqualTo(bobSasBytes);
                assertThat(aliceSasBytes.emojiIndices())
                        .as("The two sides calculated different emoji indices.")
                        .isEqualTo(bobSasBytes.emojiIndices());
                assertThat(aliceSasBytes.decimals())
                        .as("The two sides calculated different decimals.")
                        .isEqualTo(bobSasBytes.decimals());
                assertThat(aliceSasBytes.bytes())
                        .as("The two sides have raw bytes.")
                        .isEqualTo(bobSasBytes.bytes());

                byte[] aliceBytes = aliceEstablishedSas.bytesRaw(info, 32);
                byte[] bobBytes = bobEstablishedSas.bytesRaw(info, 32);
                assertThat(aliceBytes)
                        .as("The two sides have same generated raw bytes.")
                        .isEqualTo(bobBytes);

                assertThatThrownBy(() -> aliceEstablishedSas.bytesRaw(info, 32 * 255 + 1))
                        .as("Established Sas can't generate more than 8160 bytes")
                        .isInstanceOf(SasException.class)
                        .hasMessage("The given count of bytes was too large");
                copy = aliceEstablishedSas;
                assertThat(copy.isClosed()).isFalse();
            }
        }
        assertThat(copy.isClosed()).isTrue();
        assertThatThrownBy(copy::ourPublicKey)
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("EstablishedSas has been closed");
        copy.close();
        assertThat(copy.isClosed()).isTrue();
    }

    @Test
    void testCalculateMac() {
        String badMac = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        String info = "MATRIX_KEY_VERIFICATION_MAC" + BOB_MXID + BOB_DEVICE_ID + ALICE_MXID + ALICE_DEVICE_ID
                + "1234567890" + "KEY_IDS";

        String message = "ed25519:" + BOB_DEVICE_ID;
        try (Sas aliceSas = new Sas(); Sas bobSas = new Sas()) {
            String alicePublicKey = aliceSas.publicKey();
            String bobPublicKey = bobSas.publicKey();

            try (EstablishedSas aliceEstablishedSas = aliceSas.diffieHellman(bobPublicKey);
                    EstablishedSas bobEstablishedSas = bobSas.diffieHellman(alicePublicKey)) {

                String aliceMac = aliceEstablishedSas.calculateMac(message, info);
                String bobMac = bobEstablishedSas.calculateMac(message, info);

                assertThat(aliceMac).isNotEmpty().isEqualTo(bobMac);

                String aliceinvalidB64Mac = aliceEstablishedSas.calculateMacInvalidBase64(message, info);
                assertThat(aliceinvalidB64Mac).isNotEqualTo(aliceMac);

                assertThatCode(() -> aliceEstablishedSas.verifyMac(message, info, bobMac))
                        .doesNotThrowAnyException();
                assertThatCode(() -> bobEstablishedSas.verifyMac(message, info, aliceMac))
                        .doesNotThrowAnyException();
                assertThatCode(() -> aliceEstablishedSas.verifyMac(message, info, badMac))
                        .isInstanceOf(SasException.class)
                        .hasMessage("The SAS MAC validation didn't succeed: MAC tag mismatch");
            }
        }
    }

    @Test
    void testSasBytesEqualityAndHashCode() {
        SasBytes bytes = new SasBytes(new byte[0], new int[0], new String[0]);

        assertThat(bytes)
                .isNotEqualTo(new Object())
                .doesNotHaveSameHashCodeAs(new Object());
    }
}
