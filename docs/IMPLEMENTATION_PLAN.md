# Implementation Plan — vodozemac-java

- **Date**: 2026-09-07
- **Derived from**: `docs/CODE_REVIEW.md` (review of `fix/More_Code_Improvement` @ `54511a6`, PR #47)
- **Scope**: every open item (R1-R8) identified by the 2026-09-07 code review

## Guiding constraints

- CI must stay green: `mvn verify` (167+ Java tests, Checkstyle 0 violations, JaCoCo ≥80% instructions / 0 missed classes), `cargo test` (110 tests), `cargo clippy -- -D warnings`, `cargo fmt -- --check`.
- No comments in code unless requested; `@author François HERBRETEAU` on new classes; Checkstyle `FinalClass` rule respected.
- `docs/` changes ship in a dedicated docs-only PR (per `AGENTS.md`).

## Phase A — Fix result-class `equals` (R2, High)

**Problem**: the three creation-result classes compare the native handle first. Distinct objects always have different handles, so the data comparison is unreachable; the "equal handle, different data" branch cannot be covered without a double-free. Java branch coverage is 97.1% (101/104).

**Tasks**

1. In `olm/InboundCreationResult`, `ecies/InboundCreationResult`, `ecies/OutboundCreationResult`:
   - Revert `equals` to compare payload data only (`plaintext` / `initialMessage`), as before the handle comparison was introduced.
   - Keep `hashCode` on the payload only (drop the handle component).
   - Keep `toString` unchanged.
2. Update the affected tests (`OlmSessionTest`, `EciesTest`) to assert two distinct results with equal payloads are `equalTo` each other and have identical hash codes, in addition to the existing distinct-result assertions.
3. Verify JaCoCo reports 100% branch coverage for these classes and overall Java branches at 100%.

**Acceptance criteria**: `mvn verify` green; JaCoCo branches 104/104; `docs/CODE_REVIEW.md` updated.

**Estimate**: S (half a session).

## Phase B — Centralise Java parameter-name constants (R4, Medium)

**Problem**: the S1192 fix introduced per-class `private static final String PICKLE_DATA = "pickleData"` in 4 classes plus `INPUT`/`INFO` in `EstablishedSas` — the literal duplication moved, not disappeared.

**Tasks**

1. Add a package-private final constants holder, e.g. `ParamNames.java` at the bindings root (or per-package holders if coupling is a concern), containing `PICKLE_DATA`, `PICKLE_KEY`, `KEY`, `INPUT`, `INFO`, `MAC`, `MESSAGE`, `PLAINTEXT`.
2. Replace the per-class constants in `Account`, `OlmSession`, `InboundGroupSession`, `OutboundGroupSession`, `EstablishedSas` (and standardise the remaining inline literals such as `"key"`, `"mac"` while there).
3. Keep messages identical so test assertions on exception messages stay valid.

**Acceptance criteria**: no duplicated string literals across classes; `mvn checkstyle:check` clean; `mvn verify` green.

**Estimate**: S.

## Phase C — Deduplicate Rust JNI helpers (R3, Medium)

**Problem**: `to_java_curve25519` / `to_java_ed25519` / `to_java_signature` (`rust/src/types/mod.rs:90-122`) share identical bodies; `olm_session_config_from_version` / `megolm_session_config_from_version` (`rust/src/helpers.rs:21-45`) duplicate the same version `match`.

**Tasks**

1. Introduce a private trait, e.g. `trait JniBase64Value { const CLASS: &JNIStr; fn to_base64(&self) -> String; }`, implemented for `Curve25519PublicKey`, `Ed25519PublicKey`, `Ed25519Signature`; collapse the three `to_java_*` functions into one generic `to_java_base64_value<T: JniBase64Value>`.
2. Collapse the session-config pair with a macro (e.g. `session_config_from_version!`) or a small generic over a `VersionedConfig` trait, preserving identical error messages (`"Invalid session config version: {version}"`).
3. Run `cargo clippy -- -D warnings`, `cargo fmt -- --check`, `cargo test`.

**Acceptance criteria**: no behaviour change (110 Rust tests green); duplicate bodies removed.

**Estimate**: M.

## Phase D — Wire sample classes into CI (R5, Low)

**Problem**: `SampleOlm`, `SampleMegolm`, `SampleSas`, `SampleEcies` are `main()`-based demos in the test sourceset; they compile but never run, so their correctness is unchecked.

**Tasks**

1. Add a JUnit test that invokes each sample's `main` (or refactor samples to expose a parameterless `run()` returning void and call that), asserting no exception escapes.
2. Alternative (preferred if cleaner): keep demos out of the test classpath by moving them to a dedicated `examples/` directory compiled by `maven-exec-plugin` in a non-default profile, and delete them from `src/test`.
3. Confirm JaCoCo unaffected (test sourceset excluded).

**Acceptance criteria**: samples either executed by `mvn verify` or no longer compiled in the test sourceset; build green.

**Estimate**: S.

## Phase E — Documentation hygiene (R6, Low)

**Problem**: `AGENTS.md` tool versions drift from `pom.xml` (Checkstyle 14.0.0 vs 14.1.0; JUnit 6 without minor vs 6.1.3).

**Tasks**

1. Update `AGENTS.md` Dependencies section to mirror `pom.xml`/`Cargo.toml` exactly (Checkstyle 14.1.0, JUnit Jupiter 6.1.3, AssertJ 3.27.7).
2. Add a step to the release checklist (CONTRIBUTING or AGENTS) to re-sync these versions on dependency bumps.
3. This PR (docs-only) resolves R7; R6 ships as a normal PR since `AGENTS.md` lives at the root.

**Acceptance criteria**: `AGENTS.md` matches build files; no other doc drift (README verified 2026-09-07).

**Estimate**: S.

## Phase F — Upstream-blocked: legacy pickle export for sessions (R1, Low)

**Problem**: `unpickleLegacy()` works for all session types, but `pickleLegacy()` exists only on `Account` and `PkDecryption`; vodozemac 0.10.0 does not expose legacy pickling for Olm/Megolm sessions.

**Tasks**

1. Track/raise upstream issue on `matrix-org/vodozemac` requesting session legacy pickle support.
2. When available: add `pickleLegacy(byte[] pickleKey)` to `OlmSession`, `InboundGroupSession`, `OutboundGroupSession` following the existing pickle template; wire the corresponding Rust JNI functions beside the existing unpickle ones.
3. Update README API tables, CHANGELOG, and tests in the same PR.

**Acceptance criteria**: blocked until upstream lands; do not merge half-solutions.

**Estimate**: M (once unblocked).

## Phase G — Standing security practices (R8 monitoring, ongoing)

**Problem**: accepted residual risks require periodic re-evaluation rather than code changes.

**Tasks**

1. Each release: re-check SonarCloud language plan for Rust support (unlock native Rust analysis + combined coverage dashboard if pricing allows).
2. Each release: review the three vodozemac feature flags (`libolm-compat`, `experimental-session-config`, `insecure-pk-encryption`) for deprecation upstream and drop them when no longer needed.
3. Keep `cargo audit` and Trivy gating releases; monitor Dependabot/renovate PRs for SHA re-pinning.

**Acceptance criteria**: documented decision per release in CHANGELOG.

## Execution order and dependencies

| Order | Phase | Depends on |
|---|---|---|
| 1 | A (equals fix) | none — highest value, unblocks 100% branch coverage |
| 2 | B (Java constants) | none |
| 3 | C (Rust dedup) | none (independent of A/B) |
| 4 | D (samples) | none |
| 5 | E (AGENTS.md) | this docs PR merged (R7) |
| 6 | F (pickleLegacy) | upstream vodozemac |
| 7 | G | ongoing |

Phases A-D can ship as one PR or separate small PRs; B and C must not be mixed in a single commit (different languages, independent verification).

## Final verification checklist

- [ ] `mvn verify` — 167+ tests, Checkstyle 0 violations, JaCoCo branches 100% after Phase A
- [ ] `cargo test` — 110 tests, 0 failures
- [ ] `cargo clippy --all-targets -- -D warnings` — 0 warnings
- [ ] `cargo fmt -- --check` — clean
- [ ] SonarCloud quality gate passes on the PR
- [ ] `docs/CODE_REVIEW.md` re-scored after Phases A-D (target: Maintainability 9/10, Code Quality 10/10)
