use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::pk_encryption::PkEncryption;
use vodozemac::{Curve25519PublicKey, base64_encode};

use crate::errors::{throw_encryption_error, throw_key_error};
use crate::helpers::{box_to_jlong, catch_panic, check_ptr, native_free};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkEncryption_nativeFromKey(
    mut env: EnvUnowned,
    _class: JClass,
    key: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let public_key = Curve25519PublicKey::from_base64(&key.to_string())
                .map_err(|e| throw_key_error(env, e))?;

            let encryption = PkEncryption::from(public_key);

            Ok(box_to_jlong(encryption))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkEncryption_nativeEncrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    plaintext: JByteArray,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let encryption = unsafe { &*(ptr as *const PkEncryption) };
            let data = env.convert_byte_array(plaintext)?;

            let result = encryption.encrypt(&data)
                .map_err(|e| throw_encryption_error(env, e))?;
            let ciphertext = env.new_string(base64_encode(result.ciphertext))?;
            let mac = env.new_string(base64_encode(result.mac))?;
            let ephemeral_key = env.new_string(result.ephemeral_key.to_base64())?;

            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/backup/PkMessage"),
                 jni_sig!((ciphertext: java.lang.String, mac: java.lang.String, ephemeralKey: java.lang.String) -> void),
                 &[
                    JValue::Object(&ciphertext),
                    JValue::Object(&mac),
                    JValue::Object(&ephemeral_key)])?;

            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkEncryption_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<PkEncryption>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
