# Code Review — vodozemac-java

- **Review date**: 2026-10-15
- **Reviewed revision**: `b45f23c` (branch `docs/Code_Review_and_Implementation_Plan`)
- **Scope**: Java API and tests, Rust JNI bridge and tests, Maven/Cargo configuration and lockfile, workflows, and user documentation.
- **Method**: Static code review plus local lint, test, coverage, audit, demo, and documentation builds. This is a current-tree review; earlier review tasks are not carried forward unless still open.

## Verification Results

| Check | Command | Result |
|---|---|---|
| Java build, Checkstyle, tests, JaCoCo gate | `mvn verify` | **PASS** — build successful; Checkstyle 0 violations; 190 tests, 0 failures/errors/skips; JaCoCo configured coverage checks passed. |
| Java demo programs | `mvn -Psample test` | **PASS** — 190 Java tests and all six demos (Olm, Megolm, SAS, ECIES, backup, cross-signing) completed. |
| Rust formatting | `cargo fmt --manifest-path rust/Cargo.toml -- --check` | **PASS** — clean. |
| Rust lint | `cargo clippy --manifest-path rust/Cargo.toml` | **PASS** — no warnings. |
| Rust tests | `cargo test --manifest-path rust/Cargo.toml` | **PASS** — 130 passed, 0 failed. Includes JVM-attached JNI tests. |
| Rust coverage | `cargo llvm-cov --manifest-path rust/Cargo.toml --summary-only` | **PASS, target gap** — 53.74% lines (58.67% regions), below the repository's documented 80% coverage target. |
| Rust dependency audit | `cargo audit --file rust/Cargo.lock` | **PASS** — scanned 96 dependencies; no advisory findings reported. Run from repository root without `--file` fails because the lockfile is under `rust/`; CI correctly runs it from `rust/`. |
| Site links/build | `python3 -m mkdocs build --strict` (from `site/`) | **PASS** — strict build completed without broken-link errors. |
| Workflow lint | `actionlint` | **WARN** — workflow validation completed but ShellCheck reported SC2012 in `.github/workflows/build.yml` and SC2034 in `.github/workflows/prepare-release.yml`; see CR-7. |

The local Rust coverage result is an aggregate measurement, not a claim that the CI new-code coverage gate failed. The stated repository-wide 80% target is nevertheless not currently met by this measurement and needs explicit resolution.

## Outstanding Work — Incomplete Tasks Only

| ID | Finding | Evidence | Priority |
|---|---|---|---|
| CR-1 | Plaintext is exposed by `toString()` on decrypted-value objects. Logging or debugging these objects can disclose application messages. | `src/main/java/io/github/fherbreteau/vodozemac/megolm/DecryptedMessage.java:68`; `src/main/java/io/github/fherbreteau/vodozemac/olm/InboundCreationResult.java:63`; `src/main/java/io/github/fherbreteau/vodozemac/ecies/InboundCreationResult.java:77` | High |
| CR-2 | `EstablishedEcies.equals()` and `hashCode()` use the mutable native pointer. Closing changes the pointer to zero, changing the hash code and making distinct closed handles equal. | `src/main/java/io/github/fherbreteau/vodozemac/ecies/EstablishedEcies.java:110`; pointer is zeroed by `NativeHandle.close()` at `src/main/java/io/github/fherbreteau/vodozemac/NativeHandle.java:79` | Medium |
| CR-3 | Megolm's Rust `u32` indices are cast to Java `int` (`jint`), which can expose negative values above `Integer.MAX_VALUE`. Resolve the supported range or widen the Java API; do not silently wrap. | `rust/src/megolm/mod.rs:29,47`; `rust/src/megolm/outbound_group_session.rs:59`; `rust/src/megolm/inbound_group_session.rs:69`; Java index accessors use `int`. | Medium |
| CR-4 | The backup persistence example calls nonexistent `PkDecryption.pickle()` and `PkDecryption.unpickle(...)` methods. The available API is legacy pickle/unpickle. | `site/docs/samples/backup.md:42-48`; supported API at `src/main/java/io/github/fherbreteau/vodozemac/backup/PkDecryption.java:133-152` | Medium |
| CR-5 | `AGENTS.md` dependency metadata is stale: it says vodozemac 0.10.0, Checkstyle 14.1.0, and Maven wrapper 3.9.16; current configuration is vodozemac requirement 0.11.0 (lock resolves 0.11.1), Checkstyle 14.3.0, and Maven wrapper 3.10.0. | `AGENTS.md:5,142-146`; `rust/Cargo.toml:12`; `rust/Cargo.lock`; `pom.xml:39`; `.mvn/wrapper/maven-wrapper.properties:3` | Low |
| CR-6 | Rust line coverage is 53.74%, below the repository's documented 80% coverage target. Decide whether to raise coverage or formally reconcile the target with actual enforced gates, then track the result. | `cargo llvm-cov --manifest-path rust/Cargo.toml --summary-only`; stated target in `AGENTS.md:72-89` | Medium |
| CR-7 | Workflow shell scripts have two outstanding ShellCheck findings: unsafe filename handling via `ls`/`head`, and an apparently unused `patch` variable. | `.github/workflows/build.yml:183`; `.github/workflows/prepare-release.yml:59-83` | Low |

## Duplicated Code

| Pattern | Locations | Review assessment |
|---|---|---|
| Result-object equality, hashing, lifecycle delegation, and formatting | `olm/InboundCreationResult.java`, `ecies/InboundCreationResult.java`, `ecies/OutboundCreationResult.java` | Similar value-wrapper boilerplate remains. Their payload and ownership semantics differ, so only consolidate if a shared abstraction makes behavior clearer; this is not currently a correctness defect. |
| Static native-library loading blocks | JNI-backed Java classes | Repeated `NativeLibraryLoader.loadLibrary()` static initialization is conventional and delegates to the same idempotent loader; no change recommended. |

Previously reported Java parameter-name duplication and Rust JNI conversion/session-config duplication have already been consolidated (`ParamNames.java`, `rust/src/types/mod.rs`, and `rust/src/helpers.rs`). Demo programs are already isolated in `src/demos/java` and run under the `sample` Maven profile. These are not open tasks.

## Summary Scorecard

| Category | Score | Basis |
|---|---:|---|
| Lint Compliance | 8/10 | Java Checkstyle, Rust fmt/Clippy pass; workflow ShellCheck has two findings. |
| Code Quality | 7/10 | Core Java/Rust tests pass; mutable-handle equality and integer-boundary behavior remain. |
| Security | 6/10 | Strong CI and JNI controls; decrypted plaintext appears in `toString()`, creating a logging disclosure risk. |
| Maintainability | 7/10 | Clear module layout and shared helpers; wrapper boilerplate and outdated build metadata remain. |
| Documentation | 6/10 | Strict site build passes, but a published example is incompatible with the API and `AGENTS.md` versions drift. |
| Idempotency | 8/10 | Handle close and library loading are idempotent; pointer-based equality/hash behavior is unstable across close. |
| **Overall** | **7/10** | Tests and principal build gates are green; several concrete API/security/docs gaps and a documented coverage-target gap remain. |

## Repository Structure

```text
vodozemac-java/
├── .github/
│   ├── workflows/                 # Build, test, release, pages, security scans
│   ├── scripts/                   # CI helper scripts, including coverage report
│   └── dependabot.yml
├── rust/
│   ├── Cargo.toml / Cargo.lock    # JNI crate manifest and locked dependency graph
│   └── src/                       # JNI exports, helpers, errors, crypto module bindings
├── src/
│   ├── main/java/io/github/...    # Public Java API and NativeHandle lifecycle
│   ├── test/java/io/github/...    # JUnit/AssertJ Java integration tests
│   └── demos/java/io/github/...   # Runnable examples, invoked by Maven sample profile
├── site/                          # MkDocs user guide and runnable sample documentation
├── pom.xml                        # Java build, native build, tests, coverage, Checkstyle
├── checkstyle.xml
├── rust-toolchain.toml
└── README.md, CHANGELOG.md, AGENTS.md, SECURITY.md, CONTRIBUTING.md, etc.
```

The structure separates public Java API, Rust JNI implementation, tests, demos, and user documentation. No repository-layout defect was identified in this review.

## Security Review

### Controls verified

- Java and Rust CodeQL workflows plus Trivy filesystem scanning are configured in `.github/workflows/security.yml`.
- Rust dependency audit is configured and the current `rust/Cargo.lock` audit found no advisories.
- GitHub Actions references inspected are pinned to full commit SHAs.
- JNI calls use panic containment and null-pointer checks; native handle release is centralized and idempotent.
- Native library extraction uses owner-only POSIX permissions where supported; Windows extraction remains user-temp-directory scoped.
- Strict input validation and typed exception mapping are used across the bindings.
- The PK backup algorithm's lack of ciphertext authentication is prominently documented; it remains a compatibility limitation, not a newly discovered defect.

### Open security and boundary concerns

- **CR-1:** decrypted plaintext byte arrays are rendered by `toString()` and may leak through logs.
- Java `byte[]` secret copies cannot be reliably zeroized; this is a JVM/API limitation and remains a residual risk.
- Rust `check_ptr` can reject null pointers but cannot validate arbitrary dangling pointers; safety depends on the JNI ownership/lifecycle invariants.
- **CR-3:** signed Java index conversions can misrepresent the full Megolm `u32` range.

## Dependency Matrix

| Dependency/tool | Declared/current version | Scope and notes |
|---|---|---|
| Java | 25 | Compiler release and CI runtime. |
| Maven wrapper | 3.10.0 | SHA-256 distribution checksum is configured; minimum Maven enforced by the build is 3.6.3. |
| vodozemac | Requirement 0.11.0; lock resolves 0.11.1 | Rust cryptography; enables `libolm-compat`, `experimental-session-config`, and `insecure-pk-encryption`. |
| `jni` | 0.22.4 | Rust JNI runtime and invocation-enabled dev dependency. |
| `serde` | 1.x | Rust serialization. |
| `serde_json` | Requirement 1.0.141; lock resolves 1.0.154 | Rust JSON serialization. |
| JUnit Jupiter | 6.1.3 | Java tests. |
| AssertJ | 3.27.7 | Java test assertions. |
| `mjson` | 1.4.2 | Demo-profile dependency. |
| Maven Compiler / Surefire | 3.16.0 / 3.6.0 | Java compile and test execution. |
| Maven Checkstyle / Checkstyle | 3.6.0 / 14.3.0 | Java style validation. |
| JaCoCo | 0.8.15 | Java coverage and configured build gate. |
| Maven Exec / Dependency / Enforcer | 3.6.4 / 3.11.0 / 3.6.3 | Native build invocation, dependency properties, and toolchain enforcement. |
| Rust toolchain | Stable in `rust-toolchain.toml`; CI test pins 1.88.0 | Includes rustfmt, Clippy, and llvm-cov components. |

Dependency versions above were checked against `pom.xml`, `rust/Cargo.toml`, `rust/Cargo.lock`, the Maven wrapper properties, and CI workflow configuration. Version drift in `AGENTS.md` is tracked as CR-5.
