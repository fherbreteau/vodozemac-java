# Vodozemac Java Bindings

[![Build Status](https://github.com/fherbreteau/vodozemac-java/actions/workflows/build.yml/badge.svg)](https://github.com/fherbreteau/vodozemac-java/actions/workflows/build.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Java 17+](https://img.shields.io/badge/Java-17+-red.svg)](https://www.oracle.com/java/technologies/javase/jdk17-archive-downloads.html)
[![Maven Central](https://img.shields.io/maven-central/v/io.github.fherbreteau/vodozemac-java.svg)](https://search.maven.org/artifact/io.github.fherbreteau/vodozemac-java)

**Java bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Matrix cryptography library**

## 🚀 Overview

Vodozemac Java provides Java Native Interface (JNI) bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Rust library, which implements the [OLM](https://gitlab.matrix.org/matrix-org/olm) cryptographic ratchet for Matrix end-to-end encryption.

## ✨ Features

- **🔐 Cryptographic Operations**: Curve25519 and Ed25519 key generation, message signing
- **🌍 Cross-Platform**: Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), Windows
- **📦 Maven Integration**: Automatic Rust compilation and native library packaging
- **🗑️ Memory Safety**: Proper resource management with Java's `AutoCloseable`
- **🧪 Comprehensive Testing**: AssertJ-based test suite with 6 test cases
- **🔧 GitHub CI/CD**: Multi-platform build and test pipeline

## 📋 Requirements

- **Java 17+** (Required for JNI and modern features)
- **Maven 3.8+** (For building the project)
- **Rust toolchain** (For native library compilation)
- **Git** (For version control)

## 🛠️ Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/fherbreteau/vodozemac-java.git
cd vodozemac-java

# Build the project (compiles both Rust and Java)
mvn clean package
```

### Maven Dependency

Add to your `pom.xml`:

```xml
<dependency>
    <groupId>io.github.fherbreteau</groupId>
    <artifactId>vodozemac-java</artifactId>
    <version>1.0.0</version>
</dependency>
```

## 📖 Usage

### Basic Example

```java
import io.github.fherbreteau.vodozemac.VodozemacAccount;

public class MatrixCryptoExample {
    public static void main(String[] args) {
        // Create a new cryptographic account
        try (VodozemacAccount account = new VodozemacAccount()) {
            // Generate cryptographic keys
            String curve25519Key = account.curve25519Key();
            String ed25519Key = account.ed25519Key();

            System.out.println("🔑 Curve25519 Key: " + curve25519Key);
            System.out.println("🔑 Ed25519 Key: " + ed25519Key);

            // Sign a message
            String message = "Hello Matrix!";
            String signature = account.sign(message);

            System.out.println("📝 Signature: " + signature);
        } // Account automatically closed and resources freed
    }
}
```

### Resource Management

The `VodozemacAccount` implements `AutoCloseable` for safe resource management:

```java
// Recommended: Use try-with-resources
try (VodozemacAccount account = new VodozemacAccount()) {
    String key = account.curve25519Key();
    // Use the account...
} // Automatically freed when block exits

// Manual management also supported
VodozemacAccount account = new VodozemacAccount();
try {
    String key = account.ed25519Key();
} finally {
    account.close(); // Explicit cleanup
}
```

## 🗃️ API Reference

### VodozemacAccount

Main class providing cryptographic operations.

#### Constructor

```java
VodozemacAccount() throws RuntimeException
```
Creates a new cryptographic account. Automatically loads the appropriate native library.

#### Methods

```java
String curve25519Key()
```
- **Returns**: Curve25519 public key (base64 encoded)
- **Throws**: `IllegalStateException` if account is closed

```java
String ed25519Key()
```
- **Returns**: Ed25519 public key (base64 encoded)
- **Throws**: `IllegalStateException` if account is closed

```java
String sign(String message)
```
- **Parameters**: `message` - Message to sign
- **Returns**: Signature (base64 encoded)
- **Throws**: `IllegalStateException` if account is closed

```java
void close()
```
- **Description**: Frees native resources
- **Note**: Called automatically with try-with-resources

## 🏗️ Architecture

### Project Structure

```
vodozemac-java/
├── .github/
│   └── workflows/          # GitHub Actions CI/CD
│       └── build.yml       # Multi-platform build pipeline
├── rust/                  # Rust JNI bindings
│   ├── Cargo.toml         # Rust project configuration
│   └── src/lib.rs         # JNI implementation
├── src/                   # Java source code
│   ├── main/java/         # Main Java classes
│   └── test/java/         # Test classes
├── pom.xml                # Maven build configuration
└── README.md              # This file
```

### Build Process

1. **Rust Compilation**: Maven automatically compiles Rust code into native libraries
2. **Java Compilation**: Java classes compiled with JNI native method declarations
3. **Resource Packaging**: Native libraries packaged in JAR file
4. **Native Loading**: At runtime, libraries are extracted and loaded automatically

### Cross-Platform Support

| Platform | Architecture | Rust Target | Status |
|----------|--------------|-------------|--------|
| 🐧 Linux | x86_64 | `x86_64-unknown-linux-gnu` | ✅ Supported |
| 🐧 Linux | ARM64 | `aarch64-unknown-linux-gnu` | ✅ Supported |
| 🍎 macOS | Intel | `x86_64-apple-darwin` | ✅ Supported |
| 🍎 macOS | Apple Silicon | `aarch64-apple-darwin` | ✅ Supported |
| 🪟 Windows | x86_64 | `x86_64-pc-windows-msvc` | ✅ Supported |

## 👨‍💻 Development

### Building

```bash
# Clean build (compiles everything)
mvn clean package

# Java-only build (skip Rust compilation)
mvn package -DskipRustBuild=true

# Run tests
mvn test

# Run specific test
mvn test -Dtest=VodozemacAccountTest
```

### Adding New Bindings

1. **Declare native method** in Java class:
   ```java
   private native String nativeNewMethod(long ptr, String param);
   ```

2. **Implement JNI function** in `rust/src/lib.rs`:
   ```rust
   #[no_mangle]
   pub extern "system" fn Java_io_github_fherbreteau_vodozemac_VodozemacAccount_nativeNewMethod(
       env: EnvUnowned,
       _class: JClass,
       ptr: jlong,
       param: JString,
   ) -> jstring {
       // Implementation here
   }
   ```

3. **Rebuild** the project:
   ```bash
   mvn clean package
   ```

### Debugging JNI

Add these JVM options for JNI debugging:

```bash
java -Djava.library.path=/path/to/native/libs \
     -agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=5005 \
     YourMainClass
```

## 🧪 Testing

The project includes a comprehensive test suite using AssertJ:

```bash
mvn test
```

### Test Coverage

- ✅ **Account Creation**: Tests successful account initialization
- ✅ **Key Generation**: Validates Curve25519 and Ed25519 keys
- ✅ **Message Signing**: Verifies cryptographic signing
- ✅ **Resource Management**: Ensures proper cleanup
- ✅ **Concurrency**: Tests multiple account handling
- ✅ **Key Validation**: Checks base64 format and length

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/your-feature`
3. **Commit** your changes: `git commit -am 'Add some feature'`
4. **Push** to the branch: `git push origin feature/your-feature`
5. **Submit** a pull request

### Code Standards

- Follow existing code style and conventions
- Write comprehensive tests for new functionality
- Update documentation as needed
- Keep commits focused and descriptive

## 📄 License

This project is licensed under the **Apache License 2.0**. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgements

- [Vodozemac](https://github.com/matrix-org/vodozemac) - Rust Matrix cryptography library
- [JNI](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/) - Java Native Interface
- [Maven](https://maven.apache.org/) - Build automation tool
- [AssertJ](https://assertj.github.io/doc/) - Fluent assertions for testing

## 🆘 Support

For issues, questions, or feature requests:

- 🐙 **GitHub Issues**: [Open an issue](https://github.com/fherbreteau/vodozemac-java/issues)
- 💬 **Discussions**: [Start a discussion](https://github.com/fherbreteau/vodozemac-java/discussions)
- 📖 **Documentation**: Check this README and JavaDoc

---

**© 2024 François Herbreteau | [GitHub](https://github.com/fherbreteau) | [Matrix](https://matrix.to/#/@fherbreteau:matrix.org)**