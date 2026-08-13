package io.github.fherbreteau.vodozemac.account;

/**
 * Result of creating a dehydrated device from an {@link Account}.
 * <p>
 * A dehydrated device is a device that is stored encrypted on the server
 * and can receive messages when the user has no other active devices.
 *
 * @see Account#toDehydratedDevice(byte[])
 * @see Account#fromDehydratedDevice(String, String, byte[])
 */
public class DehydratedDeviceResult {
    private final String ciphertext;
    private final String nonce;

    /**
     * Constructs a new {@code DehydratedDeviceResult}.
     *
     * @param ciphertext the encrypted dehydrated device, as a base64-encoded string
     * @param nonce      the nonce used for encryption, as a base64-encoded string
     */
    public DehydratedDeviceResult(String ciphertext, String nonce) {
        this.ciphertext = ciphertext;
        this.nonce = nonce;
    }

    /**
     * Returns the encrypted dehydrated device ciphertext.
     *
     * @return the ciphertext as a base64-encoded string
     */
    public String getCiphertext() {
        return ciphertext;
    }

    /**
     * Returns the nonce used to encrypt the dehydrated device.
     *
     * @return the nonce as a base64-encoded string
     */
    public String getNonce() {
        return nonce;
    }
}
