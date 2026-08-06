# Missing Features Implementation Plan

This document tracks the gap between the vodozemac 0.9.0 Rust API and the Java bindings,
as well as issues identified in the code review (`CODE_REVIEW.md`).
It is organized into phases by priority, with each phase being independently deliverable.

Phases 1-9 cover missing vodozemac features.
Phases 10-14 cover code review fixes (duplicated code, security hardening, build/config, documentation, and refactoring).

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

## Phase 10: Code quality and duplication fixes (High priority)

Addresses CODE_REVIEW.md findings C2, C4, C5, C6, C7, C13, C14, and duplication items 5.1, 5.4.

### 10.1 Fix `checkNotClosed()` error messages

The `checkNotClosed()` method in `OlmSession`, `OutboundGroupSession`, and `InboundGroupSession`
all throw `"Account has been closed"` — a copy-paste bug from `Account`.

- **Java**: Update the error message in each class to reference the correct class name:
  - `OlmSession.checkNotClosed()` → `"OlmSession has been closed"`
  - `OutboundGroupSession.checkNotClosed()` → `"OutboundGroupSession has been closed"`
  - `InboundGroupSession.checkNotClosed()` → `"InboundGroupSession has been closed"`
- **Tests**: Update assertions in `OlmSessionTest`, `OutboundGroupSessionTest`,
  `InboundGroupSessionTest` that check for `"Account has been closed"`
- **Files**: `OlmSession.java:118`, `OutboundGroupSession.java:62`, `InboundGroupSession.java:57`

### 10.2 Add 32-byte key validation to Megolm pickle/unpickle methods

`OutboundGroupSession` and `InboundGroupSession` pickle/unpickle methods accept `byte[] key`
without validating that the key is 32 bytes, unlike `Account` and `OlmSession`.

- **Java**: Add the following validation to 4 methods:
  - `OutboundGroupSession.pickle(byte[] key)` — `OutboundGroupSession.java:40`
  - `OutboundGroupSession.unpickle(String, byte[])` — `OutboundGroupSession.java:50`
  - `InboundGroupSession.pickle(byte[] key)` — `InboundGroupSession.java:35`
  - `InboundGroupSession.unpickle(String, byte[])` — `InboundGroupSession.java:45`
- **Pattern** (consistent with `Account` and `OlmSession`):
  ```java
  if (key.length != 32) {
      throw new VodozemacException("Encrypted Key must be 256-bit (32-byte)");
  }
  ```
- **Tests**: Add tests for invalid key length on all 4 methods (same pattern as
  `AccountTest.testEncryptedPickleWithInvalidKeyThrowsException`)

### 10.3 Replace `.unwrap()` in Rust `helpers.rs:wrap()` with proper error handling

`helpers.rs:6` uses `unwrap()` which panics on wrong key length instead of returning a JNI error.

- **Rust**: Change `wrap()` to return `Result<[T; 32], jni::errors::Error>` and propagate the error:
  ```rust
  pub(crate) fn wrap<T>(v: Vec<T>) -> Result<[T; 32], jni::errors::Error> {
      v.try_into().map_err(|v: Vec<T>| {
          // This will be caught by the JNI error handler
          jni::errors::Error::JavaException
      })
  }
  ```
- **Rust**: Update all callers in `account.rs`, `session.rs`, `inbound_group_session.rs`,
  `outbound_group_session.rs` to use `?` operator instead of `.unwrap()`

### 10.4 Replace `.unwrap()` on `convert_byte_array` in Rust JNI pickle functions

Several Rust JNI functions call `env.convert_byte_array(key).unwrap()` which will panic
on failure instead of throwing a Java exception.

- **Rust**: Replace `.unwrap()` with `?` operator in:
  - `account.rs:374` (`nativeEncryptedPickle`), `account.rs:407` (`nativeEncryptedUnpickle`),
    `account.rs:445` (`nativeFromDehydratedDevice`), `account.rs:463` (`nativeToDehydratedDevice`)
  - `session.rs:113` (`nativeEncryptedPickle`), `session.rs:147` (`nativeEncryptedUnpickle`)
  - `inbound_group_session.rs:115` (`nativeEncryptedPickle`), `inbound_group_session.rs:148` (`nativeEncryptedUnpickle`)
  - `outbound_group_session.rs:114` (`nativeEncryptedPickle`), `outbound_group_session.rs:147` (`nativeEncryptedUnpickle`)
- **Pattern**: `let key = wrap(env.convert_byte_array(key)?)?;` (combines with 10.3)

### 10.5 Remove unused import in Rust tests

- **Rust**: Remove `use crate::helpers::PICKLE_KEY;` from `rust/src/olm/session.rs:177`
  (unused since the test module doesn't use `PICKLE_KEY`)

### Estimated effort: ~0 JNI functions, ~0 Java classes, Rust + Java fixes across existing files

---

## Phase 11: Build and configuration fixes (High priority)

Addresses CODE_REVIEW.md findings D1, D2, D3, C12, and the JaCoCo coverage enforcement gap.

### 11.1 Fix `pom.xml` `mainClass` reference

`pom.xml:353` declares `mainClass` as `io.github.fherbreteau.Main` — a class that does not exist.
The actual demo class is `io.github.fherbreteau.Sample`.

- **pom.xml**: Change `<mainClass>io.github.fherbreteau.Main</mainClass>` to
  `<mainClass>io.github.fherbreteau.Sample</mainClass>`
- **Alternative**: Remove the `mainClass` configuration entirely if the JAR is not meant to be
  directly executable (it's a library binding, not an application)

### 11.2 Fix undefined Maven property `${dependencies-version.version}`

`pom.xml:197` references `${dependencies-version.version}` for the `versions-maven-plugin`
but no such property is defined.

- **Option A**: Define the property: `<dependencies-version.version>3.10.0</dependencies-version.version>`
  in the `<properties>` section
- **Option B**: Hardcode the version in the plugin declaration and remove the property reference
- **Recommended**: Option A, consistent with how other plugin versions are managed

### 11.3 Remove stale root-level `Cargo.lock`

A root-level `Cargo.lock` (1087 lines) exists alongside `rust/Cargo.lock` (941 lines) with
different dependency versions (e.g., anyhow 1.0.98 vs 1.0.104). The Rust project lives in `rust/`
so the root-level lockfile is stale and misleading.

- **Action**: Delete the root-level `Cargo.lock`
- **`.gitignore`**: Ensure root `Cargo.lock` is ignored (but NOT `rust/Cargo.lock`)

### 11.4 Align Java version across pom.xml and CI workflows

| Location | Current | Target |
|---|---|---|
| `pom.xml` (`java.version`) | 17 | Decide on one version and align |
| `build.yml` (`JAVA_VERSION`) | 25 | Same as pom.xml |
| `test.yml` (`java-version`) | 25 | Same as pom.xml |
| `release.yml` (`JAVA_VERSION`) | 17 | Same as pom.xml |

- **Decision needed**: Either bump `pom.xml` to 25 (matching CI) or downgrade CI to 17 (matching pom.xml)
- **Recommendation**: Bump `pom.xml` to 25 to match CI, since CI is already running on 25

### 11.5 Enforce JaCoCo coverage ratio or remove the property

`pom.xml` declares `jacoco.coverage.ratio` at 95% but no JaCoCo `check` goal is configured.
Current Java coverage is 63.9% — well below the target.

- **Option A**: Add a `check` execution to `jacoco-maven-plugin` that enforces the 95% ratio
  (will fail the build until coverage improves)
- **Option B**: Lower the ratio to a realistic target (e.g., 80%) and add the `check` execution
- **Option C**: Remove the property if coverage enforcement is not desired
- **Recommendation**: Option B — add a `check` execution with a realistic ratio that can be
  incrementally raised as coverage improves

### Estimated effort: 0 code, build/config changes only

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

### 13.3 Add cause chaining to `VodozemacException`

`VodozemacException` only has a `String` constructor, preventing exception cause chaining.

- **Java**: Add a second constructor:
  ```java
  public VodozemacException(String message, Throwable cause) {
      super(message, cause);
  }
  ```
- **Impact**: Allows Java callers to wrap JNI exceptions with context while preserving the
  original cause

### 13.4 Set restrictive permissions on extracted native library

`NativeLibraryLoader.java:96-106` extracts the native library to a temp directory but does
not set file permissions, potentially allowing other users to read/modify the library.

- **Java**: After `Files.copy()`, set owner-only permissions:
  ```java
  Files.setPosixFilePermissions(tempLib,
      Set.of(PosixFilePermission.OWNER_READ, PosixFilePermission.OWNER_WRITE));
  ```
- **Fallback**: On Windows (no POSIX permissions), use `tempLib.toFile().setReadable(false, false)`
  and `setWritable(false, false)` to restrict access for other users

### 13.5 Add `--enable-native-access=ALL-UNNAMED` for Java 25+ compatibility

The JVM warns about restricted native access when calling `System.load()`. On Java 25+,
this may become a hard error.

- **pom.xml**: Add to `maven-surefire-plugin` argLine:
  `--enable-native-access=ALL-UNNAMED`
- **Documentation**: Document the requirement in README for consumers of the library

### 13.6 Fix Javadoc `@link` with wrong method signature

- `Account.java:278`: `@link Account#toDehydratedDevice(String)` should be
  `@link Account#toDehydratedDevice(byte[])` — the actual method takes `byte[]`, not `String`

### Estimated effort: ~0 JNI functions, ~1 Java class change (VodozemacException), security hardening

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
| 1. InboundGroupSession management  | High     | ~6            | 1 enum       | Existing `megolm/` |
| 2. SAS module                      | High     | ~12           | 2-4          | New `sas/`       |
| 3. ECIES module                    | High     | ~15           | 5            | New `ecies/`     |
| 4. PK Encryption                   | High     | ~10           | 3            | New `pk_encryption/` + Cargo.toml |
| 5. Structured messages             | Medium   | ~8 changes    | 2-3          | Existing modules |
| 6. Crypto key types                | Medium   | ~6            | 3            | New `types/`     |
| 7. Missing methods                 | Medium   | ~4            | 1            | Existing modules |
| 8. Error types                      | Low      | ~0 (mapping)  | ~10          | Existing modules |
| 9. Utilities                       | Low      | ~3            | 1            | New `utils/`     |
| 10. Code quality & duplication     | High     | ~0            | ~0           | Existing modules (Rust fixes) |
| 11. Build & configuration          | High     | N/A           | N/A          | N/A (config only) |
| 12. Documentation overhaul          | Medium   | N/A           | N/A          | N/A (docs only)  |
| 13. Security hardening             | Medium   | ~0            | ~1           | N/A              |
| 14. Refactoring & deduplication    | Low      | ~0            | ~3           | Existing `helpers.rs` |
| **Total**                          |          | **~64**       | **~38**      | **4 new + helpers** |

### Code review findings coverage

| CODE_REVIEW.md finding | Phase | Section |
|---|---|---|
| C1: Typo `createOutbpundSession` | Phase 7 | 7.4 |
| C2: Wrong `checkNotClosed()` error messages | Phase 10 | 10.1 |
| C3: `OlmSession` constructor is public | Phase 13 | 13.1 |
| C4-C7: Missing key validation in Megolm | Phase 10 | 10.2 |
| C8: Unused import in Rust tests | Phase 10 | 10.5 |
| C9: `VodozemacException` no cause chaining | Phase 13 | 13.3 |
| C10: `InboundCreationResult` constructor public | Phase 13 | 13.2 |
| C11: Javadoc `@link` wrong signature | Phase 13 | 13.6 |
| C12: `pom.xml` mainClass wrong | Phase 11 | 11.1 |
| C13: `helpers.rs:wrap()` uses `unwrap()` | Phase 10 | 10.3 |
| C14: Rust JNI `.unwrap()` on `convert_byte_array` | Phase 10 | 10.4 |
| C15: README imports `VodozemacAccount` | Phase 12 | 12.1 |
| D1: Undefined Maven property | Phase 11 | 11.2 |
| D2: Stale root `Cargo.lock` | Phase 11 | 11.3 |
| D3: Java version mismatch | Phase 11 | 11.4 |
| DOC1-DOC4: README inaccuracies | Phase 12 | 12.1 |
| DOC5-DOC6: `Sample.java` wrong log messages | Phase 12 | 12.3 |
| DOC7: Missing `CODE_OF_CONDUCT.md` | Phase 12 | 12.7 |
| DOC8-DOC10: Javadoc typos | Phase 12 | 12.2 |
| S1: Fake GPG key in SECURITY.md | Phase 12 | 12.4 |
| S2: Missing key validation (Megolm) | Phase 10 | 10.2 |
| S3: Native pointer exposed in `InboundCreationResult` | Phase 13 | 13.2 |
| S4: Inconsistent contact info | Phase 12 | 12.5 |
| S5: French error messages | Phase 12 | 12.6 |
| S6: Temp file permissions | Phase 13 | 13.4 |
| S7: `System.load` restricted warning | Phase 13 | 13.5 |
| 5.1-5.8: Duplicated code patterns | Phase 14 | 14.1-14.6 |
| JaCoCo coverage not enforced | Phase 11 | 11.5 |
