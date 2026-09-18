# API reference

> Complete Javadoc for every released version: [javadoc.io/doc/io.github.fherbreteau/vodozemac-java](https://javadoc.io/doc/io.github.fherbreteau/vodozemac-java)


## Account

Main class for Olm account management — identity keys, one-time keys, fallback keys, session creation, signing, pickle/unpickle, and dehydrated devices.

| Method | Description |
|--------|-------------|
| `IdentityKeys identityKeys()` | Get both Ed25519 and Curve25519 public keys |
| `Ed25519PublicKey ed25519Key()` | Get Ed25519 public key |
| `Curve25519PublicKey curve25519Key()` | Get Curve25519 public key |
| `Ed25519Signature sign(String message)` | Sign a string message with Ed25519 key |
| `Ed25519Signature sign(byte[] message)` | Sign raw bytes with Ed25519 key |
| `long maxNumberOfOneTimeKeys()` | Get max one-time keys to store |
| `OneTimeKeyGenerationResult generateOneTimeKeys(long count)` | Generate one-time keys |
| `long storedOneTimeKeyCount()` | Get number of stored one-time keys |
| `Map<String, Curve25519PublicKey> unpublishedOneTimeKeys()` | Get unpublished one-time keys |
| `Optional<Curve25519PublicKey> generateFallbackKey()` | Generate a fallback key |
| `Map<String, Curve25519PublicKey> unpublishedFallbackKey()` | Get unpublished fallback key |
| `boolean forgetFallbackKey()` | Forget previously used fallback key |
| `void markKeysAsPublished()` | Mark keys as published |
| `OlmSession createOutboundSession(...)` | Create an outbound Olm session |
| `InboundCreationResult createInboundSession(...)` | Create an inbound Olm session from a pre-key `OlmMessage` |
| `String pickle()` / `pickle(byte[] key)` | Serialize account (plain or encrypted) |
| `String pickleLegacy(byte[] key)` | Serialize account to libolm pickle format |
| `static Account unpickle(...)` | Restore account from pickle |
| `static Account unpickleLegacy(...)` | Restore from libolm legacy pickle |
| `DehydratedDeviceResult toDehydratedDevice(byte[] key)` | Create a dehydrated device |
| `static Account fromDehydratedDevice(...)` | Restore from a dehydrated device |

## OlmSession

Represents an Olm session for 1-to-1 encrypted communication.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `SessionKeys sessionKeys()` | Get the keys used to establish this session |
| `OlmSessionVersion sessionConfig()` | Get the session protocol version |
| `boolean hasReceivedMessage()` | Check if a message has been received |
| `OlmMessage encrypt(byte[] plaintext)` | Encrypt a message and return a typed `OlmMessage` |
| `byte[] decrypt(OlmMessage message)` | Decrypt an `OlmMessage` |
| `String pickle()` / `pickle(byte[] key)` | Serialize session |
| `static OlmSession unpickle(...)` | Restore from pickle |
| `static OlmSession unpickleLegacy(...)` | Restore from libolm legacy pickle |

## OlmMessage

A structured Olm message consisting of a `MessageType` and a base64-encoded ciphertext body.
Produced by `OlmSession.encrypt()` and consumed by `OlmSession.decrypt()` and
`Account.createInboundSession()`.

| Method | Description |
|--------|-------------|
| `MessageType type()` | Get the message type (pre-key or normal) |
| `String body()` | Get the base64-encoded ciphertext body |
| `String toJson()` | Get the JSON representation for Matrix wire format |

## MessageType

Represents the type of an Olm message.

| Value | Description |
|-------|-------------|
| `PRE_KEY` (0) | Pre-key message, used to establish a new Olm session |
| `NORMAL` (1) | Normal message, sent over an already-established session |

## SessionKeys

The set of Curve25519 public keys that were used to establish an Olm session.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the globally unique session ID (SHA-256 of the three keys) |
| `Curve25519PublicKey identityKey()` | Get the long-term Curve25519 identity key of the session initiator |
| `Curve25519PublicKey baseKey()` | Get the ephemeral Curve25519 base key created by the initiator |
| `Curve25519PublicKey oneTimeKey()` | Get the one-time Curve25519 key used to establish the session |

## OutboundGroupSession

Megolm outbound group session for multi-recipient encrypted communication.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `int messageIndex()` | Get current message index |
| `String sessionKey()` | Get the session key for sharing with recipients |
| `MegolmSessionVersion sessionConfig()` | Get the session protocol version |
| `MegolmMessage encrypt(byte[] plaintext)` | Encrypt a message and return a typed `MegolmMessage` |
| `String pickle()` / `pickle(byte[] key)` | Serialize session |
| `static OutboundGroupSession unpickle(...)` | Restore from pickle |
| `static OutboundGroupSession unpickleLegacy(...)` | Restore from libolm legacy pickle |

## InboundGroupSession

Megolm inbound group session for receiving encrypted group messages.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `int firstKnownIndex()` | Get the first known message index |
| `DecryptedMessage decrypt(MegolmMessage message)` | Decrypt a `MegolmMessage` |
| `Optional<String> exportAt(int index)` | Export session key at a given index |
| `Optional<String> exportAtFirstKnownIndex()` | Export session key at first known index |
| `boolean advanceTo(int index)` | Advance the session to a given index |
| `boolean connected(InboundGroupSession other)` | Check if two sessions are connected |
| `SessionOrdering compare(InboundGroupSession other)` | Compare two sessions |
| `Optional<InboundGroupSession> merge(InboundGroupSession other)` | Merge two connected sessions |
| `static InboundGroupSession importSession(...)` | Import from an exported session key |
| `String pickle()` / `pickle(byte[] key)` | Serialize session |
| `static InboundGroupSession unpickle(...)` | Restore from pickle |
| `static InboundGroupSession unpickleLegacy(...)` | Restore from libolm legacy pickle |

## MegolmMessage

An encrypted Megolm message produced by `OutboundGroupSession.encrypt()` and consumed by
`InboundGroupSession.decrypt()`.

| Method | Description |
|--------|-------------|
| `String ciphertext()` | Get the base64-encoded ciphertext |
| `int messageIndex()` | Get the message index this message was encrypted at |
| `String mac()` | Get the base64-encoded MAC |
| `Ed25519Signature signature()` | Get the Ed25519 signature of the ciphertext |
| `static MegolmMessage fromBase64(String base64)` | Decode and validate a base64-encoded message |

## DecryptedMessage

The result of a successful Megolm decryption.

| Method | Description |
|--------|-------------|
| `byte[] plaintext()` | Get the decrypted plaintext bytes |
| `int messageIndex()` | Get the message index the message was encrypted at |

## Sas

Short Authentication String (SAS) verification for interactive key verification between devices.

| Method | Description |
|--------|-------------|
| `String publicKey()` | Get the ephemeral Curve25519 public key |
| `EstablishedSas diffieHellman(String theirPublicKey)` | Establish shared secret (consumes this `Sas`) |

## EstablishedSas

An established SAS channel with a shared secret, used for key verification and MAC exchange.

| Method | Description |
|--------|-------------|
| `SasBytes bytes(String info)` | Generate SAS bytes for visual verification |
| `byte[] bytesRaw(String info, int count)` | Generate raw bytes (max 8160) |
| `String calculateMac(String input, String info)` | Calculate a MAC for the given input |
| `String calculateMacInvalidBase64(String input, String info)` | Calculate a MAC with libolm-compatible invalid base64 encoding |
| `void verifyMac(String input, String info, String mac)` | Verify a MAC from the other party |
| `String ourPublicKey()` | Get our Curve25519 public key |
| `String theirPublicKey()` | Get the other party's Curve25519 public key |

## SasBytes

Short authentication string bytes for visual key verification (emoji indices and decimal numbers).

| Method | Description |
|--------|-------------|
| `int[] emojiIndices()` | Get 7 emoji indices for visual verification |
| `String[] decimals()` | Get 3 decimal numbers for visual verification |
| `byte[] bytes()` | Get the raw 6 bytes of the SAS |

## Ecies

Unestablished ECIES channel for QR-code-based device login (MSC3886).

| Method | Description |
|--------|-------------|
| `Ecies()` | Create with default `MATRIX_QR_CODE_LOGIN` info |
| `static Ecies withInfo(String info)` | Create with custom application info |
| `String publicKey()` | Get the ephemeral Curve25519 public key |
| `OutboundCreationResult establishOutboundChannel(String theirPublicKey, byte[] plaintext)` | Establish outbound channel (consumes this `Ecies`) |
| `InboundCreationResult establishInboundChannel(String message)` | Establish inbound channel from initial message (consumes this `Ecies`) |

## EstablishedEcies

An established ECIES channel for encrypting and decrypting messages using ChaCha20-Poly1305.

| Method | Description |
|--------|-------------|
| `String publicKey()` | Get our Curve25519 public key |
| `CheckCode checkCode()` | Get the check code for out-of-band MITM verification |
| `String encrypt(byte[] plaintext)` | Encrypt a message (base64-encoded) |
| `byte[] decrypt(String message)` | Decrypt a base64-encoded message |

## CheckCode

A two-digit check code for out-of-band verification of an ECIES session.

| Method | Description |
|--------|-------------|
| `byte[] asBytes()` | Get the raw 2-byte check code |
| `int toDigit()` | Get the check code as a two-digit number (0–99) |

## PkEncryption

The encryption component of the PK Encryption module for Megolm key backup. Implements `m.megolm_backup.v1.curve25519-aes-sha2`.

**Warning:** The algorithm contains a critical flaw — the MAC does not authenticate the ciphertext. This module is restricted to interoperability with the Matrix secure backup (`m.megolm_backup.v1.curve25519-aes-sha2`) and with libolm's PK encryption; it must never be reused for new protocols, because tampering with a ciphertext cannot be detected by the receiver.

| Method | Description |
|--------|-------------|
| `static PkEncryption fromKey(String publicKey)` | Create from a base64-encoded Curve25519 public key |
| `PkMessage encrypt(byte[] plaintext)` | Encrypt plaintext and return a `PkMessage` |

## PkDecryption

The decryption component of the PK Encryption module, holding a Curve25519 secret key.

**Warning:** Subject to the same restriction as `PkEncryption`: backup/libolm interoperability only — never reuse for new protocols (see the `PkEncryption` warning above).

| Method | Description |
|--------|-------------|
| `PkDecryption()` | Create with a fresh random Curve25519 key pair |
| `static PkDecryption fromKey(String secretKey)` | Create from a base64-encoded Curve25519 secret key |
| `String secretKey()` | Get the base64-encoded Curve25519 secret key |
| `String publicKey()` | Get the base64-encoded Curve25519 public key |
| `byte[] decrypt(PkMessage message)` | Decrypt a `PkMessage` |
| `String pickleLegacy(byte[] pickleKey)` | Serialize to libolm legacy pickle format |
| `static PkDecryption unpickleLegacy(String pickleData, byte[] pickleKey)` | Restore from a libolm legacy pickle |

## PkMessage

An encrypted message produced by `PkEncryption`, consisting of three base64-encoded components.

| Method | Description |
|--------|-------------|
| `String ciphertext()` | Get the base64-encoded ciphertext |
| `String mac()` | Get the base64-encoded MAC (does not authenticate the ciphertext) |
| `String ephemeralKey()` | Get the base64-encoded ephemeral Curve25519 public key |

## Cryptographic Key Types

Typed wrappers around base64-encoded cryptographic keys, providing validation on construction
and type-safe usage throughout the API. Located in `io.github.fherbreteau.vodozemac.types`.

#### Ed25519PublicKey

An Ed25519 public key used for signature verification.

| Method | Description |
|--------|-------------|
| `static Ed25519PublicKey fromBase64(String base64)` | Decode and validate a base64 key |
| `String toBase64()` | Get the base64-encoded key |
| `boolean verify(String message, Ed25519Signature signature)` | Verify a signature (returns `false` on failure) |
| `boolean verify(byte[] message, Ed25519Signature signature)` | Verify a signature over raw bytes |

#### Ed25519Signature

An Ed25519 signature produced by `Account.sign()`.

| Method | Description |
|--------|-------------|
| `static Ed25519Signature fromBase64(String base64)` | Decode and validate a base64 signature |
| `String toBase64()` | Get the base64-encoded signature |

#### Curve25519PublicKey

A Curve25519 public key used for X25519 key agreement.

| Method | Description |
|--------|-------------|
| `static Curve25519PublicKey fromBase64(String base64)` | Decode and validate a base64 key |
| `String toBase64()` | Get the base64-encoded key |

#### Ed25519KeyPair

An Ed25519 key pair used to sign and verify messages. Native resource — use with
try-with-resources.

| Method | Description |
|--------|-------------|
| `Ed25519KeyPair()` | Generate a new key pair with a random private key |
| `Ed25519PublicKey publicKey()` | Get the Ed25519 public key |
| `Ed25519Signature sign(String message)` | Sign a string message with the private key |
| `Ed25519Signature sign(byte[] message)` | Sign raw bytes with the private key |
| `String pickle()` | Serialize the key pair to a JSON string |
| `static Ed25519KeyPair unpickle(String pickleData)` | Restore a key pair from its JSON representation |

## Exceptions

All exceptions extend `VodozemacException` (which extends `RuntimeException`):

| Exception | Thrown when |
|-----------|------------|
| `PickleException` | Pickle/unpickle or dehydrated device errors |
| `DecryptionException` | Olm or Megolm decryption failures |
| `SessionCreationException` | Inbound session creation errors |
| `KeyException` | Key decoding or validation errors |
| `SignatureException` | Signature verification failures |
| `SasException` | SAS MAC verification failures or byte generation errors |
| `EciesException` | ECIES channel establishment or decryption errors |
| `EncryptionException` | PK encryption failures (e.g. non-contributory key) |
| `ConversionException` | Rust types that could not be parsed into Java types |

## Vodozemac

Utility class providing base64 encoding/decoding and access to the vodozemac library version.

| Method | Description |
|--------|-------------|
| `static String base64Encode(byte[] src)` | Encode bytes to unpadded base64 |
| `static byte[] base64Decode(String src)` | Decode base64 (padded or unpadded) to bytes |
| `static String version()` | Get the vodozemac Rust crate version |

