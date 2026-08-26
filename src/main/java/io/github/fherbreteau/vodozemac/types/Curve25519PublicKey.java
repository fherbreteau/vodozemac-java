package io.github.fherbreteau.vodozemac.types;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.NativeLibraryLoader;

/**
 * Represents a Curve25519 public key used for X25519 key agreement.
 * <p>
 * Instances are created by decoding a base64-encoded key string via
 * {@link #fromBase64(String)} and can be converted back to base64 with
 * {@link #toBase64()}.
 *
 * @author François HERBRETEAU
 */
public final class Curve25519PublicKey {

    static {
        NativeLibraryLoader.loadLibrary();
    }

    private final String base64;

    Curve25519PublicKey(String base64) {
        this.base64 = base64;
    }

    public static Curve25519PublicKey fromBase64(String base64) {
        nativeValidate(base64);
        return new Curve25519PublicKey(base64);
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
        if (!(o instanceof Curve25519PublicKey signature)) {
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
