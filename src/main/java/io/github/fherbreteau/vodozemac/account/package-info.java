/**
 * The Olm account management.
 * <p>
 * An {@link io.github.fherbreteau.vodozemac.account.Account} manages all
 * cryptographic keys used on a device: identity keys (Ed25519 for signing and
 * Curve25519 for key agreement), one-time keys, and fallback keys. It is used
 * to create and accept Olm sessions for end-to-end encrypted communication
 * with other devices, to sign messages and keys, and to pickle/unpickle its
 * state for persistence, including support for legacy libolm pickles.
 */
package io.github.fherbreteau.vodozemac.account;

/**
 * @author François HERBRETEAU
 */
