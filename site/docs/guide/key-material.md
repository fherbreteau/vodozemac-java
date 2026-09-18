# Key material handling

Secret material (private keys, `pickle()` outputs, decrypted payloads) is
returned as immutable Java `String`s or `byte[]`s. The JVM may keep these
objects in memory indefinitely (GC heuristics) and they will be captured by
heap dumps and core dumps.

- Never enable heap dump capture (`-XX:+HeapDumpOnOutOfMemoryError`, `jmap`,
  `jcmd GC.heap_dump`) or core dumps on hosts that process key material
  without reviewing where the dumps are stored.
- Prefer the `byte[]`-based APIs where available (e.g. encrypted pickles
  accept a `byte[]` key).
- Close native handles promptly so the native copies of secret material are
  released as well.

!!! warning "Encrypted pickles"

    `pickle()` outputs contain the private key in clear text. Prefer the
    encrypted variants (`pickle(byte[] key)` / `unpickle(String, byte[])`)
    offered by `Account`, the Olm and Megolm sessions, and store pickle keys
    in your secret storage.

!!! warning "PK encryption scope"

    The PK encryption module is restricted to interoperability with the
    Matrix secure backup (`m.megolm_backup.v1.curve25519-aes-sha2`) and
    libolm's PK encryption; it must never be reused for new protocols.

See [SECURITY.md](https://github.com/fherbreteau/vodozemac-java/blob/main/SECURITY.md)
for the full security policy.
