# Missing Features Implementation Plan

This document tracks the gap between the vodozemac 0.10.0 Rust API and the Java bindings,
as well as issues identified in the code review (`docs/CODE_REVIEW.md`).
It is organized into phases by priority, with each phase being independently deliverable.

Phases are numbered by priority: Phase 1 (critical security) through Phase 10 (minor cleanup).

---

## Phase 1: Rust JNI Memory-Safety Fixes (Critical priority)

Addresses CODE_REVIEW.md findings S1, S3, S4, S5.

### 1.1 Null pointer checks in all Rust JNI functions

Every JNI function that receives a `ptr: jlong` must check for 0 before casting to a raw pointer.

- **Rust**: Add a helper in `helpers.rs`:
  ```rust
  pub(crate) fn check_ptr(ptr: jlong) -> Result<(), jni::errors::Error> {
      if ptr == 0 {
          return Err(jni::errors::Error::JavaException); // throw_generic_error
      }
      Ok(())
  }
  ```
- **Impact**: All ~50 JNI functions that accept `ptr: jlong` gain a guard.
  Alternatively, add `if ptr == 0 { return Err(throw_generic_error(env, "Null native pointer")); }`
  at the top of each function body.

### 1.2 Fix use-after-free on error path in `Sas::nativeDiffieHellman`

**Current** (`rust/src/sas/sas.rs:39-63`): `Box::from_raw` consumes the `Sas` before
calling `diffie_hellman`, which can fail. On failure, the Java side still holds the pointer.

**Fix**: Restructure to validate the key first, then consume:
```rust
let sas = unsafe { &mut *(ptr as *mut Sas) };  // borrow, don't consume
let their_public_key = Curve25519PublicKey::from_base64(...)
    .map_err(|e| throw_key_error(env, e))?;
let established_sas = sas.diffie_hellman(their_public_key)
    .map_err(|e| throw_key_error(env, e))?;
// NOW consume: the Box is still valid, re-take ownership
let sas = unsafe { Box::from_raw(ptr as *mut Sas) };
// sas is moved into the result, established_sas owns it now
```
Actually, since `diffie_hellman(self, ...)` consumes `self`, the cleanest fix is:
- Parse the key first (before `Box::from_raw`)
- If parsing fails, the `Sas` is still alive — Java side still works
- If `diffie_hellman` fails after `Box::from_raw`, the `Sas` is already consumed — Java side must nullify its pointer on any error

**Alternative (simpler)**: Change the Java side to nullify `nativePtr` only on success,
and document that any error from `diffieHellman` means the `Sas` is consumed (must not be reused).

### 1.3 Fix use-after-free on error path in ECIES channel establishment

Same pattern as 1.2, in:
- `rust/src/ecies/ecies.rs:61-83` (`nativeEstablishOutboundChannel`)
- `rust/src/ecies/ecies.rs:95-116` (`nativeEstablishInboundChannel`)

**Fix**: Validate inputs (e.g. `Curve25519PublicKey::from_base64`) before calling
`Box::from_raw`. If `establish_outbound_channel`/`establish_inbound_channel` fails,
the `Ecies` is already consumed — Java side must nullify its pointer.

### 1.4 Add aliasing check in `InboundGroupSession` comparison/merge

**Current** (`rust/src/megolm/inbound_group_session.rs:189-268`): No check that
`ptr != other_ptr`, creating two `&mut` references to the same memory — UB.

**Fix**: Add at the top of `nativeConnected`, `nativeCompare`, `nativeMerge`:
```rust
if ptr == other_ptr {
    return Err(throw_generic_error(env, "Cannot compare a session with itself"));
}
```

### Estimated effort: ~30 Rust changes across 8 modules, ~20 lines of new code

---

## Phase 2: Java Memory-Safety Fixes (Critical priority)

Addresses CODE_REVIEW.md findings S1 (Java side), S2, S9.

### 2.1 Fix `Sas.diffieHellman` pointer management

**Current** (`sas/Sas.java:65-70`):
```java
long ptr = nativePtr;
nativePtr = 0;  // zeroed BEFORE native call
return nativeDiffieHellman(ptr, theirPublicKey);  // can throw
```

**Fix**: Zero after success:
```java
long ptr = nativePtr;
EstablishedSas result = nativeDiffieHellman(ptr, theirPublicKey);
nativePtr = 0;  // only zero on success
return result;
```

### 2.2 Fix ECIES result classes to cache `EstablishedEcies`

**Current** (`ecies/OutboundCreationResult.java:31-33`, `ecies/InboundCreationResult.java:31-33`):
```java
public EstablishedEcies getEstablishedEcies() {
    return new EstablishedEcies(nativePtr);  // new wrapper every call — double-free risk
}
```

**Fix**: Cache in constructor (like `olm/InboundCreationResult` does for `OlmSession`):
```java
private final EstablishedEcies establishedEcies;

OutboundCreationResult(long nativePtr, String initialMessage) {
    this.establishedEcies = new EstablishedEcies(nativePtr);
    this.initialMessage = initialMessage;
}

public EstablishedEcies getEstablishedEcies() {
    return establishedEcies;
}
```
Apply the same fix to `ecies/InboundCreationResult`.

### 2.3 Make ECIES result classes `AutoCloseable`

If the caller never calls `getEstablishedEcies()`, the native resource leaks. Making the
result classes `AutoCloseable` (delegating `close()` to the cached `EstablishedEcies`)
ensures resources are freed:
```java
public class OutboundCreationResult implements AutoCloseable {
    private final EstablishedEcies establishedEcies;
    ...
    @Override
    public void close() { establishedEcies.close(); }
}
```

### 2.4 Add defensive copies for byte array returns

Return `byte[]` copies in:
- `megolm/DecryptedMessage.plaintext()` — return `plaintext.clone()`
- `sas/SasBytes.bytes()` — return `rawBytes.clone()`
- `ecies/CheckCode.asBytes()` — return `bytes.clone()`
- `olm/InboundCreationResult.getPlaintext()` — return `plaintext.clone()`
- `ecies/InboundCreationResult.getPlaintext()` — return `plaintext.clone()`

### 2.5 Add null check to `KeyValidator.validateEncryptionKey`

```java
public static void validateEncryptionKey(byte[] key) {
    if (key == null || key.length != 32) {
        throw new KeyException("Encrypted Key must be 256-bit (32-byte)");
    }
}
```

### Estimated effort: ~5 Java files, ~30 lines changed

---

## Phase 3: Class Hardening (High priority)

Addresses CODE_REVIEW.md findings S6, Phase 4 items from CODE_REVIEW.md.

### 3.1 Make all native-handle classes `final`

Add `final` to: `Account`, `OlmSession`, `OutboundGroupSession`, `InboundGroupSession`,
`Sas`, `EstablishedSas`, `Ecies`, `EstablishedEcies`, `PkDecryption`.

### 3.2 Make `NativeHandle.isClosed()` package-private

Change `public final boolean isClosed()` to `final boolean isClosed()` (package-private).
Tests already live in subpackages — move `isClosed()` tests to a class in the root package,
or keep package-private (tests in `io.github.fherbreteau.vodozemac` package can access it).

### 3.3 Make `InboundGroupSession.connected()`, `compare()`, `merge()` public

These are currently package-private but documented as public in the README. Change access
modifiers to `public`:
- `boolean connected(InboundGroupSession other)` → `public boolean connected(...)`
- `SessionOrdering compare(InboundGroupSession other)` → `public SessionOrdering compare(...)`
- `Optional<InboundGroupSession> merge(InboundGroupSession other)` → `public Optional<InboundGroupSession> merge(...)`

### 3.4 Add `importSession(String)` default-version overload

```java
public static InboundGroupSession importSession(String sessionKey) {
    return importSession(sessionKey, MegolmSessionVersion.defaultVersion());
}
```

### 3.5 Add `(String, Throwable)` constructor to `VodozemacException` and subclasses

```java
public VodozemacException(String message, Throwable cause) {
    super(message, cause);
}
```
Add the same to all 9 subclasses.

### Estimated effort: ~15 Java files, ~40 lines changed

---

## Phase 4: Rust JNI Refactoring & Deduplication (Medium priority)

Addresses CODE_REVIEW.md section 5.1, 5.2, 5.3.

### 4.1 Extract generic `native_free` helper

Create in `helpers.rs`:
```rust
pub(crate) unsafe fn native_free<T>(ptr: jlong) {
    if ptr != 0 {
        let _ = Box::from_raw(ptr as *mut T);
    }
}
```
Each `nativeFree` function body becomes a single call: `unsafe { native_free::<T>(ptr); }`

### 4.2 Extract generic pickle/unpickle helpers

Create in `helpers.rs`:
```rust
pub(crate) fn jni_pickle<T: Serialize>(env: &mut Env, value: &T) -> Result<jstring, jni::errors::Error> { ... }
pub(crate) fn jni_encrypted_pickle<T: Serialize>(env: &mut Env, value: &T, key: &[u8; 32]) -> Result<jstring, jni::errors::Error> { ... }
pub(crate) fn jni_unpickle<T: DeserializeOwned>(pickle_str: &str) -> Result<T, jni::errors::Error> { ... }
pub(crate) fn jni_encrypted_unpickle<T: DeserializeOwned>(pickle_str: &str, key: &[u8; 32]) -> Result<T, jni::errors::Error> { ... }
```
Each JNI function body becomes a 2-3 line call to the helper.

### 4.3 Refactor `errors.rs` to reduce duplication

Replace 11 near-identical throw functions with a single lookup-table + generic function:
```rust
const EXCEPTION_CLASSES: &[(&str, ...)] = &[ ... ];
fn throw_typed(env: &mut Env, class_path: &JNIStr, message: &str) -> jni::errors::Error { ... }
```

### 4.4 Fix `helpers.rs` session config error messages

Replace bare `Err(jni::errors::Error::JavaException)` with descriptive errors:
```rust
_ => Err(throw_generic_error(env, format!("Invalid session config version: {}", version))),
```

### 4.5 Fix `EstablishedEcies::nativeEncrypt` return type consistency

Change closure return type from `Result<jobject, ...>` to `Result<jstring, ...>` to match
the function signature.

### 4.6 Standardise `convert_byte_array` parameter style

Pick one convention (by reference `&JByteArray` or by value `JByteArray`) and apply it
consistently across all modules.

### 4.7 Remove dead code in `backup/encryption.rs`

Remove `let _ = PkEncryption::from_key(public_key);` on line 19 — it creates and discards
an object with no effect.

### 4.8 Fix formatting in `backup/encryption.rs`

Fix double space on line 50 and inconsistent indentation on lines 55-59.

### Estimated effort: ~3 Rust files changed, ~100 lines removed, ~40 lines added

---

## Phase 5: Java Refactoring & Deduplication (Medium priority)

Addresses CODE_REVIEW.md section 5.4, 5.5, 5.7, 5.8, 5.9.

### 5.1 Consolidate session version enums

Option B (preserves type safety): Extract a common `SessionVersion` interface:
```java
public interface SessionVersion {
    int getValue();
}
```
Both `OlmSessionVersion` and `MegolmSessionVersion` implement it. The `fromVersion` method
can be extracted to a utility:
```java
public static <E extends Enum<E> & SessionVersion> E fromVersion(E[] values, int version) { ... }
```

### 5.2 Deduplicate session version tests

Create a shared test utility or JUnit 5 `@ParameterizedTest` that works with any
`SessionVersion` implementation, replacing the two identical test classes.

### 5.3 Extract `fromValue` pattern for `MessageType`

Apply the same shared utility from 5.1 to `MessageType.fromValue(int)`.

### Estimated effort: ~4 Java files changed, ~30 lines removed, ~20 lines added

---

## Phase 6: MegolmMessage Structured Type (Medium priority)

Expose `MegolmMessage` as a proper Java type instead of an opaque base64 string.

### 6.1 New Java class: `MegolmMessage`

| Field | Type | Description |
|---|---|---|
| `ciphertext` | `String` | Base64-encoded ciphertext |
| `messageIndex` | `int` | Message index of the ratchet |
| `mac` | `String` | Base64-encoded MAC |
| `signature` | `String` | Base64-encoded Ed25519 signature |

Methods: `getCiphertext()`, `getMessageIndex()`, `getMac()`, `getSignature()`, `toString()`
(base64 for wire format), `static fromBase64(String)`.

### 6.2 Update `OutboundGroupSession.encrypt()` to return `MegolmMessage`

### 6.3 Update `InboundGroupSession.decrypt()` to accept `MegolmMessage`

### 6.4 Update Rust JNI `nativeEncrypt` (outbound) to return `MegolmMessage` object

### 6.5 Update Rust JNI `nativeDecrypt` (inbound) to accept `MegolmMessage`

### 6.6 Design decisions

- **Clean break** (same as `OlmMessage`): Replace `String` with `MegolmMessage`, no overloads.
- The Rust `MegolmMessage` struct exposes `to_base64()` / `from_base64()` for wire format.
  The Java `toString()` should produce the base64 representation for the JNI layer.

### Estimated effort: ~8 JNI function changes, 1 Java class, 1 enum (if needed)

---

## Phase 7: Cryptographic Key Types (Medium priority)

Expose `Ed25519PublicKey`, `Ed25519Signature`, `Curve25519PublicKey` as Java types.

### 7.1 New Java classes

| Java class | vodozemac type | Key methods |
|---|---|---|
| `Ed25519PublicKey` | `vodozemac::Ed25519PublicKey` | `fromBase64(String)`, `toBase64() -> String`, `verify(String message, String signature) -> boolean` |
| `Ed25519Signature` | `vodozemac::Ed25519Signature` | `fromBase64(String)`, `toBase64() -> String` |
| `Curve25519PublicKey` | `vodozemac::Curve25519PublicKey` | `fromBase64(String)`, `toBase64() -> String` |

### 7.2 New Rust JNI module: `types/`

Thin JNI wrappers around the vodozemac key types.

### 7.3 Design decisions

- `Ed25519Keypair` and secret keys are not needed in Java (clients don't create keypairs
  directly — `Account` does).
- `Ed25519PublicKey.verify()` is the main use case — verify signatures from other devices.
- Could be simpler to just add `Account.verify(String message, String signature, String theirEd25519Key)`
  without exposing the key type. But exposing the type is more flexible.

### Estimated effort: ~6 JNI functions, 3 Java classes, 1 Rust module

---

## Phase 8: Native Library Loader Hardening (Medium priority)

Addresses CODE_REVIEW.md findings S7, S8, S11.

### 8.1 Set owner-only file permissions on extracted library

After `Files.copy`, explicitly set permissions:
```java
if (!platform.startsWith(OS_WINDOWS)) {
    Set<PosixFilePermission> perms = PosixFilePermissions.fromString("rwx------");
    Files.setPosixFilePermissions(tempLib, perms);
}
```

### 8.2 Use try-with-resources for `InputStream`

```java
try (InputStream in = NativeLibraryLoader.class.getResourceAsStream(resourcePath)) {
    if (in == null) { throw new FileNotFoundException(...); }
    Files.copy(in, tempLib, StandardCopyOption.REPLACE_EXISTING);
}
```

### 8.3 Preserve first exception via `addSuppressed`

```java
try {
    loadFromResources(resourcePath, libName, platform);
    loaded = true;
} catch (Exception firstError) {
    try {
        loadFromResources(fallbackPath, libName, platform);
        loaded = true;
    } catch (Exception e2) {
        e2.addSuppressed(firstError);
        throw new RuntimeException("Failed to load native library for " + platform, e2);
    }
}
```

### Estimated effort: 1 Java file, ~15 lines changed

---

## Phase 9: API Consistency Fixes (Low priority)

### 9.1 Add `pickleLegacy()` write methods

For symmetry with `unpickleLegacy`, add `pickleLegacy(byte[] key)` to `OlmSession`,
`OutboundGroupSession`, `InboundGroupSession` (if vodozemac supports it).

### 9.2 Standardize accessor naming convention

Choose one convention and apply it consistently:
- **Fluent** (no `get` prefix): `sessionId()`, `plaintext()`, `emojiIndices()`
- **Bean** (`get` prefix): `getType()`, `getCiphertext()`, `getSession()`

Or accept the current mixed style and document the convention (value types use fluent,
result/holder types use `get`-prefix).

### 9.3 Add `equals`/`hashCode`/`toString` to value classes

Add to: `IdentityKeys`, `SessionKeys`, `OneTimeKeyGenerationResult`, `DehydratedDeviceResult`,
`DecryptedMessage`, `PkMessage`, `InboundCreationResult` (both).

### 9.4 Fix `OlmMessage` class-level Javadoc JSON key order

Change `{"type":<int>,"body":"<base64>"}` to `{"body":"<base64>","type":<int>}` to match
the actual `toString()` output.

### 9.5 Fix `Ecies` Javadoc

Replace `Closeable` with `AutoCloseable` in the Javadoc, remove unused `import java.io.Closeable`.

### 9.6 Fix `@see` link in `olm/InboundCreationResult`

Change third parameter from `String` to `OlmMessage`:
```java
@see io.github.fherbreteau.vodozemac.account.Account#createInboundSession(OlmSessionVersion, String, OlmMessage)
```

### 9.7 Standardize `@author` tags

Add `@author François HERBRETEAU` to all classes that are missing it, or remove the tag
from all classes for consistency.

### 9.8 Fix `OlmMessage.toString()` trailing newline

Replace the text block with a single-line `String.format`:
```java
@Override
public String toString() {
    return String.format("{\"body\":\"%s\",\"type\":%d}", body, type.value());
}
```

### 9.9 Document `PkEncryption` as intentionally stateless

Add a class-level Javadoc note explaining that `PkEncryption` does not extend `NativeHandle`
because it holds no persistent native resource.

### Estimated effort: ~15 Java files, ~60 lines changed

---

## Phase 10: Missing Features (Low priority)

### 10.1 Standalone Ed25519 signature verification

Expose `Ed25519PublicKey.verify(message, signature)` so Java consumers can verify
signatures without an `Account`. This overlaps with Phase 7 (Cryptographic Key Types)
and can be implemented together.

### 10.2 `PkDecryption.pickleLegacy()`

Expose `PkDecryption::to_libolm_pickle` for symmetry with `unpickleLegacy`.

### 10.3 Expose `MegolmMessage` individual fields

Once Phase 6 (MegolmMessage structured type) is done, the individual fields (ciphertext,
mac, signature, messageIndex) will be accessible via getters. No separate work needed.

### 10.4 Remove or configure unused Maven plugins

Either configure `git-commit-id-maven-plugin` and `sonar-maven-plugin` with executions,
or remove them from `pom.xml`.

### Estimated effort: ~3 JNI functions, 1 Java method, ~5 pom.xml lines

---

## Summary by phase

| Phase | Priority | JNI functions | Java files | Rust files | Estimated effort |
|---|---|---|---|---|---|
| 1. Rust JNI memory-safety fixes | Critical | ~30 changed | 0 | 8 | ~20 lines new |
| 2. Java memory-safety fixes | Critical | 0 | 5 | 0 | ~30 lines changed |
| 3. Class hardening | High | 0 | 15 | 0 | ~40 lines changed |
| 4. Rust JNI refactoring & dedup | Medium | ~20 simplified | 0 | 3 | ~100 lines removed, ~40 added |
| 5. Java refactoring & dedup | Medium | 0 | 4 | 0 | ~30 lines removed, ~20 added |
| 6. MegolmMessage structured type | Medium | ~8 changed | 1 new | Existing | ~1 class, ~8 function changes |
| 7. Cryptographic key types | Medium | ~6 new | 3 new | 1 new | ~6 JNI functions, 3 classes |
| 8. Native library loader hardening | Medium | 0 | 1 | 0 | ~15 lines changed |
| 9. API consistency fixes | Low | 0 | ~15 | 0 | ~60 lines changed |
| 10. Missing features | Low | ~3 new | ~1 | 0 | ~5 pom.xml lines |
| **Total remaining** | | **~67** | **~30** | **~12** | **~350 lines changed** |

### Code review findings coverage

| CODE_REVIEW.md finding | Phase | Section |
|---|---|---|
| S1: `Sas.diffieHellman` leaks on failure | Phase 1.2 + Phase 2.1 | Rust + Java |
| S2: ECIES result double-free | Phase 2.2 + 2.3 | Java |
| S3: Use-after-free on error in Rust | Phase 1.2 + 1.3 | Rust |
| S4: No null pointer checks in Rust | Phase 1.1 | Rust |
| S5: Aliasing UB in InboundGroupSession | Phase 1.4 | Rust |
| S6: Native-handle classes not final | Phase 3.1 | Java |
| S7: Library file permissions | Phase 8.1 | Java |
| S8: InputStream not closed | Phase 8.2 | Java |
| S9: No defensive copies | Phase 2.4 | Java |
| S10: KeyValidator no null check | Phase 2.5 | Java |
| S11: First exception swallowed | Phase 8.3 | Java |
| 5.1: nativeFree duplication (Rust) | Phase 4.1 | Rust |
| 5.2: Pickle/unpickle duplication (Rust) | Phase 4.2 | Rust |
| 5.3: errors.rs throw functions | Phase 4.3 | Rust |
| 5.4: fromVersion duplication (Java) | Phase 5.1 | Java |
| 5.7: Pickle boilerplate (Java) | Phase 4.2 (shared) | Java |
| 5.8: Session version enum duplication | Phase 5.1 | Java |
| 5.9: Session version test duplication | Phase 5.2 | Java |
| D2: Unused Maven plugins | Phase 10.4 | pom.xml |
| Phase 2 (MegolmMessage) | Phase 6 | New |
| Phase 3 (Crypto key types) | Phase 7 | New |
