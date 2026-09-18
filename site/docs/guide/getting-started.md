# Getting started

## Basic example

```java
import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.types.Curve25519PublicKey;
import io.github.fherbreteau.vodozemac.types.Ed25519PublicKey;
import io.github.fherbreteau.vodozemac.types.Ed25519Signature;

public class MatrixCryptoExample {
    public static void main(String[] args) {
        // Create a new cryptographic account
        try (Account account = new Account()) {
            // Generate cryptographic keys
            Curve25519PublicKey curve25519Key = account.curve25519Key();
            Ed25519PublicKey ed25519Key = account.ed25519Key();

            System.out.println("Curve25519 Key: " + curve25519Key.toBase64());
            System.out.println("Ed25519 Key: " + ed25519Key.toBase64());

            // Sign a message
            String message = "Hello Matrix!";
            Ed25519Signature signature = account.sign(message);

            System.out.println("Signature: " + signature.toBase64());

            // Verify the signature
            boolean isValid = ed25519Key.verify(message, signature);
            System.out.println("Signature valid: " + isValid);
        } // Account automatically closed and resources freed
    }
}
```

## Standalone signing keys

Besides the device `Account`, a standalone Ed25519 key pair can be generated
and persisted — this is what Matrix cross-signing keys are built on:

```java
try (Ed25519KeyPair keyPair = new Ed25519KeyPair()) {
    Ed25519Signature signature = keyPair.sign("Hello Matrix!");
    keyPair.publicKey().verify("Hello Matrix!", signature);
}
```

See the [API reference](../api.md) for the complete surface, and the
[migration guide](../migration.md) if you are coming from jOlm.
