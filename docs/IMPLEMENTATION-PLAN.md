# Missing Features Implementation Plan

This document tracks the gap between the vodozemac 0.9.0 Rust API and the Java bindings,
as well as issues identified in the code review (`CODE_REVIEW.md`).
It is organized into phases by priority, with each phase being independently deliverable.

Phases 3-7, 9 cover missing vodozemac features.
Phase 14 covers code review fixes (refactoring and deduplication).

**Completed phases:** Phase 1 (InboundGroupSession session management), Phase 2 (SAS module), Phase 3 (ECIES module), Phase 4 (PK Encryption module), Phase 8 (Granular error types), Phase 10 (Code quality and duplication fixes), Phase 11 (Build and configuration fixes), Phase 12 (Documentation overhaul), and Phase 13 (Security hardening) have been implemented. Phase 13.3 (cause chaining for VodozemacException) was implemented as part of Phase 8.

---

## ~~Phase 3: ECIES module (High priority)~~

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

## ~~Phase 4: PK Encryption module (High priority)~~

Megolm key backup. Requires the `insecure-pk-encryption` cargo feature.

### ~~4.1 Cargo.toml change~~

```toml
[dependencies]
vodozemac = { version = "0.10.0", features = ["libolm-compat", "insecure-pk-encryption"] }
```

Done. Note: vodozemac 0.10.0 is used instead of the originally planned 0.9.0.

### ~~4.2 New Java classes~~

| Java class      | vodozemac type                          | Key methods                                                                    |
| --------------- | --------------------------------------- | ----------------------------------------------------------------------------- |
| `PkEncryption`  | `vodozemac::pk_encryption::PkEncryption` | `fromKey(String key) -> PkEncryption`, `encrypt(byte[] plaintext) -> PkMessage` |
| `PkDecryption`  | `vodozemac::pk_encryption::PkDecryption` | `new()`, `fromKey(String key)`, `secretKey() -> String`, `publicKey() -> String`, `decrypt(PkMessage) -> byte[]`, `unpickleLegacy(String, byte[])` |
| `PkMessage`     | `vodozemac::pk_encryption::Message`      | `getCiphertext() -> String`, `getMac() -> String`, `getEphemeralKey() -> String` |

Done. All three classes are implemented with full Javadoc. Note: PkDecryption only
implements `unpickleLegacy` — vodozemac does not provide JSON-based `pickle()`/`unpickle()`
for PkDecryption, only `to_libolm_pickle`/`from_libolm_pickle`. An `EncryptionException`
class was also created (referenced by the Rust JNI layer but previously missing in Java).

### ~~4.3 Rust JNI module: `backup/`~~

Implemented as `rust/src/backup/` (containing `decryption.rs` and `encryption.rs`)
instead of the originally planned `rust/src/pk_encryption/`.

### ~~4.4 Design decisions~~

- `PkMessage` is a simple data class (3 base64 strings) — cleaner than JSON since it has
  3 distinct fields.
- `PkDecryption` only implements `unpickleLegacy` since vodozemac only provides libolm
  pickle format for PkDecryption (no JSON-based pickle/unpickle).

### ~~Estimated effort: ~10 JNI functions, 3 Java classes, 1 Rust module, 1 Cargo.toml change~~

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

### ~~7.4 Fix typo: `createOutbpundSession` -> `createOutboundSession`~~

- ~~Rename in `Account.java` + update all callers (tests, `Sample.java`)~~
- ~~The Rust JNI name doesn't change (it's `nativeCreateOutboundSession` in Java,
  `Java_..._nativeCreateOutboundSession` in Rust — no typo there)~~

### Estimated effort: ~4 JNI functions, 1 Java class, 1 rename

---

## Phase 9: Utility functions (Low priority)

### 9.1 `base64Encode(byte[]) -> String` and `base64Decode(String) -> byte[]`

- Simple static methods on a `Vodozemac` utility class
- Thin wrappers around `vodozemac::base64_encode` / `base64_decode`

### 9.2 `VERSION` constant

- `Vodozemac.getVersion() -> String` returning the vodozemac crate version

### Estimated effort: 3 JNI functions, 1 Java class

---

## Phase 14: Refactoring — common base class and deduplication (Low priority)

Addresses CODE_REVIEW.md duplication items 5.1-5.8. This phase reduces maintenance burden
by extracting common patterns shared across `Account`, `OlmSession`, `OutboundGroupSession`,
and `InboundGroupSession`.

### 14.1 Extract `NativeHandle` abstract base class

All 4 AutoCloseable classes share identical `nativePtr`, `checkNotClosed()`, `isClosed()`,
and `close()` patterns.

- **Java**: Create `io.github.fherbreteau.vodozemac.NativeHandle`:
  ```java
  public abstract class NativeHandle implements AutoCloseable {
      protected long nativePtr;

      protected NativeHandle(long nativePtr) {
          this.nativePtr = nativePtr;
      }

      protected void checkNotClosed() {
          if (nativePtr == 0) {
              throw new IllegalStateException(getClass().getSimpleName() + " has been closed");
          }
      }

      boolean isClosed() {
          return nativePtr == 0;
      }

      protected abstract void nativeFree(long ptr);

      @Override
      public void close() {
          if (nativePtr != 0) {
              nativeFree(nativePtr);
              nativePtr = 0;
          }
      }
  }
  ```
- **Note**: `checkNotClosed()` uses `getClass().getSimpleName()` to produce the correct class
  name in the error message — fixes the copy-paste bug from 10.1 automatically
- **Impact**: `Account`, `OlmSession`, `OutboundGroupSession`, `InboundGroupSession` extend
  `NativeHandle` and only implement `nativeFree()` + their specific methods

### 14.2 Extract `KeyValidator` utility

The 32-byte key validation is duplicated in 6+ locations.

- **Java**: Create `io.github.fherbreteau.vodozemac.KeyValidator`:
  ```java
  public final class KeyValidator {
      private KeyValidator() {}

      public static void validateEncryptionKey(byte[] key) {
          if (key.length != 32) {
              throw new VodozemacException("Encrypted Key must be 256-bit (32-byte)");
          }
      }
  }
  ```
- **Impact**: Replace all `if (key.length != 32)` blocks with `KeyValidator.validateEncryptionKey(key)`

### 14.3 Consolidate session version enums

`OlmSessionVersion` and `MegolmSessionVersion` are structurally identical (V1(1), V2(2)).

- **Option A**: Replace both with a single `SessionVersion` enum in the root package
- **Option B**: Keep separate enums but have them implement a common `SessionVersion` interface
  with `getValue()` method
- **Design decision**: Option A is simpler but loses type safety (a Megolm method could accept
  an Olm version). Option B preserves type safety.
- **Recommendation**: Option B — keep separate enums for type safety, extract a common
  `SessionVersion` interface

### 14.4 Deduplicate session version tests

`OlmSessionVersionTest` and `MegolmSessionVersionTest` are structurally identical.

- If 14.3 Option B is chosen: Create a shared test utility or parameterized test that works
  with any `SessionVersion` implementation
- **Approach**: JUnit 5 `@ParameterizedTest` with arguments from both enums

### 14.5 Extract Rust JNI pickle/unpickle into a generic helper

The `nativePickle`, `nativeEncryptedPickle`, `nativeUnpickle`, `nativeEncryptedUnpickle`,
`nativeUnpickleLegacy` functions are nearly identical across all 4 Rust modules, differing
only in the Rust type name.

- **Rust**: Create generic helper functions in `helpers.rs`:
  ```rust
  pub(crate) fn jni_pickle<T: Serialize>(
      env: &mut Env,
      value: &T,
  ) -> Result<jstring, jni::errors::Error> { ... }

  pub(crate) fn jni_encrypted_pickle<T: Serialize>(
      env: &mut Env,
      value: &T,
      key: &[u8; 32],
  ) -> Result<jstring, jni::errors::Error> { ... }

  pub(crate) fn jni_unpickle<T: DeserializeOwned>(
      pickle_str: &str,
  ) -> Result<T, jni::errors::Error> { ... }

  pub(crate) fn jni_encrypted_unpickle<T: DeserializeOwned>(
      pickle_str: &str,
      key: &[u8; 32],
  ) -> Result<T, jni::errors::Error> { ... }
  ```
- **Impact**: Each JNI function body becomes a 2-3 line call to the helper

### 14.6 Extract Rust JNI `nativeFree` into a generic helper

All 4 `nativeFree` functions are identical except for the Rust type.

- **Rust**: Create a generic helper:
  ```rust
  pub(crate) unsafe fn native_free<T>(ptr: jlong) {
      let _ = Box::from_raw(ptr as *mut T);
  }
  ```
- **Impact**: Each `nativeFree` function becomes a single-line call

### Estimated effort: ~0 JNI functions, ~3 Java classes (NativeHandle, KeyValidator, SessionVersion interface), Rust helpers refactor

---

## Summary by phase

| Phase                              | Priority | JNI functions | Java classes | Rust modules     |
| ---------------------------------- | -------- | ------------- | ------------ | ---------------- |
| ~~1. InboundGroupSession management~~ | ~~High~~ | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| ~~2. SAS module~~                      | ~~High~~   | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| ~~3. ECIES module~~                    | ~~High~~   | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| ~~4. PK Encryption~~                 | ~~High~~   | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| 5. Structured messages             | Medium   | ~8 changes    | 2-3          | Existing modules |
| 6. Crypto key types                | Medium   | ~6            | 3            | New `types/`     |
| 7. Missing methods                 | Medium   | ~4            | 1            | Existing modules |
| ~~8. Error types~~                    | ~~Low~~   | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| 9. Utilities                       | Low      | ~3            | 1            | New `utils/`     |
| ~~10. Code quality & duplication~~ | ~~High~~ | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| ~~11. Build & configuration~~      | ~~High~~ | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| ~~12. Documentation overhaul~~     | ~~Medium~~ | ~~Done~~   | ~~Done~~     | ~~Done~~         |
| ~~13. Security hardening~~         | ~~Medium~~ | ~~Done~~   | ~~Done~~     | ~~Done~~         |
| 14. Refactoring & deduplication    | Low      | ~0            | ~3           | Existing `helpers.rs` |
| **Remaining total**                |          | **~48**       | **~20**      | **3 new + helpers** |

### Code review findings coverage

| CODE_REVIEW.md finding | Phase | Section |
|---|---|---|
| ~~C1: Typo `createOutbpundSession`~~ | ~~Phase 7~~ | ~~Done~~ |
| ~~C2: Wrong `checkNotClosed()` error messages~~ | ~~Phase 10~~ | ~~Done~~ |
| ~~C3: `OlmSession` constructor is public~~ | ~~Phase 13~~ | ~~Done~~ |
| ~~C4-C7: Missing key validation in Megolm~~ | ~~Phase 10~~ | ~~Done~~ |
| ~~C8: Unused import in Rust tests~~ | ~~Phase 10~~ | ~~Done (already absent)~~ |
| ~~C9: `VodozemacException` no cause chaining~~ | ~~Phase 13~~ | ~~Done (Phase 8)~~ |
| ~~C10: `InboundCreationResult` constructor public~~ | ~~Phase 13~~ | ~~Done~~ |
| ~~C11: Javadoc `@link` wrong signature~~ | ~~Phase 13~~ | ~~Done~~ |
| ~~C12: `pom.xml` mainClass wrong~~ | ~~Phase 11~~ | ~~Done~~ |
| ~~C13: `helpers.rs:wrap()` uses `unwrap()`~~ | ~~Phase 10~~ | ~~Done~~ |
| ~~C14: Rust JNI `.unwrap()` on `convert_byte_array`~~ | ~~Phase 10~~ | ~~Done~~ |
| ~~C15: README imports `VodozemacAccount`~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~D1: Undefined Maven property~~ | ~~Phase 11~~ | ~~Done~~ |
| ~~D2: Stale root `Cargo.lock`~~ | ~~Phase 11~~ | ~~Done~~ |
| ~~D3: Java version mismatch~~ | ~~Phase 11~~ | ~~Done~~ |
| ~~DOC1-DOC4: README inaccuracies~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~DOC5-DOC6: `Sample.java` wrong log messages~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~DOC7: Missing `CODE_OF_CONDUCT.md`~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~DOC8-DOC10: Javadoc typos~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~S1: Fake GPG key in SECURITY.md~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~S2: Missing key validation (Megolm)~~ | ~~Phase 10~~ | ~~Done~~ |
| ~~S3: Native pointer exposed in `InboundCreationResult`~~ | ~~Phase 13~~ | ~~Done~~ |
| ~~S4: Inconsistent contact info~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~S5: French error messages~~ | ~~Phase 12~~ | ~~Done~~ |
| ~~S6: Temp file permissions~~ | ~~Phase 13~~ | ~~Done~~ |
| ~~S7: `System.load` restricted warning~~ | ~~Phase 13~~ | ~~Done~~ |
| 5.1-5.8: Duplicated code patterns | Phase 14 | 14.1-14.6 |
| ~~JaCoCo coverage not enforced~~ | ~~Phase 11~~ | ~~Done~~ |
