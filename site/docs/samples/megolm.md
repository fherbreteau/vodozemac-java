# Megolm group sessions

!!! tip "Full source"

    [`SampleMegolm.java`](https://github.com/fherbreteau/vodozemac-java/blob/main/src/demos/java/io/github/fherbreteau/vodozemac/examples/SampleMegolm.java)
    — runnable through `mvn -Psample test`.

Megolm provides **group** encryption (chat rooms): one sender ratchets
forward, and any participant holding the *session key* can decrypt.

## 1. Create the outbound session and export the session key

```java
try (OutboundGroupSession outbound = new OutboundGroupSession()) {
    String sessionId = outbound.sessionId();
    // Share this with the room members through Olm channels:
    String sessionKey = outbound.sessionKey();
```

## 2. Encrypt messages — the ratchet advances

Each `encrypt` produces a `MegolmMessage` with a message index (the ratchet
position), a ciphertext, a MAC and a signature:

```java
MegolmMessage encrypted = outbound.encrypt("This is a message".getBytes(UTF_8));
outbound.encrypt("This is a message".getBytes(UTF_8)); // index 1
outbound.encrypt("This is a message".getBytes(UTF_8)); // index 2
MegolmMessage encrypted4 = outbound.encrypt("This is a message".getBytes(UTF_8)); // index 3

encrypted.messageIndex(); // 0
encrypted.ciphertext();
encrypted.mac();
encrypted.signature();
```

## 3. Members create their inbound session and decrypt

```java
try (InboundGroupSession inbound = new InboundGroupSession(sessionKey)) {
    inbound.sessionId(); // matches the outbound session id

    DecryptedMessage decrypted = inbound.decrypt(encrypted);
    String plaintext = new String(decrypted.plaintext(), UTF_8);
    decrypted.messageIndex(); // 0 — the sender's ratchet position

    inbound.firstKnownIndex(); // how much of the ratchet we know

    // out-of-order delivery works — decrypting a later message first:
    DecryptedMessage decrypted2 = inbound.decrypt(encrypted4); // index 3
}
```

## Ratchet reconciliation

If participants hold different views of the ratchet (e.g. after importing
backups), `InboundGroupSession` offers `connected`, `compare`, `merge` and
`advanceTo` — see the [API reference](../api.md#megolm-group-sessions).
