# Code Review: Vodozemac Java Bindings

**Project**: `io.github.fherbreteau:vodozemac-java` v1.0.0  
**Date**: 2026-08-13  
**Reviewer**: Automated Code Review  
**vodozemac crate**: 0.10.0

---

## 1. Summary Scorecard

| Category | Score | Notes |
|---|---|---|
| Lint Compliance | 10/10 | Checkstyle 0 violations, Clippy 0 warnings, fmt clean |
| Code Quality | 6/10 | Use-after-free risk in Sas/ECIES on error; `nativeImport` int/long mismatch; package-private methods documented as public; inconsistent accessor naming; missing `equals`/`hashCode` on value classes |
| Security | 5/10 | `Sas.diffieHellman` leaks native memory on failure; ECIES result double-free via repeated `getEstablishedEcies()`; no null pointer checks in Rust JNI; native-handle classes not `final`; `NativeLibraryLoader` doesn't set file permissions on library; `InputStream` not closed on failure |
| Maintainability | 6/10 | Good modular structure; ~200 lines of duplicated pickle/unpickle and `nativeFree` boilerplate in Rust; 11 near-identical throw functions in `errors.rs`; `fromVersion` duplicated across 2 enums; `KeyValidator` already extracted |
| Documentation | 7/10 | Javadoc present on most classes; class-level `OlmMessage` Javadoc has wrong JSON key order vs `toString()`; `Ecies` Javadoc references `Closeable` instead of `AutoCloseable`; `@author` tag inconsistent; `@see` link in `InboundCreationResult` has wrong signature |
| Idempotency | 9/10 | `close()` is idempotent (tested); `NativeLibraryLoader` is synchronized and guarded; `Sas.diffieHellman` zeroes pointer before native call (not idempotent on failure) |
| **Overall** | **6/10** | Solid foundation with good test coverage (101 Java + 29 Rust tests), but critical memory-safety issues in Sas/ECIES and missing defense-in-depth in Rust JNI need attention |

---

## 2. Repository Structure

```
vodozemac-java/
├── .cargo/config.toml                  # Rust cross-compilation linker config
├── .github/
│   ├── dependabot.yml                  # Dependabot: maven, cargo, rust-toolchain, github-actions (weekly)
│   ├── scripts/coverage-report.py      # Combined Rust + Java coverage markdown generator
│   └── workflows/
│       ├── build.yml                   # Multi-platform native build + Maven package + test
│       ├── test.yml                    # PR quick-test: Rust tests, clippy, fmt, Java verify, coverage
│       └── release.yml                 # Tag-triggered publish to GitHub Packages
├── docs/
│   ├── CODE_REVIEW.md                  # This file
│   ├── IMPLEMENTATION_PLAN.md          # Action items from this review
│   └── .gitkeep
├── rust/
│   ├── Cargo.toml                      # vodozemac 0.10.0, jni 0.22.4, serde_json 1.0
│   ├── Cargo.lock
│   └── src/
│       ├── lib.rs                      # Module declarations
│       ├── errors.rs                   # JNI error mapping (11 throw functions)
│       ├── helpers.rs                  # Shared helpers (wrap, session config, test JVM)
│       ├── utils/
│       │   └── mod.rs                  # JNI: Vodozemac (base64, version)
│       ├── olm/
│       │   ├── mod.rs
│       │   ├── account.rs               # JNI: Account (19 functions)
│       │   └── session.rs               # JNI: OlmSession (11 functions)
│       ├── megolm/
│       │   ├── mod.rs
│       │   ├── inbound_group_session.rs  # JNI: InboundGroupSession (14 functions)
│       │   └── outbound_group_session.rs # JNI: OutboundGroupSession (11 functions)
│       ├── sas/
│       │   ├── mod.rs
│       │   ├── sas.rs                   # JNI: Sas (3 functions)
│       │   └── established_sas.rs       # JNI: EstablishedSas (8 functions)
│       ├── ecies/
│       │   ├── mod.rs
│       │   ├── ecies.rs                 # JNI: Ecies (4 functions)
│       │   └── established_ecies.rs      # JNI: EstablishedEcies (4 functions)
│       └── backup/
│           ├── mod.rs
│           ├── encryption.rs             # JNI: PkEncryption (2 functions)
│           └── decryption.rs             # JNI: PkDecryption (6 functions)
├── src/
│   ├── main/java/io/github/fherbreteau/
│   │   ├── Sample.java                  # Demo application
│   │   └── vodozemac/
│   │       ├── NativeHandle.java         # Base class for native pointer lifecycle
│   │       ├── NativeLibraryLoader.java  # Classpath extraction + System.load
│   │       ├── KeyValidator.java        # 32-byte key validation utility
│   │       ├── Vodozemac.java            # Utility class (base64, version)
│   │       ├── account/
│   │       │   ├── Account.java          # AutoCloseable Olm account
│   │       │   ├── IdentityKeys.java      # ed25519 + curve25519 key pair
│   │       │   ├── OneTimeKeyGenerationResult.java
│   │       │   └── DehydratedDeviceResult.java
│   │       ├── olm/
│   │       │   ├── OlmSession.java       # AutoCloseable Olm session
│   │       │   ├── OlmSessionVersion.java # Enum V1(1), V2(2)
│   │       │   ├── OlmMessage.java        # Structured Olm message (type + body)
│   │       │   ├── MessageType.java       # Enum PRE_KEY(0), NORMAL(1)
│   │       │   ├── SessionKeys.java       # Session identity/base/one-time keys
│   │       │   └── InboundCreationResult.java
│   │       ├── megolm/
│   │       │   ├── OutboundGroupSession.java  # AutoCloseable Megolm outbound
│   │       │   ├── InboundGroupSession.java   # AutoCloseable Megolm inbound
│   │       │   ├── MegolmSessionVersion.java  # Enum V1(1), V2(2)
│   │       │   ├── SessionOrdering.java        # Enum for session comparison
│   │       │   └── DecryptedMessage.java
│   │       ├── sas/
│   │       │   ├── Sas.java              # SAS verification
│   │       │   ├── EstablishedSas.java    # Established SAS channel
│   │       │   └── SasBytes.java         # SAS bytes (emoji/decimal)
│   │       ├── ecies/
│   │       │   ├── Ecies.java            # Unestablished ECIES channel
│   │       │   ├── EstablishedEcies.java  # Established ECIES channel
│   │       │   ├── CheckCode.java        # 2-digit check code
│   │       │   ├── OutboundCreationResult.java
│   │       │   └── InboundCreationResult.java
│   │       ├── backup/
│   │       │   ├── PkEncryption.java     # PK encryption (stateless)
│   │       │   ├── PkDecryption.java     # PK decryption (AutoCloseable)
│   │       │   └── PkMessage.java        # Encrypted PK message
│   │       └── exception/
│   │           ├── VodozemacException.java
│   │           ├── PickleException.java
│   │           ├── DecryptionException.java
│   │           ├── SessionCreationException.java
│   │           ├── KeyException.java
│   │           ├── SignatureException.java
│   │           ├── SasException.java
│   │           ├── EciesException.java
│   │           └── EncryptionException.java
│   └── test/java/io/github/fherbreteau/vodozemac/
│       ├── VodozemacTest.java            # 2 tests
│       ├── SampleOlm.java               # Olm sample (no @Test)
│       ├── SampleMegolm.java             # Megolm sample (no @Test)
│       ├── SampleSas.java                # SAS sample (no @Test)
│       ├── SampleEcies.java              # ECIES sample (no @Test)
│       ├── account/AccountTest.java      # 19 tests
│       ├── olm/OlmSessionTest.java       # 9 tests
│       ├── olm/OlmSessionVersionTest.java # 6 tests
│       ├── megolm/OutboundGroupSessionTest.java # 9 tests
│       ├── megolm/InboundGroupSessionTest.java  # 29 tests
│       ├── megolm/MegolmSessionVersionTest.java # 6 tests
│       ├── sas/SasTest.java              # 4 tests
│       ├── ecies/EciesTest.java          # 12 tests
│       └── backup/PkEncryptionTest.java  # 5 tests
├── checkstyle.xml                       # Checkstyle configuration
├── checkstyle.suppression.xml           # Empty suppressions
├── pom.xml                              # Maven build: Rust compile, JNI, JaCoCo, Checkstyle
├── README.md
├── SECURITY.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── coverage-report.md                   # Latest coverage report
└── LICENSE                              # Apache-2.0
```

---

## 3. Lint & Test Results

### 3.1 Java Checkstyle

```
mvn checkstyle:check
→ 0 violations — PASS
```

### 3.2 Rust Clippy

```
cargo clippy
→ 0 warnings — PASS
```

### 3.3 Rust Format

```
cargo fmt -- --check
→ 0 issues — PASS
```

### 3.4 Rust Tests

```
cargo test
→ 29 tests, 0 failures, 0 errors — PASS
```

### 3.5 Java Tests

```
mvn test
→ 101 tests, 0 failures, 0 errors, 0 skipped — PASS
```

| Test Class | Tests | Status |
|---|---|---|
| `VodozemacTest` | 2 | PASS |
| `AccountTest` | 19 | PASS |
| `OlmSessionTest` | 9 | PASS |
| `OlmSessionVersionTest` | 6 | PASS |
| `OutboundGroupSessionTest` | 9 | PASS |
| `InboundGroupSessionTest` | 29 | PASS |
| `MegolmSessionVersionTest` | 6 | PASS |
| `SasTest` | 4 | PASS |
| `EciesTest` | 12 | PASS |
| `PkEncryptionTest` | 5 | PASS |
| **Total** | **101** | **ALL PASS** |

### 3.6 Coverage

| Language | Lines | Functions/Methods | Branches |
|---|---|---|---|
| Java | 133/208 (63.9%) | 55/88 (62.5%) | 21/34 (61.8%) |
| Rust | 253/781 (32.4%) | 30/96 (31.2%) | - |

**Note**: JaCoCo coverage is enforced at 80% (via `jacoco-maven-plugin`). All coverage checks pass.

---

## 4. Incomplete Tasks

The following features from `docs/IMPLEMENTATION_PLAN.md` remain unimplemented:

### Phase 2: MegolmMessage Structured Type (Medium Priority)
- [ ] `MegolmMessage` Java class with ciphertext, messageIndex, mac, signature
- [ ] Update `OutboundGroupSession.encrypt()` to return `MegolmMessage`
- [ ] Update `InboundGroupSession.decrypt()` to accept `MegolmMessage`

### Phase 3: Cryptographic Key Types (Medium Priority)
- [ ] `Ed25519PublicKey` — `fromBase64()`, `toBase64()`, `verify()`
- [ ] `Ed25519Signature` — `fromBase64()`, `toBase64()`
- [ ] `Curve25519PublicKey` — `fromBase64()`, `toBase64()`

### Phase 4: API Consistency Fixes (Medium Priority)
- [ ] Make `InboundGroupSession.connected()`, `compare()`, `merge()` public (currently package-private but documented as public in README)
- [ ] Add `importSession(String)` default-version overload to `InboundGroupSession`
- [ ] Add `pickleLegacy()` write methods to `OlmSession`, `OutboundGroupSession`, `InboundGroupSession` for symmetry with `unpickleLegacy`
- [ ] Add `(String, Throwable)` constructor to `VodozemacException` and all subclasses for cause chaining
- [ ] Make `NativeHandle.isClosed()` package-private (currently public despite "test only" comment)
- [ ] Make all native-handle classes `final` to prevent subclassing and protect `nativePtr`
- [ ] Add `equals`/`hashCode`/`toString` to value classes (`IdentityKeys`, `SessionKeys`, `OneTimeKeyGenerationResult`, `DehydratedDeviceResult`, `DecryptedMessage`, `PkMessage`, `InboundCreationResult`)
- [ ] Fix `OlmMessage` class-level Javadoc JSON key order (`{"type":...,"body":...}` vs actual `{"body":...,"type":...}`)
- [ ] Fix `Ecies` Javadoc: references `Closeable` instead of `AutoCloseable`, remove unused `import java.io.Closeable`
- [ ] Fix `@see` link in `olm/InboundCreationResult` — third param is `OlmMessage` not `String`
- [ ] Standardize accessor naming convention (fluent vs `get`-prefix)
- [ ] Add `@author` tag consistently or remove it entirely

### Phase 5: Rust JNI Hardening (High Priority)
- [ ] Add null pointer checks before all `ptr as *mut T` / `ptr as *const T` casts (systemic — affects all modules)
- [ ] Fix use-after-free on error path in `Sas::nativeDiffieHellman` — `Box::from_raw` consumes before operation that can fail
- [ ] Fix use-after-free on error path in `Ecies::nativeEstablishOutboundChannel` and `nativeEstablishInboundChannel`
- [ ] Add aliasing check in `InboundGroupSession::nativeConnected`, `nativeCompare`, `nativeMerge` when `ptr == other_ptr`
- [ ] Add descriptive error messages for invalid session config versions in `helpers.rs` (currently returns bare `JavaException`)
- [ ] Fix return type inconsistency in `EstablishedEcies::nativeEncrypt` (`jstring` declared vs `jobject` in closure)
- [ ] Standardise `convert_byte_array` parameter style (by value vs by reference)
- [ ] Remove dead code in `backup/encryption.rs:19` (`let _ = PkEncryption::from_key(...)`)

### Phase 6: Native Library Loader Hardening (Medium Priority)
- [ ] Set owner-only file permissions on extracted library file (not just the temp directory)
- [ ] Use try-with-resources for `InputStream` in `loadFromResources`
- [ ] Preserve first exception via `addSuppressed` in fallback load path

### Phase 7: Memory Safety in Java Result Classes (High Priority)
- [ ] Fix `Sas.diffieHellman` — zero `nativePtr` after native call succeeds, not before (leaks on failure)
- [ ] Fix `ecies/OutboundCreationResult` and `ecies/InboundCreationResult` — cache `EstablishedEcies` in constructor (like `olm/InboundCreationResult` does), prevent double-free from repeated `getEstablishedEcies()`
- [ ] Make ECIES result classes `AutoCloseable` or ensure ownership transfer is one-time
- [ ] Add null check to `KeyValidator.validateEncryptionKey(byte[])`
- [ ] Return defensive copies of byte arrays in `DecryptedMessage.plaintext()`, `SasBytes.bytes()`, `CheckCode.asBytes()`, `InboundCreationResult.getPlaintext()`

### Phase 8: Missing Features (Low Priority)
- [ ] Standalone Ed25519 signature verification (`Ed25519PublicKey.verify(message, signature)`)
- [ ] `PkDecryption.pickleLegacy()` (libolm pickle write — only `unpickleLegacy` exists)
- [ ] Expose `MegolmMessage` individual fields (ciphertext, mac, signature, messageIndex)
- [ ] Document `PkEncryption` as intentionally stateless (not extending `NativeHandle`)

---

## 5. Duplicated Code

### 5.1 `nativeFree` Function (Rust JNI) — 9 copies

Identical pattern in all 9 Rust modules, differing only in the Rust type name:

```rust
#[unsafe(no_mangle)]
pub extern "system" fn Java_..._nativeFree(_env: EnvUnowned, _class: JClass, ptr: jlong) {
    unsafe { let _ = Box::from_raw(ptr as *mut T); }
}
```

| File | Lines | Type |
|---|---|---|
| `rust/src/olm/account.rs` | 523-532 | `Account` |
| `rust/src/olm/session.rs` | 10-19 | `Session` |
| `rust/src/megolm/inbound_group_session.rs` | 270-279 | `InboundGroupSession` |
| `rust/src/megolm/outbound_group_session.rs` | 198-207 | `GroupSession` |
| `rust/src/sas/sas.rs` | 65-74 | `Sas` |
| `rust/src/sas/established_sas.rs` | 199-208 | `EstablishedSas` |
| `rust/src/ecies/ecies.rs` | 118-127 | `Ecies` |
| `rust/src/ecies/established_ecies.rs` | 85-94 | `EstablishedEcies` |
| `rust/src/backup/decryption.rs` | 126-135 | `PkDecryption` |

### 5.2 Pickle/Unpickle Pattern (Rust JNI) — 4 types × 5 variants ≈ 20 functions

Five pickle/unpickle variants repeated for `Account`, `Session`, `InboundGroupSession`, and `OutboundGroupSession`:

| Variant | Pattern |
|---|---|
| `nativePickle` | `obj.pickle()` → `serde_json::to_string` → JString |
| `nativeEncryptedPickle` | `obj.pickle()` → `.encrypt(&key)` → JString |
| `nativeUnpickle` | `serde_json::from_str` → `T::from_pickle` → Box → jlong |
| `nativeEncryptedUnpickle` | `T::Pickle::from_encrypted` → `T::from_pickle` → Box → jlong |
| `nativeUnpickleLegacy` | `T::from_libolm_pickle` → Box → jlong |

Locations: `rust/src/olm/account.rs:362-472`, `rust/src/olm/session.rs:140-230`, `rust/src/megolm/inbound_group_session.rs:97-334`, `rust/src/megolm/outbound_group_session.rs:107-196`

### 5.3 `errors.rs` Throw Functions — 11 near-identical functions

`rust/src/errors.rs:7-111` contains 11 functions that all follow the same pattern:
```rust
fn throw_X_error<E: Display>(env: &mut Env, error: E) -> jni::errors::Error {
    throw(env, jni_str!(".../XException"), &error.to_string())
}
```
A table-driven or macro-based approach would reduce this to a single lookup table.

### 5.4 `fromVersion(int)` Factory — 2 identical copies

`OlmSessionVersion.java:48-53` and `MegolmSessionVersion.java:48-53` contain the identical implementation:
```java
return Stream.of(values())
    .filter(v -> version == v.value)
    .findFirst()
    .orElseThrow(() -> new VodozemacException("unknown version " + version));
```

### 5.5 `MessageType.fromValue(int)` Factory — identical pattern

`MessageType.java:21-26` follows the same `Stream.of(values()).filter(...).orElseThrow(...)` pattern as 5.4.

### 5.6 `static { NativeLibraryLoader.loadLibrary(); }` — 7 copies

Repeated in `Vodozemac`, `Account`, `OutboundGroupSession`, `Sas`, `Ecies`, `PkEncryption`, `PkDecryption`. Intentional (ensures loading regardless of entry point) but repetitive.

### 5.7 Pickle/Unpickle Java Boilerplate — 4 classes

`Account`, `OlmSession`, `OutboundGroupSession`, `InboundGroupSession` each repeat the same 5-method pickle API (~60 lines each): `pickle()`, `pickle(byte[])`, `unpickle(String)`, `unpickle(String, byte[])`, `unpickleLegacy(String, byte[])`, plus 5 native method declarations.

### 5.8 Session Version Enum Structure — 2 structurally identical enums

`OlmSessionVersion` and `MegolmSessionVersion` are structurally identical: `V1(1)`, `V2(2)`, `getValue()`, `defaultVersion()`, `fromVersion(int)`. A shared `SessionVersion` interface or a single generic enum would eliminate this.

### 5.9 Session Version Tests — 2 structurally identical test classes

`OlmSessionVersionTest` and `MegolmSessionVersionTest` contain the same 6 test methods with only the class name changed.

---

## 6. Security Review

### 6.1 Findings

| # | Severity | Finding | Location | Details |
|---|---|---|---|---|
| S1 | **Critical** | **`Sas.diffieHellman` leaks native memory on failure** | `sas/Sas.java:65-70` | `nativePtr` is zeroed *before* the native call. If `nativeDiffieHellman` throws (invalid key), the native `Sas` resource is leaked forever and `close()` becomes a no-op. Should zero pointer *after* success. |
| S2 | **Critical** | **ECIES result double-free / use-after-free** | `ecies/OutboundCreationResult.java:31-33`, `ecies/InboundCreationResult.java:31-33` | `getEstablishedEcies()` creates a new `EstablishedEcies(nativePtr)` on every call. Two calls yield two objects sharing the same pointer; closing one causes use-after-free in the other. Should cache the instance in the constructor (like `olm/InboundCreationResult` does). |
| S3 | **Critical** | **Use-after-free on error in Rust JNI** | `rust/src/sas/sas.rs:39-63`, `rust/src/ecies/ecies.rs:61-83,95-116` | `Box::from_raw` consumes the native object before calling `diffie_hellman`/`establish_outbound_channel`/`establish_inbound_channel`, which can fail. On failure, the Java side still holds the pointer — any subsequent call is use-after-free. |
| S4 | **High** | **No null pointer checks in Rust JNI** | All Rust modules (systemic) | Every JNI function casts `ptr: jlong` to a raw pointer without checking for 0. If Java passes a freed pointer, this is undefined behavior (null dereference). The Java `checkNotClosed()` guard is the only defense, with no Rust-level safety net. |
| S5 | **High** | **Aliasing UB in `InboundGroupSession` comparison/merge** | `rust/src/megolm/inbound_group_session.rs:189-203,223-268` | If `ptr == other_ptr`, two `&mut` references to the same memory are created — undefined behavior in Rust. No same-pointer check exists. |
| S6 | **High** | **Native-handle classes not `final`** | `Account`, `OlmSession`, `OutboundGroupSession`, `InboundGroupSession`, `Sas`, `EstablishedSas`, `Ecies`, `EstablishedEcies`, `PkDecryption` | `NativeHandle.nativePtr` is `protected`. Non-final classes can be subclassed, exposing the raw pointer to third-party code. |
| S7 | **Medium** | **Library file not given owner-only permissions** | `NativeLibraryLoader.java:128` | Only the temp *directory* gets `rwx------`. The library *file* is written via `Files.copy` with no permission restriction. Javadoc claims owner-only permissions. |
| S8 | **Medium** | **`InputStream` not closed on failure** | `NativeLibraryLoader.java:112-129` | `in.close()` is only reached if `Files.copy` succeeds. If it throws, the `InputStream` leaks. Should use try-with-resources. |
| S9 | **Medium** | **No defensive copies of byte arrays** | `DecryptedMessage.plaintext()`, `SasBytes.bytes()`, `CheckCode.asBytes()`, `InboundCreationResult.getPlaintext()` (both) | Internal `byte[]` fields are returned directly; callers can mutate them, corrupting object state. |
| S10 | **Low** | **`KeyValidator` does not null-check** | `KeyValidator.java:26-29` | A `null` key throws `NullPointerException` instead of a meaningful `KeyException`. |
| S11 | **Low** | **First exception silently swallowed** | `NativeLibraryLoader.java:60-73` | The root-cause exception from the first load attempt is discarded; only the fallback exception is reported. Using `addSuppressed` would preserve it. |

### 6.2 Positive Security Practices

- Uses established `vodozemac` Rust library (audited by Matrix.org)
- `AutoCloseable` pattern ensures native resources are freed
- `close()` is idempotent — safe to call multiple times (tested)
- `NativeLibraryLoader` is `synchronized` and guarded by `loaded` flag — prevents double-loading
- Pickle encryption uses 32-byte keys with AES
- Legacy pickle support for libolm compatibility
- Typed exception hierarchy mapped from Rust errors to Java exceptions
- `InboundCreationResult` (olm) correctly caches the `OlmSession` in the constructor — safe pattern

---

## 7. Dependency Matrix

### 7.1 Java / Maven Dependencies

| Dependency | GroupId | ArtifactId | Version | Scope | Purpose |
|---|---|---|---|---|---|
| JUnit Jupiter | `org.junit.jupiter` | `junit-jupiter` | 5.10.2 | test | Unit testing framework |
| AssertJ Core | `org.assertj` | `assertj-core` | 3.27.7 | test | Fluent assertions |

### 7.2 Maven Plugins

| Plugin | Version | Purpose |
|---|---|---|
| `exec-maven-plugin` | 3.2.0 | Invoke `cargo build` during `generate-resources` |
| `maven-resources-plugin` | 3.3.1 | Copy native lib to `target/classes` |
| `maven-compiler-plugin` | 3.13.0 | Java 25 compilation |
| `maven-surefire-plugin` | 3.2.5 | Test execution |
| `maven-jar-plugin` | 3.4.1 | JAR packaging |
| `maven-clean-plugin` | 3.3.2 | Clean Rust `target/` too |
| `maven-checkstyle-plugin` | 3.6.0 | Checkstyle validation |
| `jacoco-maven-plugin` | 0.8.14 | Code coverage (80% enforced) |
| `git-commit-id-maven-plugin` | 10.0.0 | Git metadata (declared, not configured) |
| `sonar-maven-plugin` | 5.6.0.6792 | SonarCloud (declared, not configured) |
| `versions-maven-plugin` | — | Dependency version properties (declared) |
| `maven-dependency-plugin` | — | Properties goal for version info |

### 7.3 Rust Dependencies

| Crate | Version | Purpose | Features |
|---|---|---|---|
| `vodozemac` | 0.10.0 | Core Matrix crypto (OLM, Megolm, SAS, ECIES, PK) | `libolm-compat`, `experimental-session-config`, `insecure-pk-encryption` |
| `jni` | 0.22.4 | JNI bindings for Rust | (dev: `invocation`) |
| `serde_json` | 1.0.141 | JSON serialization for pickle data | - |

### 7.4 Rust Dev Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `jni` | 0.22.4 | With `invocation` feature for JVM in tests |

### 7.5 CI/CD External Actions

| Action | Version | Used In |
|---|---|---|
| `actions/checkout` | v7 | build.yml, test.yml, release.yml |
| `actions/setup-java` | v5 | build.yml, test.yml, release.yml |
| `actions/upload-artifact` | v6 | build.yml |
| `actions/download-artifact` | v8 | build.yml, release.yml |
| `dtolnay/rust-toolchain` | stable | build.yml, test.yml |
| `Swatinem/rust-cache` | v2 | build.yml, test.yml |
| `taiki-e/install-action` | v2 | test.yml (cargo-llvm-cov) |
| `marocchino/sticky-pull-request-comment` | v3 | test.yml |
| `softprops/action-gh-release` | v2 | release.yml |

### 7.6 Dependency Issues

| # | Issue | Details | Status |
|---|---|---|---|
| D1 | **Rust edition 2024** | `Cargo.toml` uses `edition = "2024"` which requires Rust 1.85+. CI pins Rust 1.88.0. | Informational |
| D2 | **`git-commit-id-maven-plugin` not configured** | Declared in `pom.xml` but no execution is bound. | Low |
| D3 | **`sonar-maven-plugin` not configured** | Declared in `pom.xml` but no execution is bound. | Low |
