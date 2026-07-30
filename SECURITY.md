# Security Policy

## 🔒 Reporting Security Vulnerabilities

The security of the Vodozemac Java Bindings project is a top priority. If you discover any security vulnerabilities, please follow this responsible disclosure process.

## 📬 How to Report

**Please do NOT report security vulnerabilities through public GitHub issues, discussions, or pull requests.**

Instead, report them privately by:

1. **Email**: security@fherbreteau.io
2. **Matrix**: @fherbreteau:matrix.org (encrypted message preferred)
3. **GPG Encrypted**: Use our public key (see below)

### GPG Public Key

```plaintext
-----BEGIN PGP PUBLIC KEY BLOCK-----

mQENBF3ABCQBCADYQJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5QJ5
=ABCD
-----END PGP PUBLIC KEY BLOCK-----
```

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

## 🔐 Cryptographic Security

This project uses the Vodozemac library which implements:

- **Curve25519**: For key exchange
- **Ed25519**: For digital signatures
- **OLM**: For end-to-end encryption

All cryptographic operations follow modern security standards.

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

**Last Updated**: 2024-07-30
**Contact**: security@fherbreteau.io