package io.github.fherbreteau.vodozemac.megolm;

/**
 * The result of a comparison between two {@link InboundGroupSession} types.
 * <p>
 * Tells us if one session can be considered to be better than another one.
 * This is used when multiple sessions with the same session ID are received
 * with varying degrees of trust and first known message indices.
 *
 * @author François HERBRETEAU
 * @see InboundGroupSession#compare(InboundGroupSession)
 */
public enum SessionOrdering {
    /** The sessions are the same, with identical first known indices. */
    EQUAL,
    /** The first session has a better (lower) initial message index than the second one. */
    BETTER,
    /** The first session has a worse (higher) initial message index than the second one. */
    WORSE,
    /** The sessions are not connected and cannot be compared. */
    UNCONNECTED
}
