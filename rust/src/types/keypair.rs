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
}
