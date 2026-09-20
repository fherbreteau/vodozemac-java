/**
 * The Olm sessions.
 * <p>
 * An {@link io.github.fherbreteau.vodozemac.olm.OlmSession} implements
 * double-ratchet end-to-end encryption between two devices. Created from an
 * account and a peer's identity and one-time keys, it exchanges
 * {@link io.github.fherbreteau.vodozemac.olm.OlmMessage OlmMessages}
 * (pre-key or normal messages, discriminated by
 * {@link io.github.fherbreteau.vodozemac.olm.MessageType MessageType}) and can
 * be pickled/unpickled for persistence, including support for legacy libolm
 * pickles.
 */
package io.github.fherbreteau.vodozemac.olm;

/**
 * @author François HERBRETEAU
 */
