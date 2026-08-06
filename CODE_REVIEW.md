# Code Review: Vodozemac Java Bindings

**Project**: `io.github.fherbreteau:vodozemac-java` v1.0.0  
**Date**: 2026-08-07  
**Reviewer**: Automated Code Review

---

## 1. Summary Scorecard

| Category | Score | Notes |
|---|---|---|
| Lint Compliance | 9/10 | Checkstyle 0 violations, Clippy 0 warnings, fmt clean; 1 unused import in Rust tests |
| Code Quality | 6/10 | Heavy duplication across 4 JNI wrapper classes; typo in public API; missing key-length validation in 2 classes |
| Security | 6/10 | Fake GPG key in SECURITY.md; native pointer exposed in `InboundCreationResult`; missing key validation in 2 pickle methods |
| Maintainability | 6/10 | Good modular structure but duplication increases maintenance burden; README/CHANGELOG inaccurate |
| Documentation | 4/10 | README references non-existent `VodozemacAccount` class, wrong test count, wrong architecture; Javadoc sparse on most classes |
| Idempotency | 9/10 | `close()` is idempotent (tested); `NativeLibraryLoader` is synchronized and guarded |
| **Overall** | **6/10** | Solid foundation with good test coverage, but documentation drift, duplication, and API inconsistencies need attention |

---

## 2. Repository Structure

```
vodozemac-java/
├── .cargo/config.toml              # Rust cross-compilation linker config
├── .github/
│   ├── dependabot.yml              # Dependabot: maven, cargo, rust-toolchain, github-actions (weekly)
│   ├── scripts/coverage-report.py  # Combined Rust + Java coverage markdown generator
│   └── workflows/
│       ├── build.yml               # Multi-platform native build + Maven package + test
│       ├── test.yml                # PR quick-test: Rust tests, clippy, fmt, Java verify, coverage
│       └── release.yml             # Tag-triggered publish to GitHub Packages
├── docs/IMPLEMENTATION-PLAN.md     # Gap analysis: 9 phases of missing features
├── rust/
│   ├── Cargo.toml                  # Rust crate: vodozemac 0.9.0, jni 0.22.4, serde_json 1.0
│   ├── Cargo.lock
│   └── src/
│       ├── lib.rs                  # Module declarations
│       ├── helpers.rs               # Shared helpers (wrap, session config, test JVM)
│       ├── olm/
│       │   ├── mod.rs
│       │   ├── account.rs           # JNI: Account (new, keys, sign, sessions, pickle, dehydrate)
│       │   └── session.rs           # JNI: OlmSession (encrypt, decrypt, pickle)
│       └── megolm/
│           ├── mod.rs
│           ├── inbound_group_session.rs   # JNI: InboundGroupSession (decrypt, pickle)
│           └── outbound_group_session.rs # JNI: OutboundGroupSession (encrypt, pickle)
├── src/
│   ├── main/java/io/github/fherbreteau/
│   │   ├── Sample.java              # Demo application
│   │   └── vodozemac/
│   │       ├── NativeLibraryLoader.java  # Classpath extraction + System.load
│   │       ├── VodozemacException.java     # RuntimeException wrapper
│   │       ├── account/
│   │       │   ├── Account.java              # AutoCloseable Olm account
│   │       │   ├── IdentityKeys.java          # ed25519 + curve25519 key pair
│   │       │   ├── OneTimeKeyGenerationResult.java
│   │       │   └── DehydratedDeviceResult.java
│   │       ├── olm/
│   │       │   ├── OlmSession.java            # AutoCloseable Olm session
│   │       │   ├── OlmSessionVersion.java     # Enum V1(1), V2(2)
│   │       │   └── InboundCreationResult.java
│   │       └── megolm/
│   │           ├── OutboundGroupSession.java  # AutoCloseable Megolm outbound
│   │           ├── InboundGroupSession.java   # AutoCloseable Megolm inbound
│   │           ├── MegolmSessionVersion.java  # Enum V1(1), V2(2)
│   │           └── DecryptedMessage.java
│   └── test/java/io/github/fherbreteau/vodozemac/
│       ├── account/AccountTest.java            # 17 tests
│       ├── olm/OlmSessionTest.java             # 7 tests
│       ├── olm/OlmSessionVersionTest.java      # 4 tests
│       ├── megolm/OutboundGroupSessionTest.java # 7 tests
│       ├── megolm/InboundGroupSessionTest.java  # 7 tests
│       └── megolm/MegolmSessionVersionTest.java # 4 tests
├── checkstyle.xml                  # Checkstyle configuration
├── checkstyle.suppression.xml      # Empty suppressions
├── pom.xml                          # Maven build: Rust compile, JNI, JaCoCo, Checkstyle
├── Cargo.lock                       # Root-level (stale — should not exist)
├── README.md
├── SECURITY.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── coverage-report.md               # Latest coverage report
└── LICENSE                          # Apache-2.0
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
→ 1 warning: unused import `crate::helpers::PICKLE_KEY` in rust/src/olm/session.rs:177
```

### 3.5 Java Tests

```
mvn test
→ 46 tests, 0 failures, 0 errors, 0 skipped — PASS
```

| Test Class | Tests | Status |
|---|---|---|
| `AccountTest` | 17 | PASS |
| `OlmSessionTest` | 7 | PASS |
| `OlmSessionVersionTest` | 4 | PASS |
| `OutboundGroupSessionTest` | 7 | PASS |
| `InboundGroupSessionTest` | 7 | PASS |
| `MegolmSessionVersionTest` | 4 | PASS |
| **Total** | **46** | **ALL PASS** |

### 3.6 Coverage

| Language | Lines | Functions/Methods | Branches |
|---|---|---|---|
| Java | 133/208 (63.9%) | 55/88 (62.5%) | 21/34 (61.8%) |
| Rust | 253/781 (32.4%) | 30/96 (31.2%) | - |

**Note**: pom.xml declares `jacoco.coverage.ratio` at 95% but this is not enforced by any plugin configuration. Java coverage falls well short of this target.

---

## 4. Incomplete Tasks

All 9 phases from `docs/IMPLEMENTATION-PLAN.md` remain unimplemented:

### Phase 1: InboundGroupSession Session Management (High Priority)
- [ ] `InboundGroupSession.import(ExportedSessionKey, MegolmSessionVersion)` — constructor/static factory
- [ ] `export_at(index)` and `export_at_first_known_index()` — export methods
- [ ] `advance_to(index)` — session advancement
- [ ] `connected(other)`, `compare(other)`, `merge(other)` + `SessionOrdering` enum

### Phase 2: SAS Module (High Priority)
- [ ] `Sas` class — `new()`, `publicKey()`, `diffieHellman(String)`
- [ ] `EstablishedSas` class — `bytes()`, `calculateMac()`, `verifyMac()`, key accessors
- [ ] `SasBytes` class — `emojiIndices()`, `decimals()`
- [ ] Rust JNI module `rust/src/sas/`

### Phase 3: ECIES Module (High Priority)
- [ ] `Ecies` class — `new()`, `establishOutboundChannel()`, `establishInboundChannel()`
- [ ] `EstablishedEcies` class — `encrypt()`, `decrypt()`, `checkCode()`
- [ ] `CheckCode`, `EciesOutboundCreationResult`, `EciesInboundCreationResult` classes
- [ ] Rust JNI module `rust/src/ecies/`

### Phase 4: PK Encryption Module (High Priority)
- [ ] `PkEncryption`, `PkDecryption`, `PkMessage` classes
- [ ] Cargo.toml feature: `insecure-pk-encryption`
- [ ] Rust JNI module `rust/src/pk_encryption/`

### Phase 5: Structured Message Types (Medium Priority)
- [ ] `OlmMessage` Java class with `MessageType` enum (`PRE_KEY`, `NORMAL`)
- [ ] `MegolmMessage` Java class with ciphertext, messageIndex, mac, signature
- [ ] Update `OlmSession.encrypt()`/`decrypt()` to use typed messages
- [ ] Update `InboundGroupSession.decrypt()` to use typed messages

### Phase 6: Cryptographic Key Types (Medium Priority)
- [ ] `Ed25519PublicKey` — `fromBase64()`, `toBase64()`, `verify()`
- [ ] `Ed25519Signature` — `fromBase64()`, `toBase64()`
- [ ] `Curve25519PublicKey` — `fromBase64()`, `toBase64()`

### Phase 7: Missing Methods on Existing Classes (Medium Priority)
- [ ] `Account.toLibolmPickle(byte[] key)` method
- [ ] `OlmSession.sessionKeys()` and `OlmSession.sessionConfig()`
- [ ] `OutboundGroupSession.sessionConfig()`
- [ ] Fix typo: `createOutbpundSession` -> `createOutboundSession` (Account.java:95, Sample.java:38, AccountTest.java:405, OlmSessionTest.java:33)

### Phase 8: Granular Error Types (Low Priority)
- [ ] Typed exception hierarchy (PickleException, DecryptionException, SessionCreationException, KeyException, SignatureException)
- [ ] JNI error mapping changes

### Phase 9: Utility Functions (Low Priority)
- [ ] `base64Encode(byte[])` and `base64Decode(String)` utility methods
- [ ] `Vodozemac.getVersion()` constant

---

## 5. Duplicated Code

### 5.1 `checkNotClosed()` Method (Java)

Identical implementation in 4 classes with **incorrect error message** in 3 of them:

| Class | File | Line |
|---|---|---|
| `Account` | `account/Account.java` | :292 |
| `OlmSession` | `olm/OlmSession.java` | :116 |
| `OutboundGroupSession` | `megolm/OutboundGroupSession.java` | :60 |
| `InboundGroupSession` | `megolm/InboundGroupSession.java` | :55 |

All four throw `"Account has been closed"` — only `Account` is correct. The other three should say `"OlmSession has been closed"`, `"OutboundGroupSession has been closed"`, `"InboundGroupSession has been closed"` respectively. This is a copy-paste bug.

### 5.2 `close()` Method (Java)

Identical pattern in all 4 AutoCloseable classes:

```java
@Override
public void close() {
    if (nativePtr != 0) {
        nativeFree(nativePtr);
        nativePtr = 0;
    }
}
```

| Class | File | Line |
|---|---|---|
| `Account` | `account/Account.java` | :309 |
| `OlmSession` | `olm/OlmSession.java` | :133 |
| `OutboundGroupSession` | `megolm/OutboundGroupSession.java` | :77 |
| `InboundGroupSession` | `megolm/InboundGroupSession.java` | :72 |

### 5.3 `isClosed()` Method (Java)

Identical in all 4 classes:

```java
boolean isClosed() {
    return nativePtr == 0;
}
```

### 5.4 32-byte Key Validation (Java)

Duplicated in 6 locations across 3 files:

```java
if (key.length != 32) {
    throw new VodozemacException("Encrypted Key must be 256-bit (32-byte)");
}
```

| Location | File | Line |
|---|---|---|
| `Account.pickle(byte[])` | `account/Account.java` | :205 |
| `Account.unpickle(String, byte[])` | `account/Account.java` | :231 |
| `Account.toDehydratedDevice(byte[])` | `account/Account.java` | :268 |
| `Account.fromDehydratedDevice(...)` | `account/Account.java` | :285 |
| `OlmSession.pickle(byte[])` | `olm/OlmSession.java` | :71 |
| `OlmSession.unpickle(String, byte[])` | `olm/OlmSession.java` | :96 |

**Missing** in `OutboundGroupSession.pickle(byte[])` and `InboundGroupSession.pickle(byte[])` — these accept any key length without validation, which is inconsistent and potentially a security issue.

### 5.5 Pickle/Unpickle Pattern (Rust JNI)

The `nativePickle`, `nativeEncryptedPickle`, `nativeUnpickle`, `nativeEncryptedUnpickle`, `nativeUnpickleLegacy` functions follow an almost identical pattern across all 4 Rust modules:

| Module | File | Functions |
|---|---|---|
| Account | `rust/src/olm/account.rs` | :347, :364, :382, :398, :416 |
| Session | `rust/src/olm/session.rs` | :87, :104, :122, :138, :156 |
| InboundGroupSession | `rust/src/megolm/inbound_group_session.rs` | :88, :105, :123, :139, :157 |
| OutboundGroupSession | `rust/src/megolm/outbound_group_session.rs` | :87, :104, :122, :138, :156 |

Each is ~15-20 lines of nearly identical code differing only in the Rust type name.

### 5.6 `nativeFree` Function (Rust JNI)

Identical pattern in all 4 Rust modules:

```rust
#[unsafe(no_mangle)]
pub extern "system" fn Java_..._nativeFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut T);
    }
}
```

### 5.7 Session Version Enums (Java)

`OlmSessionVersion` and `MegolmSessionVersion` are structurally identical:

```java
public enum XxxSessionVersion {
    V1(1),
    V2(2);
    private final int value;
    XxxSessionVersion(int value) { this.value = value; }
    public int getValue() { return value; }
}
```

### 5.8 Session Version Tests (Java)

`OlmSessionVersionTest` and `MegolmSessionVersionTest` are structurally identical — same 4 test methods with only the class name changed.

### 5.9 Recommendation

Consider extracting a common base class or utility:
- `NativeHandle` abstract class with `nativePtr`, `checkNotClosed()`, `isClosed()`, `close()`
- `KeyValidator.validateKey(byte[] key)` utility method
- A single `SessionVersion` enum shared between Olm and Megolm (or a generic parametrized version)

---

## 6. Security Review

### 6.1 Findings

| # | Severity | Finding | Location |
|---|---|---|---|
| S1 | Medium | **Fake GPG key** in SECURITY.md — repetitive placeholder data, not a real key | `SECURITY.md:22-33` |
| S2 | Medium | **Missing key-length validation** in `OutboundGroupSession.pickle(byte[])` and `InboundGroupSession.pickle(byte[])` | `megolm/OutboundGroupSession.java:40`, `megolm/InboundGroupSession.java:35` |
| S3 | Low | **Native pointer exposed** — `InboundCreationResult` constructor is public and accepts `long sessionPtr`, exposing the raw native pointer to any caller | `olm/InboundCreationResult.java:10` |
| S4 | Low | **Inconsistent contact info** — `security@fherbreteau.io` (SECURITY.md), `fherbreteau@protonmail.com` (CHANGELOG.md), `fherbreteau@gmail.com` (pom.xml) — users may not know which channel to trust | Multiple files |
| S5 | Low | **French error messages** in `NativeLibraryLoader` — "Impossible de charger la librairie native", "OS non supporté" — may confuse international users and security auditors | `NativeLibraryLoader.java:49,63,72` |
| S6 | Info | **Temp file permissions** — `NativeLibraryLoader` extracts native library to a temp directory with `Files.createTempDirectory` but does not set restrictive permissions on the extracted file | `NativeLibraryLoader.java:96-106` |
| S7 | Info | **`System.load` restricted warning** — JVM warns about restricted native access; should add `--enable-native-access=ALL-UNNAMED` for Java 25+ compatibility | Test output |

### 6.2 Positive Security Practices

- Uses established `vodozemac` Rust library (audited by Matrix.org)
- `AutoCloseable` pattern ensures native resources are freed
- `close()` is idempotent — safe to call multiple times
- `NativeLibraryLoader` is `synchronized` and guarded by a `loaded` flag — prevents double-loading
- Pickle encryption uses 32-byte keys with AES
- Legacy pickle support for libolm compatibility

---

## 7. Dependency Matrix

### 7.1 Java / Maven Dependencies

| Dependency | GroupId | ArtifactId | Version | Scope | Purpose |
|---|---|---|---|---|---|
| JUnit Jupiter | `org.junit.jupiter` | `junit-jupiter` | 5.10.2 | test | Unit testing framework |
| AssertJ Core | `org.assertj` | `assertj-core` | 3.25.3 | test | Fluent assertions |

### 7.2 Maven Plugins

| Plugin | Version | Purpose |
|---|---|---|
| `exec-maven-plugin` | 3.2.0 | Invoke `cargo build` during `generate-resources` |
| `maven-resources-plugin` | 3.3.1 | Copy native lib to `target/classes` |
| `maven-compiler-plugin` | 3.13.0 | Java 17 compilation |
| `maven-surefire-plugin` | 3.2.5 | Test execution |
| `maven-jar-plugin` | 3.4.1 | JAR packaging |
| `maven-clean-plugin` | 3.3.2 | Clean Rust `target/` too |
| `maven-checkstyle-plugin` | 3.6.0 | Checkstyle validation |
| `jacoco-maven-plugin` | 0.8.14 | Code coverage |
| `git-commit-id-maven-plugin` | 10.0.0 | Git metadata (declared, not configured) |
| `sonar-maven-plugin` | 5.6.0.6792 | SonarCloud (declared, not configured) |
| `versions-maven-plugin` | - | Dependency version properties (declared, property `${dependencies-version.version}` **undefined**) |
| `maven-dependency-plugin` | - | Properties goal for version info |

### 7.3 Rust Dependencies

| Crate | Version | Purpose | Features |
|---|---|---|---|
| `vodozemac` | 0.9.0 | Core Matrix crypto (OLM, Megolm) | `libolm-compat` |
| `jni` | 0.22.4 | JNI bindings for Rust | (dev: `invocation`) |
| `serde_json` | 1.0.141 | JSON serialization for pickle data | - |

### 7.4 Rust Dev Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `jni` | 0.22.4 | With `invocation` feature for JVM in tests |

### 7.5 CI/CD External Actions

| Action | Version | Used In |
|---|---|---|
| `actions/checkout` | v7, v4 | build.yml, test.yml, release.yml |
| `actions/setup-java` | v5, v4 | build.yml, test.yml, release.yml |
| `actions/upload-artifact` | v6 | build.yml |
| `actions/download-artifact` | v7, v4 | build.yml, release.yml |
| `dtolnay/rust-toolchain` | stable | build.yml, test.yml |
| `Swatinem/rust-cache` | v2 | build.yml, test.yml |
| `taiki-e/install-action` | v2 | test.yml (cargo-llvm-cov) |
| `marocchino/sticky-pull-request-comment` | v3 | test.yml |
| `softprops/action-gh-release` | v2 | release.yml |

### 7.6 Dependency Issues

| # | Issue | Details |
|---|---|---|
| D1 | **Undefined Maven property** | `pom.xml:197` references `${dependencies-version.version}` but no such property is defined — plugin version will resolve to literal string |
| D2 | **Stale root `Cargo.lock`** | Root-level `Cargo.lock` (1087 lines) exists alongside `rust/Cargo.lock` (941 lines) with different versions (e.g., anyhow 1.0.98 vs 1.0.104). Root-level file should not exist. |
| D3 | **Java version mismatch** | `pom.xml` targets Java 17, `build.yml`/`test.yml` use Java 25, `release.yml` uses Java 17. The `mainClass` in `pom.xml:353` is `io.github.fherbreteau.Main` — a class that doesn't exist (actual: `io.github.fherbreteau.Sample`) |
| D4 | **Rust edition 2024** | `Cargo.toml` uses `edition = "2024"` which requires a very recent Rust toolchain (1.85+). CI pins Rust 1.88.0. |

---

## 8. Additional Findings

### 8.1 Documentation Drift

| # | Issue | Details |
|---|---|---|
| DOC1 | README references `VodozemacAccount` class | `README.md:60,65,89,112,119,124,132` — actual class is `Account` |
| DOC2 | README says "6 test cases" | `README.md:20` — actual count is 46 tests |
| DOC3 | README architecture shows `rust/src/lib.rs` as JNI implementation | `README.md:155` — code has been split into `olm/` and `megolm/` modules |
| DOC4 | README API reference lists `curve25519Key()`, `ed25519Key()`, `sign()` on `VodozemacAccount` | `README.md:118-136` — these are on `Account`, and many more methods exist |
| DOC5 | `Sample.java:52` has wrong log message | Says "Bob: Received message" but this is Alice decrypting Bob's reply |
| DOC6 | `Sample.java:62` has wrong log message | Same issue — says "Bob: Received message" but Alice is receiving |
| DOC7 | `CONTRIBUTING.md:22` references `CODE_OF_CONDUCT.md` which does not exist | |
| DOC8 | Javadoc on `Account.ed25519Key()` says "Curve25519" | `Account.java:50` — `@return a base 64 representation of the public Curve25519 key` should say Ed25519 |
| DOC9 | Javadoc `@InheritDoc` is misspelled | `Account.java:307`, `OlmSession.java:131`, `OutboundGroupSession.java:74`, `InboundGroupSession.java:70` — should be `@inheritDoc` |
| DOC10 | `Account.createInboundSession` Javadoc has typo "recieved" and "sebder" | `Account.java:107` — "recieved from the sebder" should be "received from the sender" |

### 8.2 Code Issues

| # | Issue | Location | Severity |
|---|---|---|---|
| C1 | **Typo in public API**: `createOutbpundSession` should be `createOutboundSession` | `Account.java:95` | High |
| C2 | **Wrong error message**: `checkNotClosed()` says "Account has been closed" in non-Account classes | `OlmSession.java:118`, `OutboundGroupSession.java:62`, `InboundGroupSession.java:57` | Medium |
| C3 | **`OlmSession` constructor is public** — allows constructing with arbitrary pointer | `OlmSession.java:8` | Low |
| C4 | **`OutboundGroupSession.pickle(byte[])` missing key validation** | `OutboundGroupSession.java:40` | Medium |
| C5 | **`InboundGroupSession.pickle(byte[])` missing key validation** | `InboundGroupSession.java:35` | Medium |
| C6 | **`InboundGroupSession.unpickle(String, byte[])` missing key validation** | `InboundGroupSession.java:45` | Medium |
| C7 | **`OutboundGroupSession.unpickle(String, byte[])` missing key validation** | `OutboundGroupSession.java:50` | Medium |
| C8 | **Unused import** in Rust tests | `rust/src/olm/session.rs:177` | Low |
| C9 | **`VodozemacException` only has String constructor** — no cause chaining | `VodozemacException.java` | Low |
| C10 | **`InboundCreationResult` constructor is public** with raw `long sessionPtr` — exposes native pointer | `InboundCreationResult.java:10` | Low |
| C11 | **Javadoc `@link` references wrong method signature** — `Account.fromDehydratedDevice` links to `toDehydratedDevice(String)` but actual signature is `toDehydratedDevice(byte[])` | `Account.java:278` | Low |
| C12 | **`pom.xml` `mainClass` references non-existent class** `io.github.fherbreteau.Main` | `pom.xml:353` | Medium |
| C13 | **`helpers.rs:wrap()` uses `unwrap()`** — panics on wrong key length instead of returning a JNI error | `rust/src/helpers.rs:6` | Medium |
| C14 | **Rust JNI functions use `.unwrap()` on `convert_byte_array`** in several pickle functions — will panic on failure | `account.rs:374,407`, `session.rs:113,147`, `inbound_group_session.rs:115,148`, `outbound_group_session.rs:114,147` | Medium |
| C15 | **`README.md` usage example imports `VodozemacAccount`** which doesn't exist | `README.md:60` | Medium |

---

## 9. Recommendations

### High Priority
1. Fix typo `createOutbpundSession` -> `createOutboundSession` in `Account.java` and all callers
2. Add 32-byte key validation to `OutboundGroupSession` and `InboundGroupSession` pickle/unpickle methods
3. Fix `checkNotClosed()` error messages to reference the correct class name
4. Replace `.unwrap()` calls in Rust JNI pickle functions with proper error handling
5. Fix `pom.xml` `mainClass` to `io.github.fherbreteau.Sample` (or remove if not needed)
6. Remove stale root-level `Cargo.lock`

### Medium Priority
7. Update README to reflect actual API (class names, method names, test count, architecture)
8. Replace fake GPG key in SECURITY.md with real key or remove the section
9. Standardize error messages to English in `NativeLibraryLoader`
10. Fix Javadoc typos (`@InheritDoc` -> `@inheritDoc`, "Curve25519" -> "Ed25519", "recieved" -> "received")
11. Define `${dependencies-version.version}` property in `pom.xml` or remove the plugin entry
12. Align Java version across `pom.xml` (17) and CI workflows (25)

### Low Priority
13. Extract common `NativeHandle` base class to eliminate duplication
14. Make `OlmSession` constructor package-private (only `InboundCreationResult` and `Account` should create sessions)
15. Make `InboundCreationResult` constructor package-private
16. Add `cause` parameter to `VodozemacException`
17. Remove unused import `PICKLE_KEY` in `rust/src/olm/session.rs:177`
18. Create `CODE_OF_CONDUCT.md` or remove reference from `CONTRIBUTING.md`
