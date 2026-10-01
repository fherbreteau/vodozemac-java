/**
 * The exception hierarchy of the library.
 * <p>
 * All exceptions derive from
 * {@link io.github.fherbreteau.vodozemac.exception.VodozemacException}, which
 * wraps errors raised by the native Rust code and maps them to typed Java
 * exceptions for each failure domain: key validation, encryption, decryption,
 * session creation, pickling, signatures, SAS verification, ECIES channels,
 * and type conversion.
 */
package io.github.fherbreteau.vodozemac.exception;

/**
 * @author François HERBRETEAU
 */
