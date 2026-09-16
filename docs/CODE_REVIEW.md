# Code Review — vodozemac-java

- **Date**: 2026-09-07
- **Scope reviewed**: `fix/More_Code_Improvement` @ `54511a6` (PR #47) — the most complete state of the codebase at review time
- **Reviewer**: Mammouth Code (automated full review)

## 1. Verification Results

All commands were executed against the reviewed commit.

| Check | Command | Result |
|---|---|---|
| Rust lint | `cargo clippy --all-targets -- -D warnings` | ✅ 0 warnings |
| Rust format | `cargo fmt -- --check` | ✅ clean |
| Rust tests | `cargo test` | ✅ 110 passed, 0 failed |
| Rust coverage | `cargo llvm-cov --cobertura` | ⚠️ 36.9% lines (1241/3363) — JNI entry points are exercised by Java integration tests and not attributed by llvm-cov (known caveat, also noted in the CI coverage report) |
| Java lint | `mvn checkstyle:check` | ✅ 0 violations |
| Java tests | `mvn verify` | ✅ 167 passed, 0 failed; JaCoCo coverage checks met |
| Java coverage (JaCoCo) | JaCoCo XML | ✅ Lines 100% (676/676), Methods 100% (276/276), Instructions 100%; Branches 97.1% (101/104 — 3 unreachable-by-design branches, see R2) |
| Workflow lint | `actionlint` | ✅ clean |

## 2. Open Items (tasks not completed)

Only unfinished work is listed here. Completed phases of the 2026-08-27 review are not repeated.

| ID | Item | Status | Priority |
|---|---|---|---|
| R1 | `pickleLegacy()` export for `OlmSession`, `InboundGroupSession`, `OutboundGroupSession` (decryption side `unpickleLegacy()` exists; PkDecryption already has `pickleLegacy()`) | 🔴 Blocked on upstream vodozemac exposing legacy pickle for session types | Low |
| R2 | `equals`/`hashCode` of `olm.InboundCreationResult`, `ecies.InboundCreationResult`, `ecies.OutboundCreationResult` compare the native handle first, making the data-comparison branch unreachable for distinct objects (native handles always differ) and untestable for equal handles without a double-free risk. Leaves Java branch coverage at 97.1% | 🟡 Design decision needed — see plan Phase A | High |
| R3 | Rust duplication: `to_java_curve25519` / `to_java_ed25519` / `to_java_signature` (`rust/src/types/mod.rs:90-122`) and `olm_session_config_from_version` / `megolm_session_config_from_version` (`rust/src/helpers.rs:21-45`) | 🟡 Open — see plan Phase C | Medium |
| R4 | Java duplication: `PICKLE_DATA` constant repeated in 4 classes, `INPUT`/`INFO` in `EstablishedSas` (introduced by the S1192 fix) | 🟡 Open — see plan Phase B | Medium |
| R5 | `SampleOlm`, `SampleMegolm`, `SampleSas`, `SampleEcies` are `main()`-based demos in the test sourceset with no `@Test`; they are compiled on every build but never executed by CI | 🟡 Open — see plan Phase D | Low |
| R6 | `AGENTS.md` tool versions stale (Checkstyle 14.0.0 vs actual 14.1.0; JUnit listed without minor version vs 6.1.3) | 🟡 Open — normal PR (root file) | Low |
| R7 | `docs/` working documents absent from `main` (only `.gitkeep`) | ✅ Resolved by this PR | High |
| R8 | Rust coverage is not analyzed by SonarCloud (Rust analysis requires a paid plan) | ⚪ Accepted — compensated by cargo-audit, clippy and llvm-cov in CI; revisit if Sonar pricing changes | Low |

## 3. Duplicated Code

### Java

| Duplicate | Location | Notes |
|---|---|---|
| `PICKLE_DATA` constant | `Account.java`, `OlmSession.java`, `InboundGroupSession.java`, `OutboundGroupSession.java` | Same `"pickleData"` message constant declared 4×; centralise in a shared package-private constants holder (R4) |
| `INPUT` / `INFO` constants | `EstablishedSas.java` | Same pattern as above |
| Pickle/unpickle method trio | Account, OlmSession, InboundGroupSession, OutboundGroupSession | `unpickle(String)` → `unpickle(String, byte[])` → `unpickleLegacy(String, byte[])` follow an identical validate → requireNonNull → native → wrap template; acceptable structure, consider a shared helper if a 4th variant appears |
| Static loader block | 15 classes | `static { NativeLibraryLoader.loadLibrary(); }` — required Java idiom for JNI classes; loader itself is idempotent |
| Result classes | `olm.InboundCreationResult` ≈ `ecies.InboundCreationResult` ≈ `ecies.OutboundCreationResult` | Near-identical (native handle + payload + equals/hashCode/toString/close); acceptable without generics |

### Rust

| Duplicate | Location | Notes |
|---|---|---|
| `to_java_curve25519` / `to_java_ed25519` / `to_java_signature` | `rust/src/types/mod.rs:90-122` | Identical bodies except class path and input type; a generic over a base64-encoding trait collapses the three (R3) |
| `olm_session_config_from_version` / `megolm_session_config_from_version` | `rust/src/helpers.rs:21-45` | Identical `match` blocks (version 1/2/invalid) over two crate types; a macro or shared trait collapses the pair (R3) |

### Already deduplicated (verified, no action)

- `KeyValidator.validateEncryptionKey` centralises 32-byte key checks (used by Account + all session classes)
- `errors.rs` throw helpers generated via macro; `to_java_check_code` shared by ECIES module
- `catch_panic`, `check_ptr`, `RawBox`, `native_free` shared JNI safety helpers

## 4. Summary Scorecard

| Category | Score | Rationale |
|---|---|---|
| Lint Compliance | 10/10 | clippy `-D warnings` clean, `cargo fmt` clean, Checkstyle 0 violations, `actionlint` clean |
| Code Quality | 9/10 | 277 tests green across two languages, 100% Java line/method/instruction coverage, typed APIs, zero TODO/FIXME; −1 for the unreachable `equals` branches (R2) |
| Security | 9/10 | SHA-pinned actions, CodeQL/Trivy/cargo-audit, panic-guarded FFI, owner-only temp permissions, wrapper checksum verification; −1 for accepted residual risks (insecure-pk-encryption feature, non-zeroizable JVM key arrays) |
| Maintainability | 8/10 | Clear module layout, centralised validation and JNI helpers; −2 for remaining Rust/Java duplication (R3, R4) |
| Documentation | 9/10 | README/CHANGELOG/CONTRIBUTING recently aligned with code and verified accurate; Javadoc thorough; −1 for stale `AGENTS.md` versions (R6) |
| Idempotency | 9/10 | `close()` idempotent on all native handles, `loadLibrary()` idempotent (synchronized + volatile), CI concurrency groups with cancel-in-progress, single-use semantics (`Sas.diffieHellman`, `Ecies.establish*`) documented; workflows re-runnable |
| **Overall** | **9/10** | Production-ready; remaining items are hygiene and upstream-blocked work |

## 5. Repository Structure

```
vodozemac-java/
├── .github/
│   ├── workflows/
│   │   ├── build.yml        # 6-platform native build + Maven package + Sonar
│   │   ├── test.yml         # PR quick tests: Rust+Java tests, clippy, fmt, audit, coverage summary
│   │   ├── release.yml      # Release pipeline (fat JAR with all 6 natives)
│   │   └── security.yml     # CodeQL (Java) + Trivy dependency scanning
│   ├── scripts/
│   │   └── coverage-report.py   # Combined Rust (cobertura) + Java (JaCoCo) markdown for $GITHUB_STEP_SUMMARY
│   └── CODEOWNERS
├── rust/                    # Rust JNI bindings (cdylib + rlib)
│   ├── Cargo.toml           # vodozemac 0.10.0, jni 0.22.4, [profile.release] lto/codegen-units/strip
│   └── src/
│       ├── lib.rs           # Module declarations
│       ├── classes.rs       # Centralised JNI class-path constants
│       ├── errors.rs        # Macro-generated throw_* error mapping
│       ├── helpers.rs       # catch_panic, check_ptr, RawBox, native_free, session-config helpers
│       ├── utils/mod.rs     # Vodozemac base64/version JNI
│       ├── olm/             # account.rs, session.rs
│       ├── megolm/          # inbound_group_session.rs, outbound_group_session.rs, message.rs
│       ├── sas/             # sas.rs, established_sas.rs
│       ├── ecies/           # ecies.rs, established_ecies.rs
│       ├── types/mod.rs     # Ed25519/Curve25519/signature JNI + to_java_* helpers
│       └── backup/          # encryption.rs, decryption.rs
├── src/main/java/io/github/fherbreteau/vodozemac/
│   ├── account/             # Account, IdentityKeys, OneTimeKeyGenerationResult, DehydratedDeviceResult
│   ├── olm/                 # OlmSession, OlmSessionVersion, OlmMessage, MessageType, SessionKeys, InboundCreationResult
│   ├── megolm/              # InboundGroupSession, OutboundGroupSession, MegolmSessionVersion, MegolmMessage, SessionOrdering, DecryptedMessage
│   ├── sas/                 # Sas, EstablishedSas, SasBytes
│   ├── ecies/               # Ecies, EstablishedEcies, CheckCode, OutboundCreationResult, InboundCreationResult
│   ├── backup/              # PkEncryption, PkDecryption, PkMessage
│   ├── types/               # Ed25519PublicKey, Ed25519Signature, Curve25519PublicKey
│   ├── exception/           # VodozemacException + 9 typed subclasses
│   ├── NativeHandle.java    # Native pointer lifecycle (idempotent close)
│   ├── SessionVersion.java  # Shared fromVersion/fromValue lookup
│   ├── KeyValidator.java    # 32-byte pickle key validation
│   ├── Vodozemac.java       # Base64 + crate version
│   └── NativeLibraryLoader.java
├── src/test/java/           # 167 tests (Account, Olm, Megolm, SAS, ECIES, backup, types, exceptions, NativeHandle, samples)
├── docs/                    # Working documents (this file, IMPLEMENTATION_PLAN.md)
├── pom.xml                  # Maven (Java 25, enforcer ≥3.6.3, JaCoCo ≥80%, Checkstyle)
├── checkstyle.xml
├── rust-toolchain.toml      # stable + clippy/rustfmt/llvm-tools-preview
├── AGENTS.md / README.md / CHANGELOG.md / CONTRIBUTING.md / SECURITY.md / CODE_OF_CONDUCT.md
└── LICENSE, LICENSES/
```

## 6. Security Review

### Controls in place (verified)

| Control | Implementation |
|---|---|
| Supply chain (CI actions) | All third-party actions pinned to full commit SHAs with version comments |
| Dependency scanning | CodeQL (Java), Trivy (dependency scanning), `cargo audit` (Rust advisory DB) |
| Maven wrapper | `distributionSha256Sum` checksum verification in `maven-wrapper.properties` |
| FFI panic containment | Every JNI entry point wrapped in `catch_panic` (`rust/src/helpers.rs`); `native_free` panic-guarded; JVM aborts prevented by not using `panic = "abort"` (deliberate, documented deviation) |
| Pointer safety | `check_ptr` null/dangling validation before use; `RawBox` RAII guard replacing `mem::forget` patterns |
| Native library loading | Extracted to per-process temp dir with POSIX `rwx------` owner-only permissions; `deleteOnExit` cleanup; platform-specific resource fallback |
| Input validation | `Objects.requireNonNull` on all public Java APIs; 32-byte key validation centralised in `KeyValidator`; Rust-side `u32::try_from` bounds on export/advance indices |
| Secrets | No secrets in repository; workflows use `GITHUB_TOKEN` only |
| Build hardening | `[profile.release]` with `lto`, `codegen-units = 1`, `strip`; reproducible builds via git commit timestamp |
| Code quality gate | SonarCloud quality gate (reliability/security/maintainability A on new code, ≥80% new coverage); JaCoCo ≥80% instructions + 0 missed classes/methods |

### Residual risks (accepted, documented)

| Risk | Mitigation |
|---|---|
| `insecure-pk-encryption` vodozemac feature: MAC does not authenticate ciphertext (`m.megolm_backup.v1.curve25519-aes-sha2` algorithm flaw) | Explicitly documented in README PkEncryption section; required for Matrix key-backup compatibility |
| `experimental-session-config` and `libolm-compat` features | Required for session version negotiation and libolm migration; tracked upstream |
| JVM `byte[]` secret keys cannot be zeroised after use | Native side (vodozemac) zeroises internally; Java-side copies unavoidable withoutUnsafe/foreign-memory APIs; documented accepted risk |
| 70 `unsafe` blocks (JNI inherent) | Contained by `catch_panic`, `check_ptr`, `RawBox`; clippy clean; documented safety invariants |
| Windows native extraction without POSIX permissions | Platform limitation (no POSIX ACLs); files land in user-scoped temp dir |
| Rust not analysed by SonarCloud (paid language) | Compensated by clippy `-D warnings`, cargo-audit, llvm-cov in CI (R8) |

## 7. Dependency Matrix

### Runtime / build dependencies

| Dependency | Version | Scope | Purpose | Notes |
|---|---|---|---|---|
| vodozemac (crate) | 0.10.0 | Rust runtime | Core crypto (Olm/Megolm/SAS/ECIES/PK) | Features: `libolm-compat`, `experimental-session-config`, `insecure-pk-encryption` |
| jni (crate) | 0.22.4 | Rust runtime + dev (`invocation`) | JNI bridge | |
| serde / serde_json | 1 / 1.0.141 | Rust runtime | `OlmMessage` JSON wire format | |
| JUnit Jupiter | 6.1.3 | Java test | Test framework | |
| AssertJ | 3.27.7 | Java test | Fluent assertions | |

### Maven plugins

| Plugin | Version |
|---|---|
| maven-compiler-plugin | 3.16.0 |
| maven-surefire-plugin | 3.5.6 |
| maven-jar-plugin | 3.5.1 |
| maven-clean-plugin | 3.5.0 |
| maven-resources-plugin | 3.5.0 |
| maven-exec-plugin | 3.6.3 |
| maven-dependency-plugin | 3.11.0 |
| maven-enforcer-plugin | 3.6.3 (requires Maven ≥ 3.6.3) |
| jacoco-maven-plugin | 0.8.15 |
| checkstyle plugin / checkstyle | 3.6.0 / 14.1.0 |
| sonar-maven-plugin | 5.7.0.6970 |
| git-commit-id-plugin | 10.0.0 |
| versions-plugin | 2.21.0 |

### Toolchains

| Tool | Version | Source |
|---|---|---|
| Java | 25 | `pom.xml` (`maven.compiler.release`), CI `setup-java` (temurin) |
| Maven | 3.9.16 | Wrapper (`maven-wrapper.properties`, SHA-256 verified) |
| Rust | stable | `rust-toolchain.toml` (clippy, rustfmt, llvm-tools-preview) |

### GitHub Actions (SHA-pinned)

| Action | Pin | Version |
|---|---|---|
| actions/checkout | `3d3c42e…` | v7 |
| actions/setup-java | `dd06d9c…` | v6.0.0 |
| actions/upload-artifact | `043fb46…` | v7 |
| actions/download-artifact | `3e5f45b…` | v8 |
| dtolnay/rust-toolchain | `4360b52…` | v7 |
| taiki-e/install-action | `ba47c86…` / `1ed6d7b…` | v2 |
| Swatinem/rust-cache | `6323deb…` | v2 |
| github/codeql-action (init/analyze/upload-sarif) | `cdf488f…` | v4 |
| aquasecurity/trivy-action | `ed142fd…` | v0.36.0 |
| softprops/action-gh-release | `efb3536…` | v2 |
