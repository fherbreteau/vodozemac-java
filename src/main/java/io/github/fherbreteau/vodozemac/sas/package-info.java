/**
 * The Short Authentication String (SAS) key verification.
 * <p>
 * A {@link io.github.fherbreteau.vodozemac.sas.Sas} performs interactive
 * key verification between two devices: after exchanging MACs, it
 * transitions to an
 * {@link io.github.fherbreteau.vodozemac.sas.EstablishedSas} that generates
 * emoji or decimal {@link io.github.fherbreteau.vodozemac.sas.SasBytes SAS
 * bytes} for the users to compare out-of-band.
 */
package io.github.fherbreteau.vodozemac.sas;

/**
 * @author François HERBRETEAU
 */
