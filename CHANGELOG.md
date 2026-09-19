# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-09-19

### 🐛 Fixes

- **Site Maven Snippet Update**: Fixed a stray backslash in the *Prepare Release* step that updates the site's Maven snippet (`"\$VERSION"` instead of `"$VERSION"`) — the Perl replacement then wrote an empty version (treating `$VERSION` as an undefined Perl variable), failing the workflow with `Failed to update the Maven snippet in site/docs/installation.md` on final releases (#105)
- **Release Startup Failure**: Declared `SONAR_TOKEN` in `build.yml`'s `workflow_call` trigger — passing a secret that the called workflow has not declared makes the *Release* workflow fail with a `startup_failure` before any job runs (only `secrets: inherit` bypasses the declaration, which is why every release before `1.0.0-rc5` worked) (#104)
- **Dependabot not checking rust dependencies**: The cargo ecosystem for Cargo in *dependabot.yml* was not pointing to the right folder. (#100)
- **Release Pipeline De-duplication**: The *Release* workflow no longer runs twice per release — the *Prepare Release* tag push (authenticated with the `RELEASE_TOKEN` PAT, which *does* trigger `on: push` tag workflows, contrary to the previous assumption) fires it, and the redundant explicit dispatch that made the second run fail with a `409 Conflict` on the package deployments was removed; a `concurrency` group now cancels any accidental double run (#97)
- **Immutable Releases**: The *Release* workflow now creates the GitHub release as a **draft**, uploads the jars to it, and publishes it in a follow-up step — GitHub's newly-default immutable releases lock a release as soon as it is published, which made the previous create-then-upload flow fail (`1.0.0-rc2`'s GitHub release page could not carry its assets; the Maven Central deployment was unaffected) (#96)
- **Linux ARM64 Cross-Compilation**: Fixed incorrect Rust target in Maven profile (aarch64-apple-darwin → aarch64-unknown-linux-gnu) (#1)
- **JNI 0.22+ Compatibility**: Updated Rust code to use proper EnvUnowned and with_env() pattern (#32)
- **Native Library Loading**: Enhanced NativeLibraryLoader with fallback path resolution and null-safety on system properties (#37)

### 🧹 Refactoring

- **Release Publish-Only Cleanup**: The *Release* workflow no longer touches the pom — the tagged commit already carries the release version (set by *Prepare Release*, which also owns the next-development-version bump), so the `versions:set` step, the tag-version extraction step and the unused next-version computation were all removed; the workflow now only determines whether the release is final (drives the GitHub release's pre-release flag) (#101)
- **Release Run Slimming**: Removed the redundancies from the release pipeline — the `setup-java` server entries (overwritten by the explicit `settings.xml` step) and the `secrets: inherit` into the build workflow (replaced by a selective mapping of the single secret the build consumes, `SONAR_TOKEN`, keeping the full secret set out of the called workflow). The Sonar analysis of release tags was kept: it is what tells SonarCloud about the new project version — its automatic new-code detection anchors on `sonar.projectVersion` comparisons between analyses (#101)
- **Release Workflow Responsibilities**: The *Release* workflow is now publish-only — it checks out the immutable tag with the default `GITHUB_TOKEN` and never mutates files; the *next development version* bump moved to the *Prepare Release* workflow, which already owns the `main`-branch writes through the `RELEASE_TOKEN` identity, and now applies to pre-releases too — preparing `X.Y.Z-rcN` resets `main` to the associated final version (pom `X.Y.0-SNAPSHOT`, Cargo `X.Y.0`) instead of leaving it on the release version (#97)
- **Creation-Result Equality**: `equals`/`hashCode` of the Olm and ECIES creation-result types compare payload data only, restoring 100% Java branch coverage (#27)
- **Parameter Names**: Centralised `requireNonNull` message constants in a shared `ParamNames` holder (#50)
- **Rust JNI Deduplication**: New `JniBase64Value` trait collapsing the `to_java_*` helpers and a `session_config_from_version!` macro collapsing the Olm/Megolm version-mapping pair (#35)
- **Sample Demos**: Moved out of the test sourceset into `src/demos/java`, compiled and executed through the non-default `sample` Maven profile (`mvn -Psample test`) (#50)

### 🚀 Features

- **Self-Hosted Javadoc**: The site now serves the Javadoc of the latest Maven Central release under `/apidocs/latest/` — `pages.yml` fetches and unpacks the latest released javadoc jar at every site deployment, and the site workflow redeploys automatically on every successful *Release* completion (`workflow_run`). javadoc.io proved unreliable (it generated the Javadoc for only 1 of the 4 releases, with no API to force generation, and un-generated versions silently redirect to a versions-listing page), so all Javadoc links switched to the self-hosted copy (#101)
- **Ed25519 Key Pair Generator**: Added `Ed25519KeyPair` in `io.github.fherbreteau.vodozemac.types`, a native-handle class generating a random Ed25519 signing key pair with signing, public-key export, and JSON pickle/unpickle support, backed by a new Rust JNI module (`rust/src/types/keypair.rs`) (#73)
- **Cross-Signing Demo**: Added `SampleCrossSigning` to the sample profile, demonstrating `Ed25519KeyPair` on a real use case — generating and cross-signing the Matrix master/user-signing/self-signing keys and device keys per the Matrix specification, including pickle persistence of each signing key (#85)
- **Cryptographic Key Types**: Added `Ed25519PublicKey`, `Ed25519Signature`, and `Curve25519PublicKey` typed value classes in `io.github.fherbreteau.vodozemac.types`, backed by a new Rust JNI module (`rust/src/types/mod.rs`). Keys are now typed throughout the Account, IdentityKeys, OneTimeKeyGenerationResult, and SessionKeys APIs instead of raw base64 `String`s. (#38)
- **Signature Verification**: `Ed25519PublicKey.verify(message, signature)` enables verifying Ed25519 signatures from other devices. (#38)
- **macOS Intel CI Target**: Added `darwin-x86_64` build target using `macos-15-intel` runner (#36)
- **Windows ARM64 CI Target**: Added `windows-aarch64` build target using `windows-11-arm` runner (#36)
- **Comprehensive Documentation**: Added complete README.md with usage examples, API reference, and development guide (#1)
- **AssertJ Testing**: Migrated test suite to use AssertJ for fluent assertions (#1)
- **GitHub Actions**: Configured multi-platform CI/CD pipeline with build, test, and package jobs (#1)

### 📚 Documentation

- **Site Maven Snippet Versioning**: The *Prepare Release* workflow now updates the Maven dependency snippet on the site's installation page to the released version for final releases, so the documented configuration always points at the latest final; pre-releases keep pointing consumers at the last final (#101)
- **Project Web Page**: Added a MkDocs Material user guide at <https://fherbreteau.github.io/vodozemac-java/> (`site/`), structured from the README (getting started, resource management, thread safety, key material, full API reference, jOlm migration pointer, security) and deployed by a new `pages.yml` workflow (strict build, SHA-pinned actions) on pushes to `main` touching `site/`; the README and `pom.xml` link the site, Javadoc is served by javadoc.io for every release, and `AGENTS.md` now requires agents to update the site with any user-facing change (#76) (#100)
- **Agent Coverage Rule**: `AGENTS.md` now enforces an explicit Coverage Rule — code coverage must stay above 80% for Java and Rust (SonarCloud `new_coverage ≥ 80%` gate on every PR and on `main`), documents how to verify Rust coverage locally with `cargo llvm-cov`, and requires a JVM-attached unit test for every new `extern "system"` JNI export so the gate stays green (#94)
- **Project Metadata**: Replaced the `${project.scm.url}` expressions in the POM's `<url>` and `<connection>`/`<developerConnection>` with literal GitHub URLs — Maven deploys the raw `pom.xml` without interpolation, so the published POM on Maven Central (and mvnrepository's homepage field) contained unresolved expressions; takes effect on the next published release (#90)
- **Sample Classes Isolation**: The `sample` profile now compiles the demos into a dedicated `target/demos-classes` directory (dedicated `maven-compiler-plugin` execution + `additionalClasspathElements` on each `exec:java` run) instead of through `build-helper`'s shared `add-source`, so demo classes never land in `target/classes` — a default `mvn verify` after `mvn -Psample test` no longer fails the JaCoCo 0-missed-classes check; the orphaned `build-helper-maven-plugin` pluginManagement entry and its version property were removed since no profile uses the plugin anymore (#89)
- **CodeQL Action Update**: Bumped `github/codeql-action/{init,analyze,upload-sarif}` to 4.38.0 in a single commit — the three sub-actions must run the same version within a workflow run, so per-sub-path Dependabot PRs left mixed versions that crashed the CodeQL jobs; a `codeql-action` Dependabot group now keeps future bumps atomic (#87)
- **Reusable Workflow Secrets**: The *Release* workflow now forwards its secrets to the reusable build workflow (`secrets: inherit`); called workflows do not inherit secrets, which made the Sonar job fail and skipped artifact publishing on release runs (#68)
- **Release Credentials**: The *Release* workflow writes both Maven server credentials (GitHub Packages and Maven Central) explicitly, since `setup-java` only supports a single server entry and the GitHub Packages deploy lost its `github` server entry (#69)
- **Central Auto-Publishing**: Maven Central deployments now publish automatically (`autoPublish`) and the release workflow waits until the artifacts are live (`waitUntil published`); the previously used `autoReleaseAfterClose` parameter was silently ignored by the Central Portal plugin (#70)
- **Setup Java Inputs**: Renamed the deprecated `server-username`, `server-password` and `gpg-passphrase` inputs of `actions/setup-java` to their `-env-var` variants in the release workflow (#71)
- **SECURITY.md**: Added comprehensive security policy and vulnerability reporting guide (#1)
- **CONTRIBUTING.md**: Created detailed contribution guidelines and development workflow (#1)
- **CHANGELOG.md**: Added this changelog file for tracking changes (#1)
- **Agent Changelog Rule**: `AGENTS.md` now instructs coding agents to update the `[Unreleased]` section of `CHANGELOG.md` with the PR's main purpose before creating or updating a pull request, ending each bullet with its PR number `(#NNN)` so release notes can attribute changes (#61)
- **jOlm Migration Guide**: Added `MIGRATION-GUIDE.md` mapping the archived jOlm (libolm JNA bindings) API to vodozemac-java class by class, including the libolm pickle migration path through the `unpickleLegacy` entry points (#72)

### 🧪 Testing

- **Rust JNI Coverage**: Added 15 JVM-attached Rust unit tests that invoke the JNI exports directly (`Ed25519KeyPair` lifecycle, key/signature validation and verification, SAS bytes and MAC flows, DH ownership transfer, base64 utilities, one-time-keys limits, `RawBox` guard semantics), lifting the Rust new-code coverage on SonarCloud above the 80% Quality Gate requirement (#93)
- **Test Coverage**: Added comprehensive test cases covering all major functionality (167 test cases) (#37)
- **AssertJ Migration**: Enhanced tests with fluent assertions and better error messages (#1)
- **Test Properties**: Added key validation and property testing (#37)

### 🛡️ Security & Hardening

- **Hardening Backlog (issue #64)**: Landed the low-severity hardening backlog from the deep main analysis — scheduled `cargo-audit` in `security.yml` (L1), `Locale.ROOT`-safe platform detection plus documented temp-file cleanup in `NativeLibraryLoader` (L2), a best-effort `java.lang.Cleaner` safety net that releases native memory and logs a warning when a `NativeHandle` is garbage collected without being closed (L3), tolerant `git-commit-id-maven-plugin` settings for builds without a `.git` directory (L4), exact JNI error propagation via `JString::try_to_string` instead of the `<NULL>` fallback (L5), README documentation restricting `pk_encryption` to Matrix backup/libolm interoperability (L6), and documentation of the secret-material heap-dump exposure in `README.md`/`SECURITY.md` (M2) (#74)
- **Secret-Scanning Settings**: Enabled secret-scanning push protection (blocks pushes containing recognized secret patterns on every branch) and validity checks (provider-validated active/inactive alerts) at the repository level, and documented the active repository security settings in SECURITY.md (#75)
- **Thread-Safe Native Handles**: All native-handle classes (`Account`, sessions, `Sas`/`EstablishedSas`, `Ecies`/`EstablishedEcies`, `Ed25519KeyPair`, `PkEncryption`/`PkDecryption`) are now safe to share across threads — instance methods touching native state are `synchronized`, eliminating aliased `&mut` JNI references, use-after-free and double-free races on `close()`; `InboundGroupSession.connected/compare/merge` lock both operands in deterministic order to prevent deadlocks (#52)
- **JNI Lifecycle Hardening**: Panic-guarded `native_free`, pointer validation ordering, `catch_panic` safety on all JNI entry points, and `RawBox` guards replacing `forget` (#47)
- **Java API Finalization**: Input validation via `Objects.requireNonNull` on public APIs, `final` value classes with `equals`/`hashCode`/`toString`, session protocol version support (`OlmSessionVersion`, `MegolmSessionVersion`) (#47)
- **CI/CD Hardening**: SHA-pinned GitHub Actions, concurrency groups, security scanning (CodeQL, Trivy, `cargo-audit`), Maven wrapper checksum verification, and all six native platforms assembled in release artifacts (#47)
- **Rust Release Profile**: Enabled `lto`, single `codegen-units`, and `strip` for release builds
- **Release Push Authorization**: The release workflows authenticate their direct pushes to `main` (release preparation and final version bump) with a dedicated `RELEASE_TOKEN` identity, satisfying the default-branch ruleset that requires pull requests for every other actor (#66) (#47)
- **One-Time Keys Generation Cap**: Capped `Account.generateOneTimeKeys` at 100 keys per call (rejecting non-positive counts) in both the Java API and the JNI binding, preventing memory/CPU exhaustion that could abort the JVM from unbounded native key generation (#65)

### 🔧 Build System

- **Rust in SonarCloud**: The Sonar analysis now covers all code — `sonar.sources` includes `rust/src` (Java + Rust), the Rust coverage is generated with `cargo llvm-cov --lcov` in the quality job (`sonar.rust.lcov.reportPaths`), and the Cargo manifest is declared (`sonar.rust.cargo.manifestPath`) so Rust files are analyzed alongside Java on SonarCloud (#91)
- **Sonar Rust Follow-up**: Fixed the 5 Rust issues surfaced by the new SonarCloud analysis (4 redundant `'static` lifetime annotations in the `JniBase64Value` trait, and `mem::forget` in `RawBox::leak` replaced by the idiomatic `ManuallyDrop`), and rewrote the lcov `SF:` paths to project-relative `rust/...` after `cargo llvm-cov` — SonarCloud matched the absolute CI paths against nothing and imported the Rust coverage as 0%, dropping the overall coverage to 48.2% (#92)
- **Maven Configuration**: Fixed groupId, main class, and added AssertJ dependency (#1)
- **Cross-Platform Profiles**: Corrected all platform targets for proper cross-compilation (#1)
- **Resource Management**: Improved native library organization and packaging (#1)
- **Maven Central Publishing**: Release artifacts are now published to Maven Central (Sonatype Central Portal) in addition to GitHub Packages, with generated release notes based on the previous final published release, pre-release handling for `-rc` tags, and automatic post-release version bumps (pom to the next `-SNAPSHOT`, Rust crate to the next final version). Releasing starts with an action-triggered *Prepare Release* workflow that validates the version, renames `CHANGELOG.md`'s `[Unreleased]` to the released version (RC entries are merged into the final section when it ships), aligns the Maven and Rust crate versions, commits to `main` and creates the release tag; the tag-triggered *Release* workflow then builds and publishes the artifacts. Release notes combine the released version's changelog section with the merged pull requests that are not referenced in it (#59)
- **Release Dispatch Permission**: The *Prepare Release* job now grants `actions: write` so its final step can dispatch the tag-triggered *Release* workflow with `GITHUB_TOKEN` (#67)

### 📁 Project Structure

- **Complete Refactoring**: Organized project structure for better maintainability (#1)
- **GitHub Integration**: Added proper .github/workflows/ directory (#1)
- **License File**: Apache-2.0 `LICENSE` declared in the project root and in the Maven POM (#1)

### 🛠️ Early Development

- **0.1.0 (2025-07-24)**: Project scaffolding and basic structure, initial JNI bindings prototype, basic Maven configuration, proof-of-concept implementation.
- **1.0.0 (2026-08-13)**: Java bindings for Vodozemac cryptographic operations (Curve25519, Ed25519, message signing), automatic Rust compilation and native library packaging through Maven, cross-platform support (Linux, macOS, Windows), `AutoCloseable` resource handling (then `VodozemacAccount`, since renamed `Account`), native library loading, first GitHub Actions CI/CD pipeline.
- **MAJOR**: Breaking changes
- **MINOR**: Backwards-compatible new features
- **PATCH**: Backwards-compatible bug fixes

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