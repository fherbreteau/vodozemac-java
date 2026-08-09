package io.github.fherbreteau.vodozemac;

public class VodozemacException extends RuntimeException {

    public VodozemacException(String message) {
        super(message);
    }

    public VodozemacException(String message, Throwable cause) {
        super(message, cause);
    }
}
