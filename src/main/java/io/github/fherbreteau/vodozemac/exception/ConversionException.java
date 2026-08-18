package io.github.fherbreteau.vodozemac.exception;

/**
 * Thrown when a type from rust could not be parsed in Java.
 *
 * @author François HERBRETEAU
 */
public class ConversionException extends VodozemacException {

    /**
     * Constructs a new {@code ConversionException} with the specified detail message.
     *
     * @param message the detail message
     */
    public ConversionException(String message) {
        super(message);
    }

    /**
     * Constructs a new {@code ConversionException} with the specified detail message and a cause.
     *
     * @param message the detail message
     * @param cause the cause
     */
    public ConversionException(String message, Throwable cause) {
        super(message, cause);
    }
}
