/**
 * Server-side key backup encryption and decryption.
 * <p>
 * This package provides public-key encryption of the Megolm session keys for
 * storing them on the homeserver: a
 * {@link io.github.fherbreteau.vodozemac.backup.PkEncryption} encrypts keys
 * with a backup public key (Curve25519 with an ephemeral key pair and
 * HMAC-SHA256), while a
 * {@link io.github.fherbreteau.vodozemac.backup.PkDecryption} decrypts them
 * with the corresponding private key.
 */
package io.github.fherbreteau.vodozemac.backup;

/**
 * @author François HERBRETEAU
 */
