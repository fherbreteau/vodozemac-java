# Contributing to Vodozemac Java Bindings

🎉 **First off, thanks for taking the time to contribute!** 🎉

The following is a set of guidelines for contributing to Vodozemac Java Bindings. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## 📋 Table of Contents

- [Code of Conduct](#scroll-code-of-conduct)
- [How Can I Contribute?](#thinking-how-can-i-contribute)
- [Getting Started](#rocket-getting-started)
- [Development Setup](#computer-development-setup)
- [Pull Request Process](#git-pull-request-process)
- [Coding Standards](#memo-coding-standards)
- [Commit Message Guidelines](#writing_hand-commit-message-guidelines)
- [Documentation](#book-documentation)
- [Testing](#test_tube-testing)
- [Community](#people_hugging-community)

## 📜 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## 🤔 How Can I Contribute?

### Reporting Bugs

- **Use GitHub Issues**: [Open a new issue](https://github.com/fherbreteau/vodozemac-java/issues)
- **Include details**:
  - Version of Vodozemac Java
  - Java version
  - Operating system
  - Steps to reproduce
  - Expected vs actual behavior
  - Screenshots if applicable

### Suggesting Enhancements

- **Use GitHub Discussions**: [Start a discussion](https://github.com/fherbreteau/vodozemac-java/discussions)
- **Provide context**:
  - Use case
  - Why this enhancement would be useful
  - Potential implementation ideas

### Pull Requests

- **Fork the repository**
- **Create a feature branch**: `git checkout -b feature/your-feature`
- **Commit your changes**: `git commit -am 'Add some feature'`
- **Push to the branch**: `git push origin feature/your-feature`
- **Open a pull request**

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/your-username/vodozemac-java.git
   cd vodozemac-java
   ```
3. **Set up upstream**:
   ```bash
   git remote add upstream https://github.com/fherbreteau/vodozemac-java.git
   ```

## 💻 Development Setup

### Prerequisites

- **Java 17+**
- **Maven 3.8+**
- **Rust toolchain** (stable)
- **Git**

### Build the Project

```bash
# Clean build (compiles Rust and Java)
mvn clean package

# Java-only build (faster for Java changes)
mvn package -DskipRustBuild=true
```

### Development Workflow

```bash
# Pull latest changes from upstream
git pull upstream main

# Create a new feature branch
git checkout -b feature/your-feature

# Make your changes
# ...

# Build and test
mvn clean test

# Commit your changes
git commit -am 'Add some feature'

# Push to your fork
git push origin feature/your-feature
```

## 🔄 Pull Request Process

1. **Ensure tests pass**: `mvn test`
2. **Update documentation** if needed
3. **Follow coding standards** (see below)
4. **Write good commit messages** (see below)
5. **Open a pull request** with:
   - Clear title and description
   - Reference to related issues
   - Screenshots if applicable
6. **Address review feedback** promptly

## 📝 Coding Standards

### Java

- **Follow Oracle Java Code Conventions**
- **Use meaningful names**: `account` not `acc`, `userService` not `us`
- **Keep methods small**: Single responsibility principle
- **Add Javadoc**: For public classes and methods
- **Use final**: For variables that don't change
- **Prefer immutability**: Where possible

### Rust

- **Follow Rust API Guidelines**
- **Use clippy**: `cargo clippy`
- **Format code**: `cargo fmt`
- **Document unsafe code**: Clearly explain safety invariants
- **Use proper error handling**: Avoid panics in library code

### General

- **Consistent style**: Follow existing patterns
- **English comments**: For international collaboration
- **Avoid magic numbers**: Use named constants
- **Handle edge cases**: Gracefully

## ✍️ Commit Message Guidelines

### Format

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code changes that neither fix bugs nor add features
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(crypto): Add Ed25519 key verification

Implements key verification using the Ed25519 algorithm.
This allows users to verify signatures from other parties.

Closes #42
```

```
fix(build): Correct Linux ARM64 cross-compilation target

Changed linux-aarch64 Maven profile from aarch64-apple-darwin
to aarch64-unknown-linux-gnu to fix cross-compilation issues.

All tests passing (6/6), GitHub Actions workflow verified.
```

## 📚 Documentation

- **Update README.md** for significant changes
- **Add Javadoc** for new public APIs
- **Update examples** if behavior changes
- **Keep documentation in sync** with code

## 🧪 Testing

### Running Tests

```bash
# Run all tests
mvn test

# Run specific test
mvn test -Dtest=VodozemacAccountTest

# Run tests with coverage
mvn jacoco:prepare-agent test jacoco:report
```

### Writing Tests

- **Use AssertJ** for fluent assertions
- **Test edge cases**
- **Keep tests isolated**
- **Use descriptive names**: `testAccountCreationAndKeyGeneration` not `test1`
- **Add test descriptions**: Use `.as("description")`

## 👥 Community

- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Matrix Room**: `#vodozemac-java:matrix.org`
- **Weekly Sync**: Fridays at 15:00 UTC (optional)

### Maintainers

- **François Herbreteau**: [@fherbreteau](https://github.com/fherbreteau)
- **Contributors**: See [CONTRIBUTORS](CONTRIBUTORS)

## 🎓 Learning Resources

- **Java**: [Oracle Java Tutorials](https://docs.oracle.com/javase/tutorial/)
- **Rust**: [The Rust Book](https://doc.rust-lang.org/book/)
- **JNI**: [Java Native Interface](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/)
- **Maven**: [Maven Documentation](https://maven.apache.org/guides/)

## 🙏 Thanks!

Your contributions make this project better. Whether it's fixing bugs, adding features, improving documentation, or helping others, every contribution is valuable.

**Happy coding!** 🚀