/**
 * The ECIES (Elliptic Curve Integrated Encryption Scheme) sessions.
 * <p>
 * ECIES establishes a secure communication channel using elliptic curve
 * Diffie-Hellman (X25519) for shared secret establishment and ChaCha20-Poly1305
 * for symmetric encryption. An
 * {@link io.github.fherbreteau.vodozemac.ecies.Ecies} creates outbound
 * channels, while an
 * {@link io.github.fherbreteau.vodozemac.ecies.EstablishedEcies} is an
 * established channel usable in both directions. A
 * {@link io.github.fherbreteau.vodozemac.ecies.CheckCode} is generated upon
 * establishment so both parties can verify out-of-band that they share the
 * same channel.
 */
package io.github.fherbreteau.vodozemac.ecies;

/**
 * @author François HERBRETEAU
 */
