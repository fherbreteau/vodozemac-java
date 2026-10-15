# Implementation Plan — vodozemac-java

- **Plan date**: 2026-10-15
- **Source review**: `docs/CODE_REVIEW.md`, revision `b45f23c`
- **Scope**: all incomplete actionable findings CR-1 through CR-7. Previously completed work is deliberately excluded.

## Constraints and Baseline

- The working review PR contains only `docs/` files; implementation work must be proposed in separate PR(s) that respect the repository's docs-only constraint.
- Maintain existing Java 25, Rust/JNI, Checkstyle, and API conventions; do not add code comments unless requested.
- Security fixes that change observable API output require release-note and API/site documentation updates.
- Baseline verification on 2026-10-15:
  - `mvn verify`: success; 190 Java tests pass, Checkstyle clean, JaCoCo configured gate passes.
  - `mvn -Psample test`: success; all six demos execute.
  - `cargo fmt --manifest-path rust/Cargo.toml -- --check`, `cargo clippy --manifest-path rust/Cargo.toml`, and `cargo test --manifest-path rust/Cargo.toml`: success; 130 Rust tests pass.
  - `cargo llvm-cov --manifest-path rust/Cargo.toml --summary-only`: 53.74% line coverage, below the documented 80% target.
  - `cargo audit --file rust/Cargo.lock`: no advisories reported.
  - `python3 -m mkdocs build --strict` from `site/`: success.
  - `actionlint`: two ShellCheck findings (CR-7).

## Priority 1 — Stop Plaintext Disclosure in `toString()` (CR-1)

**Objective:** Ensure accidental object logging does not emit decrypted message contents.

**Files:**
- `src/main/java/io/github/fherbreteau/vodozemac/megolm/DecryptedMessage.java`
- `src/main/java/io/github/fherbreteau/vodozemac/olm/InboundCreationResult.java`
- `src/main/java/io/github/fherbreteau/vodozemac/ecies/InboundCreationResult.java`
- Corresponding Java tests and public API documentation where behavior is described.

**Tasks:**
1. Remove plaintext bytes from `toString()`; either omit the field or replace it with an explicit fixed redaction marker. Do not include plaintext length if the project considers length sensitive.
2. Preserve non-sensitive fields such as message index where appropriate; avoid calling nested native-backed `toString()` unless its output is independently safe.
3. Update tests to assert decrypted bytes remain available through `plaintext()` but are absent from the formatted object representation.
4. Add regression tests with a distinctive plaintext token and assert that token/byte sequence is not present in the string.
5. Update Javadocs/site API documentation only if they make guarantees about `toString()`; document the redaction behavior.

**Acceptance criteria:** no decrypted plaintext is formatted by these classes; regression tests pass; `mvn verify` and strict site build pass.

**Estimate:** S.

## Priority 2 — Stabilize `EstablishedEcies` Equality (CR-2)

**Objective:** Ensure equality and hash behavior do not change when native resources are closed.

**File:** `src/main/java/io/github/fherbreteau/vodozemac/ecies/EstablishedEcies.java`; tests in `src/test/java/io/github/fherbreteau/vodozemac/ecies/EciesTest.java`.

**Tasks:**
1. Decide whether `EstablishedEcies` represents identity or a value. The recommended option is identity semantics: remove its `equals()` and `hashCode()` overrides so Java's stable object identity applies, consistent with native handles that have no immutable comparable payload.
2. If API compatibility requires custom semantics, introduce a stable immutable identity captured at construction and never derived from the mutable native pointer; document the choice.
3. Add tests for two distinct instances, self equality, and behavior before and after `close()`.
4. Where custom `equals` remains, verify equal objects maintain equal hash codes and that a hash-based collection still supports lookup after close.

**Acceptance criteria:** closing a handle cannot mutate its hash identity or cause unrelated closed handles to compare equal; tests and `mvn verify` pass.

**Estimate:** S.

## Priority 3 — Define Megolm Index Range and Conversion (CR-3)

**Objective:** Avoid silent `u32` to signed `int` wraparound across JNI.

**Affected API and JNI files:**
- `src/main/java/io/github/fherbreteau/vodozemac/megolm/MegolmMessage.java`
- `src/main/java/io/github/fherbreteau/vodozemac/megolm/DecryptedMessage.java`
- `src/main/java/io/github/fherbreteau/vodozemac/megolm/InboundGroupSession.java`
- `src/main/java/io/github/fherbreteau/vodozemac/megolm/OutboundGroupSession.java`
- `rust/src/megolm/mod.rs`
- `rust/src/megolm/message.rs`, `rust/src/megolm/inbound_group_session.rs`, and `rust/src/megolm/outbound_group_session.rs` at every message-index conversion.

**Tasks:**
1. Inventory all message-index and first-known-index crossings and distinguish values that originate in `u32` from Java parameters that are currently `int`.
2. Select and document one policy:
   - **Recommended, full-range support:** expose these public indices as Java `long`, use JNI `jlong`, and perform checked conversion to/from Rust `u32`; or
   - **Compatibility-preserving bound:** retain Java `int`, explicitly define supported indices as nonnegative signed-int range, and fail explicitly rather than wrapping whenever an index cannot be represented.
3. Assess the impact of changing public signatures, module/API compatibility, and generated/JNI constructor signatures before implementation; update all call sites and user documentation if widening.
4. Add conversion tests at `0`, `Integer.MAX_VALUE`, `Integer.MAX_VALUE + 1`, and `u32::MAX` as appropriate to the selected policy. Test Java-facing accessors and JNI result construction, not only pure helper functions.
5. Ensure no unchecked `as jint` conversions remain for Megolm indices.

**Acceptance criteria:** conversions have explicit, tested behavior at signed and unsigned boundaries; Java and JNI types agree; `mvn verify`, Rust tests, and strict site build pass.

**Estimate:** M; widening public API may require a major/minor compatibility decision.

## Priority 4 — Repair Backup Persistence Documentation (CR-4)

**Files:** `site/docs/samples/backup.md`; optionally `src/demos/java/.../SampleBackup.java` if a runnable persistence example is added.

**Tasks:**
1. Replace unsupported `pickle()`/`unpickle()` references with the currently supported `pickleLegacy(byte[])` and `unpickleLegacy(String, byte[])` calls, or remove the unsupported persistence example and clearly state the API limitation.
2. Confirm the code sample is syntactically valid Java and uses the library's actual method names/signatures.
3. Add or adapt a demo/test to exercise pickle and restore with a 32-byte key if demonstrating persistence is intended.
4. Run the sample profile and `python3 -m mkdocs build --strict`.

**Acceptance criteria:** no docs sample references nonexistent methods; persistence behavior is accurately described; all demos and strict site build pass.

**Estimate:** S.

## Priority 5 — Synchronize `AGENTS.md` Dependency Metadata (CR-5)

**File:** `AGENTS.md`.

**Tasks:**
1. Update project overview and dependency list based on source-of-truth configuration: vodozemac declared requirement 0.11.0 (lockfile 0.11.1), Checkstyle 14.3.0, Maven wrapper 3.10.0, and current Maven plugin versions where listed.
2. Clarify declared version requirements versus lockfile-resolved versions; do not present a floating Rust stable toolchain as an exact pinned release.
3. Reconcile JUnit, AssertJ, Java, JNI, and Maven minimum versions against the actual manifests/wrapper/enforcer.
4. Add a maintenance check to the existing dependency update workflow or review checklist so future upgrades update `AGENTS.md` in the same change.

**Acceptance criteria:** every listed version agrees with its configuration source and version semantics are explicit; source-controlled working tree remains limited to allowed files in its implementation PR.

**Estimate:** S.

## Priority 6 — Address Rust Coverage Target Gap (CR-6)

**Objective:** Reconcile measured Rust line coverage of 53.74% with the documented minimum of 80%.

**Tasks:**
1. Produce a stable CI-equivalent report (`cargo llvm-cov --manifest-path rust/Cargo.toml --lcov --output-path target/rust-lcov.info`, or the CI Cobertura report) and confirm which code is excluded, particularly JNI exports and module glue.
2. Separate missed lines that can be exercised through current JVM-attached tests from unreachable/generated/platform-specific lines. Do not classify all JNI lines as invisible without checking CI's JVM-attached test process.
3. Add tests for uncovered feasible error paths, conversions, and JNI exports, using the established `get_jvm()`/`EnvUnowned` testing pattern. Avoid coverage-only tests that compromise ownership or test correctness.
4. If an 80% aggregate target is infeasible for sound architectural reasons, propose a measurable, enforced denominator/exclusion strategy and update the stated policy only with project-owner agreement; do not silently lower or waive the gate.
5. Verify coverage in CI-equivalent conditions and record the measured figure and gate configuration.

**Acceptance criteria:** enforced Rust coverage policy and measured coverage agree; any new JNI export has JVM-attached coverage; tests, fmt, Clippy, and JaCoCo remain green.

**Estimate:** L; likely multiple focused test PRs.

## Priority 7 — Resolve Workflow ShellCheck Findings (CR-7)

**Files:** `.github/workflows/build.yml`, `.github/workflows/prepare-release.yml`.

**Tasks:**
1. Replace `JAR=$(ls target/vodozemac-java-*.jar | head -1)` with robust, quoted selection that fails if zero or multiple unintended jars are present; avoid word-splitting and filename ambiguity.
2. Resolve the unused `patch` warning in the release script by removing it if unused, or using/exporting it if the script intentionally relies on it.
3. Run `actionlint` and validate behavior of the fat-JAR smoke test and release validation in a safe workflow/test context.

**Acceptance criteria:** `actionlint` reports no workflow or ShellCheck errors; smoke-test artifact selection is deterministic; release script behavior remains unchanged except removing dead variable code.

**Estimate:** S.

## Optional Maintainability Follow-up — Result Wrapper Boilerplate

This is a low-priority duplication observation, not a verified defect. Do not refactor as part of the security/correctness fixes unless a small, type-safe abstraction clearly improves clarity without obscuring distinct ownership and plaintext semantics. If pursued, add focused equality/lifecycle tests for all three result wrappers and confirm resource ownership is unchanged.

## Suggested Delivery Order

| Order | Item | Rationale/dependency |
|---:|---|---|
| 1 | CR-1 plaintext-safe formatting | Security improvement; isolated and low-risk. |
| 2 | CR-2 stable equality/hash behavior | Correctness fix with small API surface. |
| 3 | CR-4 backup documentation | Small independently verifiable documentation correction. |
| 4 | CR-7 shell lint | Small CI hygiene change. |
| 5 | CR-5 dependency metadata | Documentation-only source synchronization. |
| 6 | CR-3 Megolm index policy | Requires API compatibility decision and coordinated Java/JNI changes. |
| 7 | CR-6 Rust coverage | Larger work requiring a measurable, agreed coverage target and dedicated tests. |

CR-1 through CR-5 and CR-7 can be delivered as separate focused changes; CR-3 requires an API decision before code changes. CR-6 should be split into test-focused increments after an agreed coverage baseline is established.

## Final Verification Checklist

- [ ] `mvn verify` — Checkstyle and JaCoCo gates pass; all Java tests pass.
- [ ] `mvn -Psample test` — all six demos pass after changes affecting examples/API.
- [ ] `cargo fmt --manifest-path rust/Cargo.toml -- --check` — clean.
- [ ] `cargo clippy --manifest-path rust/Cargo.toml -- -D warnings` — no warnings.
- [ ] `cargo test --manifest-path rust/Cargo.toml` — all tests pass, including affected JVM-attached JNI tests.
- [ ] `cargo llvm-cov` report records Rust coverage and meets the agreed enforced policy.
- [ ] `cargo audit` run from `rust/` or with `--file rust/Cargo.lock` — no unreviewed advisories.
- [ ] `actionlint` — no ShellCheck/workflow errors.
- [ ] `python3 -m mkdocs build --strict` from `site/` — no broken links or invalid generated documentation.
- [ ] User-facing API or security behavior changes are reflected in canonical site documentation and changelog in their implementation PR.
