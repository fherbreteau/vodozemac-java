package io.github.fherbreteau.vodozemac.account;

public class DehydratedDeviceResult {
    private final String ciphertext;
    private final String nonce;

    public DehydratedDeviceResult(String ciphertext, String nonce) {
        this.ciphertext = ciphertext;
        this.nonce = nonce;
    }

    public String getCiphertext() {
        return ciphertext;
    }

    public String getNonce() {
        return nonce;
    }
}
