use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use vodozemac::pk_encryption::{Message, PkDecryption};
use vodozemac::{Curve25519PublicKey, Curve25519SecretKey, base64_decode, base64_encode};

use crate::errors::{
    throw_decryption_error, throw_generic_error, throw_key_error, throw_pickle_error,
};
use crate::helpers::{check_ptr, wrap};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let pk_decryption = Box::new(PkDecryption::new());

        Ok(Box::into_raw(pk_decryption) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativeFromKey(
    mut env: EnvUnowned,
    _class: JClass,
    key: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let bytes = base64_decode(key.to_string()).map_err(|e| throw_generic_error(env, e))?;
        let bytes: [u8; 32] = wrap(bytes)?;
        let secret_key = Curve25519SecretKey::from_slice(&bytes);

        let pk_decryption = Box::new(PkDecryption::from_key(secret_key));

        Ok(Box::into_raw(pk_decryption) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativeSecretKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let pk_decryption = unsafe { &*(ptr as *const PkDecryption) };

        let secret_key = pk_decryption.secret_key().to_bytes().to_vec();
        let result = env.new_string(base64_encode(secret_key))?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativePublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let pk_decryption = unsafe { &*(ptr as *const PkDecryption) };

        let public_key = pk_decryption.public_key().to_base64();
        let result = env.new_string(public_key)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativeDecrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    ciphertext: JString,
    mac: JString,
    ephemeral_key: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let pk_decryption = unsafe { &*(ptr as *const PkDecryption) };
        let ciphertext =
            base64_decode(ciphertext.to_string()).map_err(|e| throw_generic_error(env, e))?;
        let mac = base64_decode(mac.to_string()).map_err(|e| throw_generic_error(env, e))?;
        let ephemeral_key = Curve25519PublicKey::from_base64(&ephemeral_key.to_string())
            .map_err(|e| throw_key_error(env, e))?;

        let message = Message {
            ciphertext,
            mac,
            ephemeral_key,
        };

        let plaintext = pk_decryption
            .decrypt(&message)
            .map_err(|e| throw_decryption_error(env, e))?;
        let result = env.byte_array_from_slice(&plaintext)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativeUnpickleLegacy(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    pickle_key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let pickle_key = env.convert_byte_array(pickle_key)?;

        let from_olm = PkDecryption::from_libolm_pickle(&pickle_str, &pickle_key)
            .map_err(|e| throw_pickle_error(env, e))?;
        let pk_decryption = Box::new(from_olm);
        Ok(Box::into_raw(pk_decryption) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativePickleLegacy(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    key: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let pk_decryption = unsafe { &*(ptr as *const PkDecryption) };
        let key = wrap(env.convert_byte_array(key)?)?;

        let pickle = pk_decryption
            .to_libolm_pickle(&key)
            .map_err(|e| throw_pickle_error(env, e))?;
        let result = env.new_string(pickle)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        check_ptr(env, ptr)?;
        let _ = unsafe { Box::from_raw(ptr as *mut PkDecryption) };
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
