/**
 * The Megolm group sessions.
 * <p>
 * Megolm provides end-to-end encryption for group conversations: an
 * {@link io.github.fherbreteau.vodozemac.megolm.OutboundGroupSession} encrypts
 * messages to be shared with multiple recipients, and an
 * {@link io.github.fherbreteau.vodozemac.megolm.InboundGroupSession} decrypts
 * them. Outbound sessions can be exported and their ratchet imported into
 * inbound sessions, and inbound sessions support ordering, comparison, and
 * merging of ratchet states.
 */
package io.github.fherbreteau.vodozemac.megolm;

/**
 * @author François HERBRETEAU
 */
