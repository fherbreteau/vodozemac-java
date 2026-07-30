package io.github.fherbreteau;

import io.github.fherbreteau.vodozemac.VodozemacAccount;

public class Main {
    static void main() {
        try (VodozemacAccount account = new VodozemacAccount()) {
            System.out.println("Curve25519: " + account.curve25519Key());
            System.out.println("Ed25519: " + account.ed25519Key());
            System.out.println("Signature: " + account.sign("Hello Matrix!"));
        }
    }
}
