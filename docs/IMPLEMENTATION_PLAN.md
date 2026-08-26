# Implementation Plan

This document tracks the remaining work identified in `docs/CODE_REVIEW.md` (2026-08-26).
All prior critical/high security items (S1–S11) from previous reviews are **complete**.
The remaining work is organized into 7 phases, ordered by priority.

---

## Phase 1: Fix use-after-free in consuming operations (Critical)

Addresses CODE_REVIEW.md B1, S12.

### Problem

When `Sas.diffieHellman()`, `Ecies.establishOutboundChannel()`, or `Ecies.establishInboundChannel()`
is called and the Rust operation fails, the Rust side has already consumed the `Box` via
`Box::from_raw` (dropping it on error), but the Java side only zeroes `nativePtr` on success.
On the subsequent `close()` call, `nativeFree` calls `Box::from_raw` on a dangling pointer.

### 1.1 Fix Java consuming methods to zero `nativePtr` unconditionally

- **Files**:
  - `src/main/java/.../sas/Sas.java` — `diffieHellman()`
  - `src/main/java/.../ecies/Ecies.java` — `establishOutboundChannel()`, `establishInboundChannel()`
- **Change**: Use try-finally to zero `nativePtr` before the native call returns, regardless of
  success or failure:

  ```java
  public EstablishedSas diffieHellman(String theirPublicKey) {
      checkNotClosed();
      try {
          EstablishedSas result = nativeDiffieHellman(nativePtr, theirPublicKey);
          return result;
      } finally {
          nativePtr = 0;
      }
  }
  ```

  Apply the same pattern to `Ecies.establishOutboundChannel()` and `Ecies.establishInboundChannel()`.

### 1.2 Fix Rust to not consume Box on error

- **Files**: `rust/src/sas/sas.rs`, `rust/src/ecies/ecies.rs`
- **Change**: Use `&mut *ptr` instead of `Box::from_raw` for the consuming operations, and only
  take ownership (via `Box::from_raw` + `mem::forget` or `std::mem::replace`) after the operation
  succeeds. This ensures that on error, the original pointer is still valid for Java's `close()`.

  Alternative approach (simpler): keep the `Box::from_raw` pattern but rely on the Java side
  zeroing `nativePtr` in a `finally` block (1.1). This works because once `nativePtr = 0`, Java's
  `close()` is a no-op. The Rust `nativeFree` with `ptr = 0` is handled by `check_ptr` returning
  early. **This is the recommended approach** — it requires only Java-side changes.

### 1.3 Tests

- Add `SasTest`: `diffieHellman()` with invalid key → assert `KeyException` thrown, assert `Sas`
  is closed (subsequent `publicKey()` throws `IllegalStateException`).
- Add `EciesTest`: `establishOutboundChannel()` with invalid key → assert `KeyException`, assert
  `Ecies` is closed. `establishInboundChannel()` with malformed message → assert `EciesException`,
  assert `Ecies` is closed.
- Verify `close()` after failed consuming op does not crash (no double-free).

**Estimated effort**: ~2 Java files, ~3 test methods, ~20 lines changed.

---

## Phase 2: Fix resource leak on Java object construction failure (Critical)

Addresses CODE_REVIEW.md B2, S13.

### Problem

In `account.rs:126`, `account.rs:168`, `sas.rs:57`, `ecies.rs:74`, `ecies.rs:107`, the pattern is:

```rust
let session_ptr = Box::into_raw(Box::new(session)) as jlong;  // allocate
let result = env.new_object(..., &[JValue::Long(session_ptr)])?;  // may fail → leak
```

If `env.new_object()` fails, the `?` returns early and `session_ptr` is never freed.

### 2.1 Use a guard pattern

- **Files**: `rust/src/olm/account.rs`, `rust/src/sas/sas.rs`, `rust/src/ecies/ecies.rs`
- **Change**: Keep the value in a `Box`, pass a borrowed pointer to `new_object`, and only call
  `Box::into_raw` after `new_object` succeeds:

  ```rust
  let session_box = Box::new(session);
  let session_ptr = &*session_box as *const Session as jlong;
  let result = env.new_object(
      jni_str!("..."),
      jni_sig!((nativePtr: long) -> void),
      &[JValue::Long(session_ptr)],
  )?;
  std::mem::forget(session_box);  // ownership transferred to Java
  Ok(result.into_raw())
  ```

  If `new_object` fails, `session_box` is dropped normally — no leak.

### 2.2 Tests

- Difficult to test directly (requires simulating `new_object` failure). Verify via code review
  and ensure existing tests pass.
- Consider adding a Rust unit test that mocks a failed `new_object` and verifies no memory leak
  (may require test infrastructure changes).

**Estimated effort**: ~3 Rust files, ~5 sites, ~30 lines changed.

---

## Phase 3: Fix `PkEncryption.encrypt()` missing `checkNotClosed()` (High)

Addresses CODE_REVIEW.md B3.

### 3.1 Add `checkNotClosed()` call

- **File**: `src/main/java/.../backup/PkEncryption.java:72-73`
- **Change**:

  ```java
  public PkMessage encrypt(byte[] plaintext) {
      checkNotClosed();
      return nativeEncrypt(nativePtr, plaintext);
  }
  ```

### 3.2 Test

- Add `PkEncryptionTest`: `encrypt()` after `close()` → assert `IllegalStateException`.

**Estimated effort**: 1 Java file, 1 test method, ~5 lines changed.

---

## Phase 4: Fix medium-severity bugs (Medium)

Addresses CODE_REVIEW.md B4, B5, B6, B7, B8.

### 4.1 Add `static { NativeLibraryLoader.loadLibrary(); }` to `InboundGroupSession` (B4)

- **File**: `src/main/java/.../megolm/InboundGroupSession.java`
- **Change**: Add static initializer block after class declaration, matching all other
  `NativeHandle` subclasses.
- **Test**: Verify `InboundGroupSession.importSession()` works without any other class being
  loaded first (difficult to test in practice with JUnit, since other classes are loaded by the
  test framework).

### 4.2 Fix `nativeImport` signature (B5)

- **File**: `src/main/java/.../megolm/InboundGroupSession.java:345`
- **Change**: `private static native long nativeImport(String sessionKey, long version);` →
  `private static native long nativeImport(String sessionKey, int version);`
- **Verify**: Existing `importSession()` tests pass (they already call `version.value()` which
  returns `int`).

### 4.3 Fix `pom.xml` mainClass (B6)

- **File**: `pom.xml:476`
- **Change**: Remove the `<mainClass>` configuration entirely (this is a library JAR, not an
  executable JAR), or set it to a valid example class.
- **Verify**: `mvn package` succeeds and JAR manifest is correct.

### 4.4 Make `NativeLibraryLoader.loaded` volatile (B7)

- **File**: `src/main/java/.../NativeLibraryLoader.java:31`
- **Change**: `private static boolean loaded = false;` → `private static volatile boolean loaded = false;`

### 4.5 Make `olm/InboundCreationResult` implement `AutoCloseable` (B8)

- **File**: `src/main/java/.../olm/InboundCreationResult.java`
- **Change**: Implement `AutoCloseable`, delegate `close()` to `session.close()`:

  ```java
  public class InboundCreationResult implements AutoCloseable {
      ...
      @Override
      public void close() {
          session.close();
      }
  }
  ```

- **Test**: Update `AccountTest` to use try-with-resources for `InboundCreationResult`.
- **Note**: Also consider adding `session` to `equals()`/`hashCode()` for consistency (C2).

**Estimated effort**: ~4 Java files, ~1 pom.xml, ~30 lines changed.

---

## Phase 5: Fix low-severity bugs and documentation (Low)

Addresses CODE_REVIEW.md B9, B10, B11, B12, A3.

### 5.1 Fix Javadoc typo in `OlmMessage.java:10` (B9)

- `{@link io.github.fherbreau.vodozemac.account.Account...}` → `fherbreteau`

### 5.2 Fix variable typo in `AccountTest.java:282` (B10)

- `dehydratexDevice` → `dehydratedDevice` (6 occurrences in the same method)

### 5.3 Fix assertion message typo in `NativeHandleTest.java:133` (B11)

- `"Native Libray is Loaded"` → `"Native Library is Loaded"`

### 5.4 Fix `VodozemacException` Javadoc (B12)

- Add `ConversionException`, `EciesException`, `EncryptionException` to the list of subclasses.

### 5.5 Improve `wrap()` error message (A3)

- **File**: `rust/src/helpers.rs:9-11`
- **Change**: Return a descriptive error instead of `JavaException`:

  ```rust
  pub(crate) fn wrap<T>(env: &mut Env, v: Vec<T>) -> Result<[T; 32], jni::errors::Error> {
      v.try_into().map_err(|v| {
          throw_generic_error(env, format!("Expected 32-byte key, got {} bytes", v.len()))
      })
  }
  ```

  Update all call sites that currently use `wrap(env.convert_byte_array(key)?)` to pass `env`.

**Estimated effort**: ~4 Java files, ~1 Rust file, ~20 lines changed.

---

## Phase 6: Rust JNI architecture improvements (Medium)

Addresses CODE_REVIEW.md A1, A2, D1, D2, D3.

### 6.1 Add panic boundary (`catch_unwind`) (A1)

- **Files**: All JNI entry points, or a shared wrapper in `helpers.rs`
- **Change**: Wrap each JNI function body with `std::panic::catch_unwind` and convert panics to
  Java exceptions. Alternatively, create a macro that wraps the `env.with_env()` + `resolve()`
  pattern with `catch_unwind`:

  ```rust
  let outcome = env.with_env(|env| -> Result<T, jni::errors::Error> {
      std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
          // body
      })).map_err(|e| {
          throw_generic_error(env, format!("Rust panic: {e:?}"))
      })?
  });
  ```

- **Verify**: `cargo test` + `mvn verify`. Add a test that triggers a panic and verifies a Java
  exception is thrown instead of UB.

### 6.2 Validate integer indices (A2)

- **File**: `rust/src/megolm/inbound_group_session.rs:147,187`
- **Change**: Replace `index as u32` with `u32::try_from(index).map_err(|e| ...)?`

### 6.3 Generic pickle/unpickle helpers (D1)

- **File**: `rust/src/helpers.rs`
- **Change**: Add generic functions:
  - `jni_pickle<T: Serialize>(env, ptr) -> Result<jstring>`
  - `jni_encrypted_pickle<T: Serialize>(env, ptr, key) -> Result<jstring>`
  - `jni_unpickle<T: DeserializeOwned>(env, pickle_data) -> Result<jlong>`
  - `jni_encrypted_unpickle<T: DeserializeOwned>(env, pickle_data, key) -> Result<jlong>`
  - `jni_legacy_unpickle<T>(env, pickle_data, pickle_key) -> Result<jlong>`
- Refactor the 4 types to call these helpers.
- **Verify**: `cargo test` + Java pickle/unpickle tests.

### 6.4 Reduce `errors.rs` boilerplate (D2)

- **File**: `rust/src/errors.rs`
- **Change**: Introduce a small macro or `(&JNIStr, fn)` table so each typed entry is one line.
  Preserve public `throw_*` names.
- **Verify**: `cargo clippy` + `cargo test`.

### 6.5 Use `box_to_jlong` consistently (D3)

- **Files**: `rust/src/olm/account.rs`, `rust/src/sas/sas.rs`, `rust/src/ecies/ecies.rs`,
  `rust/src/megolm/inbound_group_session.rs`, `rust/src/megolm/outbound_group_session.rs`
- **Change**: Replace all `Box::into_raw(Box::new(value)) as jlong` with `box_to_jlong(value)`.
  Note: this overlaps with Phase 2 (the guard pattern changes the construction sites).

**Estimated effort**: ~5 Rust files, ~200 lines removed, ~100 added.

---

## Phase 7: Cryptographic key types (Medium, feature)

Addresses CODE_REVIEW.md F1.

### 7.1 New Java classes

| Class | vodozemac type | Methods |
|---|---|---|
| `Ed25519PublicKey` | `vodozemac::Ed25519PublicKey` | `fromBase64(String)`, `toBase64()`, `verify(String message, String signature) -> boolean` |
| `Ed25519Signature` | `vodozemac::Ed25519Signature` | `fromBase64(String)`, `toBase64()` |
| `Curve25519PublicKey` | `vodozemac::Curve25519PublicKey` | `fromBase64(String)`, `toBase64()` |

### 7.2 New Rust JNI module `rust/src/types/` with thin wrappers

- `Ed25519PublicKey`: `nativeFromBase64`, `nativeToBase64`, `nativeVerify`
- `Ed25519Signature`: `nativeFromBase64`, `nativeToBase64`
- `Curve25519PublicKey`: `nativeFromBase64`, `nativeToBase64`

### 7.3 Design decisions

- `Ed25519Keypair` and secret keys are not needed in Java (clients don't create keypairs
  directly — `Account` does).
- `Ed25519PublicKey.verify()` is the main use case — verify signatures from other devices.
- Could be simpler to add `Account.verify(String message, String signature, String theirEd25519Key)`
  without exposing the key type. But exposing the type is more flexible.

### 7.4 Tests: base64 round-trips, signature verify (valid + tampered).

**Estimated effort**: 3 new Java classes, 1 new Rust module, ~6 JNI functions, tests.

---

## Phase 8: `pickleLegacy()` for session types (Low — blocked)

Addresses CODE_REVIEW.md C1.

### 8.1 Add `pickleLegacy()` to `OlmSession`, `OutboundGroupSession`, `InboundGroupSession`

- **Blocked**: vodozemac 0.10.0 does not expose `to_libolm_pickle` for `Session`,
  `InboundGroupSession`, or `GroupSession` (only `Account` and `PkDecryption` have it).
- **Action**: monitor vodozemac releases for `to_libolm_pickle` support on these types.
  When available, add `nativePickleLegacy` Rust functions and `pickleLegacy(byte[])` Java methods.
- **Workaround**: callers can use `pickle()` (vodozemac format) instead of `pickleLegacy()` (libolm format).
  `unpickleLegacy()` remains for reading existing libolm pickles.

**Estimated effort**: 0 lines (blocked) until vodozemac adds the API; ~3 JNI functions + ~3 Java methods when unblocked.

---

## Phase 9: Test coverage gaps (Low–Medium)

Addresses CODE_REVIEW.md T1–T8.

### 9.1 Java test additions

| Test | Target | Description |
|---|---|---|
| `VodozemacTest` | `base64Decode()` invalid input | Assert `VodozemacException` thrown |
| `InboundGroupSessionTest` | `MegolmMessage.fromBase64()` invalid input | Assert `VodozemacException` thrown |
| `PkEncryptionTest` | `encrypt()` after `close()` | Assert `IllegalStateException` (exposes B3) |
| `EciesTest` | `establishOutboundChannel()` invalid key | Assert `KeyException` + `Ecies` is closed (Phase 1) |
| `EciesTest` | `establishInboundChannel()` malformed message | Assert `EciesException` + `Ecies` is closed (Phase 1) |
| `SasTest` | `diffieHellman()` invalid key | Assert `KeyException` + `Sas` is closed (Phase 1) |
| `InboundGroupSessionTest` | Session usability after `merge()` | Verify original sessions are still usable or properly closed |
| `KeyValidator` | Direct unit test | Test `null`, wrong length, correct length |

### 9.2 Rust test additions

| Module | Tests to add |
|---|---|
| `sas/sas.rs` | DH exchange, public key generation |
| `sas/established_sas.rs` | bytes generation, MAC calculation/verification |
| `ecies/ecies.rs` | outbound/inbound channel establishment |
| `ecies/established_ecies.rs` | encrypt/decrypt, check code |
| `backup/encryption.rs` | PkEncryption roundtrip |
| `backup/decryption.rs` | PkDecryption roundtrip, secret/public key |
| `megolm/inbound_group_session.rs` | export/import, compare, merge, advance |

**Estimated effort**: ~15 test methods (Java), ~10 test functions (Rust), ~300 lines added.

---

## Summary by phase

| Phase | Priority | Scope | Estimated effort |
|---|---|---|---|
| 1. Use-after-free fix | Critical | ~2 Java files, ~3 tests | ~20 lines changed |
| 2. Resource leak fix | Critical | ~3 Rust files | ~30 lines changed |
| 3. `PkEncryption` checkNotClosed | High | 1 Java file, 1 test | ~5 lines changed |
| 4. Medium bugs | Medium | ~4 Java files, 1 pom.xml | ~30 lines changed |
| 5. Low bugs & docs | Low | ~4 Java files, 1 Rust file | ~20 lines changed |
| 6. Rust architecture | Medium | ~5 Rust files | -200 / +100 lines |
| 7. Crypto key types | Medium (feature) | 3 classes + 1 Rust module | ~6 JNI + tests |
| 8. pickleLegacy (blocked) | Low (blocked) | 0 (blocked) | 0 until vodozemac adds API |
| 9. Test coverage | Low–Medium | ~15 Java tests, ~10 Rust tests | ~300 lines added |

### Coverage of CODE_REVIEW.md findings

| CODE_REVIEW.md item | Phase | Status |
|---|---|---|
| B1: Use-after-free in consuming ops | 1 | Open |
| B2: Resource leak on `new_object` failure | 2 | Open |
| B3: `PkEncryption.encrypt()` missing `checkNotClosed()` | 3 | Open |
| B4: `InboundGroupSession` missing `static{}` loader | 4.1 | Open |
| B5: `nativeImport` type mismatch | 4.2 | Open |
| B6: `pom.xml` mainClass invalid | 4.3 | Open |
| B7: `NativeLibraryLoader.loaded` not volatile | 4.4 | Open |
| B8: `olm/InboundCreationResult` not `AutoCloseable` | 4.5 | Open |
| B9: `OlmMessage.java` Javadoc typo | 5.1 | Open |
| B10: `AccountTest.java` variable typo | 5.2 | Open |
| B11: `NativeHandleTest.java` assertion typo | 5.3 | Open |
| B12: `VodozemacException` Javadoc incomplete | 5.4 | Open |
| A1: No panic boundary | 6.1 | Open |
| A2: Integer sign issues | 6.2 | Open |
| A3: `wrap()` silent error | 5.5 | Open |
| D1: Pickle/unpickle duplication | 6.3 | Open |
| D2: `errors.rs` boilerplate | 6.4 | Open |
| D3: Inconsistent `Box::into_raw` vs `box_to_jlong` | 6.5 | Open |
| C1: `pickleLegacy()` for session types | 8 | Blocked |
| C2: `InboundCreationResult.equals()` ignores session | 4.5 | Open |
| F1: Cryptographic key types | 7 | Open |
| T1–T8: Test coverage gaps | 9 | Open |

> All prior critical/security findings (S1–S11) from previous reviews are **complete** and not listed here.
