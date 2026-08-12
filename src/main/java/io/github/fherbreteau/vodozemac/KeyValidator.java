package io.github.fherbreteau.vodozemac;

import io.github.fherbreteau.vodozemac.exception.KeyException;

public final class KeyValidator {
    private KeyValidator() {
    }

    public static void validateEncryptionKey(byte[] key) {
        if (key.length != 32) {
            throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
        }
    }
}
