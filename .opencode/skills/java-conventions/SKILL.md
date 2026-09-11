---
name: java-conventions
description: Use ONLY when writing, reviewing, or debugging Java code in this vodozemac-java repo (src/main/java, src/test/java, src/demos) — native handle classes, value classes, exception types, JUnit/AssertJ tests, checkstyle, mvn verify. Not for Rust JNI bridge code under rust/.
---

# Java Conventions (vodozemac-java)

Java 25, Maven build, Checkstyle enforced (0 violations), JaCoCo coverage ≥80%
instructions with 0 missed methods/classes. Canonical reference files:
`src/main/java/io/github/fherbreteau/vodozemac/NativeHandle.java`,
`src/main/java/io/github/fherbreteau/vodozemac/sas/Sas.java` (native handle),
`src/main/java/io/github/fherbreteau/vodozemac/sas/SasBytes.java` (value class),
`src/main/java/io/github/fherbreteau/vodozemac/exception/SasException.java` (exception).

## Native-handle classes

Classes backed by a Rust object via a raw pointer:

- `public final class X extends NativeHandle`, with a static initializer:
  `static { NativeLibraryLoader.loadLibrary(); }` as the first member.
- Constructor calls `super(nativePtr)` where the pointer comes from a
  `private static native long nativeNew()` call. Package-private constructors
  `(long nativePtr)` exist only for objects constructed from JNI.
- First statement of every public method: `checkNotClosed()`.
- Private `native` method declarations go at the bottom of the class, after
  all public methods. Naming: `nativeNew`, `native<Method>(long ptr, ...)`.
  Override `protected native void nativeFree(long ptr)`.
- Methods that transfer ownership to another object (e.g.
  `Sas#diffieHellman` → `EstablishedSas`) must invalidate the old pointer in a
  `finally` block so a failed call keeps the resource valid:
  ```java
  try {
      return nativeDiffieHellman(nativePtr, theirPublicKey);
  } finally {
      nativePtr = 0;  // only zero on success
  }
  ```
- `close()` is inherited and idempotent; never re-implement it.
- Document closed-state/consumed-state behavior in Javadoc `@throws IllegalStateException`.
- Use try-with-resources in every usage and test.

## Value classes

Immutable result types (e.g. `SasBytes`, `OlmMessage`, `DecryptedMessage`,
`IdentityKeys`): `final` class, `final` fields, package-private constructor,
defensive copies of arrays in accessors (`return rawBytes.clone();`),
`equals`/`hashCode`/`toString` overrides, pattern-match `instanceof` in
`equals` (`if (!(o instanceof SasBytes sasBytes))`). No native handle.

## Naming and style

- Fluent accessors, no `get` prefix: `session.sessionId()`, `message.ciphertext()`.
- Validate parameters: `Objects.requireNonNull(name, "name");`
- Javadoc on every public class and method; `@author François HERBRETEAU`
  on all classes; `@throws` for checked exceptions and `IllegalStateException`.
- No inline code comments unless explicitly requested.
- `final` classes everywhere possible (Checkstyle `FinalClass` rule: classes
  with only private constructors must be `final`).
- Ordered imports: `java.*` group first, then project imports; no wildcards.

## Exceptions

- Extend the closest `VodozemacException` subclass in
  `io.github.fherbreteau.vodozemac.exception` (e.g. `KeyException`,
  `DecryptionException`, `SasException`); add a new subclass only for a new
  error domain. Subclasses are non-final `public class` with exactly two
  constructors: `(String message)` and `(String message, Throwable cause)`.
- Never expose Rust error strings outside typed exceptions.

## Tests (JUnit 6 + AssertJ)

- Test class per main class, mirroring the package (e.g. `Sas` →
  `src/test/java/io/github/fherbreteau/vodozemac/sas/SasTest.java`).
- AssertJ only: `assertThat`, `assertThatThrownBy`, `assertThatCode`;
  add `.as("description")` for non-obvious assertions.
- Always assert the closed state after try-with-resources:
  ```java
  assertThatThrownBy(copy::publicKey)
          .isInstanceOf(IllegalStateException.class)
          .hasMessage("Sas has been closed");
  copy.close(); // idempotent
  ```
- Cover: happy path, error/exception paths, equals/hashCode for value
  classes, closed-state behavior, round-trips (encrypt → decrypt, pickle →
  unpickle) between two parties (alice/bob constants).

## Verification (Java lane)

```bash
mvn verify          # compile + Rust build + tests + checkstyle + coverage
mvn test -Dtest=SasTest          # single test class
mvn checkstyle:check             # checkstyle only
mvn package -DskipRustBuild=true # Java-only, reuses prebuilt native lib
```

Gate: checkstyle 0 violations; JaCoCo ≥80% instructions and 0 missed
methods/classes. CI equivalent is `mvn verify`.
