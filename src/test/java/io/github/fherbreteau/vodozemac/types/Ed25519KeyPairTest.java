package io.github.fherbreteau.vodozemac.types;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import static org.assertj.core.api.InstanceOfAssertFactories.STRING;

import java.nio.charset.StandardCharsets;

import io.github.fherbreteau.vodozemac.exception.PickleException;
import org.junit.jupiter.api.Test;

class Ed25519KeyPairTest {

    @Test
    void testKeyPairCreationAndKeyGeneration() {
        try (Ed25519KeyPair keyPair = new Ed25519KeyPair()) {
            // Verify key pair was created successfully
            assertThat(keyPair)
                    .as("Ed25519 keypair should be created successfully")
                    .isNotNull();

            // Test Ed25519 key generation
            Ed25519PublicKey ed25519Key = keyPair.publicKey();
            assertThat(ed25519Key)
                    .as("Ed25519 key should be generated")
                    .isNotNull()
                    .extracting(Ed25519PublicKey::toBase64, STRING)
                    .isNotEmpty()
                    .hasSizeGreaterThan(20); // Should be a reasonable base64 string
        }
    }

    @Test
    void testMessageSigning() {
        try (Ed25519KeyPair keyPair = new Ed25519KeyPair()) {
            String message = "Hello Matrix!";
            Ed25519Signature signature = keyPair.sign(message);

            assertThat(signature)
                    .as("Message signature should be generated")
                    .isNotNull()
                    .extracting(Ed25519Signature::toBase64, STRING)
                    .isNotEmpty()
                    .hasSizeGreaterThan(20); // Should be a reasonable base64 signature

            boolean result = keyPair.publicKey().verify(message, signature);
            assertThat(result)
                    .isTrue();
        }
    }

    @Test
    void testResourceManagement() {
        Ed25519KeyPair keyPair = new Ed25519KeyPair();

        // Verify the account works before closing
        Ed25519PublicKey key = keyPair.publicKey();
        assertThat(key)
                .as("Ed25519KeyPair should work before closing")
                .isNotNull();

        // Close the account
        keyPair.close();

        // Verify that using the account after closing throws an exception
        assertThatThrownBy(keyPair::publicKey)
                .as("Using closed account should throw IllegalStateException")
                .isInstanceOf(IllegalStateException.class)
                .hasMessage("Ed25519KeyPair has been closed");
    }

    @Test
    void testSignWithNullStringMessage() {
        try (Ed25519KeyPair keyPair = new Ed25519KeyPair()) {
            assertThatThrownBy(() -> keyPair.sign((String) null))
                    .as("Signing a null string message should throw NullPointerException")
                    .isInstanceOf(NullPointerException.class)
                    .hasMessage("message");
        }
    }

    @Test
    void testSignWithNullByteArrayMessage() {
        try (Ed25519KeyPair keyPair = new Ed25519KeyPair()) {
            assertThatThrownBy(() -> keyPair.sign((byte[]) null))
                    .as("Signing a null byte array message should throw NullPointerException")
                    .isInstanceOf(NullPointerException.class)
                    .hasMessage("message");
        }
    }

    @Test
    void testSignByteArrayOverload() {
        try (Ed25519KeyPair keyPair = new Ed25519KeyPair()) {
            byte[] message = "Hello Matrix!".getBytes(StandardCharsets.UTF_8);
            Ed25519Signature signature = keyPair.sign(message);

            assertThat(keyPair.publicKey().verify(message, signature))
                    .as("Signature over raw bytes should be valid")
                    .isTrue();
        }
    }

    @Test
    void testVerificationFailureWithDifferentKey() {
        try (Ed25519KeyPair signingKeyPair = new Ed25519KeyPair();
                Ed25519KeyPair otherKeyPair = new Ed25519KeyPair()) {
            Ed25519Signature signature = signingKeyPair.sign("Hello Matrix!");

            assertThat(otherKeyPair.publicKey().verify("Hello Matrix!", signature))
                    .as("Signature from another key pair should not verify")
                    .isFalse();
        }
    }

    @Test
    void testTryWithResources() {
        // This test verifies that try-with-resources works correctly
        Ed25519PublicKey key = null;
        try (Ed25519KeyPair keyPair = new Ed25519KeyPair()) {
            key = keyPair.publicKey();
            assertThat(key)
                    .as("Ed25519KeyPair should work within try-with-resources")
                    .isNotNull();
        }

        // If we get here without exceptions, the resource management worked
        assertThat(key)
                .as("Key should be accessible after try-with-resources block")
                .isNotNull();
    }

    @Test
    void testMultipleKeyPairs() {
        // Test that we can create multiple key pairs
        try (Ed25519KeyPair keyPair1 = new Ed25519KeyPair();
                Ed25519KeyPair keyPair2 = new Ed25519KeyPair()) {

            Ed25519PublicKey key1 = keyPair1.publicKey();
            Ed25519PublicKey key2 = keyPair2.publicKey();

            assertThat(key1)
                    .as("First key pair should generate a valid key")
                    .isNotNull();

            assertThat(key2)
                    .as("Second key pair should generate a valid key")
                    .isNotNull();

            // Different key pairs should have different keys
            assertThat(key1)
                    .as("Different key pairs should have different keys")
                    .isNotEqualTo(key2);
        }
    }

    @Test
    void testPicklingAndUnpickling() {
        // Create an account and get its original keys
        Ed25519PublicKey originalEd25519Key;
        Ed25519Signature originalSignature;
        String pickleData = null;

        try (Ed25519KeyPair originalKeyPair = new Ed25519KeyPair()) {
            originalEd25519Key = originalKeyPair.publicKey();
            originalSignature = originalKeyPair.sign("Test message for pickling");

            // Pickle the account
            pickleData = originalKeyPair.pickle();

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
        try (Ed25519KeyPair unpickledKeyPair = Ed25519KeyPair.unpickle(pickleData)) {
            // Verify that the unpickled key pair has the same keys as the original
            Ed25519PublicKey unpickledEd25519Key = unpickledKeyPair.publicKey();

            assertThat(unpickledEd25519Key)
                    .as("Unpickled key pair should have the same Ed25519 key")
                    .isEqualTo(originalEd25519Key);

            // Verify that the unpickled key pair can sign messages with the same result
            Ed25519Signature unpickledSignature = unpickledKeyPair.sign("Test message for pickling");

            assertThat(unpickledSignature)
                    .as("Unpickled key pair should produce the same signature")
                    .isEqualTo(originalSignature);
        }
    }

    @Test
    void testUnpickleWithNullPickleData() {
        assertThatThrownBy(() -> Ed25519KeyPair.unpickle(null))
                .as("Unpickling null pickle data should throw NullPointerException")
                .isInstanceOf(NullPointerException.class)
                .hasMessage("pickleData");
    }

    @Test
    void testPickleExceptionOnInvalidPickleData() {
        assertThatThrownBy(() -> Ed25519KeyPair.unpickle("invalid-json"))
                .as("Unpickling invalid JSON should throw PickleException")
                .isInstanceOf(PickleException.class);
    }
}
