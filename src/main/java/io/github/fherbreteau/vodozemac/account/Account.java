package io.github.fherbreteau.vodozemac.account;

import java.util.Map;
import java.util.Optional;

import io.github.fherbreteau.vodozemac.NativeLibraryLoader;
import io.github.fherbreteau.vodozemac.olm.InboundCreationResult;
import io.github.fherbreteau.vodozemac.olm.OlmSession;
import io.github.fherbreteau.vodozemac.olm.OlmSessionVersion;

/**
 * An Olm Account manages all cryptographic keys used on a device.
 * 
 * This account links to an Account in the vodozemac Rust library.
 * 
 * @author François HERBRETEAU
 */
public class Account implements AutoCloseable {
    static {
        NativeLibraryLoader.loadLibrary();
    }

    private long nativePtr;

    /**
     * Create a new {@code Account} with new random identity keys.
     */
    public Account() {
        this.nativePtr = nativeNew();
    }

    private Account(long nativePtr) {
        this.nativePtr = nativePtr;
    }

    /**
     * Get the IdentityKeys of this {@code Account}
     * 
     * @return the identity keys
     */
    public IdentityKeys identityKeys() {
        checkNotClosed();
        return nativeIdentityKeys(nativePtr);
    }

    /**
     * Get a copy of the {@code Account}'s public Ed25519 key
     * @return a base 64 representation of the public Curve25519 key
     */
    public String ed25519Key() {
        checkNotClosed();
        return nativeEd25519Key(nativePtr);
    }

    /**
     * Get a copy of the {@code Account}'s public Curve25519 key
     * @return a base 64 representation of the public Curve25519 key
     */
    public String curve25519Key() {
        checkNotClosed();
        return nativeCurve25519Key(nativePtr);
    }

    /**
     * Sign the given message using our Ed25519 fingerprint key.
     * 
     * @param message the message to sign
     * @return the base 64 representation of message signature
     */
    public String sign(String message) {
        checkNotClosed();
        return nativeSign(nativePtr, message);
    }

    /**
     * Get the maximum number of one-time keys the client should keep on the server.
     * 
     * @return the maximum number of one-time keys
     */
    public long maxNumberOfOneTimeKeys() {
        checkNotClosed();
        return nativeMaxNumberOfOneTimeKeys(nativePtr);
    }

    /**
     * Create a {@code OlmSession} with the given identity key and one-time key.
     * @param identityKey the recipient identity key
     * @param oneTimeKey the recipient one-time key
     * @return an Olm {@code OlmSession}
     */
    public OlmSession createOutbpundSession(OlmSessionVersion sessionVersion, String identityKey, String oneTimeKey) {
        checkNotClosed();

        long sessionPtr = nativeCreateOutboundSession(nativePtr, sessionVersion.getValue(), identityKey, oneTimeKey);
        return new OlmSession(sessionPtr);
    }

    public InboundCreationResult createInboundSession(String theirIdentityKey, String preKeyMessage) {
        checkNotClosed();

        return nativeCreateInboundSession(nativePtr, theirIdentityKey, preKeyMessage);
    }

    /**
     * Generates the supplied number of one time keys.
     * 
     * @param count the number of keys to generate
     * @return the public parts of the one-time keys that were created and discarded.
     */
    public OneTimeKeyGenerationResult generateOneTimeKeys(long count) {
        checkNotClosed();
        return nativeGenerateOneTimeKeys(nativePtr, count);
    }

    /**
     * Get the number of one-time keys we have stored locally.
     * 
     * @return a number of one-time keys
     */
    public long storedOneTimeKeyCount() {
        checkNotClosed();
        return nativeStoredOneTimeKeyCount(nativePtr);
    }

    /**
     * Get the currently unpublished one-time keys.
     * 
     * @return a map of key id to public curve25519 key
     */
    public Map<String, String> getUnpublishedOneTimeKeys() {
        checkNotClosed();
        return nativeOneTimeKeys(nativePtr);
    }

    /**
     * Generate a single new fallback key.
     * 
     * @return the public Curve25519 key of the <i>previous</i> fallback key.
     */
    public Optional<String> generateFallbackKey() {
        checkNotClosed();
        return Optional.ofNullable(nativeGenerateFallbackKey(nativePtr));
    }

    /**
     * Get the currently unpublished fallback key.
     * 
     * @return a map of key id to public curve25519 key
     */
    public Map<String, String> getUnpublishedFallbackKey() {
        checkNotClosed();
        return nativeFallbackKey(nativePtr);
    }

    /**
     * Forget the previously used fallback key.
     *
     * @return {@code true} if the previous key was forgotten, {@code false} otherwise
     */
    public boolean forgetFallbackKey() {
        checkNotClosed();
        return nativeForgetFallbackKey(nativePtr);
    }

    /**
     * Mark all currently unpublished one-time and fallback keys as published.
     */
    public void markKeysAsPublished() {
        checkNotClosed();
        nativeMarkKeysAsPublished(nativePtr);
    }

    /**
     * Convert the account into a AccountPickle that is serialized as a JSON structure
     * @return a string representation of the {@code Account}
     */
    public String pickle() {
        checkNotClosed();
        return nativePickle(nativePtr);
    }

    /**
     * Restore a {@code Account} from a previously saved AccountPickle deserialized from a JSON structure.
     * @param pickleData
     * @return a {@code Account} object
     */
    public static Account unpickle(String pickleData) {
        long nativePtr = nativeUnpickle(pickleData);
        return new Account(nativePtr);
    }

    /**
     * Create an {@code Account} object by unpickling an account pickle in libolm legacy pickle format
     * @param pickleData the libolm pickle data
     * @param pickleKey the key used to encrypt the pickle data
     * @return a {@code Account} object
     */
    public static Account unpickleLegacy(String pickleData, String pickleKey) {
        long nativePtr = nativeUnpickleLegacy(pickleData, pickleKey);
        return new Account(nativePtr);
    }

    /**
     * Create a dehydrated device from the account.
     * 
     * A dehydrated device is a device that is stored encrypted on the server
     * that can receive messages when the user has no other active devices.
     * Upon login, the user can rehydrate the device (using {@link Account#fromDehydratedDevice(String, String, String)}
     * and decrypt the messages sent to the dehydrated device.
     * 
     * The account must be a newly-created account that does not have any Olm
     * sessions, since the dehydrated device format does not store sessions.
     * 
     * 
     * @param key a 256-bit (32-byte) key for encrypting the device.
     * @return the ciphertext and nonce.
     */
    public DehydratedDeviceResult toDehydratedDevice(String key) {
        checkNotClosed();
        return nativeToDehydratedDevice(nativePtr, key);
    }

    /**
     * Create an {@code Account} object from a dehydrated device.
     * 
     * @param ciphertext a ciphertext generated by {@link Account#toDehydratedDevice(String)}
     * @param nonce a nonce generated by {@link Account#toDehydratedDevice(String)}
     * @param key a 256-bit (32-byte) key for decrypting the device.
     * @return a {@code Account} object
     */
    public static Account fromDehydratedDevice(String ciphertext, String nonce, String key) {
        long nativePtr = nativeFromDehydratedDevice(ciphertext, nonce, key);
        return new Account(nativePtr);
    }

    private void checkNotClosed() {
        if (nativePtr == 0) {
            throw new IllegalStateException("Account has been closed");
        }
    }

    /**
     * Close the {@code Account} by releasing its associated native resources 
     * 
     * {@InheritDoc}
     */
    @Override
    public void close() {
        if (nativePtr != 0) {
            nativeFree(nativePtr);
            nativePtr = 0;
        }
    }

    private native long nativeNew();
    private native IdentityKeys nativeIdentityKeys(long ptr);
    private native String nativeCurve25519Key(long ptr);
    private native String nativeEd25519Key(long ptr);
    private native long nativeMaxNumberOfOneTimeKeys(long ptr);
    private native long nativeCreateOutboundSession(long ptr, int sessionVersion, String identityKey, String oneTimeKey);
    private native InboundCreationResult nativeCreateInboundSession(long ptr, String theirIdentityKey, String preKeyMessage);
    private native long nativeStoredOneTimeKeyCount(long ptr);
    private native OneTimeKeyGenerationResult nativeGenerateOneTimeKeys(long ptr, long count);
    private native Map<String, String> nativeOneTimeKeys(long ptr);
    private native String nativeGenerateFallbackKey(long ptr);
    private native Map<String, String> nativeFallbackKey(long ptr);
    private native boolean nativeForgetFallbackKey(long ptr);
    private native void nativeMarkKeysAsPublished(long ptr);
    private native String nativeSign(long ptr, String message);
    private native String nativePickle(long ptr);
    private native DehydratedDeviceResult nativeToDehydratedDevice(long ptr, String key);
    private static native long nativeUnpickle(String pickleData);
    private static native long nativeUnpickleLegacy(String pickleData, String pickleKey);
    private static native long nativeFromDehydratedDevice(String ciphertext, String nonce, String key);
    private native void nativeFree(long ptr);
}