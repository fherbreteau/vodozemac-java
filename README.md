# Vodozemac Java Bindings

[![Build Status](https://github.com/fherbreteau/vodozemac-java/actions/workflows/build.yml/badge.svg)](https://github.com/fherbreteau/vodozemac-java/actions/workflows/build.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Java 25+](https://img.shields.io/badge/Java-25+-red.svg)](https://www.oracle.com/java/technologies/downloads/)
[![Maven Central](https://img.shields.io/maven-central/v/io.github.fherbreteau/vodozemac-java.svg)](https://search.maven.org/artifact/io.github.fherbreteau/vodozemac-java)

**Java bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Matrix cryptography library**

## 🚀 Overview

Vodozemac Java provides Java Native Interface (JNI) bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Rust library, which implements the [OLM](https://gitlab.matrix.org/matrix-org/olm) cryptographic ratchet for Matrix end-to-end encryption.

## ✨ Features

- **🔐 Cryptographic Operations**: Curve25519 and Ed25519 key generation, message signing, Olm and Megolm sessions
- **🌍 Cross-Platform**: Linux (x86_64, ARM64), macOS (Apple Silicon), Windows (x86_64)
- **📦 Maven Integration**: Automatic Rust compilation and native library packaging
- **🗑️ Memory Safety**: Proper resource management with Java's `AutoCloseable`
- **🧪 Comprehensive Testing**: AssertJ-based test suite with 71 test cases
- **🔧 GitHub CI/CD**: Multi-platform build and test pipeline

## 📋 Requirements

- **Java 25+** (Required for JNI and modern features)
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
import io.github.fherbreteau.vodozemac.account.Account;

public class MatrixCryptoExample {
    public static void main(String[] args) {
        // Create a new cryptographic account
        try (Account account = new Account()) {
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

The `Account` implements `AutoCloseable` for safe resource management:

```java
// Recommended: Use try-with-resources
try (Account account = new Account()) {
    String key = account.curve25519Key();
    // Use the account...
} // Automatically freed when block exits

// Manual management also supported
Account account = new Account();
try {
    String key = account.ed25519Key();
} finally {
    account.close(); // Explicit cleanup
}
```

## 🗃️ API Reference

### Account

Main class for Olm account management — identity keys, one-time keys, fallback keys, session creation, signing, pickle/unpickle, and dehydrated devices.

| Method | Description |
|--------|-------------|
| `IdentityKeys identityKeys()` | Get both Ed25519 and Curve25519 public keys |
| `String ed25519Key()` | Get Ed25519 public key (base64) |
| `String curve25519Key()` | Get Curve25519 public key (base64) |
| `String sign(String message)` | Sign a message with Ed25519 key |
| `long maxNumberOfOneTimeKeys()` | Get max one-time keys to store |
| `OneTimeKeyGenerationResult generateOneTimeKeys(long count)` | Generate one-time keys |
| `long storedOneTimeKeyCount()` | Get number of stored one-time keys |
| `Map getUnpublishedOneTimeKeys()` | Get unpublished one-time keys |
| `Optional<String> generateFallbackKey()` | Generate a fallback key |
| `Map getUnpublishedFallbackKey()` | Get unpublished fallback key |
| `boolean forgetFallbackKey()` | Forget previously used fallback key |
| `void markKeysAsPublished()` | Mark keys as published |
| `OlmSession createOutboundSession(...)` | Create an outbound Olm session |
| `InboundCreationResult createInboundSession(...)` | Create an inbound Olm session from a pre-key message |
| `String pickle()` / `pickle(byte[] key)` | Serialize account (plain or encrypted) |
| `static Account unpickle(...)` | Restore account from pickle |
| `static Account unpickleLegacy(...)` | Restore from libolm legacy pickle |
| `DehydratedDeviceResult toDehydratedDevice(byte[] key)` | Create a dehydrated device |
| `static Account fromDehydratedDevice(...)` | Restore from a dehydrated device |

### OlmSession

Represents an Olm session for 1-to-1 encrypted communication.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `boolean hasReceivedMessage()` | Check if a message has been received |
| `String encrypt(byte[] plaintext)` | Encrypt a message (returns JSON) |
| `byte[] decrypt(String message)` | Decrypt a message |
| `String pickle()` / `pickle(byte[] key)` | Serialize session |
| `static OlmSession unpickle(...)` | Restore from pickle |
| `static OlmSession unpickleLegacy(...)` | Restore from libolm legacy pickle |

### OutboundGroupSession

Megolm outbound group session for multi-recipient encrypted communication.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `int messageIndex()` | Get current message index |
| `String sessionKey()` | Get the session key for sharing with recipients |
| `String encrypt(byte[] plaintext)` | Encrypt a message (returns base64) |
| `String pickle()` / `pickle(byte[] key)` | Serialize session |
| `static OutboundGroupSession unpickle(...)` | Restore from pickle |
| `static OutboundGroupSession unpickleLegacy(...)` | Restore from libolm legacy pickle |

### InboundGroupSession

Megolm inbound group session for receiving encrypted group messages.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `int firstKnownIndex()` | Get the first known message index |
| `DecryptedMessage decrypt(String message)` | Decrypt a message |
| `String exportAt(int index)` | Export session key at a given index |
| `String exportAtFirstKnownIndex()` | Export session key at first known index |
| `boolean advanceTo(int index)` | Advance the session to a given index |
| `boolean connected(InboundGroupSession other)` | Check if two sessions are connected |
| `SessionOrdering compare(InboundGroupSession other)` | Compare two sessions |
| `Optional<InboundGroupSession> merge(InboundGroupSession other)` | Merge two connected sessions |
| `static InboundGroupSession importSession(...)` | Import from an exported session key |
| `String pickle()` / `pickle(byte[] key)` | Serialize session |
| `static InboundGroupSession unpickle(...)` | Restore from pickle |
| `static InboundGroupSession unpickleLegacy(...)` | Restore from libolm legacy pickle |

### Exceptions

All exceptions extend `VodozemacException` (which extends `RuntimeException`):

| Exception | Thrown when |
|-----------|------------|
| `PickleException` | Pickle/unpickle or dehydrated device errors |
| `DecryptionException` | Olm or Megolm decryption failures |
| `SessionCreationException` | Inbound session creation errors |
| `KeyException` | Key decoding or validation errors |
| `SignatureException` | Signature verification failures |

## 🏗️ Architecture

### Project Structure

```
vodozemac-java/
├── .github/
│   └── workflows/              # GitHub Actions CI/CD
│       ├── build.yml           # Multi-platform build pipeline
│       ├── test.yml            # Test pipeline
│       └── release.yml         # Release pipeline
├── rust/                       # Rust JNI bindings
│   ├── Cargo.toml              # Rust project configuration
│   └── src/
│       ├── lib.rs              # Module declarations
│       ├── errors.rs           # JNI error mapping helpers
│       ├── helpers.rs          # Shared utilities (wrap, version config)
│       ├── olm/
│       │   ├── account.rs      # Account JNI functions
│       │   └── session.rs       # OlmSession JNI functions
│       └── megolm/
│           ├── inbound_group_session.rs   # InboundGroupSession JNI
│           └── outbound_group_session.rs # OutboundGroupSession JNI
├── src/
│   ├── main/java/io/github/fherbreteau/vodozemac/
│   │   ├── account/            # Account, IdentityKeys, etc.
│   │   ├── olm/                # OlmSession, InboundCreationResult
│   │   ├── megolm/             # InboundGroupSession, OutboundGroupSession
│   │   ├── VodozemacException.java
│   │   ├── PickleException.java
│   │   ├── DecryptionException.java
│   │   ├── SessionCreationException.java
│   │   ├── KeyException.java
│   │   ├── SignatureException.java
│   │   └── NativeLibraryLoader.java
│   └── test/java/              # Test classes
├── pom.xml                     # Maven build configuration
└── README.md                   # This file
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
| 🍎 macOS | Intel | `x86_64-apple-darwin` | ❌ Not built (disabled in CI) |
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
mvn test -Dtest=AccountTest
```

### Adding New Bindings

1. **Declare native method** in Java class:
   ```java
   private native String nativeNewMethod(long ptr, String param);
   ```

2. **Implement JNI function** in the appropriate Rust module (e.g., `rust/src/olm/account.rs`):
   ```rust
   #[unsafe(no_mangle)]
   pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeNewMethod(
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
java --enable-native-access=ALL-UNNAMED \
     -Djava.library.path=/path/to/native/libs \
     -agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=5005 \
     YourMainClass
```

## 🧪 Testing

The project includes a comprehensive test suite using AssertJ:

```bash
mvn test
```

### Test Coverage

- ✅ **Account**: Creation, identity keys, one-time keys, fallback keys, signing, pickle/unpickle, dehydrated devices
- ✅ **OlmSession**: Full session lifecycle, encrypt/decrypt, pickle/unpickle, legacy pickle
- ✅ **OutboundGroupSession**: Creation, encrypt, session key, pickle/unpickle
- ✅ **InboundGroupSession**: Decrypt, export/import, advance, connected/compare/merge, pickle/unpickle
- ✅ **Error Handling**: Typed exceptions (Pickle, Decryption, SessionCreation, Key, Signature)
- ✅ **Key Validation**: Invalid key length, invalid base64, version mismatch

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

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
