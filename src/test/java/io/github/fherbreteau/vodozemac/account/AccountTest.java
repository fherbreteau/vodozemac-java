package io.github.fherbreteau.vodozemac.account;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.*;
import static org.assertj.core.api.InstanceOfAssertFactories.STRING;

class AccountTest {

    @Test
    void testAccountCreationAndKeyGeneration() {
        try (Account account = new Account()) {
            // Verify account was created successfully
            assertThat(account)
                .as("VodozemacAccount should be created successfully")
                .isNotNull();

            // Test Curve25519 key generation
            String curve25519Key = account.curve25519Key();
            assertThat(curve25519Key)
                .as("Curve25519 key should be generated")
                .isNotNull()
                .isNotEmpty()
                .hasSizeGreaterThan(20); // Should be a reasonable base64 string

            // Test Ed25519 key generation
            String ed25519Key = account.ed25519Key();
            assertThat(ed25519Key)
                .as("Ed25519 key should be generated")
                .isNotNull()
                .isNotEmpty()
                .hasSizeGreaterThan(20); // Should be a reasonable base64 string

            // Test that keys are different (they should be different key types)
            assertThat(curve25519Key)
                .as("Curve25519 and Ed25519 keys should be different")
                .isNotEqualTo(ed25519Key);

            // Test that identity Keys contains the ed25519 and curve25519
            IdentityKeys identityKeys = account.identityKeys();

            assertThat(identityKeys)
            .as("Identity Keys should have been generated and contain the the ed25519 and curve25519 keys")
            .isNotNull()
            .extracting(IdentityKeys::getEd25519, IdentityKeys::getCurve25519)
            .containsExactly(ed25519Key, curve25519Key);
        }
    }

    @Test
    void testMessageSigning() {
        try (Account account = new Account()) {
            String message = "Hello Matrix!";
            String signature = account.sign(message);

            assertThat(signature)
                .as("Message signature should be generated")
                .isNotNull()
                .isNotEmpty()
                .hasSizeGreaterThan(20); // Should be a reasonable base64 signature
        }
    }

    @Test
    void testResourceManagement() {
        Account account = new Account();

        // Verify the account works before closing
        String key = account.curve25519Key();
        assertThat(key)
            .as("Account should work before closing")
            .isNotNull()
            .isNotEmpty();

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
        String key = null;
        try (Account account = new Account()) {
            key = account.ed25519Key();
            assertThat(key)
                .as("Account should work within try-with-resources")
                .isNotNull()
                .isNotEmpty();
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

            String key1 = account1.curve25519Key();
            String key2 = account2.curve25519Key();

            assertThat(key1)
                .as("First account should generate a valid key")
                .isNotNull()
                .isNotEmpty();

            assertThat(key2)
                .as("Second account should generate a valid key")
                .isNotNull()
                .isNotEmpty();

            // Different accounts should have different keys
            assertThat(key1)
                .as("Different accounts should have different keys")
                .isNotEqualTo(key2);
        }
    }

    @Test
    void testKeyProperties() {
        try (Account account = new Account()) {
            String curve25519Key = account.curve25519Key();
            String ed25519Key = account.ed25519Key();

            // Test that keys are valid base64 strings
            assertThat(curve25519Key)
                .as("Curve25519 key should be valid base64")
                .matches("[A-Za-z0-9+/=]+");

            assertThat(ed25519Key)
                .as("Ed25519 key should be valid base64")
                .matches("[A-Za-z0-9+/=]+");

            // Test that keys have reasonable lengths for cryptographic keys
            assertThat(curve25519Key.length())
                .as("Curve25519 key should have reasonable length")
                .isBetween(40, 50);

            assertThat(ed25519Key.length())
                .as("Ed25519 key should have reasonable length")
                .isBetween(40, 50);
        }
    }

    @Test
    void testPicklingAndUnpickling() {
        // Create an account and get its original keys
        String originalCurve25519Key;
        String originalEd25519Key;
        String originalSignature;
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
            String unpickledCurve25519Key = unpickledAccount.curve25519Key();
            String unpickledEd25519Key = unpickledAccount.ed25519Key();

            assertThat(unpickledCurve25519Key)
                .as("Unpickled account should have the same Curve25519 key")
                .isEqualTo(originalCurve25519Key);

            assertThat(unpickledEd25519Key)
                .as("Unpickled account should have the same Ed25519 key")
                .isEqualTo(originalEd25519Key);

            // Verify that the unpickled account can sign messages with the same result
            String unpickledSignature = unpickledAccount.sign("Test message for pickling");

            assertThat(unpickledSignature)
                .as("Unpickled account should produce the same signature")
                .isEqualTo(originalSignature);
        }
    }

    @Test
    void testDehydratedDeviceConversion() {
        String originalCurve25519Key;
        String originalEd25519Key;
        String originalSignature;
        DehydratedDeviceResult dehydratexDevice;
        String secretKey = "mySecretKey";

        try (Account originalAccount = new Account()) {
            originalCurve25519Key = originalAccount.curve25519Key();
            originalEd25519Key = originalAccount.ed25519Key();
            originalSignature = originalAccount.sign("Test message for pickling");

            dehydratexDevice = originalAccount.toDehydratedDevice(secretKey);

            // Verify that the dehydrated device is not null and not empty
            assertThat(dehydratexDevice)
                .as("Dehydrated device should not be null")
                .isNotNull()
                .extracting(DehydratedDeviceResult::getCiphertext, STRING)
                .as("Dehydrated device ciphertext should not be null or empty")
                .isNotNull()
                .isNotEmpty();
            assertThat(dehydratexDevice)
                .as("Dehydrated device should not be null")
                .isNotNull()
                .extracting(DehydratedDeviceResult::getNonce, STRING)
                .as("Dehydrated device nonce should not be null or empty")
                .isNotNull()
                .isNotEmpty();
        }

        try (Account rehydratedDevice = Account.fromDehydratedDevice(dehydratexDevice.getCiphertext(), dehydratexDevice.getNonce(), secretKey)) {
            String rehydratedCurve25519Key = rehydratedDevice.curve25519Key();
            String rehydratedEd25519Key = rehydratedDevice.ed25519Key();

            assertThat(rehydratedCurve25519Key)
                .as("Unpickled account should have the same Curve25519 key")
                .isEqualTo(originalCurve25519Key);

            assertThat(rehydratedEd25519Key)
                .as("Unpickled account should have the same Ed25519 key")
                .isEqualTo(originalEd25519Key);

            // Verify that the unpickled account can sign messages with the same result
            String unpickledSignature = rehydratedDevice.sign("Test message for pickling");

            assertThat(unpickledSignature)
                .as("Unpickled account should produce the same signature")
                .isEqualTo(originalSignature);

        }
    }
}