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
```

## CI Requirements

All of the following must pass before committing:

1. **Java**: `mvn verify` — compiles Java, builds Rust native library, runs tests, Checkstyle (0 violations), JaCoCo coverage (≥80% instructions, 0 missed methods/classes)
2. **Rust**: `cargo clippy` (0 warnings), `cargo fmt -- --check` (0 issues), `cargo test` (0 failures)
3. **Checkstyle config**: `checkstyle.xml` — enforces naming, imports, formatting, `FinalClass` (all classes with private constructors must be `final`)

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

- **NativeHandle lifecycle**: All native-handle classes are `final`, implement `AutoCloseable`, and use `checkNotClosed()` before accessing `nativePtr`. `close()` is idempotent.
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

- **Java 25**, Maven
- **Rust** (stable), Cargo
- **vodozemac 0.10.0** (Rust crate with features: `libolm-compat`, `experimental-session-config`, `insecure-pk-encryption`)
- **JNI 0.22.4** (Rust crate)
- **JUnit 6**, AssertJ, JaCoCo 0.8.15, Checkstyle 14.0.0

## Working Files (`docs/`)

The `docs/` folder contains working documents (`docs/CODE_REVIEW.md` and `docs/IMPLEMENTATION_PLAN.md`) used to track progress during a task. The `docs/` folder must not be created, modified, or deleted unless in the appropriate PR.

**Constraint**: The `docs/` folder may only be modified when the current Git branch is associated with a pull request whose diff contains *only* files within the `docs/` folder (no other changes). Before modifying anything in `docs/`, verify with:

```bash
gh pr view --json files --jq '.files[].path'
```

If the PR includes any files outside of the `docs/` folder, do not create or edit any files in `docs/` — instead, open a separate PR dedicated solely to these working files.

## Native Compilation

All CI targets build natively on matching GitHub-hosted runners (x86_64 and ARM64 for Linux, macOS, and Windows). No cross-compilation configuration is needed. The Maven build compiles Rust for the host platform by default; CI workflows build on each target platform separately.
