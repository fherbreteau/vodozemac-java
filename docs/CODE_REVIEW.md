# Code Review: Vodozemac Java Bindings

**Project**: `io.github.fherbreteau:vodozemac-java` v1.0.0
**Date**: 2026-08-26
**Reviewer**: Automated Code Review
**vodozemac crate**: 0.10.0
**Branch**: `docs/Code_Review_and_Implementation_Plan` @ `bf884bd`

---

## 1. Summary Scorecard

| Category | Score | Notes |
|---|---|---|
| Lint Compliance | 10/10 | Checkstyle 0 violations; Clippy 0 warnings; `cargo fmt --check` clean; Maven enforcer (Maven ≥3.6.3, Java 25) passes |
| Code Quality | 7/10 | 100% Java coverage; solid JNI architecture; `nativeFree` deduplicated. **Open bugs**: missing `checkNotClosed()` in `PkEncryption.encrypt()`, missing `static{}` loader in `InboundGroupSession`, `nativeImport` type mismatch, `pom.xml` mainClass invalid, use-after-free on consuming ops, resource leak on `new_object` failure |
| Security | 7/10 | Prior S1–S11 fixed. **New findings**: use-after-free in `Sas.diffieHellman`/`Ecies.establish*` on Rust error (double-free via `close()`), resource leak when `env.new_object` fails, no panic boundary across JNI, no key zeroization |
| Maintainability | 8/10 | Good modular structure; `SessionVersion` interface dedupes enums; `nativeFree` deduplicated; `MegolmMessage` structured type. Remaining: pickle/unpickle boilerplate (4 types × 5 variants), `errors.rs` wrappers, inconsistent `Box::into_raw` vs `box_to_jlong` |
| Documentation | 8/10 | `@author` consistent across all 42 classes; Javadoc on all public APIs. **Issues**: `OlmMessage.java:10` has typo `fherbreau` → `fherbreteau`, `VodozemacException` Javadoc omits 3 subclasses, `olm/InboundCreationResult` missing `AutoCloseable` documentation |
| Idempotency | 6/10 | `close()` idempotent (tested); `NativeLibraryLoader` synchronized. **Issue**: `Sas.diffieHellman`/`Ecies.establish*` zero `nativePtr` only on success — on failure, `close()` will double-free the already-dropped Rust `Box` |
| **Overall** | **7/10** | Solid foundation with 100% Java coverage and all prior security fixes. New critical/high bugs in Rust JNI lifecycle management (use-after-free, resource leaks) and missing safety guards must be addressed before production use |

---

## 2. Repository Structure

```
vodozemac-java/
├── .github/
│   ├── dependabot.yml                  # maven, cargo, rust-toolchain, github-actions (weekly)
│   ├── scripts/coverage-report.py      # Combined Rust+Java coverage markdown generator
│   └── workflows/
│       ├── build.yml                   # 6-platform native build + Maven package + test + Sonar
│       ├── test.yml                    # PR: Rust tests/clippy/fmt, Java verify, coverage comment
│       └── release.yml                 # Tag-triggered publish to GitHub Packages
├── docs/
│   ├── CODE_REVIEW.md                  # This file
│   ├── IMPLEMENTATION_PLAN.md          # Action items from this review
│   └── .gitkeep
├── rust/
│   ├── Cargo.toml                      # vodozemac 0.10.0, jni 0.22.4, serde_json 1.0.141, edition 2024
│   ├── Cargo.lock
│   └── src/
│       ├── lib.rs                      # Module declarations
│       ├── errors.rs                   # JNI error mapping (shared `throw` core + 13 typed wrappers)
│       ├── helpers.rs                  # check_ptr, native_free, session-config helpers, wrap, test JVM
│       ├── utils/mod.rs                # JNI: Vodozemac (base64, version)
│       ├── olm/{mod,account,session}.rs
│       ├── megolm/{mod,inbound_group_session,outbound_group_session,message}.rs
│       ├── sas/{mod,sas,established_sas}.rs
│       ├── ecies/{mod,ecies,established_ecies}.rs
│       └── backup/{mod,encryption,decryption}.rs
├── src/
│   ├── main/java/io/github/fherbreteau/
│   │   └── vodozemac/
│   │       ├── NativeHandle.java        # Base native-pointer lifecycle (AutoCloseable)
│   │       ├── NativeLibraryLoader.java # Classpath extraction + System.load (excluded from coverage)
│   │       ├── KeyValidator.java        # 32-byte key validation (null-checked)
│   │       ├── SessionVersion.java      # Shared interface + fromVersion() utility
│   │       ├── Vodozemac.java           # Utility class (base64, version)
│   │       ├── account/{Account,IdentityKeys,OneTimeKeyGenerationResult,DehydratedDeviceResult}.java
│   │       ├── olm/{OlmSession,OlmSessionVersion,OlmMessage,MessageType,SessionKeys,InboundCreationResult}.java
│   │       ├── megolm/{OutboundGroupSession,InboundGroupSession,MegolmSessionVersion,SessionOrdering,DecryptedMessage,MegolmMessage}.java
│   │       ├── sas/{Sas,EstablishedSas,SasBytes}.java
│   │       ├── ecies/{Ecies,EstablishedEcies,CheckCode,OutboundCreationResult,InboundCreationResult}.java
│   │       ├── backup/{PkEncryption,PkDecryption,PkMessage}.java
│   │       └── exception/{VodozemacException + 9 subclasses + ConversionException}.java
│   └── test/java/io/github/fherbreteau/vodozemac/
│       ├── VodozemacTest.java           # 2 tests
│       ├── NativeHandleTest.java        # 10 tests
│       ├── SessionVersionTest.java      # 23 tests (parameterized)
│       ├── ExceptionTests.java          # 9 tests
│       ├── account/AccountTest.java      # 22 tests
│       ├── olm/OlmSessionTest.java       # 12 tests
│       ├── megolm/OutboundGroupSessionTest.java # 8 tests
│       ├── megolm/InboundGroupSessionTest.java  # 31 tests
│       ├── sas/SasTest.java             # 4 tests
│       ├── ecies/EciesTest.java         # 14 tests
│       └── backup/PkEncryptionTest.java # 7 tests
├── checkstyle.xml / checkstyle.suppression.xml
├── pom.xml                              # Maven 3.6.3+, Java 25, Rust build, JaCoCo 80%, Checkstyle 14, Sonar profile
├── README.md / SECURITY.md / CONTRIBUTING.md / CHANGELOG.md / LICENSE
```

---

## 3. Lint & Test Results

### 3.1 Java Checkstyle
```
mvn checkstyle:check → 0 violations — PASS
```

### 3.2 Rust Clippy
```
cargo clippy → 0 warnings — PASS
```

### 3.3 Rust Format
```
cargo fmt -- --check → 0 issues — PASS
```

### 3.4 Rust Tests
```
cargo test → 29 tests, 0 failures — PASS
```

### 3.5 Java Tests (`mvn verify`)
```
142 tests, 0 failures, 0 errors, 0 skipped — BUILD SUCCESS
```

| Test Class | Tests | Status |
|---|---|---|
| `VodozemacTest` | 2 | PASS |
| `NativeHandleTest` | 10 | PASS |
| `SessionVersionTest` | 23 | PASS |
| `ExceptionTests` | 9 | PASS |
| `AccountTest` | 22 | PASS |
| `OlmSessionTest` | 12 | PASS |
| `OutboundGroupSessionTest` | 8 | PASS |
| `InboundGroupSessionTest` | 31 | PASS |
| `SasTest` | 4 | PASS |
| `EciesTest` | 14 | PASS |
| `PkEncryptionTest` | 7 | PASS |
| **Total** | **142** | **ALL PASS** |

### 3.6 Coverage

**Java (JaCoCo, `NativeLibraryLoader` excluded):** all counters at 100%.

| Counter | Missed | Covered | Ratio |
|---|---|---|---|
| INSTRUCTION | 0 | 1980 | 100% |
| BRANCH | 0 | 84 | 100% |
| LINE | 0 | 512 | 100% |
| COMPLEXITY | 0 | 284 | 100% |
| METHOD | 0 | 242 | 100% |
| CLASS | 0 | 41 | 100% |

JaCoCo gate enforces ≥80% instructions and 0 missed methods/classes — **all checks met**.

**Rust (`cargo-llvm-cov`, unit tests only):**

| Metric | Covered | Total | Ratio |
|---|---|---|---|
| Regions | 909 | 4114 | 22.10% |
| Lines | 466 | 2440 | 19.10% |
| Functions | 54 | 334 | 16.17% |

> Note: Rust JNI functions (the bulk of the crate) are **not** invoked by Rust unit tests;
> they are exercised through the Java integration tests above. The low cargo-llvm-cov ratio
> reflects that the JNI layer is tested from Java, not from Rust. `helpers.rs` reaches 50% line
> coverage from Rust tests; all other modules are driven by Java tests.

---

## 4. Incomplete Tasks

Only items that remain open are listed. All prior critical/high security tasks (S1–S11) from
previous reviews are complete.

### Bugs — Critical

- [ ] **B1: Use-after-free in consuming operations** (`sas/sas.rs:52`, `ecies/ecies.rs:68,101`): Rust takes ownership via `Box::from_raw`, then if the operation fails (e.g. invalid key), the `Box` is dropped — freeing the memory. But the Java side (`Sas.java:70`, `Ecies.java:114,144`) only zeroes `nativePtr` on **success**. On failure, `nativePtr` still holds the old (now-freed) pointer value. When `close()` is called (via try-with-resources or GC), `nativeFree` calls `Box::from_raw` on a dangling pointer → double-free / use-after-free.
- [ ] **B2: Resource leak on Java object construction failure** (`olm/account.rs:126,168`, `sas/sas.rs:57`, `ecies/ecies.rs:74,107`): `Box::into_raw` allocates native memory and converts to `jlong` **before** `env.new_object()`. If `new_object` fails, the `?` operator returns early and the raw pointer is orphaned — native memory is never freed.

### Bugs — High

- [ ] **B3: `PkEncryption.encrypt()` missing `checkNotClosed()`** (`PkEncryption.java:72-73`): This is the only public method on a `NativeHandle` subclass that does not call `checkNotClosed()` before accessing `nativePtr`. If `close()` has been called, `nativePtr` is 0 and the native call receives a null pointer, likely causing a native crash.

### Bugs — Medium

- [ ] **B4: `InboundGroupSession` missing `static { NativeLibraryLoader.loadLibrary(); }`** (`InboundGroupSession.java`): Every other `NativeHandle` subclass (8 classes) has this static initializer. `InboundGroupSession` is the only one missing it. If `InboundGroupSession` is the first class loaded (e.g. via `importSession()` or `unpickle()`), the native library may not be loaded, causing `UnsatisfiedLinkError`.
- [ ] **B5: `InboundGroupSession.nativeImport` type mismatch** (`InboundGroupSession.java:345`): Declared as `nativeImport(String sessionKey, long version)` but `nativeNew` at line 313 uses `int version`. Both are called with `version.value()` which returns `int`. The `long` parameter should be `int` for consistency and correct JNI signature matching.
- [ ] **B6: `pom.xml` mainClass references non-existent class** (`pom.xml:476`): `<mainClass>io.github.fherbreteau.Sample</mainClass>` — no class with this name exists. The example classes are in `io.github.fherbreteau.vodozemac.examples`. The JAR manifest will reference a non-existent main class, making `java -jar` fail. Should be removed or corrected.

### Bugs — Low

- [ ] **B7: `NativeLibraryLoader.loaded` not `volatile`** (`NativeLibraryLoader.java:31`): `private static boolean loaded = false;` is accessed outside the synchronized block by `isLoaded()`. While `loadLibrary()` is synchronized, `isLoaded()` is not. The `loaded` field should be `volatile` to ensure visibility across threads.
- [ ] **B8: `olm/InboundCreationResult` does not implement `AutoCloseable`** (`olm/InboundCreationResult.java:17`): It holds an `OlmSession` (a `NativeHandle`) but does not implement `AutoCloseable`. The caller must manually close the session via `result.session().close()`. Compare with `ecies/OutboundCreationResult` and `ecies/InboundCreationResult` which both implement `AutoCloseable` and delegate `close()` to the inner `EstablishedEcies`.
- [ ] **B9: `OlmMessage.java:10` Javadoc typo** — `{@link io.github.fherbreau.vodozemac.account.Account...}` missing `t` in `fherbreteau`. Broken `@link` reference.
- [ ] **B10: `AccountTest.java:282` variable typo** — `dehydratexDevice` should be `dehydratedDevice`.
- [ ] **B11: `NativeHandleTest.java:133` assertion message typo** — `"Native Libray is Loaded"` should be `"Native Library is Loaded"`.
- [ ] **B12: `VodozemacException` Javadoc incomplete** (`VodozemacException.java:8-9`) — omits `ConversionException`, `EciesException`, and `EncryptionException` from the list of subclasses.

### Rust JNI — Architecture (Medium)

- [ ] **A1: No panic boundary across JNI** — All JNI entry points lack `std::panic::catch_unwind`. A Rust panic crossing the FFI boundary is undefined behavior. While no `unwrap()`/`expect()` calls exist in non-test code, a panic in the vodozemac crate itself would cause UB.
- [ ] **A2: Integer sign issues** (`megolm/inbound_group_session.rs:147,187`) — `index as u32` where `index` is `jint` (i32). Negative indices wrap to large u32 values. Should validate with `u32::try_from(index)` before casting.
- [ ] **A3: `wrap()` lacks descriptive error** (`helpers.rs:9-11`) — Returns `JavaException` without setting a Java exception or message when the key is not 32 bytes. Results in a generic `RuntimeException` with no diagnostic information.

### Rust JNI — Deduplication (Medium, maintainability)

- [ ] **D1: Pickle/unpickle duplication**: the 5-variant pickle family (`nativePickle`, `nativeEncryptedPickle`, `nativeUnpickle`, `nativeEncryptedUnpickle`, `nativeUnpickleLegacy`) is hand-rolled per type across `account.rs`, `session.rs`, `inbound_group_session.rs`, `outbound_group_session.rs`. Extract generic serde-based helpers in `helpers.rs`.
- [ ] **D2: `errors.rs` wrappers**: a shared `throw` core exists (good), but 13 individual `throw_*` wrappers remain. A `jni_str!`-table + single generic dispatcher (or a small macro) would reduce boilerplate.
- [ ] **D3: Inconsistent `Box::into_raw` vs `box_to_jlong`** — Constructors use `Box::into_raw(Box::new(value)) as jlong` directly, while unpickle functions use the `box_to_jlong(value)` helper. Should use `box_to_jlong` consistently.

### Java — API consistency (Low–Medium)

- [ ] **C1: `pickleLegacy()` write methods missing for 3 session types**: `OlmSession`, `OutboundGroupSession`, and `InboundGroupSession` have `unpickleLegacy()` (read) but no corresponding `pickleLegacy()` (write). `Account` and `PkDecryption` have both. Note: vodozemac 0.10.0 does not expose `to_libolm_pickle` for `Session`, `InboundGroupSession`, or `GroupSession` — this task is blocked unless a future vodozemac version adds the API.
- [ ] **C2: `olm/InboundCreationResult.equals()` ignores `session` field** (`olm/InboundCreationResult.java:50`) — Two results with the same plaintext but different sessions are considered equal. Same pattern in `ecies/OutboundCreationResult.equals()` which only compares `initialMessage`.

### Feature gaps (Medium, tracked in IMPLEMENTATION_PLAN.md)

- [x] **F1: Cryptographic key types**: `Ed25519PublicKey`, `Ed25519Signature`, `Curve25519PublicKey` Java classes now exist in `io.github.fherbreteau.vodozemac.types`, backed by `rust/src/types/mod.rs`. Keys are typed throughout the Account, IdentityKeys, OneTimeKeyGenerationResult, and SessionKeys APIs. — **Complete** (Phase 7, PR #38)

### Test Coverage Gaps

- [ ] **T1: `Vodozemac.base64Decode()` error case** — No test for invalid base64 input.
- [ ] **T2: `MegolmMessage.fromBase64()` error case** — Tested for valid input only.
- [ ] **T3: `PkEncryption.encrypt()` after close** — Would expose bug B3.
- [ ] **T4: `Ecies` establishment failure cases** — No tests for invalid/non-contributory public key, null public key, malformed initial message.
- [ ] **T5: `Sas.diffieHellman()` with invalid key** — Not tested (would expose bug B1).
- [ ] **T6: `InboundGroupSession` behavior after `merge()`** — Not tested whether original sessions are still usable.
- [ ] **T7: Rust tests for SAS, ECIES, Backup modules** — 5 files, 630 lines total, have zero Rust tests.
- [ ] **T8: `KeyValidator` dedicated test** — Only tested indirectly through pickle error tests.

---

## 5. Duplicated Code

### 5.1 Pickle/Unpickle family — 4 types × 5 variants (REMAINS)
`nativePickle`, `nativeEncryptedPickle`, `nativeUnpickle`, `nativeEncryptedUnpickle`, `nativeUnpickleLegacy` are hand-rolled in `account.rs`, `session.rs`, `inbound_group_session.rs`, `outbound_group_session.rs`. Generic serde-based helpers in `helpers.rs` would remove the boilerplate.

| Function | account.rs | session.rs | inbound_group_session.rs | outbound_group_session.rs | decryption.rs |
|---|---|---|---|---|---|
| `nativePickle` | ✓ | ✓ | ✓ | ✓ | — |
| `nativeEncryptedPickle` | ✓ | ✓ | ✓ | ✓ | — |
| `nativeUnpickle` | ✓ | ✓ | ✓ | ✓ | — |
| `nativeEncryptedUnpickle` | ✓ | ✓ | ✓ | ✓ | — |
| `nativePickleLegacy` | ✓ | — | — | — | — |
| `nativeUnpickleLegacy` | ✓ | ✓ | ✓ | ✓ | ✓ |

### 5.2 `errors.rs` wrappers — 13 typed functions (PARTIALLY FIXED)
A shared `throw` core exists (`errors.rs:7-10`), but 13 `throw_*` wrappers remain (`errors.rs:12-141`). A table/macro dispatch would reduce them further.

### 5.3 `static { NativeLibraryLoader.loadLibrary(); }` — 8 copies (INTENTIONAL)
Present in `Vodozemac`, `Account`, `OutboundGroupSession`, `Sas`, `Ecies`, `PkEncryption`, `PkDecryption`, `MegolmMessage`. Repetitive but intentional (ensures loading regardless of entry point). **Missing** from `InboundGroupSession` (bug B4).

### 5.4 Java pickle boilerplate — 4 classes (REMAINS)
`Account`, `OlmSession`, `OutboundGroupSession`, `InboundGroupSession` each repeat the same 5-method pickle API (~60 lines each). A shared interface or default-method trait would reduce this (overlaps with 5.1).

### 5.5 JNI function boilerplate — ~55 occurrences (ACCEPTABLE)
Every JNI function follows the `env.with_env(|env| -> Result<T> { ... })` + `outcome.resolve()` pattern. A macro could reduce this but the explicit pattern is clear and debuggable.

### 5.6 Value class `equals`/`hashCode`/`toString` — 13 classes (ACCEPTABLE)
All value classes implement nearly identical `equals`/`hashCode`/`toString`. Standard Java boilerplate; could use records but current approach is fine.

> **Previously deduplicated (complete):** `nativeFree` — all 9 copies now use `native_free::<T>(env, ptr)` from `helpers.rs`. `SessionVersion` interface eliminates enum factory duplication. `convert_byte_array` call style is consistent.

---

## 6. Security Review

### 6.1 Previously Resolved Findings (S1–S11)

All previously identified security findings are **resolved**:

| # | Prior finding | Status | Evidence |
|---|---|---|---|
| S1 | `Sas.diffieHellman` leaks on failure | **FIXED** | `Sas.java:70` zeroes `nativePtr` after success; `sas.rs:48-56` validates key before `Box::from_raw` |
| S2 | ECIES result double-free | **FIXED** | `OutboundCreationResult`/`InboundCreationResult` cache `EstablishedEcies` in constructor + `AutoCloseable` |
| S3 | Use-after-free on error in Rust | **FIXED** | `ecies.rs:63-68,98-101` validate inputs before `Box::from_raw` |
| S4 | No null-pointer checks in Rust | **FIXED** | `check_ptr` in `helpers.rs:75`; called by every `ptr`-taking JNI function |
| S5 | Aliasing UB in InboundGroupSession | **FIXED** | `check_self_pointer` at `inbound_group_session.rs`, called by `nativeConnected`/`nativeCompare`/`nativeMerge` |
| S6 | Native-handle classes not `final` | **FIXED** | all 9 classes are `final` |
| S7 | Library file not owner-only | **FIXED** | `NativeLibraryLoader.java:132-135` sets `rwx------` on the file |
| S8 | `InputStream` not closed on failure | **FIXED** | try-with-resources at `:116` |
| S9 | No defensive copies of byte arrays | **FIXED** | `.clone()` in all 5 return sites |
| S10 | `KeyValidator` no null check | **FIXED** | `KeyValidator.java:27` checks `null` before length |
| S11 | First exception swallowed | **FIXED** | `addSuppressed` at `:72` |

### 6.2 New Findings

| # | Finding | Severity | Location | Impact |
|---|---|---|---|---|
| S12 | Use-after-free on consuming ops failure | **HIGH** | `sas/sas.rs:52`, `ecies/ecies.rs:68,101` | Rust drops `Box` on error, Java `close()` double-frees the dangling pointer |
| S13 | Resource leak on `new_object` failure | **CRITICAL** | `account.rs:126,168`, `sas.rs:57`, `ecies.rs:74,107` | Native memory orphaned if Java object construction fails |
| S14 | No panic boundary across JNI | **MEDIUM** | All JNI entry points | UB if any Rust panic crosses FFI boundary |
| S15 | No key material zeroization | **LOW** | `decryption.rs:53`, pickle functions | Cryptographic keys remain in heap memory after use |
| S16 | Integer sign issues | **MEDIUM** | `inbound_group_session.rs:147,187` | Negative indices wrap to large u32 values |
| S17 | `NativeLibraryLoader.loaded` not `volatile` | **LOW** | `NativeLibraryLoader.java:31` | Visibility issue across threads for `isLoaded()` |
| S18 | `wrap()` silent error | **LOW** | `helpers.rs:9-11` | Generic `RuntimeException` with no message for wrong key length |

### 6.3 Positive security practices

- All native-handle classes `final`; `NativeHandle.nativePtr` lifecycle guarded by `checkNotClosed()`; `close()` idempotent.
- `NativeLibraryLoader` synchronized + `loaded` flag (no double-load); owner-only file permissions; try-with-resources; `addSuppressed` preserves root cause.
- All JNI entry points null-checked via `check_ptr`; aliasing guarded for session comparison/merge.
- Typed exception hierarchy mapped from Rust; `(String, Throwable)` constructors on all exceptions for cause chaining.
- Defensive copies for all byte-array returns (`plaintext.clone()`, `SasBytes.*.clone()`, `CheckCode.asBytes().clone()`).
- Built on the audited `vodozemac` 0.10.0.
- PK Encryption MAC flaw documented in Javadoc (`PkEncryption.java:19-22`).
- `insecure-pk-encryption` feature explicitly enabled and documented.

---

## 7. Dependency Matrix

### 7.1 Java / Maven Dependencies

| Dependency | GroupId | ArtifactId | Version | Scope | Purpose |
|---|---|---|---|---|---|
| JUnit Jupiter | `org.junit.jupiter` | `junit-jupiter` | 6.1.3 | test | Unit testing (JUnit 6) |
| AssertJ Core | `org.assertj` | `assertj-core` | 3.27.7 | test | Fluent assertions |

### 7.2 Maven Plugins

| Plugin | Version | Purpose | Status |
|---|---|---|---|
| `maven-enforcer-plugin` | 3.6.3 | Enforce Maven ≥3.6.3, Java 25 | Configured (execution bound) |
| `exec-maven-plugin` | 3.6.3 | Invoke `cargo build` at `generate-resources` | Configured |
| `maven-resources-plugin` | 3.5.0 | Copy native lib to `target/classes` | Configured |
| `maven-compiler-plugin` | 3.15.0 | Java 25 compilation | Configured |
| `maven-surefire-plugin` | 3.5.6 | Test execution | Configured |
| `maven-jar-plugin` | 3.5.1 | JAR packaging | Configured (mainClass invalid — bug B6) |
| `maven-clean-plugin` | 3.5.0 | Clean Rust `target/` too | Configured |
| `maven-checkstyle-plugin` | 3.6.0 | Checkstyle validation | Configured (`validate` phase) |
| `checkstyle` | 14.0.0 | Checkstyle engine | — |
| `jacoco-maven-plugin` | 0.8.15 | Coverage (80% instructions, 0 missed method/class) | Configured |
| `git-commit-id-maven-plugin` | 10.0.0 | Git metadata (reproducible builds) | Configured (`initialize` phase) |
| `versions-maven-plugin` | 2.21.0 | Version checks (ignores SNAPSHOT/M/alpha/beta) | Configured |
| `maven-dependency-plugin` | 3.11.0 | `properties` goal for version info | Configured |
| `sonar-maven-plugin` | 5.7.0.6970 | SonarCloud analysis | Configured (`sonar` profile) |

### 7.3 Rust Dependencies

| Crate | Version | Purpose | Features |
|---|---|---|---|
| `vodozemac` | 0.10.0 | Core Matrix crypto (Olm, Megolm, SAS, ECIES, PK) | `libolm-compat`, `experimental-session-config`, `insecure-pk-encryption` |
| `jni` | 0.22.4 | JNI bindings | — |
| `serde_json` | 1.0.141 | JSON for pickle data | — |

### 7.4 Rust Dev Dependencies

| Crate | Version | Purpose | Features |
|---|---|---|---|
| `jni` | 0.22.4 | JVM in Rust tests | `invocation` |

### 7.5 CI/CD External Actions

| Action | Ref | Used in |
|---|---|---|
| `actions/checkout` | v7 | build, test, release |
| `actions/setup-java` | v5 | build, test, release |
| `actions/upload-artifact` | v7 | build |
| `actions/download-artifact` | v8 | build, release |
| `dtolnay/rust-toolchain` | `4360b52…` (SHA-pinned, `# v7`) | build, test |
| `Swatinem/rust-cache` | `6323deb…` (SHA-pinned, `# v2`) | build, test |
| `taiki-e/install-action` | `ba47c86…` (SHA-pinned, `# v2`) | test (cargo-llvm-cov) |
| `marocchino/sticky-pull-request-comment` | `5770ad5…` (SHA-pinned, `# v3`) | test |
| `softprops/action-gh-release` | `3d0d988…` (SHA-pinned, `# v2`) | release |

### 7.6 Dependency Issues

| # | Issue | Details | Status |
|---|---|---|---|
| D1 | Rust edition 2024 | `Cargo.toml` uses `edition = "2024"` (requires Rust ≥1.85); CI uses pinned `1.88.0` | Informational |
| D2 | `insecure-pk-encryption` feature | Enabled for PkEncryption/PkDecryption (Megolm backup); no forward secrecy; documented | Informational |
| D3 | `pom.xml` mainClass invalid | `io.github.fherbreteau.Sample` does not exist (bug B6) | Open |
