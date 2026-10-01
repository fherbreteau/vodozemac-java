/**
 * Java bindings for the vodozemac Rust cryptographic library, providing the
 * building blocks of the Matrix end-to-end encryption: Olm accounts and
 * sessions, Megolm group sessions, ECIES channels, SAS key verification, and
 * server-side key backups.
 * <p>
 * All objects wrapping native memory implement {@link AutoCloseable} and must
 * be closed (typically via try-with-resources) to release the underlying Rust
 * structures.
 */
package io.github.fherbreteau.vodozemac;

/**
 * @author François HERBRETEAU
 */
