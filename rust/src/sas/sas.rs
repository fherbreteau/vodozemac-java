use std::mem::forget;

use jni::objects::{JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::Curve25519PublicKey;
use vodozemac::sas::{EstablishedSas, Sas};

use crate::errors::throw_key_error;
use crate::helpers::{box_to_jlong, catch_panic, check_ptr, native_free};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_Sas_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let sas = Sas::new();

        Ok(box_to_jlong(sas))
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

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
            let result = env.new_string(public_key)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_Sas_nativeDiffieHellman(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    their_public_key: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let their_public_key_str = their_public_key.to_string();
            let their_public_key = Curve25519PublicKey::from_base64(&their_public_key_str)
                .map_err(|e| throw_key_error(env, e))?;
            let sas = unsafe { Box::from_raw(ptr as *mut Sas) };

            let established_sas = sas
                .diffie_hellman(their_public_key)
                .map_err(|e| throw_key_error(env, e))?;
            let established_sas = Box::new(established_sas);
            let established_sas_ptr = &*established_sas as *const EstablishedSas as jlong;
            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/sas/EstablishedSas"),
                jni_sig!((nativePtr: long) -> void),
                &[JValue::Long(established_sas_ptr)],
            )?;
            forget(established_sas);
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_Sas_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<Sas>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
