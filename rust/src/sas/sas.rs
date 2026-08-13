use jni::objects::{JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::Curve25519PublicKey;
use vodozemac::sas::Sas;

use crate::errors::throw_key_error;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_Sas_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let sas = Box::new(Sas::new());

        Ok(Box::into_raw(sas) as jlong)
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
        let sas = unsafe { &*(ptr as *const Sas) };

        let public_key = sas.public_key().to_base64();
        let result = env.new_string(public_key)?;
        Ok(result.into_raw())
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
        let sas = unsafe { Box::from_raw(ptr as *mut Sas) };
        let their_public_key_str = their_public_key.to_string();
        let their_public_key = Curve25519PublicKey::from_base64(&their_public_key_str)
            .map_err(|e| throw_key_error(env, e))?;

        let established_sas = sas
            .diffie_hellman(their_public_key)
            .map_err(|e| throw_key_error(env, e))?;
        let established_sas_ptr = Box::into_raw(Box::new(established_sas)) as jlong;
        let result = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/sas/EstablishedSas"),
            jni_sig!((nativePtr: long) -> void),
            &[JValue::Long(established_sas_ptr)],
        )?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_Sas_nativeFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut Sas);
    }
}
