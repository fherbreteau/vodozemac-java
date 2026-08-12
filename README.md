# Vodozemac Java Bindings

[![Build Status](https://github.com/fherbreteau/vodozemac-java/actions/workflows/build.yml/badge.svg)](https://github.com/fherbreteau/vodozemac-java/actions/workflows/build.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Java 25+](https://img.shields.io/badge/Java-25+-red.svg)](https://www.oracle.com/java/technologies/downloads/)
[![Maven Central](https://img.shields.io/maven-central/v/io.github.fherbreteau/vodozemac-java.svg)](https://search.maven.org/artifact/io.github.fherbreteau/vodozemac-java)

**Java bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Matrix cryptography library**

## 🚀 Overview

Vodozemac Java provides Java Native Interface (JNI) bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Rust library, which implements the [OLM](https://gitlab.matrix.org/matrix-org/olm) cryptographic ratchet for Matrix end-to-end encryption.

## ✨ Features

- **🔐 Cryptographic Operations**: Curve25519 and Ed25519 key generation, message signing, Olm and Megolm sessions, session keys
- **🔑 SAS Verification**: Short Authentication String (SAS) for key verification with emoji and decimal rendering
- **🔗 ECIES Channels**: Elliptic Curve Integrated Encryption Scheme for QR-code-based device login (MSC3886)
- **💾 PK Encryption**: Megolm key backup using Curve25519-AES-SHA2 hybrid encryption
- **🔄 libolm Compatibility**: Legacy pickle support for migrating from libolm
- **🌍 Cross-Platform**: Linux (x86_64, ARM64), macOS (Apple Silicon), Windows (x86_64)
- **📦 Maven Integration**: Automatic Rust compilation and native library packaging
- **🗑️ Memory Safety**: Proper resource management with Java's `AutoCloseable`
- **🧪 Comprehensive Testing**: AssertJ-based test suite with 100 test cases
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
| `String pickleLegacy(byte[] key)` | Serialize account to libolm pickle format |
| `static Account unpickle(...)` | Restore account from pickle |
| `static Account unpickleLegacy(...)` | Restore from libolm legacy pickle |
| `DehydratedDeviceResult toDehydratedDevice(byte[] key)` | Create a dehydrated device |
| `static Account fromDehydratedDevice(...)` | Restore from a dehydrated device |

### OlmSession

Represents an Olm session for 1-to-1 encrypted communication.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `SessionKeys sessionKeys()` | Get the keys used to establish this session |
| `OlmSessionVersion sessionConfig()` | Get the session protocol version |
| `boolean hasReceivedMessage()` | Check if a message has been received |
| `String encrypt(byte[] plaintext)` | Encrypt a message (returns JSON) |
| `byte[] decrypt(String message)` | Decrypt a message |
| `String pickle()` / `pickle(byte[] key)` | Serialize session |
| `static OlmSession unpickle(...)` | Restore from pickle |
| `static OlmSession unpickleLegacy(...)` | Restore from libolm legacy pickle |

### SessionKeys

The set of Curve25519 public keys that were used to establish an Olm session.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the globally unique session ID (SHA-256 of the three keys) |
| `String identityKey()` | Get the long-term Curve25519 identity key of the session initiator |
| `String baseKey()` | Get the ephemeral Curve25519 base key created by the initiator |
| `String oneTimeKey()` | Get the one-time Curve25519 key used to establish the session |

### OutboundGroupSession

Megolm outbound group session for multi-recipient encrypted communication.

| Method | Description |
|--------|-------------|
| `String sessionId()` | Get the session ID |
| `int messageIndex()` | Get current message index |
| `String sessionKey()` | Get the session key for sharing with recipients |
| `MegolmSessionVersion sessionConfig()` | Get the session protocol version |
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

### Sas

Short Authentication String (SAS) verification for interactive key verification between devices.

| Method | Description |
|--------|-------------|
| `String publicKey()` | Get the ephemeral Curve25519 public key |
| `EstablishedSas diffieHellman(String theirPublicKey)` | Establish shared secret (consumes this `Sas`) |

### EstablishedSas

An established SAS channel with a shared secret, used for key verification and MAC exchange.

| Method | Description |
|--------|-------------|
| `SasBytes bytes(String info)` | Generate SAS bytes for visual verification |
| `byte[] bytesRaw(String info, int count)` | Generate raw bytes (max 8160) |
| `String calculateMac(String input, String info)` | Calculate a MAC for the given input |
| `String calculateMacInvalidBase64(String input, String info)` | Calculate a MAC with libolm-compatible invalid base64 encoding |
| `void verifyMac(String input, String info, String mac)` | Verify a MAC from the other party |
| `String ourPublicKey()` | Get our Curve25519 public key |
| `String theirPublicKey()` | Get the other party's Curve25519 public key |

### SasBytes

Short authentication string bytes for visual key verification (emoji indices and decimal numbers).

| Method | Description |
|--------|-------------|
| `int[] emojiIndices()` | Get 7 emoji indices for visual verification |
| `String[] decimals()` | Get 3 decimal numbers for visual verification |
| `byte[] bytes()` | Get the raw 6 bytes of the SAS |

### Ecies

Unestablished ECIES channel for QR-code-based device login (MSC3886).

| Method | Description |
|--------|-------------|
| `Ecies()` | Create with default `MATRIX_QR_CODE_LOGIN` info |
| `static Ecies withInfo(String info)` | Create with custom application info |
| `String publicKey()` | Get the ephemeral Curve25519 public key |
| `OutboundCreationResult establishOutboundChannel(String theirPublicKey, byte[] plaintext)` | Establish outbound channel (consumes this `Ecies`) |
| `InboundCreationResult establishInboundChannel(String message)` | Establish inbound channel from initial message (consumes this `Ecies`) |

### EstablishedEcies

An established ECIES channel for encrypting and decrypting messages using ChaCha20-Poly1305.

| Method | Description |
|--------|-------------|
| `String publicKey()` | Get our Curve25519 public key |
| `CheckCode checkCode()` | Get the check code for out-of-band MITM verification |
| `String encrypt(byte[] plaintext)` | Encrypt a message (base64-encoded) |
| `byte[] decrypt(String message)` | Decrypt a base64-encoded message |

### CheckCode

A two-digit check code for out-of-band verification of an ECIES session.

| Method | Description |
|--------|-------------|
| `byte[] asBytes()` | Get the raw 2-byte check code |
| `int toDigit()` | Get the check code as a two-digit number (0–99) |

### PkEncryption

The encryption component of the PK Encryption module for Megolm key backup. Implements `m.megolm_backup.v1.curve25519-aes-sha2`.

**Warning:** The algorithm contains a critical flaw — the MAC does not authenticate the ciphertext.

| Method | Description |
|--------|-------------|
| `static PkEncryption fromKey(String publicKey)` | Create from a base64-encoded Curve25519 public key |
| `PkMessage encrypt(byte[] plaintext)` | Encrypt plaintext and return a `PkMessage` |

### PkDecryption

The decryption component of the PK Encryption module, holding a Curve25519 secret key.

| Method | Description |
|--------|-------------|
| `PkDecryption()` | Create with a fresh random Curve25519 key pair |
| `static PkDecryption fromKey(String secretKey)` | Create from a base64-encoded Curve25519 secret key |
| `String secretKey()` | Get the base64-encoded Curve25519 secret key |
| `String publicKey()` | Get the base64-encoded Curve25519 public key |
| `byte[] decrypt(PkMessage message)` | Decrypt a `PkMessage` |
| `static PkDecryption unpickleLegacy(String pickleData, byte[] pickleKey)` | Restore from a libolm legacy pickle |

### PkMessage

An encrypted message produced by `PkEncryption`, consisting of three base64-encoded components.

| Method | Description |
|--------|-------------|
| `String getCiphertext()` | Get the base64-encoded ciphertext |
| `String getMac()` | Get the base64-encoded MAC (does not authenticate the ciphertext) |
| `String getEphemeralKey()` | Get the base64-encoded ephemeral Curve25519 public key |

### Exceptions

All exceptions extend `VodozemacException` (which extends `RuntimeException`):

| Exception | Thrown when |
|-----------|------------|
| `PickleException` | Pickle/unpickle or dehydrated device errors |
| `DecryptionException` | Olm or Megolm decryption failures |
| `SessionCreationException` | Inbound session creation errors |
| `KeyException` | Key decoding or validation errors |
| `SignatureException` | Signature verification failures |
| `SasException` | SAS MAC verification failures or byte generation errors |
| `EciesException` | ECIES channel establishment or decryption errors |
| `EncryptionException` | PK encryption failures (e.g. non-contributory key) |

### Vodozemac

Utility class providing base64 encoding/decoding and access to the vodozemac library version.

| Method | Description |
|--------|-------------|
| `static String base64Encode(byte[] src)` | Encode bytes to unpadded base64 |
| `static byte[] base64Decode(String src)` | Decode base64 (padded or unpadded) to bytes |
| `static String getVersion()` | Get the vodozemac Rust crate version |

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
│       ├── utils/
│       │   └── mod.rs          # Vodozemac utility (base64, version) JNI
│       ├── olm/
│       │   ├── account.rs      # Account JNI functions
│       │   └── session.rs       # OlmSession JNI functions
│       ├── megolm/
│       │   ├── inbound_group_session.rs   # InboundGroupSession JNI
│       │   └── outbound_group_session.rs # OutboundGroupSession JNI
│       ├── sas/
│       │   ├── sas.rs          # Sas JNI functions
│       │   └── established_sas.rs # EstablishedSas JNI functions
│       ├── ecies/
│       │   ├── ecies.rs        # Ecies JNI functions
│       │   └── established_ecies.rs # EstablishedEcies JNI functions
│       └── backup/
│           ├── encryption.rs   # PkEncryption JNI functions
│           └── decryption.rs   # PkDecryption JNI functions
├── src/
│   ├── main/java/io/github/fherbreteau/vodozemac/
│   │   ├── account/            # Account, IdentityKeys, OneTimeKeyGenerationResult, DehydratedDeviceResult
│   │   ├── olm/                # OlmSession, OlmSessionVersion, SessionKeys, InboundCreationResult
│   │   ├── megolm/             # InboundGroupSession, OutboundGroupSession, MegolmSessionVersion, SessionOrdering, DecryptedMessage
│   │   ├── sas/                # Sas, EstablishedSas, SasBytes
│   │   ├── ecies/              # Ecies, EstablishedEcies, CheckCode, OutboundCreationResult, InboundCreationResult
│   │   ├── backup/             # PkEncryption, PkDecryption, PkMessage
│   │   ├── exception/          # VodozemacException, PickleException, DecryptionException, SessionCreationException, KeyException, SignatureException, SasException, EciesException, EncryptionException
│   │   ├── NativeHandle.java   # Base class for native pointer lifecycle
│   │   ├── KeyValidator.java   # 32-byte key validation utility
│   │   ├── Vodozemac.java      # Utility class (base64, version)
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

- ✅ **Account**: Creation, identity keys, one-time keys, fallback keys, signing, pickle/unpickle, dehydrated devices, legacy pickle
- ✅ **OlmSession**: Full session lifecycle, encrypt/decrypt, pickle/unpickle, legacy pickle, session keys, session config
- ✅ **OutboundGroupSession**: Creation, encrypt, session key, session config, pickle/unpickle
- ✅ **InboundGroupSession**: Decrypt, export/import, advance, connected/compare/merge, pickle/unpickle
- ✅ **SAS**: Key establishment, emoji/decimal generation, MAC calculation and verification
- ✅ **ECIES**: Outbound/inbound channel establishment, encrypt/decrypt, check code
- ✅ **PK Encryption**: Encrypt/decrypt round-trip, secret key loading, legacy libolm unpickle
- ✅ **Utility Functions**: Base64 encode/decode, vodozemac version, session keys, session config
- ✅ **Error Handling**: Typed exceptions (Pickle, Decryption, SessionCreation, Key, Signature, Sas, Ecies, Encryption)
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
