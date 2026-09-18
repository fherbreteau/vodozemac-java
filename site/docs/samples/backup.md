# PK encryption (Megolm key backup)

!!! tip "Full source"

    [`SampleBackup.java`](https://github.com/fherbreteau/vodozemac-java/blob/main/src/demos/java/io/github/fherbreteau/vodozemac/examples/SampleBackup.java)
    — runnable through `mvn -Psample test`.

The PK encryption module implements
`m.megolm_backup.v1.curve25519-aes-sha2` — the Matrix
[secure backup](https://spec.matrix.org/latest/client-server-api/#backing-up-and-restoring-keys)
algorithm used to store room keys server-side.

!!! warning "Backup interoperability only"

    The algorithm's MAC does **not** authenticate the ciphertext (a known
    flaw inherent to the Matrix backup design) — never reuse this module
    for new protocols.

## Round trip

`PkDecryption` owns the long-term backup key pair; `PkEncryption` is created
from its public part and encrypts; `PkMessage` carries the three base64
components:

```java
byte[] plaintext = "It's a secret to everybody".getBytes(UTF_8);

try (PkDecryption decryption = new PkDecryption();
        PkEncryption encryption = PkEncryption.fromKey(decryption.publicKey())) {

    PkMessage message = encryption.encrypt(plaintext);
    message.ciphertext();   // base64 ciphertext
    message.ephemeralKey(); // base64 ephemeral Curve25519 public key
    message.mac();          // base64 MAC (does not authenticate the ciphertext)

    byte[] restored = decryption.decrypt(message); // == plaintext
}
```

## Persistence

`PkDecryption` supports the encrypted pickle for long-term storage:

```java
byte[] key = /* 32-byte key from your secret storage */;
String pickle = decryption.pickle(key);
// restore later:
try (PkDecryption restored = PkDecryption.unpickle(pickle, key)) { ... }
```

A legacy libolm pickle can be imported with `unpickleLegacy(pickle, key)` —
see the [migration guide](https://github.com/fherbreteau/vodozemac-java/blob/main/MIGRATION-GUIDE.md).
