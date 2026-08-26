package io.github.fherbreteau.vodozemac.account;

import static java.nio.charset.StandardCharsets.UTF_8;
import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import static org.assertj.core.api.InstanceOfAssertFactories.STRING;
import static org.assertj.core.api.InstanceOfAssertFactories.list;

import java.security.SecureRandom;
import java.util.List;
import java.util.Map;
import java.util.Optional;

import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.olm.InboundCreationResult;
import io.github.fherbreteau.vodozemac.olm.OlmMessage;
import io.github.fherbreteau.vodozemac.olm.OlmSession;
import io.github.fherbreteau.vodozemac.olm.OlmSessionVersion;
import io.github.fherbreteau.vodozemac.types.Curve25519PublicKey;
import io.github.fherbreteau.vodozemac.types.Ed25519PublicKey;
import io.github.fherbreteau.vodozemac.types.Ed25519Signature;
import org.junit.jupiter.api.Test;

class AccountTest {

    private final SecureRandom random = new SecureRandom();

    @Test
    void testAccountCreationAndKeyGeneration() {
        try (Account account = new Account()) {
            // Verify account was created successfully
            assertThat(account)
                    .as("VodozemacAccount should be created successfully")
                    .isNotNull();

            // Test Curve25519 key generation
            Curve25519PublicKey curve25519Key = account.curve25519Key();
            assertThat(curve25519Key)
                    .as("Curve25519 key should be generated")
                    .isNotNull()
                    .extracting(Curve25519PublicKey::toBase64, STRING)
                    .isNotEmpty()
                    .hasSizeGreaterThan(20); // Should be a reasonable base64 string

            // Test Ed25519 key generation
            Ed25519PublicKey ed25519Key = account.ed25519Key();
            assertThat(ed25519Key)
                    .as("Ed25519 key should be generated")
                    .isNotNull()
                    .extracting(Ed25519PublicKey::toBase64, STRING)
                    .isNotEmpty()
                    .hasSizeGreaterThan(20); // Should be a reasonable base64 string

            // Test that keys are different (they should be different key types)
            assertThat(curve25519Key.toBase64())
                    .as("Curve25519 and Ed25519 keys should be different")
                    .isNotEqualTo(ed25519Key.toBase64());

            // Test that identity Keys contains the ed25519 and curve25519
            IdentityKeys identityKeys = account.identityKeys();

            assertThat(identityKeys)
                    .as("Identity Keys should have been generated and contain the the ed25519 and curve25519 keys")
                    .isNotNull()
                    .extracting(IdentityKeys::fingerprintKey, IdentityKeys::identityKey)
                    .containsExactly(ed25519Key, curve25519Key);
        }
    }

    @Test
    void testMessageSigning() {
        try (Account account = new Account()) {
            byte[] message = "Hello Matrix!".getBytes(UTF_8);
            Ed25519Signature signature = account.sign(message);

            assertThat(signature)
                    .as("Message signature should be generated")
                    .isNotNull()
                    .extracting(Ed25519Signature::toBase64, STRING)
                    .isNotEmpty()
                    .hasSizeGreaterThan(20); // Should be a reasonable base64 signature

            boolean result = account.ed25519Key().verify(message, signature);
            assertThat(result)
                    .isTrue();
        }
    }

    @Test
    void testResourceManagement() {
        Account account = new Account();

        // Verify the account works before closing
        Curve25519PublicKey key = account.curve25519Key();
        assertThat(key)
                .as("Account should work before closing")
                .isNotNull();

        // Close the account
        account.close();

        // Verify that using the account after closing throws an exception
        assertThatThrownBy(account::curve25519Key)
                .as("Using closed account should throw IllegalStateException")
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("Account has been closed");
    }

    @Test
    void testTryWithResources() {
        // This test verifies that try-with-resources works correctly
        Ed25519PublicKey key = null;
        try (Account account = new Account()) {
            key = account.ed25519Key();
            assertThat(key)
                    .as("Account should work within try-with-resources")
                    .isNotNull();
        }

        // If we get here without exceptions, the resource management worked
        assertThat(key)
                .as("Key should be accessible after try-with-resources block")
                .isNotNull();
    }

    @Test
    void testMultipleAccounts() {
        // Test that we can create multiple accounts
        try (Account account1 = new Account();
                Account account2 = new Account()) {

            Curve25519PublicKey key1 = account1.curve25519Key();
            Curve25519PublicKey key2 = account2.curve25519Key();

            assertThat(key1)
                    .as("First account should generate a valid key")
                    .isNotNull();

            assertThat(key2)
                    .as("Second account should generate a valid key")
                    .isNotNull();

            // Different accounts should have different keys
            assertThat(key1)
                    .as("Different accounts should have different keys")
                    .isNotEqualTo(key2);
        }
    }

    @Test
    void testKeyProperties() {
        try (Account account = new Account()) {
            Curve25519PublicKey curve25519Key = account.curve25519Key();
            Ed25519PublicKey ed25519Key = account.ed25519Key();

            // Test that keys are valid base64 strings
            assertThat(curve25519Key)
                    .as("Curve25519 key should be valid base64")
                    .extracting(Curve25519PublicKey::toBase64, STRING)
                    .isBase64();

            assertThat(ed25519Key)
                    .as("Ed25519 key should be valid base64")
                    .extracting(Ed25519PublicKey::toBase64, STRING)
                    .isBase64();

            // Test that keys have reasonable lengths for cryptographic keys
            assertThat(curve25519Key.toBase64().length())
                    .as("Curve25519 key should have reasonable length")
                    .isBetween(40, 50);

            assertThat(ed25519Key.toBase64().length())
                    .as("Ed25519 key should have reasonable length")
                    .isBetween(40, 50);
        }
    }

    @Test
    void testPicklingAndUnpickling() {
        // Create an account and get its original keys
        Curve25519PublicKey originalCurve25519Key;
        Ed25519PublicKey originalEd25519Key;
        Ed25519Signature originalSignature;
        String pickleData = null;

        try (Account originalAccount = new Account()) {
            originalCurve25519Key = originalAccount.curve25519Key();
            originalEd25519Key = originalAccount.ed25519Key();
            originalSignature = originalAccount.sign("Test message for pickling");

            // Pickle the account
            pickleData = originalAccount.pickle();

            // Verify that pickle data is not null and not empty
            assertThat(pickleData)
                    .as("Pickle data should not be null or empty")
                    .isNotNull()
                    .isNotEmpty();

            // Verify that pickle data is valid JSON
            assertThat(pickleData)
                    .as("Pickle data should start with { and end with }")
                    .startsWith("{")
                    .endsWith("}");
        }

        // Unpickle the account
        try (Account unpickledAccount = Account.unpickle(pickleData)) {
            // Verify that the unpickled account has the same keys as the original
            Curve25519PublicKey unpickledCurve25519Key = unpickledAccount.curve25519Key();
            Ed25519PublicKey unpickledEd25519Key = unpickledAccount.ed25519Key();

            assertThat(unpickledCurve25519Key)
                    .as("Unpickled account should have the same Curve25519 key")
                    .isEqualTo(originalCurve25519Key);

            assertThat(unpickledEd25519Key)
                    .as("Unpickled account should have the same Ed25519 key")
                    .isEqualTo(originalEd25519Key);

            // Verify that the unpickled account can sign messages with the same result
            Ed25519Signature unpickledSignature = unpickledAccount.sign("Test message for pickling");

            assertThat(unpickledSignature)
                    .as("Unpickled account should produce the same signature")
                    .isEqualTo(originalSignature);
        }
    }

    @Test
    void testPicklingAndUnpicklingWithEncryption() {
        // Create an account and get its original keys
        Curve25519PublicKey originalCurve25519Key;
        Ed25519PublicKey originalEd25519Key;
        Ed25519Signature originalSignature;
        String pickleData = null;

        byte[] key = new byte[32];
        random.nextBytes(key);

        try (Account originalAccount = new Account()) {
            originalCurve25519Key = originalAccount.curve25519Key();
            originalEd25519Key = originalAccount.ed25519Key();
            originalSignature = originalAccount.sign("Test message for pickling");

            // Pickle the account
            pickleData = originalAccount.pickle(key);

            // Verify that pickle data is not null and not empty
            assertThat(pickleData)
                    .as("Pickle data should not be null or empty")
                    .isNotNull()
                    .isNotEmpty();

            // Verify that pickle data is valid JSON
            assertThat(pickleData)
                    .as("Pickle data is encrypted")
                    .isNotEmpty()
                    .isBase64();
        }

        // Unpickle the account
        try (Account unpickledAccount = Account.unpickle(pickleData, key)) {
            // Verify that the unpickled account has the same keys as the original
            Curve25519PublicKey unpickledCurve25519Key = unpickledAccount.curve25519Key();
            Ed25519PublicKey unpickledEd25519Key = unpickledAccount.ed25519Key();

            assertThat(unpickledCurve25519Key)
                    .as("Unpickled account should have the same Curve25519 key")
                    .isEqualTo(originalCurve25519Key);

            assertThat(unpickledEd25519Key)
                    .as("Unpickled account should have the same Ed25519 key")
                    .isEqualTo(originalEd25519Key);

            // Verify that the unpickled account can sign messages with the same result
            Ed25519Signature unpickledSignature = unpickledAccount.sign("Test message for pickling");

            assertThat(unpickledSignature)
                    .as("Unpickled account should produce the same signature")
                    .isEqualTo(originalSignature);
        }
    }

    @Test
    void testDehydratedDeviceConversion() {
        Curve25519PublicKey originalCurve25519Key;
        Ed25519PublicKey originalEd25519Key;
        Ed25519Signature originalSignature;
        DehydratedDeviceResult dehydratexDevice;
        byte[] key = new byte[32];
        random.nextBytes(key);

        try (Account originalAccount = new Account()) {
            originalCurve25519Key = originalAccount.curve25519Key();
            originalEd25519Key = originalAccount.ed25519Key();
            originalSignature = originalAccount.sign("Test message for pickling");

            dehydratexDevice = originalAccount.toDehydratedDevice(key);

            // Verify that the dehydrated device is not null and not empty
            assertThat(dehydratexDevice)
                    .as("Dehydrated device should not be null")
                    .isNotNull()
                    .extracting(DehydratedDeviceResult::ciphertext, STRING)
                    .as("Dehydrated device ciphertext should not be null or empty")
                    .isNotNull()
                    .isNotEmpty();
            assertThat(dehydratexDevice)
                    .as("Dehydrated device should not be null")
                    .isNotNull()
                    .extracting(DehydratedDeviceResult::nonce, STRING)
                    .as("Dehydrated device nonce should not be null or empty")
                    .isNotNull()
                    .isNotEmpty();
        }

        try (Account rehydratedDevice = Account.fromDehydratedDevice(dehydratexDevice.ciphertext(),
                dehydratexDevice.nonce(), key)) {
            Curve25519PublicKey rehydratedCurve25519Key = rehydratedDevice.curve25519Key();
            Ed25519PublicKey rehydratedEd25519Key = rehydratedDevice.ed25519Key();

            assertThat(rehydratedCurve25519Key)
                    .as("Unpickled account should have the same Curve25519 key")
                    .isEqualTo(originalCurve25519Key);

            assertThat(rehydratedEd25519Key)
                    .as("Unpickled account should have the same Ed25519 key")
                    .isEqualTo(originalEd25519Key);

            // Verify that the unpickled account can sign messages with the same result
            Ed25519Signature unpickledSignature = rehydratedDevice.sign("Test message for pickling");

            assertThat(unpickledSignature)
                    .as("Unpickled account should produce the same signature")
                    .isEqualTo(originalSignature);
        }
    }

    @Test
    void testKeyGeneration() {
        try (Account account = new Account()) {
            long maxOneTimeKeys = account.maxNumberOfOneTimeKeys();
            assertThat(maxOneTimeKeys)
                    .as("Return the number of One-time Keys to deploy to the server")
                    .isBetween(0L, 100L);
            long storedKeyCount = account.storedOneTimeKeyCount();
            assertThat(storedKeyCount)
                    .as("A new account should have no one-time keys")
                    .isZero();
            OneTimeKeyGenerationResult result = account.generateOneTimeKeys(1L);
            assertThat(result)
                    .as("Should generate at least 1 one-time key")
                    .isNotNull()
                    .extracting(OneTimeKeyGenerationResult::created, list(Curve25519PublicKey.class))
                    .isNotEmpty()
                    .singleElement()
                    .extracting(Curve25519PublicKey::toBase64, STRING)
                    .isNotEmpty();
            assertThat(result.removed())
                    .as("No one-time key should be removed on first generation")
                    .isEmpty();
            Map<String, Curve25519PublicKey> oneTimeKeys = account.unpublishedOneTimeKeys();
            assertThat(oneTimeKeys)
                    .hasSize(1);
            storedKeyCount = account.storedOneTimeKeyCount();
            assertThat(storedKeyCount)
                    .as("A one-time key should be unpublished")
                    .isEqualTo(1);
            Optional<Curve25519PublicKey> fallbackKey = account.generateFallbackKey();
            assertThat(fallbackKey)
                    .as("First fallback key generation should not return a previous key")
                    .isEmpty();
            Map<String, Curve25519PublicKey> fallbackKeys = account.unpublishedFallbackKey();
            assertThat(fallbackKeys)
                    .hasSize(1);

            // Forget the previously used fallback key
            // No session was established with the fallback key, so nothing was "used"
            boolean forgot = account.forgetFallbackKey();
            assertThat(forgot)
                    .as("No previously used fallback key to forget")
                    .isFalse();

            // Mark all the keys as published
            account.markKeysAsPublished();
            oneTimeKeys = account.unpublishedOneTimeKeys();
            assertThat(oneTimeKeys)
                    .isEmpty();
            fallbackKeys = account.unpublishedFallbackKey();
            assertThat(fallbackKeys)
                    .isEmpty();
        }
    }

    @Test
    void testCreateOutboundSession() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {

            // Bob generates a one-time key
            OneTimeKeyGenerationResult result = bobAccount.generateOneTimeKeys(1L);
            assertThat(result)
                    .as("Bob should generate one-time keys")
                    .isNotNull()
                    .extracting(OneTimeKeyGenerationResult::created, list(Curve25519PublicKey.class))
                    .singleElement()
                    .isNotNull();

            Map<String, Curve25519PublicKey> oneTimeKeys = bobAccount.unpublishedOneTimeKeys();
            assertThat(oneTimeKeys)
                    .as("Bob should have one unpublished one-time key")
                    .hasSize(1);

            Curve25519PublicKey bobCurve25519Key = bobAccount.curve25519Key();
            Curve25519PublicKey bobOneTimeKey = oneTimeKeys.values().iterator().next();

            // Alice creates an outbound session with Bob's identity key and one-time key
            try (OlmSession session = aliceAccount.createOutboundSession(
                    OlmSessionVersion.V2, bobCurve25519Key, bobOneTimeKey)) {

                assertThat(session)
                        .as("Outbound OlmSession should be created")
                        .isNotNull();
            }
        }
    }

    @Test
    void testEncryptedPickleWithInvalidKeyThrowsException() {
        try (Account account = new Account()) {
            assertThatThrownBy(() -> account.pickle(new byte[16]))
                    .as("Pickle with invalid key size should throw KeyException")
                    .isInstanceOf(KeyException.class)
                    .hasMessageContaining("256-bit (32-byte)");
        }
    }

    @Test
    void testEncryptedPickleWithNullKeyThrowsException() {
        try (Account account = new Account()) {
            assertThatThrownBy(() -> account.pickle(null))
                    .as("Pickle with invalid key size should throw KeyException")
                    .isInstanceOf(KeyException.class)
                    .hasMessageContaining("256-bit (32-byte)");
        }
    }

    @Test
    void testEncryptedUnpickleWithInvalidKeyThrowsException() {
        assertThatThrownBy(() -> Account.unpickle("invalid", new byte[16]))
                .as("Unpickle with invalid key size should throw KeyException")
                .isInstanceOf(KeyException.class)
                .hasMessageContaining("256-bit (32-byte)");
    }

    @Test
    void testDehydratedDeviceWithInvalidKeyThrowsException() {
        try (Account account = new Account()) {
            assertThatThrownBy(() -> account.toDehydratedDevice(new byte[16]))
                    .as("Dehydrated device with invalid key size should throw KeyException")
                    .isInstanceOf(KeyException.class)
                    .hasMessageContaining("256-bit (32-byte)");
        }
    }

    @Test
    void testFromDehydratedDeviceWithInvalidKeyThrowsException() {
        assertThatThrownBy(() -> Account.fromDehydratedDevice("invalid", "invalid", new byte[16]))
                .as("From dehydrated device with invalid key size should throw KeyException")
                .isInstanceOf(KeyException.class)
                .hasMessageContaining("256-bit (32-byte)");
    }

    @Test
    void testAccountUnpickleLegacy() {
        String pickleData =
                "u71hZK7akJasMFQqOKwZIyGfWiswSshezAEhIcWrNlbB7D+v0WIoPA+/gFAvzWv0"
                + "TRnZJ/torMmxEh8tM90vHJx6EZuVxFlcN9niiems6i4c46CCtN5hQ9ErXwuLSv3HF"
                + "eDbbKvNZmMXZFHPX+cZGhCX56zMg90GV2kOLRWnfrCQYMbagdW+SjnRIBaUltjy+4"
                + "HELyE70xFbJZ/9tvawDNASW5GAiHw9BGaPr8wMxoIXLCJFEjCaPg";
        byte[] pickleKey = new byte[32];

        try (Account account = Account.unpickleLegacy(pickleData, pickleKey)) {
            assertThat(account)
                    .as("Unpickled legacy account should be created")
                    .isNotNull();

            assertThat(account.ed25519Key())
                    .as("Legacy account should have a valid Ed25519 key")
                    .isNotNull()
                    .extracting(Ed25519PublicKey::toBase64, STRING)
                    .isNotEmpty();

            assertThat(account.curve25519Key())
                    .as("Legacy account should have a valid Curve25519 key")
                    .isNotNull()
                    .extracting(Curve25519PublicKey::toBase64, STRING)
                    .isNotEmpty();
            assertThat(account.pickleLegacy(pickleKey))
                .isEqualTo(pickleData);
        }
    }

    @Test
    void testCreateOutboundSessionWithDefaultVersion() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {
            bobAccount.generateOneTimeKeys(1L);
            Map<String, Curve25519PublicKey> oneTimeKeys = bobAccount.unpublishedOneTimeKeys();
            Curve25519PublicKey bobCurve25519Key = bobAccount.curve25519Key();
            Curve25519PublicKey bobOneTimeKey = oneTimeKeys.values().iterator().next();

            try (OlmSession session = aliceAccount.createOutboundSession(bobCurve25519Key, bobOneTimeKey)) {
                assertThat(session).as("Outbound session with default version should be created").isNotNull();
            }
        }
    }

    @Test
    void testCreateInboundSessionWithDefaultVersion() {
        try (Account aliceAccount = new Account();
                Account bobAccount = new Account()) {
            bobAccount.generateOneTimeKeys(1L);
            Map<String, Curve25519PublicKey> oneTimeKeys = bobAccount.unpublishedOneTimeKeys();
            Curve25519PublicKey bobCurve25519Key = bobAccount.curve25519Key();
            Curve25519PublicKey bobOneTimeKey = oneTimeKeys.values().iterator().next();

            try (OlmSession outboundSession = aliceAccount.createOutboundSession(bobCurve25519Key, bobOneTimeKey)) {
                String plaintext = "Hello Bob";
                OlmMessage encrypted = outboundSession.encrypt(plaintext.getBytes(UTF_8));

                InboundCreationResult inboundResult = bobAccount.createInboundSession(
                        aliceAccount.curve25519Key(), encrypted);
                assertThat(inboundResult).as("Inbound session with default version should be created").isNotNull();
                assertThat(new String(inboundResult.plaintext(), java.nio.charset.StandardCharsets.UTF_8))
                        .isEqualTo(plaintext);
            }
        }
    }

    @Test
    void testIdentityKeysEqualsHashCodeToString() {
        Ed25519PublicKey ed1 = Ed25519PublicKey.fromBase64("NnTo+WL1n6ZjGN1EdHKtrYMRKAlrNUlxrZLtX0hDkbs");
        Curve25519PublicKey cv1 = Curve25519PublicKey.fromBase64("WHKTK+K7GSjf83JuPfGV0KAZjxQU/3HKOb0DD1MaOm4");
        Ed25519PublicKey ed2 = Ed25519PublicKey.fromBase64("Zv95Ka4ThW9hogCkG0MLTI+0+i9K6qS/S3tdfuWsMI4");
        Curve25519PublicKey cv2 = Curve25519PublicKey.fromBase64("YEgdMTY8JPLnIvFCGg/66vxcsA53GSMlyRBsqhv4hn8");
        IdentityKeys keys = new IdentityKeys(ed1, cv1);
        IdentityKeys same = new IdentityKeys(ed1, cv1);
        IdentityKeys differentEd = new IdentityKeys(ed2, cv1);
        IdentityKeys differentCv = new IdentityKeys(ed1, cv2);

        assertThat(keys).isEqualTo(keys)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(differentEd)
                .isNotEqualTo(differentCv)
                .isNotEqualTo("not IdentityKeys")
                .isNotEqualTo(null);
        assertThat(keys.toString()).contains("ed25519", "curve25519");
    }

    @Test
    void testOneTimeKeyGenerationResultEqualsHashCodeToString() {
        Curve25519PublicKey k1 = Curve25519PublicKey.fromBase64("WHKTK+K7GSjf83JuPfGV0KAZjxQU/3HKOb0DD1MaOm4");
        Curve25519PublicKey k2 = Curve25519PublicKey.fromBase64("YEgdMTY8JPLnIvFCGg/66vxcsA53GSMlyRBsqhv4hn8");
        OneTimeKeyGenerationResult result = new OneTimeKeyGenerationResult(List.of(k1), List.of(k2));
        OneTimeKeyGenerationResult same = new OneTimeKeyGenerationResult(List.of(k1), List.of(k2));
        OneTimeKeyGenerationResult differentK1 = new OneTimeKeyGenerationResult(List.of(), List.of(k2));
        OneTimeKeyGenerationResult differentK2 = new OneTimeKeyGenerationResult(List.of(k1), List.of());

        assertThat(result).isEqualTo(result)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(differentK1)
                .isNotEqualTo(differentK2)
                .isNotEqualTo("not a result")
                .isNotEqualTo(null);
        assertThat(result.toString()).contains("created", "removed");
    }

    @Test
    void testDehydratedDeviceResultEqualsHashCodeToString() {
        DehydratedDeviceResult result = new DehydratedDeviceResult("ct", "nonce");
        DehydratedDeviceResult same = new DehydratedDeviceResult("ct", "nonce");
        DehydratedDeviceResult different = new DehydratedDeviceResult("ct2", "nonce");
        DehydratedDeviceResult different2 = new DehydratedDeviceResult("ct", "nonce2");

        assertThat(result).isEqualTo(result)
                .isEqualTo(same)
                .hasSameHashCodeAs(same)
                .isNotEqualTo(different)
                .isNotEqualTo(different2)
                .isNotEqualTo("not a result")
                .isNotEqualTo(null);
        assertThat(result.toString()).contains("ciphertext", "nonce");
    }
}
