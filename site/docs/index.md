# Vodozemac Java

**Java bindings for the [Vodozemac](https://github.com/matrix-org/vodozemac) Matrix cryptography library.**

[![Maven Central](https://img.shields.io/maven-central/v/io.github.fherbreteau/vodozemac-java.svg)](https://central.sonatype.com/artifact/io.github.fherbreteau/vodozemac-java)
[![Quality gate status](https://sonarcloud.io/api/project_badges/measure?project=fherbreteau_vodozemac-java&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=fherbreteau_vodozemac-java)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Java 25+](https://img.shields.io/badge/Java-25+-red.svg)](https://www.oracle.com/java/technologies/downloads/)

Vodozemac Java provides Java Native Interface (JNI) bindings for the
[Vodozemac](https://github.com/matrix-org/vodozemac) Rust library, which
implements the [OLM](https://gitlab.matrix.org/matrix-org/olm) cryptographic
ratchet for Matrix end-to-end encryption.

## Features

- **Cryptographic operations** — Curve25519 and Ed25519 key generation, message signing, Olm and Megolm sessions, session keys
- **SAS verification** — Short Authentication String with emoji and decimal rendering
- **ECIES channels** — Elliptic Curve Integrated Encryption Scheme for QR-code-based device login ([MSC3886](https://github.com/matrix-org/matrix-spec-proposals/pull/3886))
- **PK encryption** — Megolm key backup using Curve25519-AES-SHA2 hybrid encryption
- **libolm compatibility** — legacy pickle support for migrating from libolm
- **Cross-platform** — Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), Windows (x86_64, ARM64)
- **Memory safety** — `AutoCloseable` handles backed by a `Cleaner` safety net, thread-safe native calls

## Quick start

Add the dependency and manage your first account:

```java
import io.github.fherbreteau.vodozemac.account.Account;
import io.github.fherbreteau.vodozemac.types.Ed25519Signature;

try (Account account = new Account()) {
    Ed25519Signature signature = account.sign("Hello Matrix!");
    boolean valid = account.ed25519Key().verify("Hello Matrix!", signature);
    System.out.println("Signature valid: " + valid);
} // Native resources freed automatically
```

See [Installation](installation.md) and [Getting started](guide/getting-started.md).

## Samples

Runnable, feature-by-feature walkthroughs — [Olm sessions](samples/olm.md),
[Megolm group sessions](samples/megolm.md), [SAS verification](samples/sas.md),
[ECIES channels](samples/ecies.md), [PK encryption](samples/backup.md) and
[cross-signing keys](samples/cross-signing.md) — are all derived from the
demos shipped under `src/demos/java` (run with `mvn -Psample test`).

## API documentation

The complete Javadoc for every released version is available on
[javadoc.io](https://javadoc.io/doc/io.github.fherbreteau/vodozemac-java),
and the [API reference](api.md) page summarizes the full API surface.
