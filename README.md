# Vodozemac Java Bindings

Java bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Matrix cryptography library.

## Overview

This project provides Java bindings for the Vodozemac Rust library, which implements the [OLM](https://gitlab.matrix.org/matrix-org/olm) cryptographic ratchet for Matrix end-to-end encryption.

## Features

- **Java Native Interface (JNI) bindings** for Vodozemac's cryptographic functions
- **Cross-platform support** for Linux, macOS, and Windows
- **Maven-based build system** with automatic Rust compilation
- **Memory-safe resource management** using Java's `AutoCloseable` interface

## Prerequisites

- Java 17 or higher
- Maven 3.8+
- Rust toolchain (cargo, rustc)
- Git

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/fherbreteau/vodozemac-java.git
cd vodozemac-java

# Build the project (this will compile both Rust and Java code)
mvn clean package
```

### As a Dependency

Add this to your Maven `pom.xml`:

```xml
<dependency>
    <groupId>io.github.fherbreteau</groupId>
    <artifactId>vodozemac-java</artifactId>
    <version>1.0.0</version>
</dependency>
```

## Usage

### Basic Example

```java
import io.github.fherbreteau.vodozemac.VodozemacAccount;

public class Example {
    public static void main(String[] args) {
        // Create a new account (automatically loads native library)
        try (VodozemacAccount account = new VodozemacAccount()) {
            // Get cryptographic keys
            String curve25519Key = account.curve25519Key();
            String ed25519Key = account.ed25519Key();

            System.out.println("Curve25519 Key: " + curve25519Key);
            System.out.println("Ed25519 Key: " + ed25519Key);

            // Sign a message
            String message = "Hello Matrix!";
            String signature = account.sign(message);

            System.out.println("Signature: " + signature);
        }
    }
}
```

### Resource Management

The `VodozemacAccount` implements `AutoCloseable`, so it's recommended to use try-with-resources:

```java
try (VodozemacAccount account = new VodozemacAccount()) {
    // Use the account...
} // Automatically freed when the block exits
```

## API Reference

### VodozemacAccount

The main class that provides access to cryptographic operations.

#### Methods

- `VodozemacAccount()` - Creates a new cryptographic account
- `String curve25519Key()` - Returns the Curve25519 public key (base64 encoded)
- `String ed25519Key()` - Returns the Ed25519 public key (base64 encoded)
- `String sign(String message)` - Signs a message using the account's private key
- `void close()` - Frees native resources (called automatically with try-with-resources)

## Architecture

### Project Structure

```
vodozemac-java/
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

1. **Rust Compilation**: The Maven build automatically compiles the Rust code into a native library
2. **Java Compilation**: Java classes are compiled with JNI native method declarations
3. **Resource Packaging**: The native library is packaged in the JAR file
4. **Native Loading**: At runtime, the native library is extracted and loaded automatically

### Cross-Platform Support

The project includes Maven profiles for different platforms:

- **Linux**: x86_64 and aarch64
- **macOS**: Intel (x86_64) and Apple Silicon (aarch64)
- **Windows**: x86_64

## Development

### Building

```bash
# Clean build
mvn clean package

# Skip Rust compilation (for Java-only changes)
mvn package -DskipRustBuild=true

# Run tests
mvn test
```

### Adding New Bindings

1. Add the native method declaration in the Java class
2. Implement the JNI function in `rust/src/lib.rs`
3. Update the Rust `Cargo.toml` if new dependencies are needed
4. Rebuild the project

### Debugging

For JNI debugging, you can set the following JVM options:

```bash
java -Djava.library.path=/path/to/native/libs -agentlib:jdwp=transport=dt_socket,server=y,suspend=n,address=5005 YourMainClass
```

## Testing

The project includes unit tests that verify the Java bindings work correctly:

```bash
mvn test
```

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Vodozemac](https://github.com/matrix-org/vodozemac) - The Rust Matrix cryptography library
- [JNI](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/) - Java Native Interface
- [Maven](https://maven.apache.org/) - Build automation tool

## Support

For issues, questions, or feature requests, please open an issue on the GitHub repository.