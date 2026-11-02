# Implementation Plan — vodozemac-java

- **Plan date**: 2026-11-02
- **Source review**: `docs/CODE_REVIEW.md`, revision `d762027`
- **Scope**: One currently open actionable finding from the fresh post-rebase review. Findings resolved in earlier work are intentionally excluded.

## Constraints and Baseline

- The associated review PR currently contains only files under `docs/`. Any implementation change must be delivered separately so the docs-only PR scope remains valid.
- Follow Java 25, Rust/JNI, Checkstyle, and existing native ownership conventions; do not add code comments unless explicitly requested.
- The ECIES API documents consumption of an `Ecies` object on both successful and failed establishment. Preserve this behavior while preventing native leaks and double-free.
- Baseline on 2026-11-02:
  - `./mvnw verify`: passed; 191 Java tests, no failures/errors/skips; Checkstyle and JaCoCo checks passed.
  - `./mvnw -Psample test`: passed; same 191 Java tests and all six demos completed.
  - `cargo fmt --manifest-path rust/Cargo.toml -- --check`: passed.
  - CI-equivalent `cargo clippy --manifest-path rust/Cargo.toml -- -D warnings`: passed.
  - `cargo test --manifest-path rust/Cargo.toml`: 147 passed, 0 failed.
  - `cargo llvm-cov --manifest-path rust/Cargo.toml --summary-only`: 97.89% line coverage, above the documented 80% target.
  - `actionlint`: passed with no findings.
  - `cargo audit --file rust/Cargo.lock`: reports `RUSTSEC-2026-0385` in transitive `ctutils 0.4.2`; fixed in `>=0.4.4` and explicitly ignored by CI. With that exact CI exception, `cargo audit --ignore RUSTSEC-2026-0385 --file rust/Cargo.lock` passes.
  - `mkdocs build --strict` from `site/`: passed using the pinned site requirements.
  - Optional stricter `cargo clippy --all-targets -- -D warnings`: reports five test-only needless-borrow warnings in `rust/src/olm/account.rs`; the CI lint command is clean.

## Priority 1 — Free ECIES Native State on Pre-Transfer Failures (CR-1)

**Objective:** Ensure ECIES channel-establishment failures do not orphan the native `Ecies` allocation or its key material when Java consumes the handle.

**Files:**
- `src/main/java/io/github/fherbreteau/vodozemac/ecies/Ecies.java`
- `rust/src/ecies/ecies.rs`
- `src/test/java/io/github/fherbreteau/vodozemac/ecies/EciesTest.java`
- `rust/src/ecies/ecies.rs` tests

**Current behavior:** `establishOutboundChannel` and `establishInboundChannel` always call Java `invalidate()` in `finally`. In the JNI functions, input conversion/decoding occurs before `Box::from_raw(ptr)`. If validation or conversion returns an error first, native ownership was never consumed or freed, but Java clears its pointer and Cleaner registration. The resource becomes unreachable and leaks. Existing tests check that the Java wrapper is consumed after failure but do not assert native cleanup.

**Tasks:**
1. Map the full native ownership lifecycle for both outbound and inbound JNI functions, including JNI conversion failures, invalid outbound public keys, malformed inbound initial messages, vodozemac establishment errors, Java object construction failures, and successful ownership transfer.
2. Select an ownership-safe implementation that preserves the public consume-on-call contract. Suitable approaches include taking ownership immediately after validating the pointer and using RAII so every later `?`/error path frees the object, or explicitly freeing on every failure before ownership is taken. Avoid any path where Java invalidates while Rust retains an unreachable allocation.
3. Ensure successful establishment transfers the resulting `EstablishedEcies` allocation to the Java result exactly once and frees the original `Ecies` allocation exactly once.
4. Add regression tests for invalid outbound key and malformed inbound message that verify the native allocation is released (use a test-only drop counter or an equivalent deterministic Rust ownership test; do not rely on process RSS or Cleaner timing).
5. Keep Java tests asserting the documented consumed/closed state after both success and failure, and verify no use-after-free or double-free occurs.
6. Run CI-equivalent Java/Rust tests and lints, Rust coverage, and strict site build. Add a JVM-attached Rust test for any new JNI branch/export following the established `get_jvm()`/`EnvUnowned` pattern.

**Acceptance criteria:** all consumed ECIES handles are released on every outcome; successful transfer remains valid; no double-free, leak, or use-after-free; tests deterministically cover cleanup paths; all project gates remain green.

**Estimate:** M.

## Separate Maintenance Follow-ups

These are observations from validation, not open code-review findings; handle independently if desired:

- **RustSec exception:** track upstream availability of `ctutils >=0.4.4` through the dependency graph, then update the Rust lockfile and remove `RUSTSEC-2026-0385` from CI audit ignore lists only after verifying the resolved graph. Do not make an unreviewed direct transitive pin solely to silence the advisory.
- **Test-target lint:** optional all-target lint identified five needless borrows in `rust/src/olm/account.rs:970,985,1001,1017,1131`; standard CI Clippy passes. These can be cleaned up in a separate Rust hygiene change if appropriate.

## Suggested Delivery Order

| Order | Item | Rationale |
|---:|---|---|
| 1 | CR-1 ECIES ownership cleanup | Correctness/resource safety issue; changes must be guarded by deterministic native ownership tests. |
| 2 | RustSec dependency update | Separate when an upstream-compatible patched graph is available; remove CI suppression only after audit passes without ignore. |
| 3 | Optional test-target lint cleanup | Low-risk hygiene; separate from the ownership fix unless naturally touched. |

## Final Verification Checklist

- [ ] `./mvnw verify` — Java tests, Checkstyle, native build, and JaCoCo checks pass.
- [ ] `./mvnw -Psample test` — demos pass if API/sample code is affected.
- [ ] `cargo fmt --manifest-path rust/Cargo.toml -- --check` — clean.
- [ ] `cargo clippy --manifest-path rust/Cargo.toml -- -D warnings` — CI lint clean.
- [ ] `cargo test --manifest-path rust/Cargo.toml` — all tests pass, including JNI ownership/error-path tests.
- [ ] `cargo llvm-cov --manifest-path rust/Cargo.toml --cobertura --output-path target/coverage/rust-coverage.xml` — report generated; coverage remains above 80%.
- [ ] `cargo audit --file rust/Cargo.lock` — no unreviewed advisories; document any still-active upstream exception.
- [ ] `actionlint` — no workflow or ShellCheck errors.
- [ ] `mkdocs build --strict` from `site/` — no broken links or documentation errors.
- [ ] `git diff --check` — no whitespace errors; implementation PR includes no `docs/` changes unless it separately satisfies the repository’s docs-only PR constraint.
