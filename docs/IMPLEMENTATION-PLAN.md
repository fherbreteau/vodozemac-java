# Missing Features Implementation Plan

This document tracks the gap between the vodozemac 0.9.0 Rust API and the Java bindings,
as well as issues identified in the code review (`CODE_REVIEW.md`).
It is organized into phases by priority, with each phase being independently deliverable.

Phases 2-7, 9 cover missing vodozemac features.
Phases 12-14 cover code review fixes (security hardening, build/config, documentation, and refactoring).

**Completed phases:** Phase 1 (InboundGroupSession session management), Phase 8 (Granular error types), Phase 10 (Code quality and duplication fixes), and Phase 11 (Build and configuration fixes) have been implemented. Phase 13.3 (cause chaining for VodozemacException) was implemented as part of Phase 8.

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

## Phase 9: Utility functions (Low priority)

### 9.1 `base64Encode(byte[]) -> String` and `base64Decode(String) -> byte[]`

- Simple static methods on a `Vodozemac` utility class
- Thin wrappers around `vodozemac::base64_encode` / `base64_decode`

### 9.2 `VERSION` constant

- `Vodozemac.getVersion() -> String` returning the vodozemac crate version

### Estimated effort: 3 JNI functions, 1 Java class

---

## Phase 12: Documentation overhaul (Medium priority)

Addresses CODE_REVIEW.md findings DOC1-DOC10, C15, S1, S4, DOC7.

### 12.1 Rewrite README.md to match actual API

The README references `VodozemacAccount` (a class that doesn't exist), says "6 test cases"
(actual: 46), and shows an outdated architecture diagram.

- **Class name**: Replace all `VodozemacAccount` references with `Account` (`README.md:60,65,89,112,119,124,132`)
- **Import statements**: Update example imports from `io.github.fherbreteau.vodozemac.VodozemacAccount`
  to `io.github.fherbreteau.vodozemac.account.Account`
- **Test count**: Update "6 test cases" to actual count (`README.md:20`)
- **Architecture**: Update project structure to show `olm/` and `megolm/` modules instead of
  flat `rust/src/lib.rs` (`README.md:155`)
- **API reference**: Update to list all actual methods on `Account` (identityKeys, maxNumberOfOneTimeKeys,
  generateOneTimeKeys, storedOneTimeKeyCount, getUnpublishedOneTimeKeys, generateFallbackKey,
  getUnpublishedFallbackKey, forgetFallbackKey, markKeysAsPublished, pickle/unpickle, dehydrated device)
- **Cross-platform table**: Remove or mark macOS Intel as unsupported (commented out in CI)

### 12.2 Fix Javadoc typos and inaccuracies

| # | File | Line | Fix |
|---|---|---|---|
| 1 | `Account.java` | :50 | `@return a base 64 representation of the public Curve25519 key` → `Ed25519 key` |
| 2 | `Account.java` | :107 | `recieved from the sebder` → `received from the sender` |
| 3 | `Account.java` | :278 | `@link Account#toDehydratedDevice(String)` → `@link Account#toDehydratedDevice(byte[])` |
| 4 | `Account.java` | :307 | `@InheritDoc` → `@inheritDoc` |
| 5 | `OlmSession.java` | :131 | `@InheritDoc` → `@inheritDoc` |
| 6 | `OutboundGroupSession.java` | :74 | `@InheritDoc` → `@inheritDoc`, `Account` → `OutboundGroupSession` |
| 7 | `InboundGroupSession.java` | :70 | `@InheritDoc` → `@inheritDoc`, `Account` → `InboundGroupSession` |

### 12.3 Fix `Sample.java` log messages

- `Sample.java:52`: Says `"Bob: Received message"` but this is Alice decrypting Bob's reply → `"Alice: Received message"`
- `Sample.java:62`: Same issue → `"Alice: Received message"`

### 12.4 Replace or remove fake GPG key in SECURITY.md

`SECURITY.md:22-33` contains a placeholder GPG key with repetitive `QJ5QJ5...` data.

- **Option A**: Replace with the maintainer's real GPG public key
- **Option B**: Remove the GPG section entirely and direct users to email/Matrix only
- **Recommendation**: Option B if no real key is available; a fake key is worse than no key

### 12.5 Standardize contact information

Inconsistent contact emails across files:

| File | Current | |
|---|---|---|
| `SECURITY.md` | `security@fherbreteau.io` | |
| `CHANGELOG.md` | `fherbreteau@protonmail.com` | |
| `pom.xml` | `fherbreteau@gmail.com` | |

- **Action**: Pick one authoritative email and use it consistently
- **Recommendation**: Use `fherbreteau@gmail.com` (from pom.xml) everywhere, or set up a
  dedicated `security@` address and use it in both SECURITY.md and pom.xml

### 12.6 Standardize error messages to English in `NativeLibraryLoader`

`NativeLibraryLoader.java` has French error messages while the rest of the codebase uses English.

| Line | Current (French) | Replacement (English) |
|---|---|---|
| :49 | `"Impossible de charger la librairie native pour "` | `"Failed to load native library for "` |
| :63 | `"OS non supporté: "` | `"Unsupported OS: "` |
| :72 | `"Architecture non supportée: "` | `"Unsupported architecture: "` |
| :86 | `"OS non supporté: "` | `"Unsupported OS: "` |
| :92 | `"Ressource native introuvable: "` | `"Native resource not found: "` |

### 12.7 Create `CODE_OF_CONDUCT.md` or remove reference

`CONTRIBUTING.md:22` references `CODE_OF_CONDUCT.md` which does not exist.

- **Option A**: Create a `CODE_OF_CONDUCT.md` (e.g., Contributor Covenant 2.1)
- **Option B**: Remove the "Code of Conduct" section and link from `CONTRIBUTING.md`
- **Recommendation**: Option A for community projects

### Estimated effort: 0 code logic changes, documentation only

---

## Phase 13: Security hardening (Medium priority)

Addresses CODE_REVIEW.md findings S3, S5, S6, S7, C3, C9, C10, C11.

### 13.1 Make `OlmSession` constructor package-private

`OlmSession.java:8` has a public constructor that accepts a raw `long nativePtr`, allowing
any caller to construct a session with an arbitrary pointer.

- **Java**: Change `public OlmSession(long nativePtr)` to `OlmSession(long nativePtr)`
  (package-private)
- **Impact**: `InboundCreationResult` (same package `io.github.fherbreteau.vodozemac.olm`)
  can still call it; `Account` calls `nativeCreateOutboundSession` which returns a `long`,
  then constructs `OlmSession` — `Account` is in a different package
  (`io.github.fherbreteau.vodozemac.account`), so it needs access
- **Design decision**: Either move `OlmSession` to the same package as `Account`, or create a
  package-private factory method, or keep public but document the constraint

### 13.2 Make `InboundCreationResult` constructor package-private

`InboundCreationResult.java:10` has a public constructor accepting `long sessionPtr` and
`byte[] plaintext`, exposing the raw native pointer.

- **Java**: Change `public InboundCreationResult(long sessionPtr, byte[] plaintext)` to
  package-private
- **Impact**: Only `Account` (via JNI `nativeCreateInboundSession`) constructs this — `Account`
  is in a different package
- **Design decision**: Same as 13.1 — need cross-package access solution

### 13.3 Set restrictive permissions on extracted native library

`NativeLibraryLoader.java:96-106` extracts the native library to a temp directory but does
not set file permissions, potentially allowing other users to read/modify the library.

- **Java**: After `Files.copy()`, set owner-only permissions:
  ```java
  Files.setPosixFilePermissions(tempLib,
      Set.of(PosixFilePermission.OWNER_READ, PosixFilePermission.OWNER_WRITE));
  ```
- **Fallback**: On Windows (no POSIX permissions), use `tempLib.toFile().setReadable(false, false)`
  and `setWritable(false, false)` to restrict access for other users

### 13.4 Add `--enable-native-access=ALL-UNNAMED` for Java 25+ compatibility

The JVM warns about restricted native access when calling `System.load()`. On Java 25+,
this may become a hard error.

- **pom.xml**: Add to `maven-surefire-plugin` argLine:
  `--enable-native-access=ALL-UNNAMED`
- **Documentation**: Document the requirement in README for consumers of the library

### 13.5 Fix Javadoc `@link` with wrong method signature

- `Account.java:278`: `@link Account#toDehydratedDevice(String)` should be
  `@link Account#toDehydratedDevice(byte[])` — the actual method takes `byte[]`, not `String`

### Estimated effort: ~0 JNI functions, security hardening (VodozemacException cause chaining done in Phase 8)

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
| 2. SAS module                      | High     | ~12           | 2-4          | New `sas/`       |
| 3. ECIES module                    | High     | ~15           | 5            | New `ecies/`     |
| 4. PK Encryption                   | High     | ~10           | 3            | New `pk_encryption/` + Cargo.toml |
| 5. Structured messages             | Medium   | ~8 changes    | 2-3          | Existing modules |
| 6. Crypto key types                | Medium   | ~6            | 3            | New `types/`     |
| 7. Missing methods                 | Medium   | ~4            | 1            | Existing modules |
| ~~8. Error types~~                    | ~~Low~~   | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| 9. Utilities                       | Low      | ~3            | 1            | New `utils/`     |
| ~~10. Code quality & duplication~~ | ~~High~~ | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| ~~11. Build & configuration~~      | ~~High~~ | ~~Done~~    | ~~Done~~     | ~~Done~~         |
| 12. Documentation overhaul          | Medium   | N/A           | N/A          | N/A (docs only)  |
| 13. Security hardening             | Medium   | ~0            | ~0           | N/A              |
| 14. Refactoring & deduplication    | Low      | ~0            | ~3           | Existing `helpers.rs` |
| **Remaining total**                |          | **~58**       | **~23**      | **4 new + helpers** |

### Code review findings coverage

| CODE_REVIEW.md finding | Phase | Section |
|---|---|---|
| C1: Typo `createOutbpundSession` | Phase 7 | 7.4 |
| ~~C2: Wrong `checkNotClosed()` error messages~~ | ~~Phase 10~~ | ~~Done~~ |
| C3: `OlmSession` constructor is public | Phase 13 | 13.1 |
| ~~C4-C7: Missing key validation in Megolm~~ | ~~Phase 10~~ | ~~Done~~ |
| ~~C8: Unused import in Rust tests~~ | ~~Phase 10~~ | ~~Done (already absent)~~ |
| ~~C9: `VodozemacException` no cause chaining~~ | ~~Phase 13~~ | ~~Done (Phase 8)~~ |
| C10: `InboundCreationResult` constructor public | Phase 13 | 13.2 |
| C11: Javadoc `@link` wrong signature | Phase 13 | 13.5 |
| ~~C12: `pom.xml` mainClass wrong~~ | ~~Phase 11~~ | ~~Done~~ |
| ~~C13: `helpers.rs:wrap()` uses `unwrap()`~~ | ~~Phase 10~~ | ~~Done~~ |
| ~~C14: Rust JNI `.unwrap()` on `convert_byte_array`~~ | ~~Phase 10~~ | ~~Done~~ |
| C15: README imports `VodozemacAccount` | Phase 12 | 12.1 |
| ~~D1: Undefined Maven property~~ | ~~Phase 11~~ | ~~Done~~ |
| ~~D2: Stale root `Cargo.lock`~~ | ~~Phase 11~~ | ~~Done~~ |
| ~~D3: Java version mismatch~~ | ~~Phase 11~~ | ~~Done~~ |
| DOC1-DOC4: README inaccuracies | Phase 12 | 12.1 |
| DOC5-DOC6: `Sample.java` wrong log messages | Phase 12 | 12.3 |
| DOC7: Missing `CODE_OF_CONDUCT.md` | Phase 12 | 12.7 |
| DOC8-DOC10: Javadoc typos | Phase 12 | 12.2 |
| S1: Fake GPG key in SECURITY.md | Phase 12 | 12.4 |
| ~~S2: Missing key validation (Megolm)~~ | ~~Phase 10~~ | ~~Done~~ |
| S3: Native pointer exposed in `InboundCreationResult` | Phase 13 | 13.2 |
| S4: Inconsistent contact info | Phase 12 | 12.5 |
| S5: French error messages | Phase 12 | 12.6 |
| S6: Temp file permissions | Phase 13 | 13.3 |
| S7: `System.load` restricted warning | Phase 13 | 13.4 |
| 5.1-5.8: Duplicated code patterns | Phase 14 | 14.1-14.6 |
| ~~JaCoCo coverage not enforced~~ | ~~Phase 11~~ | ~~Done~~ |
