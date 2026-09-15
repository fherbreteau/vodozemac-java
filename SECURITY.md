# Security Policy

## 🔒 Reporting Security Vulnerabilities

The security of the Vodozemac Java Bindings project is a top priority. If you discover any security vulnerabilities, please follow this responsible disclosure process.

## 📬 How to Report

**Please do NOT report security vulnerabilities through public GitHub issues, discussions, or pull requests.**

Instead, report them privately by:

1. **Email**: fherbreteau@gmail.com
2. **Matrix**: @fherbreteau:matrix.org (encrypted message preferred)

## 🛡️ Supported Versions

Security updates are provided for the following versions:

| Version | Supported          | Security Updates |
|---------|--------------------|------------------|
| 1.x     | ✅ Actively Supported | ✅ Yes |
| 0.x     | ❌ Not Supported    | ❌ No |

## 🕒 Response Process

1. **Acknowledgment**: You will receive an acknowledgment within 24 hours
2. **Assessment**: Our security team will assess the vulnerability within 72 hours
3. **Patch Development**: Critical vulnerabilities will be patched within 7 days
4. **Disclosure**: Coordinated disclosure with credit to reporter

## ⚠️ Security Best Practices

### For Users

- Always use the latest version
- Verify checksums of downloaded files
- Use HTTPS for all communications
- Keep your Java and Rust toolchains updated
- Review dependencies regularly

### For Developers

- Follow secure coding practices
- Use parameterized queries to prevent injection
- Validate all inputs and outputs
- Implement proper error handling
- Use cryptographic best practices

## 🛡️ Repository Security Settings

The following repository-level protections are enabled under
*Settings > Code security and analysis*:

| Capability | Status |
|------------|--------|
| Secret scanning | ✅ Enabled |
| Secret scanning push protection | ✅ Enabled |
| Secret scanning validity checks | ✅ Enabled |
| Dependabot security updates | ✅ Enabled |
| CodeQL + Trivy (scheduled) | ✅ Enabled (`.github/workflows/security.yml`) |
| cargo-audit | ✅ Enabled (`.github/workflows/test.yml`) |

**Push protection** blocks pushes containing recognized secret patterns
(GitHub tokens, cloud provider keys, etc.) on every branch at push time,
so a leaked credential cannot sit unnoticed in a feature branch or fork.
Overriding a blocked push is possible with a stated reason and should be
reserved for false positives and test fixtures.

**Validity checks** validate detected secrets against the issuing
provider, so alerts are marked active (live credential — act now) or
inactive (revoked credential — archive), keeping triage focused on
credentials that actually need rotation.

**Partner alerts**: because this is a public repository, GitHub always
notifies the secret's issuing partner when secret scanning detects a
provider-pattern secret, independently of the repository settings above.
Partners may revoke the exposed credential directly, so any secret
pushed to a public branch should be considered compromised and rotated
even if it is removed afterwards.

## 🔐 Cryptographic Security

This project uses the Vodozemac library which implements:

- **Curve25519**: For key exchange
- **Ed25519**: For digital signatures
- **OLM**: For end-to-end encryption

All cryptographic operations follow modern security standards.

## 🧠 Key Material and Heap Exposure

Secret material (private keys, `pickle()` outputs, decrypted payloads) is
exposed to Java as immutable `String`s or `byte[]`s. The JVM may keep copies
of these objects in memory indefinitely (GC heuristics, string interning) and
they are captured by heap dumps and core dumps. This is inherent to the JVM:
there is no `String`-erasing API, and native `free()` only releases the Rust
copies.

- Never enable heap dump capture (`-XX:+HeapDumpOnOutOfMemoryError`, `jmap`,
  `jcmd GC.heap_dump`) or core dumps on hosts that process key material
  without reviewing where the dumps are stored and who can read them.
- Restrict physical access to machines handling key material (disable
  swap or use encrypted swap, protect memory with OS controls).
- Prefer the `byte[]`-based APIs where available (e.g. encrypted pickles
  accept a `byte[]` key) and zero such buffers after use when possible.
- Close native handles (try-with-resources) so native copies of secret
  material are released promptly; leaked handles keep their native memory
  until the process exits.

## 📋 Security Checklist

- [x] Secure coding practices
- [x] Regular dependency updates
- [x] Cryptographic best practices
- [x] Secure build process
- [x] Responsible disclosure policy
- [x] Security documentation

## 🤝 Responsible Disclosure

We follow responsible disclosure principles:

1. Private reporting of vulnerabilities
2. Coordinated patch release
3. Public disclosure after patch
4. Credit to security researchers

## 📄 Legal

By reporting security vulnerabilities, you agree to:

- Keep the vulnerability confidential until patch release
- Allow us reasonable time to develop and test fixes
- Not exploit the vulnerability for malicious purposes
- Follow our responsible disclosure process

## 🙏 Acknowledgments

We appreciate the security community's efforts in making our software more secure. Security researchers who responsibly disclose vulnerabilities will be acknowledged in our release notes (unless anonymity is requested).

---

**Last Updated**: 2026-08-13
**Contact**: fherbreteau@gmail.com