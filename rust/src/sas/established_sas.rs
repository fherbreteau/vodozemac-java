use jni::objects::{JByteArray, JClass, JIntArray, JObjectArray, JString};
use jni::sys::{jint, jlong, jobject, jstring};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::sas::{EstablishedSas, Mac, SasBytes};

use crate::errors::{throw_generic_error, throw_invalid_count_error, throw_sas_error};
use crate::helpers::{check_ptr, native_free};

fn to_decimal_array<'local>(
    env: &mut Env<'local>,
    bytes: &SasBytes,
) -> Result<JObjectArray<'local, JString<'local>>, jni::errors::Error> {
    let decimals_array = JObjectArray::<JString>::new(env, 3, JString::null())?;
    let (d1, d2, d3) = bytes.decimals();

    for (i, d) in [d1, d2, d3].iter().enumerate() {
        let jstr = env.new_string(d.to_string())?;
        decimals_array.set_element(env, i, &jstr)?;
    }
    Ok(decimals_array)
}

fn to_emoji_array<'local>(
    env: &mut Env<'local>,
    bytes: &SasBytes,
) -> Result<JIntArray<'local>, jni::errors::Error> {
    let emoji_array = env.new_int_array(7)?;
    let jints = bytes.emoji_indices().map(|b| b as i32);

    emoji_array.set_region(env, 0, &jints)?;
    Ok(emoji_array)
}

fn to_raw_byte_array<'local>(
    env: &mut Env<'local>,
    bytes: &SasBytes,
) -> Result<JByteArray<'local>, jni::errors::Error> {
    let bytes_array = env.new_byte_array(6)?;
    let jbytes = bytes.as_bytes().map(|b| b as i8);

    bytes_array.set_region(env, 0, &jbytes)?;
    Ok(bytes_array)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeBytes(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    info: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
        let info = info.to_string();

        let bytes = established_sas.bytes(&info);
        let decimals = to_decimal_array(env, &bytes)?;
        let emoji_indices = to_emoji_array(env, &bytes)?;
        let raw_bytes = to_raw_byte_array(env, &bytes)?;

        let result = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/sas/SasBytes"),
            jni_sig!((rawBytes:byte[], emojiIndices: int[] , decimals: java.lang.String[] ) -> void),
            &[JValue::Object(&raw_bytes), JValue::Object(&emoji_indices), JValue::Object(&decimals)],
        )?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn to_byte_array<'local>(
    env: &mut Env<'local>,
    bytes: &[u8],
) -> Result<JByteArray<'local>, jni::errors::Error> {
    let result = env.new_byte_array(bytes.len())?;
    let byte_array: Vec<i8> = bytes.iter().map(|&b| b as i8).collect();

    result.set_region(env, 0, &byte_array)?;
    Ok(result)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeBytesRaw(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    info: JString,
    count: jint,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
        let info = info.to_string();
        let count = usize::try_from(count).map_err(|e| throw_generic_error(env, e))?;

        let bytes = established_sas
            .bytes_raw(&info, count)
            .map_err(|e| throw_invalid_count_error(env, e))?;

        let result = to_byte_array(env, &bytes)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeCalculateMac(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    input: JString,
    info: JString,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
        let input = input.to_string();
        let info = info.to_string();

        let mac = established_sas.calculate_mac(&input, &info);
        let mac_str = env.new_string(mac.to_base64())?;
        Ok(mac_str.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeCalculateMacInvalidBase64(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    input: JString,
    info: JString,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
        let input = input.to_string();
        let info = info.to_string();

        let mac = established_sas.calculate_mac_invalid_base64(&input, &info);
        let mac_str = env.new_string(mac)?;
        Ok(mac_str.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeVerifyMac(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    input: JString,
    info: JString,
    mac: JString,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        check_ptr(env, ptr)?;
        let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
        let input = input.to_string();
        let info = info.to_string();
        let mac = Mac::from_base64(&mac.to_string()).map_err(|e| throw_generic_error(env, e))?;

        established_sas
            .verify_mac(&input, &info, &mac)
            .map_err(|e| throw_sas_error(env, e))?;
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeOurPublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let established_sas = unsafe { &*(ptr as *const EstablishedSas) };

        let our_public_key = established_sas.our_public_key().to_base64();
        let result = env.new_string(&our_public_key)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeTheirPublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        check_ptr(env, ptr)?;
        let established_sas = unsafe { &*(ptr as *const EstablishedSas) };

        let their_public_key = established_sas.their_public_key().to_base64();
        let result = env.new_string(&their_public_key)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_sas_EstablishedSas_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<EstablishedSas>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
