# Olm sessions

!!! tip "Full source"

    [`SampleOlm.java`](https://github.com/fherbreteau/vodozemac-java/blob/main/src/demos/java/io/github/fherbreteau/vodozemac/examples/SampleOlm.java)
    — runnable through `mvn -Psample test`.

Olm provides **private, one-to-one** encrypted communication between two
devices. This sample walks the full flow between two accounts, *Alice* and
*Bob*.

## 1. Identity keys and signing

Each `Account` owns a Curve25519 identity key (key agreement) and an Ed25519
fingerprint key (signing):

```java
try (Account aliceAccount = new Account(); Account bobAccount = new Account()) {
    aliceAccount.identityKeys().fingerprintKey(); // Ed25519 public key
    aliceAccount.identityKeys().identityKey();    // Curve25519 public key

    Ed25519Signature aliceSignature = aliceAccount.sign("Hello Matrix!");
    boolean isValid = aliceAccount.ed25519Key().verify("Hello Matrix!", aliceSignature);
}
```

## 2. Bob publishes a one-time key

An outbound session consumes one of the recipient's one-time keys:

```java
OneTimeKeyGenerationResult bobOneTimeKeys = bobAccount.generateOneTimeKeys(1L);
Curve25519PublicKey bobOneTimeKey = bobOneTimeKeys.created().iterator().next();
bobAccount.markKeysAsPublished(); // key is now published to the server
```

## 3. Alice creates the outbound session and encrypts

```java
String alicePickleSession;
OlmMessage encrypted;
try (OlmSession outbound = aliceAccount.createOutboundSession(
        bobAccount.curve25519Key(), bobOneTimeKey)) {
    outbound.sessionId();
    SessionKeys keys = outbound.sessionKeys(); // identity/base/one-time keys
    encrypted = outbound.encrypt("Hello Bob".getBytes());
    alicePickleSession = outbound.pickle();    // persist for later
}
```

## 4. Bob establishes the inbound session and decrypts

The first message is a *pre-key message*: creating the inbound session from
it also decrypts the plaintext in one step:

```java
InboundCreationResult result =
        bobAccount.createInboundSession(aliceAccount.curve25519Key(), encrypted);
try (OlmSession inbound = result.session()) {
    String plaintext = new String(result.plaintext(), UTF_8); // "Hello Bob"

    // Bob replies with a normal (non pre-key) message
    encrypted = inbound.encrypt("Hello Alice".getBytes());
}
```

## 5. Alice restores her session and decrypts the reply

```java
try (OlmSession restored = OlmSession.unpickle(alicePickleSession)) {
    byte[] reply = restored.decrypt(encrypted); // "Hello Alice"
}
```

After the handshake, both sides keep exchanging messages with `encrypt` /
`decrypt` — the ratchet advances with every message.
