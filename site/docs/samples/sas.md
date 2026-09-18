# SAS verification

!!! tip "Full source"

    [`SampleSas.java`](https://github.com/fherbreteau/vodozemac-java/blob/main/src/demos/java/io/github/fherbreteau/vodozemac/examples/SampleSas.java)
    — runnable through `mvn -Psample test`.

Short Authentication String lets two devices verify each other's keys by
showing the user a short sequence of emojis or decimals derived from the
shared secret.

## 1. Exchange ephemeral public keys

```java
String agreedInfo = "AGREED_INFO"; // protocol-defined context string

try (Sas alice = new Sas(); Sas bob = new Sas()) {
    // exchange alice.publicKey() and bob.publicKey() out of band
    String bobPublicKey = bob.publicKey();
```

## 2. Establish the shared secret (consumes each `Sas`)

`diffieHellman` transfers ownership of the native handle to the returned
`EstablishedSas` — the original object becomes unusable:

```java
    try (EstablishedSas bobSas = bob.diffieHellman(alice.publicKey());
            EstablishedSas aliceSas = alice.diffieHellman(bobPublicKey)) {
```

## 3. Show the user the SAS bytes

Both sides derive the **same** emoji/decimal sequence:

```java
        SasBytes aliceSasBytes = aliceSas.bytes(agreedInfo);
        SasBytes bobSasBytes = bobSas.bytes(agreedInfo);

        String[] decimals = aliceSasBytes.decimals();   // e.g. "1234", "5678"
        aliceSasBytes.emojiIndices();                   // emoji table indexes
```

The users compare what they see; matching sequences mean the channel is
 authentic.

## 4. Cross-verify with MACs

```java
        String aliceMac = aliceSas.calculateMac("message", agreedInfo);
        String bobMac   = bobSas.calculateMac("message", agreedInfo);

        aliceSas.verifyMac("message", agreedInfo, bobMac);
        bobSas.verifyMac("message", agreedInfo, aliceMac);
        // a mismatching MAC throws a SasException
    }
}
```
