# AGENTS.md

## Project Overview

Java bindings for the [vodozemac](https://github.com/matrix-org/vodozemac) Rust cryptographic library (v0.10.0), providing Olm, Megolm, SAS, ECIES, and PK Encryption/Decryption functionality for the Matrix protocol.

## Project Documentation

The following documentation files live at the root of the repository (not in `docs/`):

- **`README.md`** — Project introduction, usage examples, and getting started guide
- **`CHANGELOG.md`** — Release history and changes per version
- **`CONTRIBUTING.md`** — Guidelines for contributing to the project
- **`CODE_OF_CONDUCT.md`** — Code of conduct for contributors
- **`SECURITY.md`** — Security policy and vulnerability reporting

## Build & Test Commands

### Java (Maven)

```bash
# Full build with tests, coverage, and checkstyle
mvn verify

# Compile only (skips tests)
mvn compile

# Run Java tests only
mvn test

# Run Java tests and the sample demo programs (non-default profile)
mvn -Psample test

# Run a single test class
mvn test -Dtest=InboundGroupSessionTest

# Skip Rust compilation (use pre-built native library)
mvn package -DskipRustBuild=true

# Checkstyle only
mvn checkstyle:check
```

### Rust (Cargo)

```bash
# Build the native library
cargo build --release --manifest-path rust/Cargo.toml

# Run Rust tests
cargo test --manifest-path rust/Cargo.toml

# Lint
cargo clippy --manifest-path rust/Cargo.toml

# Format check
cargo fmt --manifest-path rust/Cargo.toml -- --check

# Coverage report (HTML or LCOV for Sonar) — see the Coverage Rule below
cargo llvm-cov --manifest-path rust/Cargo.toml --open
cargo llvm-cov --lcov --output-path ../target/rust-lcov.info
```

## CI Requirements

All of the following must pass before committing:

1. **Java**: `mvn verify` — compiles Java, builds Rust native library, runs tests, Checkstyle (0 violations), JaCoCo coverage (≥80% instructions, 0 missed methods/classes)
2. **Rust**: `cargo clippy` (0 warnings), `cargo fmt -- --check` (0 issues), `cargo test` (0 failures)
3. **Checkstyle config**: `checkstyle.xml` — enforces naming, imports, formatting, `FinalClass` (all classes with private constructors must be `final`)

### Coverage Rule (≥80%, both languages)

Code coverage must remain **above 80%** for Java and Rust. The SonarCloud
Quality Gate enforces `new_coverage ≥ 80%` on every PR and on `main`, and a
red coverage gate blocks the merge — never land code that drops coverage
below the gate, and never waive it; add tests in the same change instead.

- **Java**: enforced locally by `mvn verify` (JaCoCo check fails the build).
- **Rust**: there is no local hard gate — verify the effect of your change
  with `cargo llvm-cov` (from `rust/`: `cargo llvm-cov --lcov --output-path
  ../target/rust-lcov.info`, or `cargo llvm-cov --open` for a browsable
  report) and confirm the *Sonar code analysis* check stays green on the PR.
- **JNI exports are invisible to pure-Rust tests**: every new
  `extern "system"` export function needs a JVM-attached unit test
  (`get_jvm()` + `EnvUnowned::from_raw(env.get_raw())`, mirroring how the JVM
  invokes native methods — see the tests in `rust/src/types/keypair.rs` and
  `rust/src/sas/established_sas.rs`), otherwise it drags the Rust coverage
  down and breaks the gate.

## Pull Requests

Before creating a pull request — and before pushing any update to an existing one — update `CHANGELOG.md`:

1. Locate the `## [Unreleased]` section at the top of `CHANGELOG.md`.
2. Add a single bullet describing the **main purpose** of the PR (not a per-commit log): `- **Bold summary**: what changed and why (#123).` End the bullet with the PR number in parentheses — at creation time the number is unknown, so append it in the first update after the PR is created.
3. Place it under the matching category header (`🚀 Features`, `🐛 Fixes`, `🛡️ Security & Hardening`, `🧹 Refactoring`, `📚 Documentation`, `🔧 Build System`, `🧪 Testing`, `📁 Project Structure`), creating the category if it does not exist yet.
4. When updating an existing PR, refine its existing entry instead of adding a duplicate.
5. Never modify released sections (`## [X.Y.Z] - date`); they are frozen once published.

The PR reference matters: the release notes separate changelog-covered changes from the remaining merged pull requests by matching these `(#NNN)` references.

## Architecture

### Java ↔ Rust JNI Bridge

- **Java side** (`src/main/java/io/github/fherbreteau/vodozemac/`): Each native-handle class extends `NativeHandle` (abstract, manages `nativePtr` lifecycle via `AutoCloseable`). Package-private constructors for objects created from JNI; public constructors/factories for user-created objects.
- **Rust side** (`rust/src/`): JNI functions named `Java_io_github_fherbreteau_vodozemac_<module>_<Class>_native<Method>`. Each wraps a vodozemac type via `Box::from_raw` / `Box::into_raw`.
- **Error mapping**: Rust errors are mapped to typed Java exceptions via `rust/src/errors.rs` (e.g., `throw_decryption_error` → `DecryptionException`).
- **Object construction**: Rust constructs Java objects via `env.new_object(...)` with `jni_str!` / `jni_sig!` macros (see `OlmMessage`, `DecryptedMessage`, `MegolmMessage`).

### Module Layout

| Java package | Rust module | Description |
|---|---|---|
| `account` | `olm/account.rs` | Olm Account (identity keys, one-time keys, sessions) |
| `olm` | `olm/session.rs` | OlmSession, OlmMessage, MessageType |
| `megolm` | `megolm/` | OutboundGroupSession, InboundGroupSession, MegolmMessage, DecryptedMessage |
| `sas` | `sas/` | Sas, EstablishedSas, SasBytes |
| `ecies` | `ecies/` | Ecies, EstablishedEcies, CheckCode, result types |
| `backup` | `backup/` | PkEncryption (stateless), PkDecryption (native handle), PkMessage |
| `types` | `types/mod.rs` | Ed25519PublicKey, Ed25519Signature, Curve25519PublicKey (value classes, no native handle) |
| `exception` | `errors.rs` | Exception hierarchy (VodozemacException base) |

### Key Patterns

- **NativeHandle lifecycle**: All native-handle classes are `final`, implement `AutoCloseable`, and use `checkNotClosed()` before accessing the native pointer (via the `State` holder and `nativePtr()` accessor). `close()` is idempotent. Subclasses pass their `private static native void nativeFree(long)` function to the `NativeHandle` constructor (`super(ptr, Subclass::nativeFree)`), which also registers the handle with a `Cleaner` that releases the native memory and logs a warning if an instance becomes unreachable without being closed. Instance methods touching native state are `synchronized` (and the `State` pointer is `volatile`), so handles can be shared across threads; `InboundGroupSession.connected/compare/merge` lock both operands in deterministic order (`withLocks`).
- **Value classes**: Result types (`IdentityKeys`, `SessionKeys`, `MegolmMessage`, `OlmMessage`, etc.) have `equals`/`hashCode`/`toString`.
- **Accessors**: Fluent style (no `get` prefix) — e.g., `session.sessionId()`, `message.ciphertext()`.
- **SessionVersion interface**: Shared by `OlmSessionVersion`, `MegolmSessionVersion`, and `MessageType` for `fromVersion`/`fromValue` lookups.
- **Exceptions**: `VodozemacException` (protected constructors) is the base; subclasses have `(String)` and `(String, Throwable)` constructors.

## Code Style

- Java: Checkstyle enforces no trailing whitespace, LF line endings, no tabs, `FinalClass` rule, ordered imports (java group first), `EmptyLineSeparator` between methods.
- Rust: `cargo fmt` style, `clippy` with no warnings.
- No comments in code unless explicitly requested.
- `@author François HERBRETEAU` on all classes.

## Dependencies

- **Java 25**, **Maven 3.9.16** (wrapper), **Maven ≥ 3.6.3** (enforcer)
- **Rust** (stable, `rust-toolchain.toml`), Cargo
- **vodozemac 0.10.0** (Rust crate with features: `libolm-compat`, `experimental-session-config`, `insecure-pk-encryption`)
- **JNI 0.22.4** (Rust crate)
- **JUnit Jupiter 6.1.3**, **AssertJ 3.27.7**, JaCoCo 0.8.15, Checkstyle 14.1.0

Keep this section in sync with `pom.xml` and `rust/Cargo.toml` when dependencies are bumped.

**Never remove `org.eclipse.m2e:lifecycle-mapping` from `pom.xml`**: it is intentional IDE metadata (no build participation) used by the Eclipse m2e plugin and the VS Code Java extension to import the project properly (ignores `checkstyle:check` executions, skips `exec:exec` cargo invocations on incremental builds).

## Documentation Site (`site/`)

The user guide lives in `site/` (MkDocs Material) and is deployed to
<https://fherbreteau.github.io/vodozemac-java/> by the *GitHub Pages*
workflow (`.github/workflows/pages.yml`) on every push to `main` touching
`site/`.

**Update the site in the same PR** whenever a change affects user-facing
documentation: new/changed public APIs (`site/docs/api.md`), usage behavior,
installation requirements, thread-safety or resource-management semantics,
security guidance, or the migration guide. `README.md` stays the repository
landing page; the site is the canonical user guide. Verify locally with
`pip install -r site/requirements.txt && (cd site && mkdocs build --strict)`
— strict mode fails on broken internal links. Never edit the deployed site
manually.

## Working Files (`docs/`)

The `docs/` folder contains working documents (`docs/CODE_REVIEW.md` and `docs/IMPLEMENTATION_PLAN.md`) used to track progress during a task. The `docs/` folder must not be created, modified, or deleted unless in the appropriate PR.

**Constraint**: The `docs/` folder may only be modified when the current Git branch is associated with a pull request whose diff contains *only* files within the `docs/` folder (no other changes). Before modifying anything in `docs/`, verify with:

```bash
gh pr view --json files --jq '.files[].path'
```

If the PR includes any files outside of the `docs/` folder, do not create or edit any files in `docs/` — instead, open a separate PR dedicated solely to these working files.

## Native Compilation

All CI targets build natively on matching GitHub-hosted runners (x86_64 and ARM64 for Linux, macOS, and Windows). No cross-compilation configuration is needed. The Maven build compiles Rust for the host platform by default; CI workflows build on each target platform separately.
