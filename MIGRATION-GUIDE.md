# Migrating from jOlm (`io.github.brevilo:jolm`) to Vodozemac Java

This guide helps you migrate an application using [jOlm](https://github.com/brevilo/jolm)
(`io.github.brevilo:jolm`, JNI/JNA bindings to the C **libolm** library) to
**Vodozemac Java** (`io.github.fherbreteau:vodozemac-java`), the bindings for
[vodozemac](https://github.com/matrix-org/vodozemac) — the Rust reference
implementation that supersedes libolm.

jOlm is archived and on hiatus (its final release is
[1.1.1](https://github.com/brevilo/jolm/releases/tag/1.1.1)); upstream libolm is
deprecated and no longer developed, which makes the migration a matter of
*when*, not *if*.

## 1. Why migrate

| | jOlm 1.1.1 | Vodozemac Java |
|---|---|---|
| Underlying library | libolm 3.2.8+ (C, deprecated) | vodozemac 0.10 (Rust, security-audited) |
| Native library | **Must be installed** on every system (package manager or manual build) | **Bundled** in the JAR for Linux (x86_64, ARM64), macOS (Intel, Apple Silicon) and Windows (x86_64, ARM64) — extracted and loaded automatically |
| Binding technology | JNA | JNI (Rust bridge) |
| Java version | Java 8+ | **Java 25+** |
| Maintenance | Archived / on hiatus | Active |
| Pickle compatibility | libolm pickles (native) | Native pickles + **reads libolm pickles** (`unpickleLegacy`) |
| Protocol compatibility | libolm Olm/Megolm | libolm-compatible protocol (v1) for interoperability, plus experimental v2 |

## 2. Update the dependency

```xml
<!-- Remove -->
<dependency>
  <groupId>io.github.brevilo</groupId>
  <artifactId>jolm</artifactId>
  <version>1.1.1</version>
</dependency>

<!-- Add -->
<dependency>
  <groupId>io.github.fherbreteau</groupId>
  <artifactId>vodozemac-java</artifactId>
  <version>1.0.0-rc1</version>
</dependency>
```

Nothing else is required: there is no libolm to install and no native
dependency to manage. The JRE must be Java 25 or later.

## 3. Concepts that carry over

The domain model is identical — both libraries expose the Olm/Megolm
primitives: accounts with identity/one-time/fallback keys, Olm sessions,
Megolm group sessions, key verification, and PK encryption for Megolm key
backup. If you know jOlm, you already know what the classes do; the mapping
below shows where names and shapes changed.

## 4. Class and API mapping

### Packages and lifecycle

| jOlm | Vodozemac Java | Notes |
|---|---|---|
| `io.github.brevilo.jolm.*` | `io.github.fherbreteau.vodozemac.*` (sub-packages: `account`, `olm`, `megolm`, `sas`, `ecies`, `backup`, `types`, `exception`) | |
| `account.clear()`, `session.clear()` | `close()` (`AutoCloseable`, idempotent) | Use try-with-resources everywhere: resources are **not** released by finalizers |

### Account

| jOlm | Vodozemac Java |
|---|---|
| `new Account()` | `new Account()` |
| `account.identityKeys()` → `IdentityKeys` (JSON model, `getCurve25519()`/`getEd25519()` base64 `String`s) | `account.identityKeys()` → `IdentityKeys` value class: `identityKey()` → `Curve25519PublicKey`, `fingerprintKey()` → `Ed25519PublicKey`; or directly `account.curve25519Key()` / `account.ed25519Key()` |
| `account.generateOneTimeKeys(long count)` | `account.generateOneTimeKeys(count)` — **count is limited to 1..100** (see §7) — returns `OneTimeKeyGenerationResult` with `created()` / `removed()` lists of `Curve25519PublicKey` |
| `account.oneTimeKeys()` → `OneTimeKeys` (`Map<String, String>` keyId → base64) | `account.unpublishedOneTimeKeys()` → `Map<String, Curve25519PublicKey>` |
| `account.markKeysAsPublished()` | `account.markKeysAsPublished()` |
| `account.maxNumberOfOneTimeKeys()` | `account.maxNumberOfOneTimeKeys()` |
| `account.removeOneTimeKeys(Session)` | **Automatic**: `account.createInboundSession(...)` consumes the one-time key; the used key is exposed via `result.session().sessionKeys().oneTimeKey()` (`SessionKeys` also carries `sessionId()`, `identityKey()`, `baseKey()`) |
| `account.generateFallbackKey()` (`void`) | `account.generateFallbackKey()` returns `Optional<Curve25519PublicKey>` (the new fallback key) |
| `account.unpublishedFallbackKey()` → `OneTimeKeys` | `account.unpublishedFallbackKey()` → `Map<String, Curve25519PublicKey>` |
| `account.forgetFallbackKey()` | `account.forgetFallbackKey()` |
| `account.sign(String message)` → base64 `String` | `account.sign(message)` (`String` or `byte[]`) → `Ed25519Signature` (`.toBase64()` when you need the string) |
| `account.pickle(String key)` / `Account.unpickle(String key, String pickle)` | `account.pickle()` (JSON, unencrypted), `account.pickle(byte[] key)` (AES-encrypted, **key must be exactly 32 bytes**), `Account.unpickle(pickleData)` / `Account.unpickle(pickleData, byte[] key)` — and `Account.unpickleLegacy(pickleData, byte[] key)` to read jOlm pickles (see §5) |

### Olm sessions

| jOlm | Vodozemac Java |
|---|---|
| `Session.createOutboundSession(account, theirIdentityKey, theirOneTimeKey)` | `account.createOutboundSession(theirIdentityKey, theirOneTimeKey)` (typed keys) → `OlmSession` |
| `Session.createInboundSession(account, oneTimeKeyMessage)` | `account.createInboundSession(theirIdentityKey, preKeyMessage)` → `InboundCreationResult` — **the pre-key message is decrypted during creation**; get `result.session()` and `result.plaintext()` |
| `Session.createInboundSessionFrom(account, theirIdentityKey, oneTimeKeyMessage)` | Same as above: pass `theirIdentityKey` explicitly |
| `new Account()` + inbound creation … | Sessions returned by the account are already linked to it; no manual association |
| `session.encrypt(String plainText)` → `Message` (`Message.PreKey` / `Message.Normal`, `getCipherText()`, `type()` as `Long`) | `session.encrypt(plaintext)` (`byte[]`) → `OlmMessage` with `type()` → `MessageType.PRE_KEY` / `MessageType.NORMAL` and `body()` (base64 ciphertext) |
| `session.decrypt(Message message)` → `String` | `session.decrypt(olmMessage)` → `DecryptedMessage` (`plaintext()` as `byte[]`) |
| `session.sessionId()` | `session.sessionId()` |
| `session.hasReceivedMessage()` | `session.hasReceivedMessage()` |
| `session.matchesInboundSession(oneTimeKeyMessage)` / `matchesInboundSessionFrom(...)` | Not exposed; inbound creation validates and reports failures through typed exceptions |
| `session.pickle(String key)` / `Session.unpickle(key, pickle)` | `session.pickle()` / `session.pickle(byte[] key)` / `OlmSession.unpickle(pickleData)` / `OlmSession.unpickle(pickleData, byte[] key)` / `OlmSession.unpickleLegacy(pickleData, byte[] key)` |

### Megolm group sessions

| jOlm | Vodozemac Java |
|---|---|
| `new OutboundGroupSession()` | `new OutboundGroupSession()` (or with a `MegolmSessionVersion`) |
| `outbound.sessionKey()` | `outbound.sessionKey()` |
| `outbound.messageIndex()` | `outbound.messageIndex()` |
| `outbound.encrypt(String plainText)` → base64 `String` | `outbound.encrypt(plaintext)` (`byte[]`) → `MegolmMessage` |
| `new InboundGroupSession(sessionKey)` | `new InboundGroupSession(sessionKey)` (or with a `MegolmSessionVersion`) |
| `inbound.decrypt(String message)` → `GroupMessage` (`getPlainText()`, `getMessageIndex()`) | `inbound.decrypt(megolmMessage)` → `DecryptedMessage` (`plaintext()`, `messageIndex()`) |
| `inbound.firstKnownIndex()` | `inbound.firstKnownIndex()` |
| `inbound.isVerified()` | `inbound.isVerified()` |
| `inbound.exportKey(messageIndex)` / `inbound.importKey(sessionKey)` | `inbound.exportAt(index)` / `inbound.exportAtFirstKnownIndex()` → `Optional<String>`, then `InboundGroupSession.importSession(sessionKey)` |
| `inbound/outbound.pickle(key)` / `unpickle(key, pickle)` | Native pickles (`pickle()` / `pickle(byte[])` / `unpickle(...)`) + `unpickleLegacy(pickleData, byte[] key)` |
| — | New: `connected()`, `compare()`, `merge()`, `advanceTo()` for ratchet reconciliation |

### Utility

| jOlm | Vodozemac Java |
|---|---|
| `new Utility().verifyEd25519(key, message, signature)` (throws on invalid signature) | `Ed25519PublicKey.fromBase64(key).verify(message, Ed25519Signature.fromBase64(signature))` → **returns `boolean`** instead of throwing |
| `new Utility().sha256(input)` | Not provided — use a standard library (`MessageDigest.getInstance("SHA-256")`) |

### PK encryption (Megolm key backup)

| jOlm | Vodozemac Java |
|---|---|
| `new PkEncryption(recipientKey)` | `PkEncryption.fromKey(recipientKey)` |
| `pkEncryption.encrypt(String plainText)` → `PkMessage` | `pkEncryption.encrypt(plaintext)` (`byte[]`) → `PkMessage` (`ciphertext()`, `mac()`, `ephemeralKey()` — all base64) |
| `new PkDecryption()` | `new PkDecryption()` |
| `pkDecryption.publicKey()` | `pkDecryption.publicKey()` |
| `pkDecryption.privateKey()` | `pkDecryption.secretKey()` |
| `pkDecryption.decrypt(PkMessage)` → `String` | `pkDecryption.decrypt(PkMessage)` → `byte[]` |
| `pkDecryption.pickle(key)` / `PkDecryption.unpickle(key, pickle)` | `pkDecryption.pickleLegacy(byte[] key)` / `PkDecryption.unpickleLegacy(pickleData, byte[] key)` + native pickles (`unpickle(pickleData, byte[] key)`) |

⚠️ The underlying `m.megolm_backup.v1.curve25519-aes-sha2` algorithm does not
authenticate ciphertexts (a documented flaw inherited from libolm and preserved
for compatibility). Do not use it for anything other than Megolm key backup.

### SAS verification

| jOlm | Vodozemac Java |
|---|---|
| `new Sas()` | `new Sas()` |
| `sas.publicKey()` | `sas.publicKey()` |
| `sas.setTheirKey(theirKey)` / `sas.isTheirKeySet()` | `sas.diffieHellman(theirKey)` → `EstablishedSas` — **consumes the `Sas` object** (like the Rust API); the established object holds both public keys (`ourPublicKey()`, `theirPublicKey()`) |
| `sas.calculateMac(message, info)` | `established.calculateMac(input, info)` / `established.verifyMac(input, info, mac)` (throws on mismatch) |
| — | New: `established.bytes(info)` → `SasBytes` with `emojiIndices()` / `decimals()` for emoji verification, and `bytesRaw(info, count)` |

### PK signing and cross-signing keys

jOlm's `PkSigning` (seed-based Ed25519 signing detached from the account) maps
to the standalone **`Ed25519KeyPair`** (`io.github.fherbreteau.vodozemac.types`)
— the same primitive you need for a **Matrix cross-signing key** (the master,
self-signing, and user-signing keys are all Ed25519 key pairs):

| jOlm | Vodozemac Java |
|---|---|
| `PkSigning` (from seed) | `new Ed25519KeyPair()` — generates a random key pair (native handle, use try-with-resources) |
| `signing.seed()` | — (no seed API) — persist the key pair instead with `pickle()` |
| `signing.sign(message)` | `keyPair.sign(String)` / `keyPair.sign(byte[])` → `Ed25519Signature` |
| `signing.publicKey()` | `keyPair.publicKey()` → `Ed25519PublicKey` (verify with `publicKey.verify(message, signature)`) |
| — | New: `pickle()` / `Ed25519KeyPair.unpickle(pickleData)` — persist and restore the full key pair |

Notes for cross-signing setups:

- There is **no seed-based construction** (`PkSigning.fromSeed` equivalent).
  Generate each key pair once, then treat `pickle()` output as the recoverable
  secret: it is JSON containing the **private key in clear text**, so store it
  in your secret storage (SSSS/keystore) and restore with
  `Ed25519KeyPair.unpickle(...)` — the restored pair produces identical
  signatures.
- Publish `keyPair.publicKey().toBase64()` wherever you previously published
  the `PkSigning` public key.
- The instance is thread-safe and `AutoCloseable` like every other handle.

### New capabilities with no jOlm equivalent

- **ECIES** (`Ecies`, `EstablishedEcies`, `CheckCode`) — the
  [MSC3886](https://github.com/matrix-org/matrix-spec-proposals/pull/3886)
  channel for QR-code-based device login.
- **`Ed25519KeyPair`** — standalone Ed25519 signing key pair usable as a
  cross-signing key (replaces jOlm's `PkSigning`, see above).
- **Encrypted pickles** for every native-handle type (AES with a 32-byte key).
- **Session protocol versions** (`OlmSessionVersion`, `MegolmSessionVersion`) —
  v1 keeps interoperability with libolm peers, v2 is the newer experiment.
- **Megolm ratchet reconciliation** (`connected`, `compare`, `merge`,
  `advanceTo`).

## 5. Migrating pickled data (the critical part)

jOlm pickles are **libolm pickles**. Vodozemac Java can read them for every
type that exists in both libraries:

| jOlm pickle | Vodozemac Java import |
|---|---|
| `Account.unpickle(key, pickle)` | `Account.unpickleLegacy(pickle, keyBytes)` |
| `Session.unpickle(key, pickle)` | `OlmSession.unpickleLegacy(pickle, keyBytes)` |
| `InboundGroupSession.unpickle(key, pickle)` | `InboundGroupSession.unpickleLegacy(pickle, keyBytes)` |
| `OutboundGroupSession.unpickle(key, pickle)` | `OutboundGroupSession.unpickleLegacy(pickle, keyBytes)` |
| `PkDecryption.unpickle(key, pickle)` | `PkDecryption.unpickleLegacy(pickle, keyBytes)` |

Key handling during migration:

1. jOlm pickle keys are `String`s, used by libolm as **UTF-8 bytes of any
   length**. `unpickleLegacy` takes `byte[]` — convert with
   `oldKey.getBytes(StandardCharsets.UTF_8)`.
2. After importing, **re-pickle** into the native format with
   `pickle()` (JSON) or `pickle(byte[] key)`. The new encrypted pickles
   require a **32-byte** key (libolm keys were arbitrary length) — derive one
   from your stored secret with a KDF if needed, e.g.
   `SHA-256(oldSecret)` gives exactly 32 bytes, or better, a keyed KDF such as
   HKDF/HMAC-SHA256 with an application-specific salt.
3. Recommended one-time migration flow per stored object:
   `unpickleLegacy(oldPickle, oldKeyBytes)` → `pickle(newKey32)` → persist →
   delete the legacy pickle. The legacy path exists for as long as you need
   it, but a single conversion simplifies everything downstream.

## 6. Error handling and message-shape differences

| jOlm | Vodozemac Java |
|---|---|
| `OlmException` for everything | `VodozemacException` sub-types by domain: `KeyException`, `DecryptionException`, `EncryptionException`, `SessionCreationException`, `PickleException`, `SignatureException`, `SasException`, `EciesException`, `ConversionException` |
| Runtime failures surfaced as `RuntimeException`/`IllegalStateException` | `IllegalStateException` on closed handles, `IllegalArgumentException` for invalid arguments |
| `Message.type()` → `Long` (0/1) | `OlmMessage.type()` → `MessageType.PRE_KEY` / `MessageType.NORMAL` enum |
| Plain text as `String` everywhere | Plaintexts are `byte[]`; convert with `new String(bytes, StandardCharsets.UTF_8)` |
| Message index `long` | `messageIndex()` is `int` |

All Java-visual types (`Curve25519PublicKey`, `Ed25519PublicKey`,
`Ed25519Signature`) convert with `fromBase64(String)` /
`toBase64()`, and compare with `equals`/`hashCode` — store and exchange them
instead of raw strings where you can.

## 7. Behavioral notes and gotchas

- **One-time key generation is capped at 100** per call (matching libolm's
  limit and preventing native memory exhaustion); counts outside `1..100`
  throw `IllegalArgumentException`.
- **Consuming operations**: `Sas.diffieHellman(...)` invalidates the `Sas`
  object it was called on (the handle transfers to `EstablishedSas`). Note
  that `InboundCreationResult.close()` closes the session it wraps — extract
  the session only if you intend to keep using it, and close whichever handle
  you keep.
- **Instances are thread-safe** — every handle class serializes its native
  calls with an internal monitor, so sharing one handle across threads is safe
  (no external locking needed); distinct handles can be used concurrently.
  Value classes are immutable. This is stricter than jOlm, where concurrent
  use of a single account/session was undefined.
- **No system libolm anymore**: remove any packaging/install scripts for
  libolm; the native library ships inside the JAR. If your environment blocks
  temp-file execution, see `NativeLibraryLoader` for how the library is
  extracted (`rwx------` temp directory) before loading.
- **Java 25 required** — if you are on Java 8-17, plan the runtime upgrade
  first (this is usually the biggest chunk of the migration).

## 8. Migration checklist

1. ☐ Move to Java 25+
2. ☐ Swap the dependency (`io.github.brevilo:jolm` → `io.github.fherbreteau:vodozemac-java`)
3. ☐ Remove libolm installation/packaging steps
4. ☐ Replace `Utility.sha256` with a standard SHA-256 implementation
5. ☐ Replace `Utility.verifyEd25519` with `Ed25519PublicKey.verify(...)` (boolean result)
6. ☐ Replace `PkSigning` with `Ed25519KeyPair` (cross-signing keys: persist with `pickle()`, restore with `unpickle()`)
7. ☐ Adapt session creation/decryption to the typed API (`OlmMessage`, `DecryptedMessage`, `InboundCreationResult`)
8. ☐ Replace one-time key bookkeeping (`removeOneTimeKeys` is automatic)
9. ☐ Convert all pickle handling: `unpickleLegacy` + re-pickle with 32-byte keys
10. ☐ Wrap every handle in try-with-resources (`close()` instead of `clear()`)
11. ☐ Map `OlmException` handling to the `VodozemacException` hierarchy
12. ☐ Test interop: messages encrypted by a libolm peer must decrypt with the migrated code (v1 protocol), and legacy pickles must import cleanly
