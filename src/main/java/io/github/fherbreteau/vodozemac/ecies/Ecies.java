package io.github.fherbreteau.vodozemac.ecies;

import io.github.fherbreteau.vodozemac.NativeHandle;
import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.exception.EciesException;
import io.github.fherbreteau.vodozemac.exception.KeyException;

/**
 * An unestablished ECIES (Elliptic Curve Integrated Encryption Scheme) session.
 * <p>
 * ECIES is a hybrid encryption scheme that uses elliptic curve Diffie-Hellman
 * (X25519) for shared secret establishment and ChaCha20-Poly1305 for symmetric
 * encryption of individual messages. HMAC-SHA256 is used as the key derivation
 * function.
 * <p>
 * ECIES allows a party (the initiator) to establish a communication channel
 * toward another party (the recipient) given knowledge of only its public
 * key. The initiator's key pair is ephemeral and generated anew for each new
 * channel. The initiator must send their ephemeral public key to the recipient
 * so that the recipient can complete the channel establishment on their end.
 * <p>
 * Since the initiator's key is unauthenticated, the recipient has no way of
 * knowing who is contacting them. To protect against active man-in-the-middle
 * (MITM) attacks, an out-of-band confirmation is required after channel
 * establishment using the {@link CheckCode} facility.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 * Once a channel is established via {@link #establishOutboundChannel} or
 * {@link #establishInboundChannel}, this {@code Ecies} object is consumed and
 * can no longer be used.
 *
 * @author François HERBRETEAU
 */
public final class Ecies extends NativeHandle {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    /**
     * Creates a new, random, unestablished ECIES session.
     * <p>
     * This method uses the {@code MATRIX_QR_CODE_LOGIN} application info
     * prefix. If you are using this for a different purpose, consider using
     * {@link #withInfo(String)} instead.
     */
    public Ecies() {
        super(nativeNew());
    }

    private Ecies(long nativePtr) {
        super(nativePtr);
    }

    /**
     * Creates a new, random, unestablished ECIES session with the given
     * application info.
     * <p>
     * The application info is used to derive the various secrets and provide
     * domain separation. Both parties must use the same application info to
     * successfully establish a channel.
     *
     * @param info the application info string
     * @return a new {@code Ecies} session
     */
    public static Ecies withInfo(String info) {
        long ptr = nativeWithInfo(info);
        return new Ecies(ptr);
    }

    /**
     * Returns the ephemeral Curve25519 public key associated with this
     * session.
     * <p>
     * This public key needs to be sent to the other side so that they can
     * establish an ECIES channel.
     *
     * @return the base64-encoded public key
     * @throws IllegalStateException if this {@code Ecies} has been closed or
     *         consumed by channel establishment
     */
    public String publicKey() {
        checkNotClosed();
        return nativePublicKey(nativePtr);
    }

    /**
     * Establishes an outbound ECIES channel using the other side's Curve25519
     * public key and an initial plaintext message.
     * <p>
     * After the channel has been established, the returned
     * {@link EstablishedEcies} can be used to encrypt and decrypt further
     * messages. The other side uses the initial message to establish the same
     * channel on their end via {@link #establishInboundChannel(String)}.
     * <p>
     * This method <b>consumes</b> the {@code Ecies} object — after this call,
     * the {@code Ecies} is invalidated and can no longer be used. The native
     * resources are transferred to the returned {@code OutboundCreationResult}.
     *
     * @param theirPublicKey   a base64-encoded Curve25519 public key
     * @param initialPlaintext the initial plaintext message to encrypt
     * @return an {@link OutboundCreationResult} containing the established
     *         channel and the initial message
     * @throws IllegalStateException if this {@code Ecies} has been closed or
     *         consumed
     * @throws KeyException         if the given public key is invalid or
     *         cannot be decoded
     * @throws EciesException       if channel establishment fails, e.g. due to
     *         a non-contributory key
     */
    public OutboundCreationResult establishOutboundChannel(String theirPublicKey, byte[] initialPlaintext) {
        checkNotClosed();
        try {
            return nativeEstablishOutboundChannel(nativePtr, theirPublicKey, initialPlaintext);
        } finally {
            nativePtr = 0;
        }
    }

    /**
     * Establishes an inbound ECIES channel from an initial message encrypted
     * by the other side.
     * <p>
     * The initial message is obtained from
     * {@link OutboundCreationResult#initialMessage()} and contains the
     * sender's ephemeral public key along with the ciphertext. After the
     * channel has been established, the returned {@link EstablishedEcies} can
     * be used to encrypt and decrypt further messages.
     * <p>
     * This method <b>consumes</b> the {@code Ecies} object — after this call,
     * the {@code Ecies} is invalidated and can no longer be used. The native
     * resources are transferred to the returned {@code InboundCreationResult}.
     *
     * @param message the initial message received from the initiator,
     *        as produced by {@link OutboundCreationResult#initialMessage()}
     * @return an {@link InboundCreationResult} containing the established
     *         channel and the decrypted plaintext
     * @throws IllegalStateException if this {@code Ecies} has been closed or
     *         consumed
     * @throws EciesException       if channel establishment fails, e.g. due to
     *         a non-contributory key or a malformed message
     */
    public InboundCreationResult establishInboundChannel(String message) {
        checkNotClosed();
        try {
            return nativeEstablishInboundChannel(nativePtr, message);
        } finally {
            nativePtr = 0;
        }
    }

    private static native long nativeNew();

    private static native long nativeWithInfo(String info);

    private native String nativePublicKey(long ptr);

    private native OutboundCreationResult nativeEstablishOutboundChannel(long ptr, String theirPublicKey, byte[] initialPlaintext);

    private native InboundCreationResult nativeEstablishInboundChannel(long ptr, String message);

    protected native void nativeFree(long ptr);

}
