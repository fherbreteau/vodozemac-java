package io.github.fherbreteau.vodozemac;

public class SessionCreationException extends VodozemacException {

    public SessionCreationException(String message) {
        super(message);
    }

    public SessionCreationException(String message, Throwable cause) {
        super(message, cause);
    }
}
