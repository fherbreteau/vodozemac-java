---
name: rust-jni-conventions
description: Use ONLY when writing, reviewing, or debugging Rust JNI bridge code in this vodozemac-java repo (rust/src) — JNI functions, helpers.rs, classes.rs, errors.rs error mapping, pointer lifecycle, cargo clippy/fmt/test. Not for Java-side code under src/.
---

# Rust JNI Bridge Conventions (vodozemac-java)

Rust stable (see `rust-toolchain.toml`), `jni` crate 0.22, wrapping
vodozemac 0.10. Canonical reference pair:
`src/main/java/io/github/fherbreteau/vodozemac/sas/Sas.java` (Java side) ↔
`rust/src/sas/sas.rs` (Rust side). Shared helpers live in `rust/src/helpers.rs`.

## JNI function naming

Every Rust function implements a `private native` declaration on the Java
side. The name must match exactly — signature drift breaks the link:

```
Java_io_github_fherbreteau_vodozemac_<module>_<Class>_native<Method>
```

with `#[unsafe(no_mangle)]` and `pub extern "system" fn`. The Java-side
declaration (e.g. `private native EstablishedSas nativeDiffieHellman(long ptr,
String theirPublicKey)`) maps to parameters `(mut env: EnvUnowned, _class:
JClass, ptr: jlong, their_public_key: JString)`. When adding/renaming a
method, update BOTH sides in the same change and re-check the mangled name.

## Standard function shape

Every JNI function follows the same wrapper:

```rust
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_Sas_nativePublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let sas = unsafe { &*(ptr as *const Sas) };
            let public_key = sas.public_key().to_base64();
            string_to_jstring(env, public_key)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
```

## Pointer lifecycle

- `check_ptr(env, ptr)?` first in every function that receives a handle.
- Accessors borrow: `unsafe { &*(ptr as *const T) }` — ownership stays with Java.
- Consuming methods (Java sets `nativePtr = 0` after the call) take
  ownership: `unsafe { Box::from_raw(ptr as *mut T) }`. Java zeroes
  `nativePtr` in a `finally` block — only-safe-on-success invariant; if the
  Rust call errors after `from_raw`, the box is still dropped, so make sure
  the Java side zeros the pointer only via `finally` after a successful
  transfer (see `Sas#diffieHellman`).
- Construction from Rust: `box_to_jlong(value)` for the Java-side
  `super(nativePtr)` constructor; for result objects built mid-call use
  `RawBox::new(...)` + `as_jlong()` in `env.new_object`, then `leak()` only
  after `new_object` succeeds (RawBox's `Drop` frees on error paths).
- Freeing: always route through `native_free::<T>(env, ptr)` from helpers.
- Reuse helpers instead of re-implementing: `box_to_jlong`, `check_ptr`,
  `catch_panic`, `string_to_jstring`, `json_to_jstring`, `from_json`,
  `native_free`, `RawBox`, `wrap` (32-byte key check),
  `olm_session_config_from_version` / `megolm_session_config_from_version`.

## Constructing Java objects

- Class-name constants live in `rust/src/classes.rs` as
  `jni_str!("io/github/fherbreteau/vodozemac/...")` `&JNIStr` constants.
- Build with `env.new_object(CLASS, jni_sig!((field: type) -> void), &[...])`.
- Strings: `env.new_string(...)` or the `string_to_jstring` helper;
  class paths/messages use `jni_str!` / `JNIString::from`.
- New Java classes constructed from JNI need a new constant in `classes.rs`.

## Error mapping (rust/src/errors.rs)

- Convert Rust errors to typed Java exceptions: call the matching
  `throw_*` function and return `Err(jni::errors::Error::JavaException)`
  via `?`. Available: `throw_pickle_error`, `throw_decryption_error`,
  `throw_encryption_error`, `throw_session_creation_error`,
  `throw_key_error`, `throw_signature_error`, `throw_ecies_error`,
  `throw_conversion_error`, `throw_sas_error`, `throw_invalid_count_error`,
  `throw_megolm_decryption_error`.
- New exception type = Java class in `io.github.fherbreteau.vodozemac.exception`
  + class constant in `errors.rs` + `throw_typed!(throw_x_error, X_EXCEPTION);`
- Megolm errors may carry a signature error: use the `throw_with_signature`
  split (`SignatureSplitError` trait) so signature failures surface as
  `SignatureException`.
- Panics are caught by `catch_panic` and surface as `ConversionException` —
  never let a panic unwind through JNI.

## Rust unit tests

Some Rust modules carry `#[cfg(test)]` tests that boot an in-process JVM via
`helpers::get_jvm()` (classpath `target/classes`) — keep that pattern for
tests exercising JNI round-trips from the Rust side. Pure logic can be tested
without the JVM.

## Verification (Rust lane)

```bash
cargo clippy --manifest-path rust/Cargo.toml   # 0 warnings required
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo test --manifest-path rust/Cargo.toml
cargo build --release --manifest-path rust/Cargo.toml
```

Gate: clippy 0 warnings, fmt clean, all tests pass. The Maven build
(`mvn verify`) compiles Rust for the host platform by default; CI builds each
target platform natively.
