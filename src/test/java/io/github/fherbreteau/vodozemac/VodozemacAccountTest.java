package io.github.fherbreteau.vodozemac;

import org.junit.jupiter.api.Test;
import static org.assertj.core.api.Assertions.*;

class VodozemacAccountTest {

    @Test
    void testAccountCreationAndKeyGeneration() {
        try (VodozemacAccount account = new VodozemacAccount()) {
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
        }
    }

    @Test
    void testMessageSigning() {
        try (VodozemacAccount account = new VodozemacAccount()) {
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
        VodozemacAccount account = new VodozemacAccount();

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
        try (VodozemacAccount account = new VodozemacAccount()) {
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
        try (VodozemacAccount account1 = new VodozemacAccount();
             VodozemacAccount account2 = new VodozemacAccount()) {

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
        try (VodozemacAccount account = new VodozemacAccount()) {
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
}