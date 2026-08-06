# Missing Features Implementation Plan

This document tracks the gap between the vodozemac 0.9.0 Rust API and the Java bindings.
It is organized into phases by priority, with each phase being independently deliverable.

---

## Phase 1: InboundGroupSession session management (High priority)

These methods are needed for session sharing, key rotation, and trust management —
core Megolm functionality that clients need.

### 1.1 `InboundGroupSession.import(ExportedSessionKey, MegolmSessionVersion)`

- **Java**: Add constructor `InboundGroupSession(ExportedSessionKey key, MegolmSessionVersion version)`
  or static factory `InboundGroupSession.import(String exportedKey, MegolmSessionVersion version)`
- **Rust**: `Java_..._InboundGroupSession_nativeImport(String, jint) -> jlong` calling
  `InboundGroupSession::import(&ExportedSessionKey::from_base64(...), config)`
- **Java type**: `ExportedSessionKey` — either a new class or just a base64 `String` parameter
  (simpler, consistent with `SessionKey` handling)

### 1.2 `export_at(index)` and `export_at_first_known_index()`

- **Java**: `String exportAt(int index)` returning base64 `ExportedSessionKey`,
  `String exportAtFirstKnownIndex()` returning base64
- **Rust**: `Java_..._InboundGroupSession_nativeExportAt(long, jint) -> jstring` calling
  `session.export_at(index)` -> `ExportedSessionKey::to_base64()`
- **Rust**: `Java_..._InboundGroupSession_nativeExportAtFirstKnownIndex(long) -> jstring`
  calling `session.export_at_first_known_index().to_base64()`

### 1.3 `advance_to(index)`

- **Java**: `boolean advanceTo(int index)`
- **Rust**: `Java_..._InboundGroupSession_nativeAdvanceTo(long, jint) -> jboolean` calling
  `session.advance_to(index)`

### 1.4 `connected(other)`, `compare(other)`, `merge(other)` + `SessionOrdering` enum

- **Java**: `boolean connected(InboundGroupSession other)`,
  `SessionOrdering compare(InboundGroupSession other)`,
  `Optional<InboundGroupSession> merge(InboundGroupSession other)`
- **Java enum**: `SessionOrdering` with values `EQUAL`, `BETTER`, `WORSE`, `UNCONNECTED`
- **Rust**: 3 JNI functions + Java object construction for `InboundGroupSession` result from merge
- **Note**: `merge` returns `Option<InboundGroupSession>` — needs to return either a new
  `InboundGroupSession` pointer or null

### Estimated effort: ~6 JNI functions, 1 Java enum, ~4 new Java methods

---

## Phase 2: SAS module (High priority)

Short Authentication String verification — needed for device verification flows.

### 2.1 New Java classes

| Java class           | vodozemac type                  | Key methods                                                                        |
| -------------------- | ------------------------------- | ---------------------------------------------------------------------------------- |
| `Sas`                | `vodozemac::sas::Sas`           | `new()`, `publicKey() -> String`, `diffieHellman(String) -> EstablishedSas`       |
| `EstablishedSas`     | `vodozemac::sas::EstablishedSas`| `bytes(String info) -> byte[]`, `calculateMac(String, String) -> String`, `verifyMac(...)`, `ourPublicKey()`, `theirPublicKey()` |
| `SasBytes`           | `vodozemac::sas::SasBytes`      | `emojiIndices() -> int[]`, `decimals() -> String[]`                                |
| `Mac`                | `vodozemac::sas::Mac`           | `toBase64() -> String` (or just use String throughout)                              |

### 2.2 Rust JNI module: `sas/`

New directory `rust/src/sas/` with:
- `mod.rs` — module declarations
- `sas.rs` — `Sas` JNI functions (`nativeNew`, `nativePublicKey`, `nativeDiffieHellman`, `nativeFree`) + tests
- `established_sas.rs` — `EstablishedSas` JNI functions (`nativeBytes`, `nativeCalculateMac`, `nativeVerifyMac`, `nativeOurPublicKey`, `nativeTheirPublicKey`, `nativeFree`) + tests

### 2.3 Design decisions

- **`SasBytes`**: Could be exposed as a simple class with `emojiIndices()` returning `int[]`
  and `decimals()` returning `String[]` (3 decimal pairs). Or embed it in `EstablishedSas.bytes()` return.
- **`Mac`**: Could be just a `String` (base64) rather than a full class — simpler API.
- **AutoCloseable**: Both `Sas` and `EstablishedSas` need `close()` to free native resources.

### Estimated effort: ~12 JNI functions, 2-4 Java classes, 1 Rust module

---

## Phase 3: ECIES module (High priority)

QR-code-based device login (MSC3886).

### 3.1 New Java classes

| Java class                  | vodozemac type                          | Key methods                                                                     |
| --------------------------- | --------------------------------------- | ------------------------------------------------------------------------------- |
| `Ecies`                     | `vodozemac::ecies::Ecies`               | `new()`, `withInfo(String)`, `establishOutboundChannel(String) -> OutboundCreationResult`, `establishInboundChannel(String) -> InboundCreationResult`, `publicKey() -> String` |
| `EstablishedEcies`          | `vodozemac::ecies::EstablishedEcies`    | `publicKey() -> String`, `checkCode() -> CheckCode`, `encrypt(byte[]) -> String`, `decrypt(String) -> byte[]`, `close()` |
| `CheckCode`                 | `vodozemac::ecies::CheckCode`           | `asBytes() -> byte[]`, `toDigit() -> int`                                       |
| `EciesOutboundCreationResult` | `vodozemac::ecies::OutboundCreationResult` | `getEstablishedEcies() -> EstablishedEcies`, `getInitialMessage() -> String`   |
| `EciesInboundCreationResult`  | `vodozemac::ecies::InboundCreationResult`  | `getEstablishedEcies() -> EstablishedEcies`, `getPlaintext() -> byte[]`        |

### 3.2 Rust JNI module: `ecies/`

New directory `rust/src/ecies/` with JNI functions following the same pattern as
`olm/account.rs` (for `Ecies`) and `olm/session.rs` (for `EstablishedEcies`).

### 3.3 Design decisions

- `InitialMessage` and `Message` in Rust have `encode()`/`decode()` methods — in Java,
  these can be represented as `String` (base64), same as `OlmMessage` is JSON/base64.
- `CheckCode` is a small immutable value — simple class with 2 methods.

### Estimated effort: ~15 JNI functions, 5 Java classes, 1 Rust module

---

## Phase 4: PK Encryption module (High priority)

Megolm key backup. Requires the `insecure-pk-encryption` cargo feature.

### 4.1 Cargo.toml change

```toml
[dependencies]
vodozemac = { version = "0.9.0", features = ["libolm-compat", "insecure-pk-encryption"] }
```

### 4.2 New Java classes

| Java class      | vodozemac type                          | Key methods                                                                    |
| --------------- | --------------------------------------- | ----------------------------------------------------------------------------- |
| `PkEncryption`  | `vodozemac::pk_encryption::PkEncryption` | `fromKey(String key) -> PkEncryption`, `encrypt(byte[] plaintext) -> PkMessage` |
| `PkDecryption`  | `vodozemac::pk_encryption::PkDecryption` | `new()`, `fromKey(String key)`, `secretKey() -> String`, `publicKey() -> String`, `decrypt(PkMessage) -> byte[]`, `pickle(key)` / `unpickle` / `unpickleLegacy` |
| `PkMessage`     | `vodozemac::pk_encryption::Message`      | `getCiphertext() -> String`, `getMac() -> String`, `getEphemeralKey() -> String` |

### 4.3 Rust JNI module: `pk_encryption/`

New directory `rust/src/pk_encryption/` with JNI functions.

### 4.4 Design decisions

- `PkMessage` could be a simple data class (3 base64 strings) or a JSON-serialized `String`
  (consistent with `OlmMessage`). Data class is cleaner since it has 3 distinct fields.
- `PkDecryption` needs pickle/unpickle/legacy pickle (same pattern as Account/Session).

### Estimated effort: ~10 JNI functions, 3 Java classes, 1 Rust module, 1 Cargo.toml change

---

## Phase 5: Structured message types (Medium priority)

Expose `OlmMessage`, `MegolmMessage` as proper Java types instead of opaque strings.

### 5.1 `OlmMessage` Java class

- Fields: `MessageType type` (enum: `PRE_KEY(0)`, `NORMAL(1)`), `String body` (base64 ciphertext)
- Methods: `getType()`, `getBody()`, `toBase64() -> String`, `static fromBase64(String) -> OlmMessage`
- Update `OlmSession.encrypt()` to return `OlmMessage` instead of `String`
- Update `OlmSession.decrypt()` to accept `OlmMessage` (or keep `String` overload for backward compat)
- Update `Account.createInboundSession()` to accept `OlmMessage`

### 5.2 `MegolmMessage` Java class

- Fields: `String ciphertext` (base64), `int messageIndex`, `String mac`, `String signature`
- Methods: getters, `toBase64()`, `fromBase64()`
- Update `OutboundGroupSession.encrypt()` to return `MegolmMessage`
- Update `InboundGroupSession.decrypt()` to accept `MegolmMessage`

### 5.3 Design decisions

- **Backward compatibility**: Keep `String` overloads of `encrypt`/`decrypt` for migration,
  or make a clean break.
- **`MessageType` enum**: Already partially exists as `OlmMessage.java` (empty class) — fill it in.

### Estimated effort: ~8 JNI function signature changes, 2-3 Java classes, enum

---

## Phase 6: Cryptographic key types (Medium priority)

Expose `Ed25519PublicKey`, `Ed25519Signature`, `Curve25519PublicKey` as Java types.

### 6.1 New Java classes

| Java class          | vodozemac type                  | Key methods                                                              |
| ------------------- | ------------------------------- | ----------------------------------------------------------------------- |
| `Ed25519PublicKey`  | `vodozemac::Ed25519PublicKey`    | `fromBase64(String)`, `toBase64() -> String`, `verify(String message, String signature) -> boolean` |
| `Ed25519Signature`  | `vodozemac::Ed25519Signature`    | `fromBase64(String)`, `toBase64() -> String`                             |
| `Curve25519PublicKey` | `vodozemac::Curve25519PublicKey` | `fromBase64(String)`, `toBase64() -> String`                          |

### 6.2 Design decisions

- `Ed25519Keypair` and secret keys are likely not needed in Java (clients don't create
  keypairs directly — `Account` does).
- `Ed25519PublicKey.verify()` is the main use case — verify signatures from other devices.
- Could be simpler to just add `Account.verify(String message, String signature, String theirEd25519Key)`
  without exposing the key type. But exposing the type is more flexible.

### Estimated effort: ~6 JNI functions, 3 Java classes, 1 Rust module

---

## Phase 7: Missing methods on existing classes (Medium priority)

### 7.1 `Account.toLibolmPickle(byte[] key)`

- **Java**: `String toLibolmPickle(byte[] key)` (with key length validation)
- **Rust**: `Java_..._Account_nativeToLibolmPickle(long, JByteArray) -> jstring` calling
  `account.to_libolm_pickle(&key)`

### 7.2 `OlmSession.sessionKeys()` and `sessionConfig()`

- **Java**: `SessionKeys sessionKeys()` returning a new `SessionKeys` class with
  `sessionId()`, `identityKey()`, `baseKey()`, `oneTimeKey()`
- **Java**: `OlmSessionVersion sessionConfig()` returning the version
- **Rust**: 2 JNI functions returning a Java object / jint

### 7.3 `OutboundGroupSession.sessionConfig()`

- **Java**: `MegolmSessionVersion sessionConfig()`
- **Rust**: 1 JNI function

### 7.4 Fix typo: `createOutbpundSession` -> `createOutboundSession`

- Rename in `Account.java` + update all callers (tests, `Sample.java`)
- The Rust JNI name doesn't change (it's `nativeCreateOutboundSession` in Java,
  `Java_..._nativeCreateOutboundSession` in Rust — no typo there)

### Estimated effort: ~4 JNI functions, 1 Java class, 1 rename

---

## Phase 8: Granular error types (Low priority)

Replace `VodozemacException(String)` with typed exceptions.

### 8.1 Approach

Option A: Subclass `VodozemacException` with specific types:
- `PickleException extends VodozemacException`
- `DecryptionException extends VodozemacException` (with variants `InvalidMAC`, `InvalidPadding`,
  `UnknownMessageIndex`, etc.)
- `SessionCreationException extends VodozemacException`
- `KeyException extends VodozemacException`
- `SignatureException extends VodozemacException`

Option B: Single `VodozemacException` with an error code enum:
- `VodozemacException(VodozemacError error, String message)`
- `enum VodozemacError { PICKLE, DECRYPTION, SESSION_CREATION, KEY, SIGNATURE, ... }`

### 8.2 Design decisions

- Option A is more idiomatic Java but requires many classes.
- Option B is simpler but less type-safe.
- **Recommendation**: Option A with a common base class, only for errors that Java callers
  need to handle differently (e.g., `UnknownMessageIndex` vs `InvalidMAC` in Megolm decryption).

### Estimated effort: ~10 exception classes, JNI error mapping changes

---

## Phase 9: Utility functions (Low priority)

### 9.1 `base64Encode(byte[]) -> String` and `base64Decode(String) -> byte[]`

- Simple static methods on a `Vodozemac` utility class
- Thin wrappers around `vodozemac::base64_encode` / `base64_decode`

### 9.2 `VERSION` constant

- `Vodozemac.getVersion() -> String` returning the vodozemac crate version

### Estimated effort: 3 JNI functions, 1 Java class

---

## Summary by phase

| Phase                              | Priority | JNI functions | Java classes | Rust modules     |
| ---------------------------------- | -------- | ------------- | ------------ | ---------------- |
| 1. InboundGroupSession management  | High     | ~6            | 1 enum       | Existing `megolm/` |
| 2. SAS module                      | High     | ~12           | 2-4          | New `sas/`       |
| 3. ECIES module                    | High     | ~15           | 5            | New `ecies/`     |
| 4. PK Encryption                   | High     | ~10           | 3            | New `pk_encryption/` + Cargo.toml |
| 5. Structured messages             | Medium   | ~8 changes    | 2-3          | Existing modules |
| 6. Crypto key types                | Medium   | ~6            | 3            | New `types/`     |
| 7. Missing methods                 | Medium   | ~4            | 1            | Existing modules |
| 8. Error types                      | Low      | ~0 (mapping)  | ~10          | Existing modules |
| 9. Utilities                       | Low      | ~3            | 1            | New `utils/`     |
| **Total**                          |          | **~64**       | **~30**      | **4 new**        |
