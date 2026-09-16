package io.github.fherbreteau.vodozemac.types;

import static java.nio.charset.StandardCharsets.UTF_8;

import java.util.Objects;

import io.github.fherbreteau.vodozemac.NativeHandle;
import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.ParamNames;
import io.github.fherbreteau.vodozemac.exception.PickleException;

/**
 * An Ed25519 key pair used to sign and verify messages.
 * <p>
 * The key pair is generated randomly at creation and can be serialized with
 * {@link #pickle()} and restored with {@link #unpickle(String)}. The public
 * part of the key pair can be shared with others so they can verify the
 * signatures produced by {@link #sign(String)} using
 * {@link Ed25519PublicKey#verify(String, Ed25519Signature)}.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 *
 * @author François HERBRETEAU
 */
public final class Ed25519KeyPair extends NativeHandle {

    static {
        NativeLibraryLoader.loadLibrary();
    }

    /**
     * Creates a new {@code Ed25519KeyPair} with a new random private key.
     */
    public Ed25519KeyPair() {
        this(nativeNew());
    }

    private Ed25519KeyPair(long ptr) {
        super(ptr, Ed25519KeyPair::nativeFree);
    }

    /**
     * Returns the public key of this key pair.
     *
     * @return the Ed25519 public key
     * @throws IllegalStateException if this key pair has been closed
     */
    public Ed25519PublicKey publicKey() {
        checkNotClosed();
        return nativePublicKey(nativePtr());
    }

    /**
     * Signs the given message using the private key.
     *
     * @param message the message to sign
     * @return the signature
     * @throws IllegalStateException if this key pair has been closed
     */
    public Ed25519Signature sign(String message) {
        Objects.requireNonNull(message, ParamNames.MESSAGE);
        return sign(message.getBytes(UTF_8));
    }

    /**
     * Signs the given message using the private key.
     *
     * @param message the message to sign
     * @return the signature
     * @throws IllegalStateException if this key pair has been closed
     */
    public Ed25519Signature sign(byte[] message) {
        Objects.requireNonNull(message, ParamNames.MESSAGE);
        checkNotClosed();
        return nativeSign(nativePtr(), message);
    }

    /**
     * Converts the key pair into a JSON string representation.
     * <p>
     * The pickle contains the private key in clear text and should be
     * stored securely.
     *
     * @return a JSON string representing the key pair
     * @throws IllegalStateException if this key pair has been closed
     */
    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr());
    }

    /**
     * Restores an {@code Ed25519KeyPair} from a previously saved JSON string.
     *
     * @param pickleData the JSON string from {@link #pickle()}
     * @return a restored {@code Ed25519KeyPair}
     * @throws PickleException if the data cannot be deserialized
     */
    public static Ed25519KeyPair unpickle(String pickleData) {
        Objects.requireNonNull(pickleData, ParamNames.PICKLE_DATA);
        long nativePtr = nativeUnpickle(pickleData);
        return new Ed25519KeyPair(nativePtr);
    }

    private static native long nativeNew();

    private static native long nativeUnpickle(String pickleData);

    private native Ed25519PublicKey nativePublicKey(long ptr);

    private native Ed25519Signature nativeSign(long ptr, byte[] message);

    private native String nativePickle(long ptr);

    private static native void nativeFree(long ptr);
}
