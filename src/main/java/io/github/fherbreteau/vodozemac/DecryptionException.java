package io.github.fherbreteau.vodozemac;

public class DecryptionException extends VodozemacException {

    public DecryptionException(String message) {
        super(message);
    }

    public DecryptionException(String message, Throwable cause) {
        super(message, cause);
    }
}
