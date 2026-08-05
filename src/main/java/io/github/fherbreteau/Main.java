package io.github.fherbreteau;

import io.github.fherbreteau.vodozemac.account.Account;

public final class Main {

    private Main() {
    }

    static void main() {
        try (Account account = new Account()) {
            System.out.println("Curve25519: " + account.curve25519Key());
            System.out.println("Ed25519: " + account.ed25519Key());
            System.out.println("Signature: " + account.sign("Hello Matrix!"));
        }
    }
}
