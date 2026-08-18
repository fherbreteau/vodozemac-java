package io.github.fherbreteau.vodozemac.account;

import static io.github.fherbreteau.vodozemac.KeyValidator.validateEncryptionKey;

import java.util.Map;
import java.util.Optional;

import io.github.fherbreteau.vodozemac.NativeHandle;
import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.exception.KeyException;
import io.github.fherbreteau.vodozemac.exception.PickleException;
import io.github.fherbreteau.vodozemac.exception.SessionCreationException;
import io.github.fherbreteau.vodozemac.olm.InboundCreationResult;
import io.github.fherbreteau.vodozemac.olm.OlmMessage;
import io.github.fherbreteau.vodozemac.olm.OlmSession;
import io.github.fherbreteau.vodozemac.olm.OlmSessionVersion;

/**
 * An Olm Account manages all cryptographic keys used on a device.
 * <p>
 * An account holds identity keys (Ed25519 for signing, Curve25519 for key
 * agreement), one-time keys, and fallback keys. It is used to create Olm
 * sessions for end-to-end encrypted communication with other devices.
 * <p>
 * This class implements {@link AutoCloseable} and should be used in a
 * try-with-resources block to ensure native resources are properly released.
 *
 * @author François HERBRETEAU
 */
public final class Account extends NativeHandle {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    /**
     * Creates a new {@code Account} with new random identity keys.
     */
    public Account() {
        super(nativeNew());
    }

    private Account(long nativePtr) {
        super(nativePtr);
    }

    /**
     * Returns the identity keys of this account.
     *
     * @return the {@link IdentityKeys} containing both Ed25519 and Curve25519 public keys
     * @throws IllegalStateException if this account has been closed
     */
    public IdentityKeys identityKeys() {
        checkNotClosed();
        return nativeIdentityKeys(nativePtr);
    }

    /**
     * Returns the public Ed25519 key of this account.
     *
     * @return the base64-encoded Ed25519 public key
     * @throws IllegalStateException if this account has been closed
     */
    public String ed25519Key() {
        checkNotClosed();
        return nativeEd25519Key(nativePtr);
    }

    /**
     * Returns the public Curve25519 key of this account.
     *
     * @return the base64-encoded Curve25519 public key
     * @throws IllegalStateException if this account has been closed
     */
    public String curve25519Key() {
        checkNotClosed();
        return nativeCurve25519Key(nativePtr);
    }

    /**
     * Signs the given message using the Ed25519 fingerprint key.
     *
     * @param message the message to sign
     * @return the base64-encoded signature
     * @throws IllegalStateException if this account has been closed
     */
    public String sign(String message) {
        checkNotClosed();
        return nativeSign(nativePtr, message);
    }

    /**
     * Returns the maximum number of one-time keys the client should keep
     * on the server.
     *
     * @return the maximum number of one-time keys
     * @throws IllegalStateException if this account has been closed
     */
    public long maxNumberOfOneTimeKeys() {
        checkNotClosed();
        return nativeMaxNumberOfOneTimeKeys(nativePtr);
    }

    /**
     * Creates an {@link OlmSession} with the given identity key and
     * one-time key, using the default session version.
     *
     * @param identityKey the recipient's Curve25519 identity key
     * @param oneTimeKey  the recipient's Curve25519 one-time key
     * @return a new {@code OlmSession}
     * @throws IllegalStateException    if this account has been closed
     * @throws KeyException            if the keys cannot be decoded
     * @throws SessionCreationException if session creation fails
     */
    public OlmSession createOutboundSession(String identityKey, String oneTimeKey) {
        return createOutboundSession(OlmSessionVersion.defaultVersion(), identityKey, oneTimeKey);
    }

    /**
     * Creates an {@link OlmSession} with the given identity key and
     * one-time key.
     *
     * @param sessionVersion the Olm session protocol version to use
     * @param identityKey    the recipient's Curve25519 identity key
     * @param oneTimeKey     the recipient's Curve25519 one-time key
     * @return a new {@code OlmSession}
     * @throws IllegalStateException    if this account has been closed
     * @throws KeyException            if the keys cannot be decoded
     * @throws SessionCreationException if session creation fails
     */
    public OlmSession createOutboundSession(OlmSessionVersion sessionVersion, String identityKey, String oneTimeKey) {
        checkNotClosed();

        return nativeCreateOutboundSession(nativePtr, sessionVersion.value(), identityKey, oneTimeKey);
    }

    /**
     * Creates an inbound {@link OlmSession} from a pre-key message, using
     * the default session version.
     *
     * @param theirIdentityKey the sender's Curve25519 identity key
     * @param preKeyMessage    the {@link OlmMessage} received from the sender;
     *                         must be a {@link io.github.fherbreteau.vodozemac.olm.MessageType#PRE_KEY pre-key message}
     * @return an {@link InboundCreationResult} containing the new session
     *         and the decrypted plaintext of the pre-key message
     * @throws IllegalStateException      if this account has been closed
     * @throws KeyException              if the identity key cannot be decoded
     * @throws SessionCreationException  if session creation fails
     */
    public InboundCreationResult createInboundSession(String theirIdentityKey, OlmMessage preKeyMessage) {
        return createInboundSession(OlmSessionVersion.defaultVersion(), theirIdentityKey, preKeyMessage);
    }

    /**
     * Creates an inbound {@link OlmSession} from a pre-key message.
     *
     * @param sessionVersion   the Olm session protocol version to use
     * @param theirIdentityKey the sender's Curve25519 identity key
     * @param preKeyMessage    the {@link OlmMessage} received from the sender;
     *                         must be a {@link io.github.fherbreteau.vodozemac.olm.MessageType#PRE_KEY pre-key message}
     * @return an {@link InboundCreationResult} containing the new session
     *         and the decrypted plaintext of the pre-key message
     * @throws IllegalStateException      if this account has been closed
     * @throws KeyException              if the identity key cannot be decoded
     * @throws SessionCreationException  if session creation fails
     */
    public InboundCreationResult createInboundSession(OlmSessionVersion sessionVersion, String theirIdentityKey,
            OlmMessage preKeyMessage) {
        checkNotClosed();

        return nativeCreateInboundSession(nativePtr, sessionVersion.value(), theirIdentityKey, preKeyMessage.toString());
    }

    /**
     * Generates the supplied number of one-time keys.
     * <p>
     * The one-time key store has a limited capacity. If new keys are
     * generated while the store is full, the oldest keys are discarded.
     *
     * @param count the number of keys to generate
     * @return a {@link OneTimeKeyGenerationResult} containing the created
     *         and discarded keys
     * @throws IllegalStateException if this account has been closed
     */
    public OneTimeKeyGenerationResult generateOneTimeKeys(long count) {
        checkNotClosed();
        return nativeGenerateOneTimeKeys(nativePtr, count);
    }

    /**
     * Returns the number of one-time keys stored locally.
     * <p>
     * This will be equal to or greater than the number of one-time keys
     * that have been published. Each time a new session is created using
     * {@link #createInboundSession}, a one-time key is used and removed.
     *
     * @return the number of stored one-time keys
     * @throws IllegalStateException if this account has been closed
     */
    public long storedOneTimeKeyCount() {
        checkNotClosed();
        return nativeStoredOneTimeKeyCount(nativePtr);
    }

    /**
     * Returns the currently unpublished one-time keys.
     * <p>
     * The one-time keys should be published to a server and marked as
     * published using {@link #markKeysAsPublished()}.
     *
     * @return a map of key ID to base64-encoded Curve25519 public key
     * @throws IllegalStateException if this account has been closed
     */
    public Map<String, String> getUnpublishedOneTimeKeys() {
        checkNotClosed();
        return nativeOneTimeKeys(nativePtr);
    }

    /**
     * Generates a single new fallback key.
     * <p>
     * The fallback key will be used by other users to establish a session
     * if all the one-time keys on the server have been used up.
     *
     * @return the public Curve25519 key of the <i>previous</i> fallback key,
     *         or an empty {@link Optional} if there was no previous key
     * @throws IllegalStateException if this account has been closed
     */
    public Optional<String> generateFallbackKey() {
        checkNotClosed();
        return Optional.ofNullable(nativeGenerateFallbackKey(nativePtr));
    }

    /**
     * Returns the currently unpublished fallback key.
     * <p>
     * The fallback key should be published just like the one-time keys,
     * and marked as published using {@link #markKeysAsPublished()}.
     *
     * @return a map of key ID to base64-encoded Curve25519 public key
     * @throws IllegalStateException if this account has been closed
     */
    public Map<String, String> getUnpublishedFallbackKey() {
        checkNotClosed();
        return nativeFallbackKey(nativePtr);
    }

    /**
     * Forgets the previously used fallback key.
     * <p>
     * The account stores at most two private parts of the fallback key.
     * This method lets us forget the previously used one.
     *
     * @return {@code true} if the previous key was forgotten,
     *         {@code false} otherwise
     * @throws IllegalStateException if this account has been closed
     */
    public boolean forgetFallbackKey() {
        checkNotClosed();
        return nativeForgetFallbackKey(nativePtr);
    }

    /**
     * Marks all currently unpublished one-time and fallback keys as
     * published.
     *
     * @throws IllegalStateException if this account has been closed
     */
    public void markKeysAsPublished() {
        checkNotClosed();
        nativeMarkKeysAsPublished(nativePtr);
    }

    /**
     * Converts the account into a JSON string representation.
     *
     * @return a JSON string representing the account
     * @throws IllegalStateException if this account has been closed
     */
    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    /**
     * Encrypts the account using the given 32-byte key and returns the
     * encrypted representation.
     *
     * @param key a 256-bit (32-byte) key for encrypting the account
     * @return an encrypted string representation of the account
     * @throws IllegalStateException if this account has been closed
     * @throws KeyException         if the key is not 32 bytes
     */
    public String pickle(byte[] key) {
        checkNotClosed();
        validateEncryptionKey(key);
        return nativeEncryptedPickle(nativePtr, key);
    }

    /**
     * Converts the account into the libolm pickle format, encrypted with
     * the given 32-byte key.
     * <p>
     * This produces a binary pickle compatible with the original libolm
     * library, useful for migrating away from libolm. The counterpart is
     * {@link #unpickleLegacy(String, byte[])}.
     *
     * @param pickleKey a 256-bit (32-byte) key for encrypting the pickle
     * @return the libolm-format pickle string of the {@code Account}
     * @throws IllegalStateException if this account has been closed
     * @throws KeyException        if the key is not 32 bytes
     * @throws PickleException     if the pickle could not be created
     */
    public String pickleLegacy(byte[] pickleKey) {
        checkNotClosed();
        return nativePickleLegacy(nativePtr, pickleKey);
    }

    /**
     * Restores an {@code Account} from a previously saved JSON string.
     *
     * @param pickleData the JSON string from {@link #pickle()}
     * @return a restored {@code Account}
     * @throws PickleException if the data cannot be deserialized
     */
    public static Account unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new Account(nativePtr);
    }

    /**
     * Restores an {@code Account} from an encrypted string using a
     * 32-byte key.
     *
     * @param pickleData the encrypted pickle data from {@link #pickle(byte[])}
     * @param key        a 256-bit (32-byte) key for decrypting the account
     * @return a restored {@code Account}
     * @throws KeyException   if the key is not 32 bytes
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static Account unpickle(String pickleData, byte[] key) {
        validateEncryptionKey(key);
        long nativePtr = nativeEncryptedUnpickle(pickleData, key);
        return new Account(nativePtr);
    }

    /**
     * Restores an {@code Account} from a legacy libolm pickle format.
     * The pickle must be encrypted with the provided key.
     *
     * @param pickleData the libolm pickle data
     * @param pickleKey  the key used to encrypt the pickle data
     * @return a restored {@code Account}
     * @throws PickleException if the data cannot be decrypted or deserialized
     */
    public static Account unpickleLegacy(String pickleData, byte[] pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new Account(nativePtr);
    }

    /**
     * Creates a dehydrated device from the account.
     * <p>
     * A dehydrated device is a device that is stored encrypted on the server
     * that can receive messages when the user has no other active devices.
     * Upon login, the user can rehydrate the device (using
     * {@link Account#fromDehydratedDevice(String, String, byte[])}) and
     * decrypt the messages sent to the dehydrated device.
     * <p>
     * The account must be a newly-created account that does not have any Olm
     * sessions, since the dehydrated device format does not store sessions.
     * <p>
     * The format used is defined in
     * <a href="https://github.com/matrix-org/matrix-spec-proposals/pull/3814">MSC3814</a>.
     *
     * @param key a 256-bit (32-byte) key for encrypting the device
     * @return the ciphertext and nonce
     * @throws IllegalStateException if this account has been closed
     * @throws KeyException         if the key is not 32 bytes
     * @throws PickleException       if creating the dehydrated device fails
     */
    public DehydratedDeviceResult toDehydratedDevice(byte[] key) {
        checkNotClosed();
        validateEncryptionKey(key);
        return nativeToDehydratedDevice(nativePtr, key);
    }

    /**
     * Creates an {@code Account} object from a dehydrated device.
     *
     * @param ciphertext the ciphertext generated by {@link Account#toDehydratedDevice(byte[])}
     * @param nonce      the nonce generated by {@link Account#toDehydratedDevice(byte[])}
     * @param key        a 256-bit (32-byte) key for decrypting the device
     * @return a restored {@code Account}
     * @throws KeyException   if the key is not 32 bytes
     * @throws PickleException if the dehydrated device cannot be decrypted
     */
    public static Account fromDehydratedDevice(String ciphertext, String nonce, byte[] key) {
        validateEncryptionKey(key);
        long nativePtr = nativeFromDehydratedDevice(ciphertext, nonce, key);
        return new Account(nativePtr);
    }

    private static native long nativeNew();

    private native IdentityKeys nativeIdentityKeys(long ptr);

    private native String nativeCurve25519Key(long ptr);

    private native String nativeEd25519Key(long ptr);

    private native long nativeMaxNumberOfOneTimeKeys(long ptr);

    private native OlmSession nativeCreateOutboundSession(long ptr, int sessionVersion, String identityKey,
            String oneTimeKey);

    private native InboundCreationResult nativeCreateInboundSession(long ptr, int sessionVersion,
            String theirIdentityKey, String preKeyMessage);

    private native long nativeStoredOneTimeKeyCount(long ptr);

    private native OneTimeKeyGenerationResult nativeGenerateOneTimeKeys(long ptr, long count);

    private native Map<String, String> nativeOneTimeKeys(long ptr);

    private native String nativeGenerateFallbackKey(long ptr);

    private native Map<String, String> nativeFallbackKey(long ptr);

    private native boolean nativeForgetFallbackKey(long ptr);

    private native void nativeMarkKeysAsPublished(long ptr);

    private native String nativeSign(long ptr, String message);

    private native String nativePickle(long ptr);

    private native String nativeEncryptedPickle(long ptr, byte[] key);

    private native String nativePickleLegacy(long ptr, byte[] key);

    private native DehydratedDeviceResult nativeToDehydratedDevice(long ptr, byte[] key);

    private static native long nativeUnpickle(String pickleData);

    private static native long nativeEncryptedUnpickle(String pickleData, byte[] key);

    private static native long nativeUnpickleLegacy(String pickleData, byte[] pickleKey);

    private static native long nativeFromDehydratedDevice(String ciphertext, String nonce, byte[] key);

    protected native void nativeFree(long ptr);
}
