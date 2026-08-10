package io.github.fherbreteau.vodozemac.sas;

import io.github.fherbreteau.vodozemac.exception.SasException;

/**
 * User-friendly key verification using short authentication strings (SAS)
 * with an established shared secret.
 * <p>
 * An {@code EstablishedSas} is obtained by calling
 * {@link Sas#diffieHellman(String)} with the other party's public key. Once
 * established, it can be used to:
 * <ul>
 *   <li>Generate {@link SasBytes} (emoji indices, decimal numbers, raw bytes)
 *       for visual key verification via {@link #bytes(String)}</li>
 *   <li>Generate and verify MACs to cryptographically confirm the shared
 *       secret via {@link #calculateMac(String, String)} and
 *       {@link #verifyMac(String, String, String)}</li>
 * </ul>
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 */
public class EstablishedSas implements AutoCloseable {

    private long nativePtr;

    EstablishedSas(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    /**
     * Generates {@link SasBytes} using HKDF with the shared secret as the
     * input key material.
     * <p>
     * The info string should be agreed upon beforehand — both parties need
     * to use the same info string.
     *
     * @param info the info string, agreed upon by both parties
     * @return a new {@code SasBytes} containing emoji indices, decimals,
     *         and raw bytes
     * @throws IllegalStateException if this {@code EstablishedSas} has been closed
     */
    public SasBytes bytes(String info) {
        checkNotClosed();
        return nativeBytes(nativePtr, info);
    }

    /**
     * Generates the given number of bytes using HKDF with the shared secret
     * as the input key material.
     * <p>
     * The info string should be agreed upon beforehand — both parties need
     * to use the same info string. The number of bytes that can be generated
     * is limited to 32 * 255 = 8160 bytes.
     *
     * @param info  the info string, agreed upon by both parties
     * @param count the number of bytes to generate
     * @return an array of bytes
     * @throws IllegalStateException if this {@code EstablishedSas} has been closed
     * @throws SasException         if the requested count exceeds the maximum
     */
    public byte[] bytesRaw(String info, int count) {
        checkNotClosed();
        return nativeBytesRaw(nativePtr, info, count);
    }

    /**
     * Calculates a MAC for the given input using the info string as
     * additional data.
     * <p>
     * This should be used to calculate a MAC of the Ed25519 identity key of
     * an {@code Account}.
     *
     * @param input the input string to MAC
     * @param info  the info string, agreed upon by both parties
     * @return the MAC as a base64-encoded string
     * @throws IllegalStateException if this {@code EstablishedSas} has been closed
     */
    public String calculateMac(String input, String info) {
        checkNotClosed();
        return nativeCalculateMac(nativePtr, input, info);
    }

    /**
     * Calculates a MAC for the given input using the info string as
     * additional data, returning the MAC as an invalid base64-encoded string.
     * <p>
     * <b>Warning</b>: This method should never be used unless you require
     * libolm compatibility. Libolm used to incorrectly encode their MAC because
     * the input buffer was reused as the output buffer. This method replicates
     * the buggy behaviour.
     *
     * @param input the input string to MAC
     * @param info  the info string, agreed upon by both parties
     * @return the MAC as an invalid base64-encoded string
     * @throws IllegalStateException if this {@code EstablishedSas} has been closed
     */
    public String calculateMacInvalidBase64(String input, String info) {
        checkNotClosed();
        return nativeCalculateMacInvalidBase64(nativePtr, input, info);
    }

    /**
     * Verifies a MAC that was previously created using
     * {@link #calculateMac(String, String)}.
     * <p>
     * Users should calculate a MAC and send it to the other side, they
     * should then verify each other's MAC using this method.
     *
     * @param input the input string that was MAC-ed
     * @param info  the info string, agreed upon by both parties
     * @param mac   the MAC received from the other user, as a base64-encoded string
     * @throws IllegalStateException if this {@code EstablishedSas} has been closed
     * @throws SasException          if the MAC verification fails
     */
    public void verifyMac(String input, String info, String mac) {
        checkNotClosed();
        nativeVerifyMac(nativePtr, input, info, mac);
    }

    /**
     * Returns the public key that was created by us, that was used to
     * establish the shared secret.
     *
     * @return the base64-encoded public key
     * @throws IllegalStateException if this {@code EstablishedSas} has been closed
     */
    public String ourPublicKey() {
        checkNotClosed();
        return nativeOurPublicKey(nativePtr);
    }

    /**
     * Returns the public key that was created by the other party, that was
     * used to establish the shared secret.
     *
     * @return the base64-encoded public key
     * @throws IllegalStateException if this {@code EstablishedSas} has been closed
     */
    public String theirPublicKey() {
        checkNotClosed();
        return nativeTheirPublicKey(nativePtr);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("EstablishedSas has been closed");
        }
    }

    /* For test usage only */
    boolean isClosed() {
        return nativePtr == 0;
    }

    /**
     * Closes the {@code EstablishedSas} by releasing its associated native
     * resources.
     *
     * {@inheritDoc}
     */
    @Override
    public void close() {
        if (nativePtr != 0) {
            nativeFree(nativePtr);
            nativePtr = 0;
        }
    }

    private native SasBytes nativeBytes(long ptr, String info);

    private native byte[] nativeBytesRaw(long ptr, String info, int count);

    private native String nativeCalculateMac(long ptr, String input, String info);

    private native String nativeCalculateMacInvalidBase64(long ptr, String input, String info);

    private native void nativeVerifyMac(long ptr, String input, String info, String mac);

    private native String nativeOurPublicKey(long ptr);

    private native String nativeTheirPublicKey(long ptr);

    private native void nativeFree(long ptr);
}
