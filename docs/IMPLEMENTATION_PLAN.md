# Implementation Plan

This document tracks the remaining work identified in `docs/CODE_REVIEW.md` (2026-08-27).
All prior critical/high security items (S1-S14) and prior bugs (B1-B12) from previous
reviews are **complete**. The remaining work is organized into 10 phases, ordered by priority.

---

## Phase 1: Fix `release.yml` missing platforms (Critical)

Addresses CODE_REVIEW.md CI1, S23.

### Problem

The `release.yml` "Organize native libraries" step creates only 4 of 6 platform directories:
`linux-x86_64`, `linux-aarch64`, `darwin-aarch64`, `windows-x86_64`. It **omits**
`darwin-x86_64` and `windows-aarch64`, so the published JAR is missing two platforms'
native libraries. The `build.yml` `package-maven` job correctly handles all 6 platforms.

### 1.1 Add missing platform directories

- **File**: `.github/workflows/release.yml`
- **Change**: Add `darwin-x86_64` and `windows-aarch64` to the `mkdir` and `cp` commands
  in the "Organize native libraries" step, matching the 6-platform matrix in `build.yml`.
- **Verify**: Trigger a test release build and verify all 6 native libraries are present
  in the assembled JAR.

**Estimated effort**: 1 file, ~4 lines changed.

---

## Phase 2: Add `catch_panic` to `MegolmMessage::nativeFromBase64` and `nativeNew` functions (High)

Addresses CODE_REVIEW.md B14, A4.

### 2.1 Wrap `nativeFromBase64` in `catch_panic`

- **File**: `rust/src/megolm/message.rs`
- **Change**: Replace `env.with_env(|env| { ... })` with `catch_panic(env, |env| { ... })`,
  matching the pattern used by all other production JNI exports.

### 2.2 Wrap `nativeNew` functions in `catch_panic`

- **Files**: `rust/src/olm/account.rs`, `rust/src/sas/sas.rs`, `rust/src/ecies/ecies.rs`,
  `rust/src/megolm/outbound_group_session.rs`
- **Change**: Wrap `Account_nativeNew`, `Sas_nativeNew`, `Ecies_nativeNew`,
  `Ecies_nativeWithInfo`, `OutboundGroupSession_nativeNew` in `catch_panic`, matching
  the pattern already used by `InboundGroupSession_nativeNew` and `PkDecryption_nativeNew`.
- **Verify**: `cargo test` + `cargo clippy` + `mvn verify`.

**Estimated effort**: ~5 Rust files, ~10 lines changed.

---

## Phase 3: Fix `nativeAdvanceTo` unchecked integer cast (High)

Addresses CODE_REVIEW.md B13.

### 3.1 Replace `as u32` with `u32::try_from`

- **File**: `rust/src/megolm/inbound_group_session.rs:202`
- **Change**:
  ```rust
  // Before:
  let result = session.advance_to(index as u32);
  // After:
  let index = u32::try_from(index).map_err(|e| throw_generic_error(env, e))?;
  let result = session.advance_to(index);
  ```
- **Verify**: Existing `advanceTo` tests pass. Add a test for negative index that verifies
  a `ConversionException` is thrown.

**Estimated effort**: 1 Rust file, ~3 lines changed.

---

## Phase 4: Fix `equals`/`hashCode` and value class `final` (Medium)

Addresses CODE_REVIEW.md B15, B16, B17, B18, B19, B20.

### 4.1 Fix `equals`/`hashCode` to include all fields

- **Files**:
  - `src/main/java/.../olm/InboundCreationResult.java` -- include `session` in `equals`/`hashCode`
  - `src/main/java/.../ecies/OutboundCreationResult.java` -- include `ecies` in `equals`/`hashCode`
  - `src/main/java/.../ecies/InboundCreationResult.java` -- include `ecies` in `equals`/`hashCode`
- **Change**: Add the missing field to `equals` and `hashCode`. Note: `OlmSession` and
  `EstablishedEcies` already have `equals`/`hashCode`, so these can be compared.
- **Verify**: Update existing tests that assert `equals` behavior for these classes.

### 4.2 Make all value classes `final`

- **Files** (13 classes): `IdentityKeys`, `OneTimeKeyGenerationResult`, `DehydratedDeviceResult`,
  `SessionKeys`, `OlmMessage`, `MegolmMessage`, `DecryptedMessage`, `SasBytes`, `CheckCode`,
  `InboundCreationResult` (olm), `OutboundCreationResult`, `InboundCreationResult` (ecies), `PkMessage`
- **Change**: Add `final` to class declaration.
- **Verify**: `mvn verify` (Checkstyle `FinalClass` rule will enforce this for classes with
  private constructors, but `final` should be explicit regardless).

### 4.3 Make `DecryptedMessage` constructor package-private

- **File**: `src/main/java/.../megolm/DecryptedMessage.java:27`
- **Change**: `public DecryptedMessage(...)` -> `DecryptedMessage(...)`
- **Verify**: `mvn verify`. If any external code relies on the public constructor, a factory
  method should be added instead.

### 4.4 Add missing `toString()` to `SasBytes` and `CheckCode`

- **Files**: `src/main/java/.../sas/SasBytes.java`, `src/main/java/.../ecies/CheckCode.java`
- **Change**: Add `toString()` override matching the pattern used by other value classes.
- **Verify**: `mvn verify`. Update tests to verify `toString()` output.

### 4.5 Fix `Curve25519PublicKey.equals` variable name

- **File**: `src/main/java/.../types/Curve25519PublicKey.java:44`
- **Change**: `signature` -> `key` in pattern variable.

### 4.6 Fix `SasBytes.hashCode` double-wrap

- **File**: `src/main/java/.../sas/SasBytes.java:74`
- **Change**: `Objects.hash(Arrays.hashCode(rawBytes))` -> `Arrays.hashCode(rawBytes)`

**Estimated effort**: ~15 Java files, ~50 lines changed.

---

## Phase 5: Fix low-severity bugs and documentation (Low)

Addresses CODE_REVIEW.md B21, B22, B23, B24, B25, B26, B27, B28.

### 5.1 Escape `OlmMessage.toString()` JSON (B21)

- **File**: `src/main/java/.../olm/OlmMessage.java:80`
- **Change**: Either escape `body` for JSON safety, or use a simpler format (e.g.,
  `"OlmMessage{body=..., type=...}"`) matching the convention of other value classes.
  The simpler format is recommended -- other value classes don't use JSON in `toString()`.

### 5.2 Fix `DecryptedMessage` Javadoc `@see` (B22)

- **File**: `src/main/java/.../megolm/DecryptedMessage.java:15`
- **Change**: `@see InboundGroupSession#decrypt(String)` -> `@see InboundGroupSession#decrypt(MegolmMessage)`

### 5.3 Fix `SampleOlm.java` label bug (B23)

- **File**: `src/main/java/.../examples/SampleOlm.java:32`
- **Change**: "Ed25519" label -> "Curve25519" for the identity key line.

### 5.4 Fix `@throws` Javadoc mismatches (B24)

- **Files**: `SessionVersion.java`, `Ed25519PublicKey.java`, `Curve25519PublicKey.java`,
  `Ed25519Signature.java`
- **Change**: Update `@throws` to reference the actual exception type
  (`ConversionException`/`KeyException`) rather than `VodozemacException`. Add missing
  `@throws` to `fromBase64` methods.

### 5.5 Add null checks to `NativeLibraryLoader` (B25)

- **File**: `src/main/java/.../NativeLibraryLoader.java:53-54`
- **Change**: Check for null before calling `.toLowerCase()` on system properties,
  throw `UnsupportedOperationException` with a descriptive message if null.

### 5.6 Extract `KeyValidator` magic number to constant (B26)

- **File**: `src/main/java/.../KeyValidator.java`
- **Change**: `key.length != 32` -> `key.length != ENCRYPTION_KEY_LENGTH_BYTES`
  with `private static final int ENCRYPTION_KEY_LENGTH_BYTES = 32;`

### 5.7 Consider `Optional<String>` for `exportAt` (B27)

- **File**: `src/main/java/.../megolm/InboundGroupSession.java:146`
- **Change**: Change return type from `String` (nullable) to `Optional<String>`.
  Alternatively, document the nullable return in Javadoc.
- **Note**: This is an API change. If backward compatibility is a concern, document
  the nullable return instead.

### 5.8 Remove comment blocks in Rust code (B28)

- **File**: `rust/src/megolm/outbound_group_session.rs:14-16`
- **Change**: Remove the `// ===...` block comment header.
- **Verify**: `cargo fmt -- --check`.

**Estimated effort**: ~8 files, ~30 lines changed.

---

## Phase 6: Fix Rust JNI architecture issues (Medium)

Addresses CODE_REVIEW.md A5, A6, A7, A8, A9, A10.

### 6.1 Wrap `native_free` in `catch_panic` (A5)

- **File**: `rust/src/helpers.rs`
- **Change**: Wrap the `Box::from_raw` + drop in `catch_unwind` and convert panics to
  Java exceptions. Alternatively, document that `native_free` is panic-safe by
  construction (the only operation is `Box::from_raw` which is infallible).

### 6.2 Fix `check_ptr` ordering in `Ecies` (A6)

- **File**: `rust/src/ecies/ecies.rs:67-72,104-108`
- **Change**: Move `check_ptr(env, ptr)?` to before any parameter decoding, matching
  the pattern used by all other JNI functions.

### 6.3 Fix `OlmSession::encrypt` error mapping (A7)

- **File**: `rust/src/olm/session.rs`
- **Change**: Replace `throw_generic_error` with `throw_encryption_error` in the
  `nativeEncrypt` error path, so encryption failures produce `EncryptionException`
  instead of `ConversionException`.
- **Verify**: Update Java tests that expect `ConversionException` to expect
  `EncryptionException` instead.

### 6.4 Rename `throw_generic_error` or split into two functions (A8)

- **File**: `rust/src/errors.rs`
- **Change**: Either rename `throw_generic_error` to `throw_conversion_error` (matching
  the Java class it produces), or create a separate `throw_conversion_error` and update
  call sites based on the semantic context. Some call sites genuinely produce
  "conversion" errors (base64 decode, type conversion), while others produce "generic"
  errors (null pointer, invalid config version).
- **Verify**: `cargo test` + `cargo clippy`.

### 6.5 Fix `Ed25519PublicKey.nativeVerify` return type (A9)

- **Files**: `rust/src/types/mod.rs`, `src/main/java/.../types/Ed25519PublicKey.java`
- **Change**: Either:
  - (a) Change `nativeVerify` to return `void` and only throw on failure (throw-only
    contract), updating the Java `verify()` method to catch the exception and return
    `false` (current Java behavior already does this), or
  - (b) Change `nativeVerify` to return `false` on verification failure instead of
    throwing, and only throw for unexpected errors (malformed key, etc.).
  - Option (b) is recommended -- it lets the Java side avoid catching exceptions for
    the common "signature is invalid" case.

### 6.6 Fix `backup/encryption.rs` formatting (A10)

- **File**: `rust/src/backup/encryption.rs:48-54`
- **Change**: Run `cargo fmt` to fix misaligned indentation.
- **Verify**: `cargo fmt -- --check`.

**Estimated effort**: ~5 Rust files, ~30 lines changed.

---

## Phase 7: Rust JNI deduplication (Medium)

Addresses CODE_REVIEW.md D4, D5, D6, D7, D8, D9, D10.

### 7.1 Generic pickle/unpickle helpers (D4)

- **File**: `rust/src/helpers.rs`
- **Change**: Add generic functions:
  - `jni_pickle<T: Serialize>(env, &T) -> Result<jstring>`
  - `jni_encrypted_pickle<T: Serialize>(env, &T, key: [u8; 32]) -> Result<jstring>`
  - `jni_unpickle<T: DeserializeOwned>(env, json) -> Result<jlong>`
  - `jni_encrypted_unpickle<T: DeserializeOwned>(env, json, key: [u8; 32]) -> Result<jlong>`
  - `jni_legacy_unpickle<T>(env, json, pickle_key: Vec<u8>) -> Result<jlong>`
- Refactor `account.rs`, `session.rs`, `inbound_group_session.rs`,
  `outbound_group_session.rs`, `decryption.rs` to call these helpers.
- **Verify**: `cargo test` + Java pickle/unpickle tests + `cargo clippy`.

### 7.2 Reduce `errors.rs` signature-splitting duplication (D5)

- **File**: `rust/src/errors.rs`
- **Change**: Introduce a trait or helper that centralizes the `Signature(e)` vs other
  pattern matching used in `throw_megolm_decryption_error`,
  `throw_session_key_decode_error`, `throw_decode_error`.
- **Verify**: `cargo clippy` + `cargo test`.

### 7.3 Extract `to_java_megolm_message` helper (D6)

- **File**: `rust/src/megolm/mod.rs` (new helper)
- **Change**: Create `pub(crate) fn to_java_megolm_message(env, &message) -> JObject`
  and call it from both `outbound_group_session.rs::nativeEncrypt` and
  `message.rs::nativeFromBase64`.
- **Verify**: `cargo test`.

### 7.4 Propagate `to_java_*` helper pattern (D7)

- **Files**: `rust/src/megolm/mod.rs`, `rust/src/olm/mod.rs`, `rust/src/sas/mod.rs`,
  `rust/src/ecies/mod.rs`, `rust/src/backup/mod.rs`
- **Change**: Add module-level `to_java_*` helpers for `PkMessage`, `OlmMessage`,
  `SessionKeys`, `IdentityKeys`, `CheckCode`, `SasBytes`, `DecryptedMessage`.
  Each module's helpers construct their corresponding Java value objects.
- **Verify**: `cargo test` + `cargo clippy`.

### 7.5 Use `box_to_jlong` consistently (D8)

- **Files**: `rust/src/olm/account.rs`, `rust/src/sas/sas.rs`, `rust/src/ecies/ecies.rs`,
  `rust/src/megolm/inbound_group_session.rs`
- **Change**: Replace all `Box::new(value)` -> `&*box as *const T as jlong` ->
  `forget(box)` patterns with `box_to_jlong(value)`. This uses `Box::into_raw` (returns
  `*mut T`) instead of `&*box` (returns `*const T`), fixing the const/mut mismatch.
  Note: the `forget`-based pattern also guards against `new_object` failure (the `Box`
  is dropped on error). When switching to `box_to_jlong`, the pointer must be obtained
  **after** `new_object` succeeds, or a guard pattern must be used to free on failure.
- **Verify**: `cargo test` + `mvn verify`.

### 7.6 Centralize exception class string constants (D9)

- **File**: `rust/src/errors.rs`
- **Change**: Define `const &str` constants for all Java exception class paths
  (e.g., `const CONVERSION_EXCEPTION: &str = "io/github/fherbreteau/vodozemac/exception/ConversionException";`).
  Use these constants in the `throw_typed!` macro and hand-written throwers.
- **Verify**: `cargo clippy` + `cargo test`.

### 7.7 Document Java class path / enum name contracts (D10)

- **Files**: Rust modules that construct Java objects
- **Change**: Either (a) define `const &str` constants for all Java class paths used
  in `env.new_object` calls, or (b) add a documented contract that Rust string
  literals must match Java class/enum names exactly, with a test that verifies the
  most critical ones.
- **Verify**: `cargo test`.

**Estimated effort**: ~15 Rust files, ~200 lines removed, ~150 added.

---

## Phase 8: Add input validation to Java public API (Medium)

Addresses CODE_REVIEW.md C5.

### 8.1 Add null checks to public methods

- **Files**: All Java public API classes
- **Change**: Add `Objects.requireNonNull(param, "param")` to public methods that
  accept reference parameters. Key methods:
  - `Vodozemac.base64Encode(byte[])`, `base64Decode(String)`
  - `Account.sign(String)`, `createOutboundSession(...)`, `createInboundSession(...)`
  - `OlmSession.encrypt(byte[])`, `decrypt(OlmMessage)`
  - `Sas.diffieHellman(String)`
  - `Ecies.establishOutboundChannel(String, byte[])`, `establishInboundChannel(String)`
  - `InboundGroupSession(String)`, `decrypt(MegolmMessage)`
  - `PkEncryption.fromKey(String)`, `encrypt(byte[])`
  - `PkDecryption.fromKey(String)`, `fromKey(String, String)`
  - `Ed25519PublicKey.fromBase64(String)`, `Ed25519Signature.fromBase64(String)`
  - `Curve25519PublicKey.fromBase64(String)`
  - All `pickle`/`unpickle` methods
- **Verify**: `mvn verify`. Add tests for null input on key methods.

### 8.2 Fix parameter naming inconsistency (C6)

- **File**: `src/main/java/.../megolm/OutboundGroupSession.java`
- **Change**: `plainText` -> `plaintext` in `encrypt(byte[])` to match `EstablishedEcies`.

**Estimated effort**: ~20 Java files, ~80 lines changed.

---

## Phase 9: CI/CD hardening (Medium)

Addresses CODE_REVIEW.md CI2-CI11, S21, S22.

### 9.1 Fix `release.yml` missing platforms (CI1)

- Already covered in Phase 1.

### 9.2 Pin Maven wrapper checksum (CI2)

- **File**: `.mvn/wrapper/maven-wrapper.properties`
- **Change**: Add `distributionSha256Sum=<hash of Maven 3.9.16>` property.

### 9.3 Add `cargo-audit` to CI (CI3)

- **File**: `.github/workflows/test.yml`
- **Change**: Add a step to run `cargo audit` after Rust tests. Install `cargo-audit`
  via `taiki-e/install-action` or `cargo install cargo-audit`.
- **Verify**: CI run passes with no advisories.

### 9.4 Add `rust-toolchain.toml` (CI7)

- **File**: `rust-toolchain.toml` (new, at repo root)
- **Change**: Pin toolchain to `1.88.0` to match CI, making the dependabot
  `rust-toolchain` entry functional.
  ```toml
  [toolchain]
  channel = "1.88.0"
  components = ["clippy", "rustfmt", "llvm-tools-preview"]
  ```

### 9.5 SHA-pin first-party GitHub Actions (CI6)

- **Files**: `.github/workflows/build.yml`, `.github/workflows/test.yml`,
  `.github/workflows/release.yml`
- **Change**: Pin `actions/checkout`, `actions/setup-java`, `actions/upload-artifact`,
  `actions/download-artifact` to commit SHAs instead of major tags. Dependabot will
  keep them updated.

### 9.6 Add workflow concurrency cancellation (CI8)

- **Files**: `.github/workflows/test.yml`, `.github/workflows/build.yml`
- **Change**: Add:
  ```yaml
  concurrency:
    group: ${{ github.workflow }}-${{ github.ref }}
    cancel-in-progress: true
  ```

### 9.7 Add `[profile.release]` hardening to `Cargo.toml` (CI9)

- **File**: `rust/Cargo.toml`
- **Change**: Add:
  ```toml
  [profile.release]
  lto = true
  codegen-units = 1
  panic = "abort"
  strip = true
  ```

### 9.8 Add `CODEOWNERS` file (CI10)

- **File**: `.github/CODEOWNERS` (new)
- **Change**: Add owner for the repository to route dependabot PRs.

### 9.9 Fix `checkstyle.xml` `NewlineAtEndOfFile` (CI11)

- **File**: `checkstyle.xml`
- **Change**: Change `<property name="lineSeparator" value="lf_cr_crlf"/>` to
  `<property name="lineSeparator" value="lf"/>` to enforce LF-only line endings.
- **Verify**: `mvn checkstyle:check`.

### 9.10 Add Java dependency scanning (CI4) and CodeQL (CI5)

- **Files**: New `.github/workflows/security.yml` or add steps to `test.yml`
- **Change**: Add Trivy or OWASP dependency-check for Java deps. Add GitHub CodeQL
  workflow for Java SAST.

**Estimated effort**: ~10 files, ~80 lines changed.

---

## Phase 10: Fix `Ed25519PublicKey.verify()` exception handling (Medium)

Addresses CODE_REVIEW.md S20.

### Problem

`Ed25519PublicKey.verify()` catches all `VodozemacException` and returns `false`.
This masks unexpected errors (e.g., native crashes mapped to exceptions) that could
indicate a compromised or malfunctioning verification path.

### 10.1 Narrow the exception catch

- **File**: `src/main/java/.../types/Ed25519PublicKey.java:45-49`
- **Change**: Only catch `SignatureException` (for invalid signature format) and
  `KeyException` (for invalid key format). Let other exceptions (e.g.,
  `VodozemacException` for native errors) propagate.
- **Alternative**: After Phase 6.5 (fixing `nativeVerify` to return `false` instead
  of throwing for invalid signatures), the Java `verify()` method may not need to
  catch any exception for the "signature invalid" case -- it would only need to
  catch exceptions for unexpected errors.
- **Verify**: `mvn verify`. Add tests for `verify()` with tampered signature,
  malformed signature, and malformed public key.

**Estimated effort**: 1 Java file, ~5 lines changed.

---

## Phase 11: Add missing static loader blocks and test coverage (Low)

Addresses CODE_REVIEW.md C4, T9-T15.

### 11.1 Add `static { NativeLibraryLoader.loadLibrary(); }` to 3 classes (C4)

- **Files**: `OlmSession.java`, `EstablishedSas.java`, `EstablishedEcies.java`
- **Change**: Add static initializer block matching all other `NativeHandle` subclasses.

### 11.2 Add test coverage gaps (T9-T15)

| Test | Target | Description |
|---|---|---|
| `TypesTest` | `Ed25519PublicKey.verify(String, Ed25519Signature)` | Test the String overload |
| `AccountTest` | `pickleLegacy()` round-trip | Write and read back a legacy pickle |
| `SasTest` | `SasBytes` equality | Test equality between two instances with same data |
| `EciesTest` | Fix fragile null-pointer test | Replace `0L` nativePtr with a valid ECIES or remove the test |
| `InboundGroupSessionTest` | Fix `"b65"` typo | Change to `"b64"` in test data |
| Rust tests | `OlmSession` v2 config | Add a v2 session encrypt/decrypt test |
| CI | Fat JAR integration test | Add a smoke test that loads the assembled multi-platform JAR |

**Estimated effort**: ~5 Java files, ~3 Rust files, ~100 lines added.

---

## Phase 12: `pickleLegacy()` for session types (Low -- blocked)

Addresses CODE_REVIEW.md C3.

### 12.1 Add `pickleLegacy()` to `OlmSession`, `OutboundGroupSession`, `InboundGroupSession`

- **Blocked**: vodozemac 0.10.0 does not expose `to_libolm_pickle` for `Session`,
  `InboundGroupSession`, or `GroupSession` (only `Account` and `PkDecryption` have it).
- **Action**: Monitor vodozemac releases for `to_libolm_pickle` support on these types.
  When available, add `nativePickleLegacy` Rust functions and `pickleLegacy(byte[])`
  Java methods.
- **Workaround**: Callers can use `pickle()` (vodozemac format) instead of
  `pickleLegacy()` (libolm format). `unpickleLegacy()` remains for reading existing
  libolm pickles.

**Estimated effort**: 0 lines (blocked) until vodozemac adds the API; ~3 JNI functions
+ ~3 Java methods when unblocked.

---

## Summary by phase

| Phase | Priority | Scope | Estimated effort |
|---|---|---|---|
| 1. `release.yml` missing platforms | Critical | 1 workflow file | ~4 lines |
| 2. `catch_panic` on `nativeFromBase64` + `nativeNew` | High | ~5 Rust files | ~10 lines |
| 3. `nativeAdvanceTo` integer validation | High | 1 Rust file | ~3 lines |
| 4. `equals`/`hashCode` + `final` + `toString` | Medium | ~15 Java files | ~50 lines |
| 5. Low-severity bugs & docs | Low | ~8 files | ~30 lines |
| 6. Rust JNI architecture fixes | Medium | ~5 Rust files | ~30 lines |
| 7. Rust JNI deduplication | Medium | ~15 Rust files | -200 / +150 lines |
| 8. Java input validation | Medium | ~20 Java files | ~80 lines |
| 9. CI/CD hardening | Medium | ~10 files | ~80 lines |
| 10. `Ed25519PublicKey.verify()` exception handling | Medium | 1 Java file | ~5 lines |
| 11. Static loaders + test coverage | Low | ~8 files | ~100 lines |
| 12. `pickleLegacy` (blocked) | Low (blocked) | 0 (blocked) | 0 until vodozemac adds API |

### Coverage of CODE_REVIEW.md findings

| CODE_REVIEW.md item | Phase | Status |
|---|---|---|
| CI1: `release.yml` missing platforms | 1 | Open |
| B14: `nativeFromBase64` missing `catch_panic` | 2 | Open |
| A4: `nativeNew` skips `catch_panic` | 2 | Open |
| B13: `nativeAdvanceTo` unchecked cast | 3 | Open |
| B15: `equals`/`hashCode` ignore key fields | 4 | Open |
| B16: Value classes not `final` | 4 | Open |
| B17: `DecryptedMessage` public constructor | 4 | Open |
| B18: Missing `toString()` | 4 | Open |
| B19: `Curve25519PublicKey` variable name | 4 | Open |
| B20: `SasBytes.hashCode` double-wrap | 4 | Open |
| B21: `OlmMessage.toString()` JSON not escaped | 5 | Open |
| B22: `DecryptedMessage` `@see` wrong | 5 | Open |
| B23: `SampleOlm.java` label bug | 5 | Open |
| B24: `@throws` Javadoc mismatches | 5 | Open |
| B25: `NativeLibraryLoader` NPE risk | 5 | Open |
| B26: `KeyValidator` magic number | 5 | Open |
| B27: `exportAt` nullable return | 5 | Open |
| B28: Rust comment blocks | 5 | Open |
| A5: `native_free` not panic-guarded | 6 | Open |
| A6: `check_ptr` ordering in `Ecies` | 6 | Open |
| A7: `OlmSession::encrypt` error mapping | 6 | Open |
| A8: `throw_generic_error` name mismatch | 6 | Open |
| A9: `nativeVerify` always returns `true` | 6 | Open |
| A10: `backup/encryption.rs` formatting | 6 | Open |
| D4: Pickle/unpickle duplication | 7 | Open |
| D5: `errors.rs` signature-splitting | 7 | Open |
| D6: `MegolmMessage` construction duplicated | 7 | Open |
| D7: `to_java_*` helpers not propagated | 7 | Open |
| D8: Inconsistent `box_to_jlong` vs `forget` | 7 | Open |
| D9: Hardcoded exception class strings | 7 | Open |
| D10: Hardcoded Java class paths | 7 | Open |
| C5: Missing input validation | 8 | Open |
| C6: Parameter naming inconsistency | 8 | Open |
| CI2: Maven wrapper not checksum-pinned | 9 | Open |
| CI3: No `cargo-audit` | 9 | Open |
| CI4: No Java dependency scanning | 9 | Open |
| CI5: No CodeQL | 9 | Open |
| CI6: First-party actions not SHA-pinned | 9 | Open |
| CI7: Dead `rust-toolchain` dependabot entry | 9 | Open |
| CI8: No concurrency cancellation | 9 | Open |
| CI9: No `[profile.release]` hardening | 9 | Open |
| CI10: No `CODEOWNERS` | 9 | Open |
| CI11: `NewlineAtEndOfFile` allows CRLF | 9 | Open |
| S20: `Ed25519PublicKey.verify()` swallows exceptions | 10 | Open |
| C4: Missing `static{}` loader in 3 classes | 11 | Open |
| T9-T15: Test coverage gaps | 11 | Open |
| C3: `pickleLegacy()` for session types | 12 | Blocked |

> All prior critical/security findings (S1-S14) and prior bugs (B1-B12) from previous
> reviews are **complete** and not listed here.
