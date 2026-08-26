package io.github.fherbreteau.vodozemac.types;

import static java.nio.charset.StandardCharsets.UTF_8;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.exception.VodozemacException;

/**
 * Represents an Ed25519 public key used for signature verification.
 * <p>
 * Instances are created by decoding a base64-encoded key string via
 * {@link #fromBase64(String)}. Signatures can be verified with
 * {@link #verify(String, Ed25519Signature)} or
 * {@link #verify(byte[], Ed25519Signature)}.
 *
 * @author François HERBRETEAU
 */
public final class Ed25519PublicKey {

    static {
        NativeLibraryLoader.loadLibrary();
    }

    private final String base64;

    Ed25519PublicKey(String base64) {
        this.base64 = base64;
    }

    public static Ed25519PublicKey fromBase64(String base64) {
        nativeValidate(base64);
        return new Ed25519PublicKey(base64);
    }

    public String toBase64() {
        return base64;
    }

    public boolean verify(String message, Ed25519Signature signature) {
        return verify(message.getBytes(UTF_8), signature);
    }

    public boolean verify(byte[] message, Ed25519Signature signature) {
        try {
            return nativeVerify(base64, message, signature.toBase64());
        } catch (VodozemacException _) {
            return false;
        }
    }

    @Override
    public String toString() {
        return toBase64();
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof Ed25519PublicKey publicKey)) {
            return false;
        }
        return Objects.equals(base64, publicKey.base64);
    }

    @Override
    public int hashCode() {
        return Objects.hash(base64);
    }

    private static native void nativeValidate(String base64);

    private native boolean nativeVerify(String key, byte[] message, String signature);
}
