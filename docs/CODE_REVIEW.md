# Code Review — vodozemac-java

- **Review date**: 2026-11-02
- **Reviewed revision**: `d762027` (branch `docs/Code_Review_and_Implementation_Plan`), rebased on `origin/main`
- **Scope**: Current Java API and tests, Rust JNI bridge and tests, dependency/build configuration, workflows, and user documentation.
- **Method**: Fresh source review plus Maven verification, Java demo execution, Rust formatting/lint/tests/coverage, dependency audit, workflow lint, and strict MkDocs build. CI-only CodeQL and Trivy jobs were not run locally.

## Verification Results

| Check | Command | Result |
|---|---|---|
| Java build, Checkstyle, tests, JaCoCo gate | `./mvnw verify` | **PASS** — 191 tests, 0 failures/errors/skips; Checkstyle 0 violations; configured JaCoCo checks passed. |
| Java tests and sample demos | `./mvnw -Psample test` | **PASS** — 191 tests and all six demos completed. |
| Rust formatting | `cargo fmt --manifest-path rust/Cargo.toml -- --check` | **PASS**. |
| Rust lint (CI command) | `cargo clippy --manifest-path rust/Cargo.toml -- -D warnings` | **PASS** — no warnings. |
| Rust lint (all targets, extra strict check) | `cargo clippy --manifest-path rust/Cargo.toml --all-targets -- -D warnings` | **WARN** — fails on five `needless_borrows_for_generic_args` warnings in Rust test code at `rust/src/olm/account.rs:970,985,1001,1017,1131`; production-target CI lint passes. |
| Rust tests | `cargo test --manifest-path rust/Cargo.toml` | **PASS** — 147 passed, 0 failed; includes JVM-attached JNI tests. |
| Rust coverage | `cargo llvm-cov --manifest-path rust/Cargo.toml --summary-only` | **PASS** — 97.89% line coverage (6,005 of 6,132 lines) and 95.79% function coverage. Above the documented 80% target. |
| Dependency audit | `cargo audit --file rust/Cargo.lock` | **POLICY EXCEPTION** — reports `RUSTSEC-2026-0385` for transitive `ctutils 0.4.2`, a constant-time mask/comparison issue; fixed upstream in `>=0.4.4`. CI explicitly ignores this advisory. `cargo audit --ignore RUSTSEC-2026-0385 --file rust/Cargo.lock` passes. Dependency path: `vodozemac -> digest / curve25519-dalek / poly1305 -> ctutils`. |
| Workflow lint | `actionlint` | **PASS** — no output/findings. |
| Site build and link validation | `mkdocs build --strict` from `site/` using pinned `site/requirements.txt` | **PASS**. |
| Whitespace validation | `git diff --check origin/main...HEAD` | **PASS**. |

Maven emitted non-fatal remote metadata/JGit lock warnings and Java native-access warnings; they did not fail verification. Installing the pinned site requirements into a temporary Python 3.14 virtual environment was necessary because MkDocs was not installed in the system Python; no environment files were added to the repository.

## Outstanding Finding

| ID | Finding | Evidence | Priority |
|---|---|---|---|
| CR-1 | ECIES establishment invalidates the Java `Ecies` handle even when JNI fails before taking ownership of the native allocation. The JNI functions decode/validate input before `Box::from_raw`; failures in those steps return without freeing the allocation, while the Java `finally` calls `invalidate()`, which clears the Cleaner without freeing it. Invalid outbound keys and malformed inbound messages can therefore leak native state/key material. | `src/main/java/io/github/fherbreteau/vodozemac/ecies/Ecies.java:119-123,149-156`; `rust/src/ecies/ecies.rs:70-77,107-113`; `src/main/java/io/github/fherbreteau/vodozemac/NativeHandle.java:87-101` | Medium |

The ECIES API deliberately consumes its handle on both success and failure, as documented. The fix must preserve that contract while ensuring the native allocation is freed on every failure path (including JNI conversion errors before native ownership transfer), and must avoid double-free on success.

## Resolved Prior Findings

The issues recorded in earlier reviews are fixed and are not open work:

- Decrypted message result `toString()` methods redact plaintext.
- `EstablishedEcies` uses stable object identity rather than native-pointer equality/hash behavior.
- Megolm message indices use Java `long` / JNI `jlong` with checked conversions and unsigned-boundary coverage.
- The backup persistence sample uses the supported legacy pickle API.
- Dependency/tool versions in `AGENTS.md` match the source configuration.
- Workflow ShellCheck issues have been fixed; `actionlint` passes.
- Rust coverage is now 97.89% locally, exceeding the documented 80% target.

## Security and Static-Analysis Notes

- CI runs CodeQL for Java and Rust and Trivy for dependency/filesystem scanning; these hosted analyses were not reproducible as local commands in this review.
- The fresh RustSec database reports the transitive `ctutils` advisory above. CI currently suppresses it explicitly in both test and security workflows; retain the exception only while tracking upstream remediation and reassess on dependency updates.
- Java byte-array copies cannot be reliably zeroized by the JVM; secret lifecycle remains an API/runtime limitation.
- JNI pointer safety still relies on the `NativeHandle` ownership and synchronization invariants; arbitrary dangling native pointers cannot be validated by null checks alone.

## Review Scorecard

| Category | Score | Basis |
|---|---:|---|
| Lint compliance | 9/10 | CI-equivalent Java, Rust, formatting, and workflow lint pass; optional all-target Rust lint reports five test-only warnings. |
| Code quality | 8/10 | Comprehensive Java/Rust tests and coverage; one concrete ECIES failure-path ownership leak remains. |
| Security | 7/10 | Strong JNI/resource controls and redacted plaintext formatting; ECIES leak and explicitly suppressed transitive advisory need follow-up. |
| Maintainability | 8/10 | Clear Java/Rust module boundaries, shared lifecycle helpers, and aligned tool metadata. |
| Documentation | 9/10 | Site builds strictly and prior sample/version issues are fixed. |
| **Overall** | **8/10** | All standard build/test/lint gates pass; address the ECIES native ownership leak. |

## Repository Structure

```text
vodozemac-java/
├── .github/workflows/                 # Build, test, release, pages, security scans
├── rust/src/                          # JNI exports, helpers, errors, crypto bindings
├── src/main/java/io/github/...        # Public Java API and native-handle lifecycle
├── src/test/java/io/github/...        # Java integration tests
├── src/demos/java/io/github/...       # Runnable demos (sample Maven profile)
├── site/                              # MkDocs user guide and examples
├── pom.xml                            # Java build, native build, tests, coverage, Checkstyle
├── rust/Cargo.toml / Cargo.lock       # JNI crate and locked dependency graph
└── checkstyle.xml, rust-toolchain.toml
```

No repository-layout issue was identified.
