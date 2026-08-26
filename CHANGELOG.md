# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 🚀 Features

- **macOS Intel CI Target**: Added `darwin-x86_64` build target using `macos-15-intel` runner
- **Windows ARM64 CI Target**: Added `windows-aarch64` build target using `windows-11-arm` runner
- **Comprehensive Documentation**: Added complete README.md with usage examples, API reference, and development guide
- **AssertJ Testing**: Migrated test suite to use AssertJ for fluent assertions
- **GitHub Actions**: Configured multi-platform CI/CD pipeline with build, test, and package jobs

### 🐛 Fixes

- **Linux ARM64 Cross-Compilation**: Fixed incorrect Rust target in Maven profile (aarch64-apple-darwin → aarch64-unknown-linux-gnu)
- **JNI 0.22+ Compatibility**: Updated Rust code to use proper EnvUnowned and with_env() pattern
- **Native Library Loading**: Enhanced NativeLibraryLoader with fallback path resolution

### 📚 Documentation

- **SECURITY.md**: Added comprehensive security policy and vulnerability reporting guide
- **CONTRIBUTING.md**: Created detailed contribution guidelines and development workflow
- **CHANGELOG.md**: Added this changelog file for tracking changes

### 🔧 Build System

- **Maven Configuration**: Fixed groupId, main class, and added AssertJ dependency
- **Cross-Platform Profiles**: Corrected all platform targets for proper cross-compilation
- **Resource Management**: Improved native library organization and packaging

### 🧪 Testing

- **Test Coverage**: Added 6 comprehensive test cases covering all major functionality
- **AssertJ Migration**: Enhanced tests with fluent assertions and better error messages
- **Test Properties**: Added key validation and property testing

### 📁 Project Structure

- **Complete Refactoring**: Organized project structure for better maintainability
- **GitHub Integration**: Added proper .github/workflows/ directory
- **License Files**: Organized LICENSE and LICENSES directories

## [1.0.0] - 2024-07-30

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

## [0.1.0] - 2024-01-15

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

**Last Updated**: 2024-07-30
**Maintainer**: François Herbreteau