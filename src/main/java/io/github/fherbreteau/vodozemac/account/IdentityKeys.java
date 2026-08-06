package io.github.fherbreteau.vodozemac.account;

public class IdentityKeys {
    private final String ed25519;
    private final String curve25519;

    public IdentityKeys(String ed25519, String curve25519) {
        this.ed25519 = ed25519;
        this.curve25519 = curve25519;
    }

    public String fingerprintKey() {
        return ed25519;
    }

    public String identityKey() {
        return curve25519;
    }
}
