use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use vodozemac::Ed25519Keypair;

use crate::helpers::{
    box_to_jlong, catch_panic, check_ptr, from_json, json_to_jstring, native_free,
};
use crate::types::to_java_base64_value;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |_env| {
            let keypair = Ed25519Keypair::new();

            Ok(box_to_jlong(keypair))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativePublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let keypair = unsafe { &*(ptr as *const Ed25519Keypair) };

            let ed25519 = to_java_base64_value(env, &(keypair.public_key()))?;
            Ok(ed25519.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeSign(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    message: JByteArray,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let keypair = unsafe { &*(ptr as *const Ed25519Keypair) };
            let msg = env.convert_byte_array(message)?;

            let signature = keypair.sign(&msg);
            let signature = to_java_base64_value(env, &signature)?;
            Ok(signature.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<Ed25519Keypair>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativePickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let keypair = unsafe { &*(ptr as *const Ed25519Keypair) };

            json_to_jstring(env, keypair)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str: String = pickle_data.to_string();

            let pickle_data: Ed25519Keypair = from_json(env, &pickle_str)?;
            Ok(box_to_jlong(pickle_data))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::get_jvm;

    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeFree as jni_free;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeNew as jni_new;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativePickle as jni_pickle;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativePublicKey as jni_public_key;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeSign as jni_sign;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519KeyPair_nativeUnpickle as jni_unpickle;

    /// Calls a JNI export with a fresh unowned env view of the current
    /// attachment, mirroring how the JVM invokes native methods.
    unsafe fn call<R>(env: &mut jni::Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_keypair_sign_verify_roundtrip() {
        let keypair = Ed25519Keypair::new();
        let message = b"Hello from test";

        let signature = keypair.sign(message);

        keypair
            .public_key()
            .verify(message, &signature)
            .expect("Signature should verify against the key pair public key");
        keypair
            .public_key()
            .verify(b"other message", &signature)
            .expect_err("Signature should not verify against another message");
    }

    #[test]
    fn test_keypair_pickle_roundtrip() {
        let keypair = Ed25519Keypair::new();

        let json = serde_json::to_string(&keypair).expect("Should serialize to JSON");
        let restored: Ed25519Keypair =
            serde_json::from_str(&json).expect("Should deserialize from JSON");

        assert_eq!(
            restored.public_key(),
            keypair.public_key(),
            "Restored key pair should have the same public key"
        );
        assert_eq!(
            restored.sign(b"message"),
            keypair.sign(b"message"),
            "Restored key pair should produce the same signature"
        );
    }

    #[test]
    fn test_jni_lifecycle_roundtrip() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let ptr = unsafe { call(env, |unowned, class| jni_new(unowned, class)) };
            assert!(ptr != 0, "nativeNew should return a valid native pointer");

            let key = unsafe { call(env, |unowned, class| jni_public_key(unowned, class, ptr)) };
            assert!(!key.is_null(), "nativePublicKey should return a Java key");

            let byte_array = env.byte_array_from_slice(b"Hello Matrix")?;
            let signature = unsafe {
                call(env, |unowned, class| {
                    jni_sign(unowned, class, ptr, byte_array)
                })
            };
            assert!(!signature.is_null(), "nativeSign should return a signature");

            let pickle = unsafe { call(env, |unowned, class| jni_pickle(unowned, class, ptr)) };
            let pickle_string = unsafe { JString::from_raw(env, pickle) }.try_to_string(env)?;
            assert!(
                pickle_string.starts_with('{') && pickle_string.ends_with('}'),
                "nativePickle should return a JSON object"
            );

            let unpickled_ptr = unsafe {
                let pickle_jstring = env.new_string(&pickle_string)?;
                call(env, |unowned, class| {
                    jni_unpickle(unowned, class, pickle_jstring)
                })
            };
            assert!(
                unpickled_ptr != 0,
                "nativeUnpickle should restore the key pair"
            );

            let repickled = unsafe {
                call(env, |unowned, class| {
                    jni_pickle(unowned, class, unpickled_ptr)
                })
            };
            let repickled_string =
                unsafe { JString::from_raw(env, repickled) }.try_to_string(env)?;
            assert_eq!(
                pickle_string, repickled_string,
                "The restored key pair should pickle identically"
            );

            unsafe {
                native_free::<Ed25519Keypair>(env, unpickled_ptr);
                call(env, |unowned, class| jni_free(unowned, class, ptr));
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_jni_null_pointer_throws() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let key = unsafe { call(env, |unowned, class| jni_public_key(unowned, class, 0)) };
            assert!(key.is_null(), "A null native pointer should produce no key");
            assert!(env.exception_check(), "check_ptr should throw");
            env.exception_clear();

            let byte_array = env.byte_array_from_slice(b"message")?;
            let signature = unsafe {
                call(env, |unowned, class| {
                    jni_sign(unowned, class, 0, byte_array)
                })
            };
            assert!(signature.is_null());
            assert!(env.exception_check());
            env.exception_clear();

            let pickle = unsafe { call(env, |unowned, class| jni_pickle(unowned, class, 0)) };
            assert!(pickle.is_null());
            assert!(env.exception_check());
            env.exception_clear();
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_jni_unpickle_invalid_json_throws() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let ptr = unsafe {
                let invalid = env.new_string("not-json")?;
                call(env, |unowned, class| jni_unpickle(unowned, class, invalid))
            };
            assert_eq!(ptr, 0, "Unpickling invalid JSON should produce no key pair");
            assert!(env.exception_check(), "from_json should throw");
            env.exception_clear();
            Ok(())
        })
        .expect("JVM test failed");
    }
}
