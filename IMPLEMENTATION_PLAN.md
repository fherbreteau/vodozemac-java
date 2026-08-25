# Implementation Plan

This document tracks the remaining work identified in `docs/CODE_REVIEW.md` (2026-08-26).
All critical/high security items (S1–S11) and Phases 1, 3 (except 3.2), 4, and 6 from the prior
plan are **complete**. The remaining work is Rust deduplication, one blocked API-symmetry task,
and one feature gap. Phases are ordered by priority.

---

## Phase 1: Rust JNI deduplication (Medium)

Addresses CODE_REVIEW.md §5.1, §5.2, §5.3.

### 1.1 Generic `native_free` helper
- **File**: add to `rust/src/helpers.rs`
  ```rust
  pub(crate) fn native_free<T>(env: &mut Env, ptr: jlong) {
      if check_ptr(env, ptr).is_err() {
          return;
      }
      unsafe { let _ = Box::from_raw(ptr as *mut T); }
  }
  ```
- Replace each of the 9 `nativeFree` bodies with `native_free::<T>(env, ptr)`.
- **Verify**: `cargo test` + full `mvn verify`.

### 1.2 Generic pickle/unpickle helpers
- **File**: add to `rust/src/helpers.rs` generic `json_to_jstring<T: Serialize>`,
  `string_to_jstring`, `from_json<T: DeserializeOwned>`, `box_to_jlong<T>`.
- Refactor the 5-variant families in `account.rs`, `session.rs`, `inbound_group_session.rs`,
  `outbound_group_session.rs` to call the helpers (keep the `Legacy` variants where vodozemac supports them).
- **Verify**: `cargo test` + Java pickle/unpickle tests.

### 1.3 Reduce `errors.rs` wrapper boilerplate
- **File**: `rust/src/errors.rs`
- **Change**: keep the shared `throw` core; introduce a `throw_typed!` macro so each
  generic `Display`-bound entry is a single line. Preserve the public `throw_*` names
  (call sites unchanged). Keep special-signature functions (`throw_sas_error`,
  `throw_invalid_count_error`) and dispatching wrappers as-is.
- **Verify**: `cargo clippy` + `cargo test`.

**Estimated effort**: ~3 Rust files, ~150 lines removed, ~60 added.

---

## Phase 2: `pickleLegacy()` for session types (Low — blocked)

Addresses CODE_REVIEW.md §4 (Java API consistency).

### 2.1 Add `pickleLegacy()` to `OlmSession`, `OutboundGroupSession`, `InboundGroupSession`

- **Blocked**: vodozemac 0.10.0 does not expose `to_libolm_pickle` for `Session`,
  `InboundGroupSession`, or `GroupSession` (only `Account` and `PkDecryption` have it).
- **Action**: monitor vodozemac releases for `to_libolm_pickle` support on these types.
  When available, add `nativePickleLegacy` Rust functions and `pickleLegacy(byte[])` Java methods.
- **Workaround**: callers can use `pickle()` (vodozemac format) instead of `pickleLegacy()` (libolm format).
  `unpickleLegacy()` remains for reading existing libolm pickles.

**Estimated effort**: 0 lines (blocked) until vodozemac adds the API; ~3 JNI functions + ~3 Java methods when unblocked.

---

## Phase 3: Cryptographic key types (Medium, feature)

Addresses CODE_REVIEW.md §4 feature gap.

### 3.1 New Java classes

| Class | vodozemac type | Methods |
|---|---|---|
| `Ed25519PublicKey` | `vodozemac::Ed25519PublicKey` | `fromBase64(String)`, `toBase64()`, `verify(String message, String signature) -> boolean` |
| `Ed25519Signature` | `vodozemac::Ed25519Signature` | `fromBase64(String)`, `toBase64()` |
| `Curve25519PublicKey` | `vodozemac::Curve25519PublicKey` | `fromBase64(String)`, `toBase64()` |

### 3.2 New Rust JNI module `rust/src/types/` with thin wrappers

- `Ed25519PublicKey`: `nativeFromBase64`, `nativeToBase64`, `nativeVerify`
- `Ed25519Signature`: `nativeFromBase64`, `nativeToBase64`
- `Curve25519PublicKey`: `nativeFromBase64`, `nativeToBase64`

### 3.3 Design decisions

- `Ed25519Keypair` and secret keys are not needed in Java (clients don't create keypairs
  directly — `Account` does).
- `Ed25519PublicKey.verify()` is the main use case — verify signatures from other devices.
- Could be simpler to add `Account.verify(String message, String signature, String theirEd25519Key)`
  without exposing the key type. But exposing the type is more flexible.

### 3.4 Tests: base64 round-trips, signature verify (valid + tampered).

**Estimated effort**: 3 new Java classes, 1 new Rust module, ~6 JNI functions, tests.

---

## Summary by phase

| Phase | Priority | Scope | Estimated effort |
|---|---|---|---|
| 1. Rust JNI deduplication | Medium | ~3 Rust files | -150 / +60 lines |
| 2. pickleLegacy for session types | Low (blocked) | 0 (blocked) | 0 until vodozemac adds API |
| 3. Cryptographic key types | Medium (feature) | 3 classes + 1 Rust module | ~6 JNI + tests |

### Coverage of CODE_REVIEW.md findings

| CODE_REVIEW.md item | Phase | Status |
|---|---|---|
| `nativeFree` duplication (§5.1) | 1.1 | Open |
| Pickle/unpickle duplication (§5.2) | 1.2 | Open |
| `errors.rs` wrappers (§5.3) | 1.3 | Open |
| `pickleLegacy()` for session types (§4) | 2.1 | Blocked |
| Cryptographic key types (§4) | 3.1 | Open |

> All prior critical/security findings (S1–S11) and Phases 1 (correctness), 3 (API consistency),
> 4 (MegolmMessage), and 6 (build hygiene) from the prior implementation plan are **complete**
> and not listed here.
