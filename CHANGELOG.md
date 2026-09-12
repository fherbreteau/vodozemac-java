# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 🚀 Features

- **Cryptographic Key Types**: Added `Ed25519PublicKey`, `Ed25519Signature`, and `Curve25519PublicKey` typed value classes in `io.github.fherbreteau.vodozemac.types`, backed by a new Rust JNI module (`rust/src/types/mod.rs`). Keys are now typed throughout the Account, IdentityKeys, OneTimeKeyGenerationResult, and SessionKeys APIs instead of raw base64 `String`s. (#38)
- **Signature Verification**: `Ed25519PublicKey.verify(message, signature)` enables verifying Ed25519 signatures from other devices. (#38)
- **macOS Intel CI Target**: Added `darwin-x86_64` build target using `macos-15-intel` runner (#36)
- **Windows ARM64 CI Target**: Added `windows-aarch64` build target using `windows-11-arm` runner (#36)
- **Comprehensive Documentation**: Added complete README.md with usage examples, API reference, and development guide (#1)
- **AssertJ Testing**: Migrated test suite to use AssertJ for fluent assertions (#1)
- **GitHub Actions**: Configured multi-platform CI/CD pipeline with build, test, and package jobs (#1)

### 🐛 Fixes

- **Linux ARM64 Cross-Compilation**: Fixed incorrect Rust target in Maven profile (aarch64-apple-darwin → aarch64-unknown-linux-gnu) (#1)
- **JNI 0.22+ Compatibility**: Updated Rust code to use proper EnvUnowned and with_env() pattern (#32)
- **Native Library Loading**: Enhanced NativeLibraryLoader with fallback path resolution and null-safety on system properties (#37)

### 🛡️ Security & Hardening

- **JNI Lifecycle Hardening**: Panic-guarded `native_free`, pointer validation ordering, `catch_panic` safety on all JNI entry points, and `RawBox` guards replacing `forget` (#47)
- **Java API Finalization**: Input validation via `Objects.requireNonNull` on public APIs, `final` value classes with `equals`/`hashCode`/`toString`, session protocol version support (`OlmSessionVersion`, `MegolmSessionVersion`) (#47)
- **CI/CD Hardening**: SHA-pinned GitHub Actions, concurrency groups, security scanning (CodeQL, Trivy, `cargo-audit`), Maven wrapper checksum verification, and all six native platforms assembled in release artifacts (#47)
- **Rust Release Profile**: Enabled `lto`, single `codegen-units`, and `strip` for release builds (#47)

### 🧹 Refactoring

- **Creation-Result Equality**: `equals`/`hashCode` of the Olm and ECIES creation-result types compare payload data only, restoring 100% Java branch coverage (#27)
- **Parameter Names**: Centralised `requireNonNull` message constants in a shared `ParamNames` holder (#50)
- **Rust JNI Deduplication**: New `JniBase64Value` trait collapsing the `to_java_*` helpers and a `session_config_from_version!` macro collapsing the Olm/Megolm version-mapping pair (#35)
- **Sample Demos**: Moved out of the test sourceset into `src/demos/java`, compiled and executed through the non-default `sample` Maven profile (`mvn -Psample test`) (#50)

### 📚 Documentation

- **SECURITY.md**: Added comprehensive security policy and vulnerability reporting guide (#1)
- **CONTRIBUTING.md**: Created detailed contribution guidelines and development workflow (#1)
- **CHANGELOG.md**: Added this changelog file for tracking changes (#1)
- **Agent Changelog Rule**: `AGENTS.md` now instructs coding agents to update the `[Unreleased]` section of `CHANGELOG.md` with the PR's main purpose before creating or updating a pull request, ending each bullet with its PR number `(#NNN)` so release notes can attribute changes (#61)

### 🔧 Build System

- **Maven Configuration**: Fixed groupId, main class, and added AssertJ dependency (#1)
- **Cross-Platform Profiles**: Corrected all platform targets for proper cross-compilation (#1)
- **Resource Management**: Improved native library organization and packaging (#1)
- **Maven Central Publishing**: Release artifacts are now published to Maven Central (Sonatype Central Portal) in addition to GitHub Packages, with generated release notes based on the previous final published release, pre-release handling for `-rc` tags, and automatic post-release version bumps (pom to the next `-SNAPSHOT`, Rust crate to the next final version). Releasing starts with an action-triggered *Prepare Release* workflow that validates the version, renames `CHANGELOG.md`'s `[Unreleased]` to the released version (RC entries are merged into the final section when it ships), aligns the Maven and Rust crate versions, commits to `main` and creates the release tag; the tag-triggered *Release* workflow then builds and publishes the artifacts. Release notes combine the released version's changelog section with the merged pull requests that are not referenced in it (#59)

### 🧪 Testing

- **Test Coverage**: Added comprehensive test cases covering all major functionality (167 test cases) (#37)
- **AssertJ Migration**: Enhanced tests with fluent assertions and better error messages (#1)
- **Test Properties**: Added key validation and property testing (#37)

### 📁 Project Structure

- **Complete Refactoring**: Organized project structure for better maintainability (#1)
- **GitHub Integration**: Added proper .github/workflows/ directory (#1)
- **License File**: Apache-2.0 `LICENSE` declared in the project root and in the Maven POM (#1)

## [1.0.0] - 2026-08-13

### 🎉 Initial Release

- **Core Functionality**: Java bindings for Vodozemac cryptographic operations
- **JNI Implementation**: Rust JNI bindings for Curve25519, Ed25519, and message signing
- **Maven Integration**: Automatic Rust compilation and native library packaging
- **Cross-Platform**: Support for Linux, macOS, and Windows
- **Resource Management**: AutoCloseable implementation for safe native resource handling

### 📦 Initial Features

- `VodozemacAccount` class with cryptographic operations
- Native library loading and management
- Maven-based build system
- Basic test coverage
- GitHub Actions CI/CD pipeline

## [0.1.0] - 2025-07-24

### 🛠️ Initial Development

- Project scaffolding and basic structure
- Initial JNI bindings prototype
- Basic Maven configuration
- Proof of concept implementation

---

## 📋 Versioning

This project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html):

- **MAJOR**: Breaking changes
- **MINOR**: Backwards-compatible new features
- **PATCH**: Backwards-compatible bug fixes

## 🗃️ Changelog Format

```markdown
## [Version] - YYYY-MM-DD

### 🚀 Features
- New features added

### 🐛 Fixes
- Bug fixes and corrections

### 📚 Documentation
- Documentation improvements

### 🔧 Build System
- Build and dependency changes

### 🧪 Testing
- Test improvements and additions

### 📁 Project Structure
- Organization and structure changes
```

## 🤝 Contributing to Changelog

When making changes, please:

1. Add entries to the **Unreleased** section
2. Follow the existing format and categories
3. Be concise but descriptive
4. Reference related issues/PRs when possible
5. Update version and date when releasing

## 📬 Contact

For questions about this changelog or versioning:

- **GitHub Issues**: [fherbreteau/vodozemac-java/issues](https://github.com/fherbreteau/vodozemac-java/issues)
- **Email**: fherbreteau@gmail.com
- **Matrix**: @fherbreteau:matrix.org

---

**Last Updated**: 2026-09-12
**Maintainer**: François Herbreteau