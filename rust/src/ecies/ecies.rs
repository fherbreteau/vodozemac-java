use std::mem::forget;

use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::Curve25519PublicKey;
use vodozemac::ecies::{Ecies, EstablishedEcies, InitialMessage};

use crate::errors::{throw_ecies_error, throw_key_error};
use crate::helpers::{box_to_jlong, catch_panic, check_ptr, native_free};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let ecies = Ecies::new();
        Ok(box_to_jlong(ecies))
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeWithInfo(
    mut env: EnvUnowned,
    _class: JClass,
    info: JString,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let info = info.to_string();

        let ecies = Ecies::with_info(&info);
        Ok(box_to_jlong(ecies))
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativePublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let ecies = unsafe { &*(ptr as *const Ecies) };

            let public_key = ecies.public_key().to_base64();
            let result = env.new_string(public_key)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeEstablishOutboundChannel(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    their_public_key: JString,
    initial_plaintext: JByteArray,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            let their_public_key_str = their_public_key.to_string();
            let their_public_key = Curve25519PublicKey::from_base64(&their_public_key_str)
                .map_err(|e| throw_key_error(env, e))?;
            let initial_plaintext = env.convert_byte_array(initial_plaintext)?;
            check_ptr(env, ptr)?;
            let ecies = unsafe { Box::from_raw(ptr as *mut Ecies) };

            let creation_result = ecies
                .establish_outbound_channel(their_public_key, &initial_plaintext)
                .map_err(|e| throw_ecies_error(env, e))?;
            let established_ecies = Box::new(creation_result.ecies);
            let established_ecies_ptr = &*established_ecies as *const EstablishedEcies as jlong;
            let message = creation_result.message.encode();
            let message_str = env.new_string(message)?;
            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/ecies/OutboundCreationResult"),
                jni_sig!((nativePtr: long, initialMessage: java.lang.String) -> void),
                &[
                    JValue::Long(established_ecies_ptr),
                    JValue::Object(&message_str),
                ],
            )?;
            forget(established_ecies);
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeEstablishInboundChannel(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    message: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            let message = InitialMessage::decode(&message.to_string())
                .map_err(|e| throw_ecies_error(env, e))?;
            check_ptr(env, ptr)?;
            let ecies = unsafe { Box::from_raw(ptr as *mut Ecies) };

            let creation_result = ecies
                .establish_inbound_channel(&message)
                .map_err(|e| throw_ecies_error(env, e))?;
            let established_ecies = Box::new(creation_result.ecies);
            let established_ecies_ptr = &*established_ecies as *const EstablishedEcies as jlong;
            let plaintext_bytes = env.byte_array_from_slice(&creation_result.message)?;
            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/ecies/InboundCreationResult"),
                jni_sig!((nativePtr: long, plaintext: byte[]) -> void),
                &[
                    JValue::Long(established_ecies_ptr),
                    JValue::Object(&plaintext_bytes),
                ],
            )?;
            forget(established_ecies);
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<Ecies>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
