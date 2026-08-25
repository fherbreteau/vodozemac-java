use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jint, jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::ecies::{EstablishedEcies, Message};

use crate::errors::throw_ecies_error;
use crate::helpers::{check_ptr, native_free};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativePublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let ecies = unsafe { &*(ptr as *const EstablishedEcies) };

        let public_key = ecies.public_key().to_base64();
        let result = env.new_string(public_key)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeCheckCode(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let ecies = unsafe { &*(ptr as *const EstablishedEcies) };

        let check_code = ecies.check_code();
        let bytes: JByteArray = env.byte_array_from_slice(check_code.as_bytes())?;
        let digit = jint::from(check_code.to_digit());
        let result = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/ecies/CheckCode"),
            jni_sig!((bytes: byte[], digit: int) -> void),
            &[JValue::Object(&bytes), JValue::Int(digit)],
        )?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeEncrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    plaintext: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let ecies = unsafe { &mut *(ptr as *mut EstablishedEcies) };
        let plaintext = env.convert_byte_array(plaintext)?;

        let message = ecies.encrypt(&plaintext).encode();
        let result = env.new_string(&message)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeDecrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    message: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let ecies = unsafe { &mut *(ptr as *mut EstablishedEcies) };
        let message =
            Message::decode(&message.to_string()).map_err(|e| throw_ecies_error(env, e))?;

        let plaintext = ecies
            .decrypt(&message)
            .map_err(|e| throw_ecies_error(env, e))?;
        let result = env.byte_array_from_slice(&plaintext)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<EstablishedEcies>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
