# Implementation Plan

This document tracks the remaining work identified in `docs/CODE_REVIEW.md` (2026-08-20).
All critical/high security items from the prior plan are **complete**; the remaining work is
correctness/consistency cleanup, deduplication, and two feature gaps. Phases are ordered by
priority.

---

## Phase 1: Rust JNI correctness & consistency (Medium)

Addresses CODE_REVIEW.md §4 (Rust correctness), §5.4.

### 1.1 Descriptive session-config error messages
- **File**: `rust/src/helpers.rs:12-30`
- **Change**: in `olm_session_config_from_version` and `megolm_session_config_from_version`, replace
  `_ => Err(jni::errors::Error::JavaException)` with
  `_ => Err(throw_generic_error(env, format!("Invalid session config version: {}", version)))`.
- **Tests**: add Rust unit tests for version 0 / 3 / negative asserting a thrown error with message.

### 1.2 Align `EstablishedEcies::nativeEncrypt` return type
- **File**: `rust/src/ecies/established_ecies.rs:55-56`
- **Change**: type the closure as `Result<jstring, jni::errors::Error>` to match the `-> jstring` signature
  (mirror `nativePublicKey` at lines 10-15). `result.into_raw()` already yields a `jstring`.
- **Verify**: `cargo build` + `cargo test` + Java `EciesTest` still pass.

### 1.3 Standardise `convert_byte_array` call style
- **Files**: all Rust modules (notably `olm/account.rs`, `olm/session.rs`)
- **Change**: pick by-value (auto-ref) call style `env.convert_byte_array(x)` (idiomatic) and apply
  consistently; remove explicit `&x` forms. Run `cargo fmt` + `cargo clippy` after.
- **Verify**: `cargo fmt --check`, `cargo clippy`, `cargo test`.

### 1.4 Remove dead code in `backup/encryption.rs`
- **File**: `rust/src/backup/encryption.rs:19`
- **Change**: delete `let _ = PkEncryption::from_key(public_key);`. (Key validity is not required here;
  the returned Java object is built from `public_key.to_base64()` independently.)
- **Verify**: `cargo test` + Java `PkEncryptionTest` pass.

### 1.5 Cosmetic: normalise double space
- **File**: `rust/src/backup/encryption.rs:50` — collapse `let mac =  env…` to single space.
- **Verify**: `cargo fmt --check` remains green.

**Estimated effort**: ~5 files, ~10 lines changed.

---

## Phase 2: Rust JNI deduplication (Medium)

Addresses CODE_REVIEW.md §5.1, §5.2, §5.3.

### 2.1 Generic `native_free` helper
- **File**: add to `rust/src/helpers.rs`
  ```rust
  pub(crate) fn native_free<T>(env: &mut Env, ptr: jlong) {
      if let Err(_) = check_ptr(env, ptr) { return; }
      unsafe { let _ = Box::from_raw(ptr as *mut T); }
  }
  ```
- Replace each of the 9 `nativeFree` bodies with `native_free::<T>(env, ptr)`.
- **Verify**: `cargo test` + full `mvn verify`.

### 2.2 Generic pickle/unpickle helpers
- **File**: add to `rust/src/helpers.rs` generic `jni_pickle<T: Serialize>`,
  `jni_encrypted_pickle<T: Serialize>`, `jni_unpickle<T: DeserializeOwned>`,
  `jni_encrypted_unpickle<T: DeserializeOwned>`.
- Refactor the 5-variant families in `account.rs`, `session.rs`, `inbound_group_session.rs`,
  `outbound_group_session.rs` to call the helpers (keep the `Legacy` variants where vodozemac supports them).
- **Verify**: `cargo test` + Java pickle/unpickle tests.

### 2.3 Reduce `errors.rs` wrapper boilerplate
- **File**: `rust/src/errors.rs`
- **Change**: keep the shared `throw` core; introduce a small macro or a `(&JNIStr, fn)` table so each
  typed entry is one line. Preserve the public `throw_*` names (call sites unchanged).
- **Verify**: `cargo clippy` + `cargo test`.

**Estimated effort**: ~3 Rust files, ~150 lines removed, ~60 added.

---

## Phase 3: Java API consistency (Low–Medium)

Addresses CODE_REVIEW.md §4 (Java consistency).

### 3.1 `OlmMessage` `equals`/`hashCode`
- **File**: `olm/OlmMessage.java`
- Add `equals`/`hashCode` over `body` + `type` (matching the existing `toString`).
- **Tests**: extend `OlmSessionTest.testOlmMessageEqualsHashCodeToString` (new) with all-field cases.

### 3.2 `pickleLegacy()` write methods
- **Files**: `OlmSession`, `OutboundGroupSession`, `InboundGroupSession`, `PkDecryption` (+ matching
  Rust `nativePickleLegacy` where vodozemac exposes `to_libolm_pickle`).
- **Tests**: round-trip `pickleLegacy` ↔ `unpickleLegacy` per class.

### 3.3 Rename accessor outliers to fluent
- `Vodozemac.getVersion()` → `version()`
- `Account.getUnpublishedOneTimeKeys()` → `unpublishedOneTimeKeys()`
- `Account.getUnpublishedFallbackKey()` → `unpublishedFallbackKey()`
- Update README and tests. **Breaking change** — coordinate with versioning/release notes.

### 3.4 Standardise `Sample*.main()` signatures
- Change `SampleEcies`, `SampleMegolm`, `SampleSas` `main()` → `main(String[] args)`.

**Estimated effort**: ~10 Java files, ~60 lines changed.

---

## Phase 4: `MegolmMessage` structured type (Medium, feature)

Addresses CODE_REVIEW.md §4 feature gap.

### 4.1 New Java class `megolm/MegolmMessage`
Fields: `ciphertext` (String/base64), `messageIndex` (int), `mac` (String/base64),
`signature` (String/base64). Methods: accessors, `equals`/`hashCode`/`toString`,
`toBase64()` / `static fromBase64(String)` for wire format. `@author` tag, `final`.

### 4.2 Update `OutboundGroupSession.encrypt(byte[])` to return `MegolmMessage`
### 4.3 Update `InboundGroupSession.decrypt(MegolmMessage)` (or overload) to accept `MegolmMessage`
### 4.4 Rust JNI: `nativeEncrypt` returns a `MegolmMessage` object; `nativeDecrypt` accepts one
### 4.5 Tests: round-trip, field access, equals/hashCode, base64 wire round-trip.

**Estimated effort**: 1 new Java class, ~8 JNI function changes, new tests.

---

## Phase 5: Cryptographic key types (Medium, feature)

Addresses CODE_REVIEW.md §4 feature gap.

### 5.1 New Java classes
| Class | vodozemac type | Methods |
|---|---|---|
| `Ed25519PublicKey` | `vodozemac::Ed25519PublicKey` | `fromBase64`, `toBase64`, `verify(message, signature)` |
| `Ed25519Signature` | `vodozemac::Ed25519Signature` | `fromBase64`, `toBase64` |
| `Curve25519PublicKey` | `vodozemac::Curve25519PublicKey` | `fromBase64`, `toBase64` |

### 5.2 New Rust JNI module `rust/src/types/` with thin wrappers.
### 5.3 Tests: base64 round-trips, signature verify (valid + tampered).

**Estimated effort**: 3 new Java classes, 1 new Rust module, ~6 JNI functions, tests.

---

## Phase 6: Build & docs hygiene (Low)

Addresses CODE_REVIEW.md §4 (build/docs), §7.6.

### 6.1 Regenerate `coverage-report.md`
- Run `.github/scripts/coverage-report.py` (or `mvn verify` + `cargo llvm-cov`) and commit refreshed
  numbers (Java 100%, Rust unit-test-only 19%).

### 6.2 `sonar-maven-plugin`
- Either bind a Sonar execution (when `sonar.profile` active) or remove the declaration from
  `pluginManagement`.

### 6.3 GitHub Actions version comments
- Add a `# v7` / `# v2` comment next to each SHA-pinned action for auditability (keep SHA-pinning).

**Estimated effort**: ~3 files, ~10 lines changed.

---

## Summary by phase

| Phase | Priority | Scope | Estimated effort |
|---|---|---|---|
| 1. Rust JNI correctness & consistency | Medium | ~5 Rust files | ~10 lines |
| 2. Rust JNI deduplication | Medium | ~3 Rust files | -150 / +60 lines |
| 3. Java API consistency | Low–Medium | ~10 Java files | ~60 lines |
| 4. MegolmMessage structured type | Medium (feature) | 1 class + JNI | ~8 JNI + tests |
| 5. Cryptographic key types | Medium (feature) | 3 classes + 1 Rust module | ~6 JNI + tests |
| 6. Build & docs hygiene | Low | 3 files | ~10 lines |

### Coverage of CODE_REVIEW.md findings

| CODE_REVIEW.md item | Phase |
|---|---|
| Session-config error messages | 1.1 |
| `EstablishedEcies::nativeEncrypt` return type | 1.2 |
| `convert_byte_array` call style | 1.3 |
| Dead code in `backup/encryption.rs` | 1.4 |
| `nativeFree` duplication | 2.1 |
| Pickle/unpickle duplication | 2.2 |
| `errors.rs` wrappers | 2.3 |
| `OlmMessage` equals/hashCode | 3.1 |
| `pickleLegacy` write methods | 3.2 |
| Accessor naming outliers | 3.3 |
| `Sample*.main()` signatures | 3.4 |
| `MegolmMessage` structured type | 4 |
| Cryptographic key types | 5 |
| Stale `coverage-report.md` | 6.1 |
| `sonar-maven-plugin` unconfigured | 6.2 |
| GitHub Actions version comments | 6.3 |

> All prior critical/security findings (S1–S11) and the original Phases 1, 2, 3, 5, 8, 9
> are **complete** and not listed here.
