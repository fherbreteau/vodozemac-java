package io.github.fherbreteau.vodozemac;

public class SignatureException extends VodozemacException {

    public SignatureException(String message) {
        super(message);
    }

    public SignatureException(String message, Throwable cause) {
        super(message, cause);
    }
}
