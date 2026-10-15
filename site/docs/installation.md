# Installation

## Requirements

- **Java 25+** — required at runtime (the native library is loaded from the JAR, no system libolm needed)
- A Matrix client or application able to use Maven Central artifacts

## Maven dependency

Add to your `pom.xml` (see the
[Maven Central page](https://central.sonatype.com/artifact/io.github.fherbreteau/vodozemac-java)
for the latest version):

```xml
<dependency>
    <groupId>io.github.fherbreteau</groupId>
    <artifactId>vodozemac-java</artifactId>
    <version>1.0.0</version>
</dependency>
```

The native library for your platform is packaged inside the JAR and extracted
to a private, owner-only temporary directory on first use — there is nothing
to install besides the dependency.

## Java Platform Module System

The library is a proper JPMS module named
`io.github.fherbreteau.vodozemac`. In a modular application, declare the
module in your `module-info.java`:

```java
requires io.github.fherbreteau.vodozemac;
```

The library remains fully usable on the classpath in non-modular
applications.

## From source

Building the project requires a JDK 25, Maven 3.6.3+ and a Rust toolchain:

```bash
git clone https://github.com/fherbreteau/vodozemac-java.git
cd vodozemac-java
mvn clean package
```

## Platform support

| OS | Architectures |
|----|---------------|
| Linux | x86_64, ARM64 |
| macOS | Intel, Apple Silicon |
| Windows | x86_64, ARM64 |
