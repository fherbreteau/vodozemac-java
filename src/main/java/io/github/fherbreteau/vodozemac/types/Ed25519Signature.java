package io.github.fherbreteau.vodozemac.types;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.NativeLibraryLoader;

/**
 * Represents an Ed25519 signature.
 * <p>
 * Instances are created by decoding a base64-encoded signature string via
 * {@link #fromBase64(String)} and can be converted back to base64 with
 * {@link #toBase64()}.
 *
 * @author François HERBRETEAU
 */
public final class Ed25519Signature {

    static {
        NativeLibraryLoader.loadLibrary();
    }

    private final String base64;

    Ed25519Signature(String base64) {
        this.base64 = base64;
    }

    public static Ed25519Signature fromBase64(String base64) {
        nativeValidate(base64);
        return new Ed25519Signature(base64);
    }

    public String toBase64() {
        return base64;
    }

    @Override
    public String toString() {
        return toBase64();
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof Ed25519Signature signature)) {
            return false;
        }
        return Objects.equals(base64, signature.base64);
    }

    @Override
    public int hashCode() {
        return Objects.hash(base64);
    }

    private static native void nativeValidate(String base64);
}
