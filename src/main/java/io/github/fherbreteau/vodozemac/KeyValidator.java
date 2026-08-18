package io.github.fherbreteau.vodozemac;

import io.github.fherbreteau.vodozemac.exception.KeyException;

/**
 * Utility for validating encryption keys used by the vodozemac pickle format.
 * <p>
 * vodozemac encrypted pickles require a 256-bit (32-byte) key. This class
 * centralises that validation to avoid duplication across the classes that
 * support encrypted pickling ({@code Account}, {@code OlmSession},
 * {@code OutboundGroupSession}, {@code InboundGroupSession}, etc.).
 *
 * @author François HERBRETEAU
 */
public final class KeyValidator {
    private KeyValidator() {
    }

    /**
     * Validates that the given key is exactly 256-bit (32-byte), as required
     * by the vodozemac encrypted pickle format.
     *
     * @param key the key to validate
     * @throws KeyException if the key is not 32 bytes long
     */
    public static void validateEncryptionKey(byte[] key) {
        if (key == null || key.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
    }
}
