package io.github.fherbreteau.vodozemac.sas;

import io.github.fherbreteau.vodozemac.NativeHandle;
import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.exception.KeyException;

/**
 * User-friendly key verification using short authentication strings (SAS).
 * <p>
 * The verification process is heavily inspired by Phil Zimmermann's ZRTP
 * key agreement handshake. A core part of key agreement in ZRTP is the
 * {@code hash commitment}: the party that begins the key sharing process sends
 * a {@code hash} of their part of the Diffie-Hellman exchange but does not send
 * the part itself until they have received the other party's part.
 * <p>
 * The verification process can be used to verify the Ed25519 identity key of
 * an {@code Account}.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 */
public final class Sas extends NativeHandle {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    /**
     * Creates a new random verification object.
     * <p>
     * This creates an ephemeral curve25519 keypair that can be used to
     * establish a shared secret.
     */
    public Sas() {
        super(nativeNew());
    }

    /**
     * Returns the public key that can be used to establish a shared secret.
     *
     * @return the base64-encoded public key
     * @throws IllegalStateException if this {@code Sas} has been closed or
     *         consumed by {@link #diffieHellman(String)}
     */
    public String publicKey() {
        checkNotClosed();
        return nativePublicKey(nativePtr);
    }

    /**
     * Establishes a SAS secret by performing a Diffie-Hellman handshake with
     * another public key.
     * <p>
     * This method <b>consumes</b> the {@code Sas} object — after this call,
     * the {@code Sas} is invalidated and can no longer be used. The native
     * resources are transferred to the returned {@link EstablishedSas}.
     *
     * @param theirPublicKey a base64-encoded Curve25519 public key
     * @return a new {@code EstablishedSas} which can be used to generate
     *         {@code SasBytes} and calculate MACs
     * @throws IllegalStateException if this {@code Sas} has already been closed
     *         or consumed
     * @throws KeyException if the given public key is invalid or cannot be
     *         decoded
     */
    public EstablishedSas diffieHellman(String theirPublicKey) {
        checkNotClosed();
        long ptr = nativePtr;
        nativePtr = 0;
        return nativeDiffieHellman(ptr, theirPublicKey);
    }

    private static native long nativeNew();

    private native String nativePublicKey(long ptr);

    private native EstablishedSas nativeDiffieHellman(long ptr, String theirPublicKey);

    protected native void nativeFree(long ptr);
}
