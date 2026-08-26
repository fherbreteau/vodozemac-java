package io.github.fherbreteau.vodozemac;

import static java.nio.charset.StandardCharsets.UTF_8;
import static org.assertj.core.api.Assertions.assertThat;

import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.account.OneTimeKeyGenerationResult;
import io.github.fherbreteau.vodozemac.backup.PkDecryption;
import io.github.fherbreteau.vodozemac.ecies.Ecies;
import io.github.fherbreteau.vodozemac.ecies.OutboundCreationResult;
import io.github.fherbreteau.vodozemac.megolm.InboundGroupSession;
import io.github.fherbreteau.vodozemac.megolm.OutboundGroupSession;
import io.github.fherbreteau.vodozemac.olm.OlmSessionVersion;
import io.github.fherbreteau.vodozemac.sas.Sas;
import io.github.fherbreteau.vodozemac.types.Curve25519PublicKey;
import org.junit.jupiter.api.Test;

class NativeHandleTest {

    @Test
    void testAccountIsClosed() {
        NativeHandle handle = new Account();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testOlmSessionIsClosed() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {
            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            Curve25519PublicKey bobOneTimeKey = result.created().iterator().next();
            bobAccount.markKeysAsPublished();

            NativeHandle handle = aliceAccount.createOutboundSession(
                    OlmSessionVersion.V2, bobAccount.curve25519Key(), bobOneTimeKey);
            assertThat(handle.isClosed()).isFalse();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
        }
    }

    @Test
    void testOutboundGroupSessionIsClosed() {
        NativeHandle handle = new OutboundGroupSession();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testInboundGroupSessionIsClosed() {
        String sessionKey;
        try (OutboundGroupSession outbound = new OutboundGroupSession()) {
            sessionKey = outbound.sessionKey();
        }

        NativeHandle handle = new InboundGroupSession(sessionKey);
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testSasIsClosed() {
        NativeHandle handle = new Sas();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testEstablishedSasIsClosed() {
        try (Sas aliceSas = new Sas(); Sas bobSas = new Sas()) {
            NativeHandle handle = aliceSas.diffieHellman(bobSas.publicKey());
            assertThat(handle.isClosed()).isFalse();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
            handle.close();
            assertThat(handle.isClosed()).isTrue();
        }
    }

    @Test
    void testEciesIsClosed() {
        NativeHandle handle = new Ecies();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testEstablishedEciesIsClosed() {
        Ecies alice = new Ecies();
        Ecies bob = new Ecies();
        OutboundCreationResult result = alice.establishOutboundChannel(
                bob.publicKey(), "plaintext".getBytes(UTF_8));
        NativeHandle handle = result.establishedEcies();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        alice.close();
        bob.close();
    }

    @Test
    void testPkDecryptionIsClosed() {
        NativeHandle handle = new PkDecryption();
        assertThat(handle.isClosed()).isFalse();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
        handle.close();
        assertThat(handle.isClosed()).isTrue();
    }

    @Test
    void testNativeLibraryIsLoaded() {
        assertThat(NativeLibraryLoader.isLoaded())
                .as("Native Libray is Loaded")
                .isTrue();
    }
}
