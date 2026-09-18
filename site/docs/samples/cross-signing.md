# Cross-signing keys

!!! tip "Full source"

    [`SampleCrossSigning.java`](https://github.com/fherbreteau/vodozemac-java/blob/main/src/demos/java/io/github/fherbreteau/vodozemac/examples/SampleCrossSigning.java)
    — runnable through `mvn -Psample test`.

Matrix [cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
uses three Ed25519 key pairs per user — the **master key** (MSK), the
**user-signing key** (USK) and the **self-signing key** (SSK). This sample
reproduces the specification's signature chains with `Ed25519KeyPair`.

## 1. Generate the three signing identities

```java
try (Ed25519KeyPair masterKeyPair = new Ed25519KeyPair();
        Ed25519KeyPair userSigningKeyPair = new Ed25519KeyPair();
        Ed25519KeyPair selfSigningKeyPair = new Ed25519KeyPair();
        Account account = new Account()) { // the device

    // spec-shaped key objects: keys, usage, user_id
    Json masterKey = buildMatrixKey(masterKeyPair.publicKey(), "master");
    Json userSigningKey = buildMatrixKey(userSigningKeyPair.publicKey(), "user_signing");
    Json selfSigningKey = buildMatrixKey(selfSigningKeyPair.publicKey(), "self_signing");

    // persist each signing key (JSON holding the private key):
    String mskPickle = masterKeyPair.pickle();
```

## 2. The master key signs the USK and the SSK

```java
    String mskKeyId = "ed25519:" + masterKeyPair.publicKey().toBase64();

    Json signedSelfSigningKey =
            addSignature(selfSigningKey, mskKeyId, masterKeyPair.sign(selfSigningKey.toString()));
    Json signedUserSigningKey =
            addSignature(userSigningKey, mskKeyId, masterKeyPair.sign(userSigningKey.toString()));
```

## 3. The device keys are signed twice

By the device's own Ed25519 key **and** by the self-signing key — this is
what makes a device "trusted" under cross-signing:

```java
    Json device = buildMatrixDevice(account);
    String deviceKeyId = "ed25519:" + DEVICE_ID;
    String sskKeyId = "ed25519:" + selfSigningKeyPair.publicKey().toBase64();

    Json signedByAccount =
            addSignature(device, deviceKeyId, account.sign(device.toString()));
    Json signedDevice =
            addSignature(signedByAccount, sskKeyId, selfSigningKeyPair.sign(device.toString()));
```

## 4. The device key signs the master key, then everything is uploaded

```java
    Json signedMasterKey =
            addSignature(masterKey, deviceKeyId, account.sign(masterKey.toString()));

    Json crossSigningKeys = Json.object()
            .set("master_key", signedMasterKey)
            .set("self_signing_key", signedSelfSigningKey)
            .set("user_signing_key", signedUserSigningKey);
    // upload crossSigningKeys + the signed device keys to /keys/upload
```

!!! note "Persistence"

    There is no seed API: generate each key pair once and treat `pickle()`
    as the recoverable secret (JSON containing the private key — store it in
    your secret storage, restore with `Ed25519KeyPair.unpickle(...)`).
