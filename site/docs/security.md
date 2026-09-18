# Security

## Reporting vulnerabilities

**Please do NOT report security vulnerabilities through public GitHub
issues.** Follow the responsible disclosure process described in
[SECURITY.md](https://github.com/fherbreteau/vodozemac-java/blob/main/SECURITY.md)
(email or encrypted Matrix message).

## Key material and heap exposure

Secret material (private keys, `pickle()` outputs, decrypted payloads) is
exposed to Java as immutable `String`s or `byte[]`s, which the JVM may keep
in memory indefinitely; they are captured by heap and core dumps. See the
[key material handling guide](guide/key-material.md) for mitigations.

## Repository security posture

Secret scanning (push protection + validity checks), Dependabot, CodeQL
(Java + Rust), Trivy and `cargo audit` are all enabled — the complete
capability table lives in
[SECURITY.md](https://github.com/fherbreteau/vodozemac-java/blob/main/SECURITY.md).
