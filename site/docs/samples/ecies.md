# ECIES channels

!!! tip "Full source"

    [`SampleEcies.java`](https://github.com/fherbreteau/vodozemac-java/blob/main/src/demos/java/io/github/fherbreteau/vodozemac/examples/SampleEcies.java)
    — runnable through `mvn -Psample test`.

ECIES implements the
[MSC3886](https://github.com/matrix-org/matrix-spec-proposals/pull/3886)
channel used by QR-code-based device login: an ephemeral initiator sets up an
authenticated two-way channel with a target device.

## 1. Establish the channel out of band

```java
byte[] plaintext = "It's a secret to everybody".getBytes(UTF_8);

try (Ecies alice = new Ecies(); Ecies bob = new Ecies()) {
    // exchange alice.publicKey() / bob.publicKey() out of band (QR code)

    OutboundCreationResult aliceResult =
            alice.establishOutboundChannel(bob.publicKey(), plaintext);
    String initialMessage = aliceResult.initialMessage();
```

## 2. The recipient accepts and decrypts

```java
    InboundCreationResult bobResult =
            bob.establishInboundChannel(initialMessage);

    byte[] received = bobResult.plaintext(); // == plaintext
```

Both `establish*Channel` calls **consume** the `Ecies` object — its native
resources are transferred to the creation results.

## 3. Verify out-of-band with the check code

The initiator's key in the initial message is unauthenticated; both sides
compare the two-digit check code over a trusted channel to rule out a MITM:

```java
    EstablishedEcies aliceEcies = aliceResult.establishedEcies();
    EstablishedEcies bobEcies = bobResult.establishedEcies();

    if (!aliceEcies.checkCode().equals(bobEcies.checkCode())) {
        throw new IllegalStateException("Check code mismatch; possible active MITM attack");
    }
```

## 4. Exchange further messages

```java
    String message = bobEcies.encrypt("Another plaintext".getBytes(UTF_8));
    byte[] decrypted = aliceEcies.decrypt(message);
    }
}
```
