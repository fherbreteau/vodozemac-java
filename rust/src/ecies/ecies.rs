use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig};
use vodozemac::Curve25519PublicKey;
use vodozemac::ecies::{Ecies, InitialMessage};

use crate::classes::{ECIES_INBOUND_CREATION_RESULT, ECIES_OUTBOUND_CREATION_RESULT};
use crate::errors::{throw_ecies_error, throw_key_error};
use crate::helpers::{
    RawBox, box_to_jlong, catch_panic, check_ptr, native_free, string_to_jstring,
};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |_env| {
            let ecies = Ecies::new();
            Ok(box_to_jlong(ecies))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeWithInfo(
    mut env: EnvUnowned,
    _class: JClass,
    info: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let info = info.try_to_string(env)?;

            let ecies = Ecies::with_info(&info);
            Ok(box_to_jlong(ecies))
        })
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
            string_to_jstring(env, public_key)
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
            check_ptr(env, ptr)?;
            let their_public_key_str = their_public_key.try_to_string(env)?;
            let their_public_key = Curve25519PublicKey::from_base64(&their_public_key_str)
                .map_err(|e| throw_key_error(env, e))?;
            let initial_plaintext = env.convert_byte_array(initial_plaintext)?;
            let ecies = unsafe { Box::from_raw(ptr as *mut Ecies) };

            let creation_result = ecies
                .establish_outbound_channel(their_public_key, &initial_plaintext)
                .map_err(|e| throw_ecies_error(env, e))?;
            let established_ecies_box = RawBox::new(creation_result.ecies);
            let message = creation_result.message.encode();
            let message_str = env.new_string(message)?;
            let result = env.new_object(
                ECIES_OUTBOUND_CREATION_RESULT,
                jni_sig!((ptr: long, initialMessage: java.lang.String) -> void),
                &[
                    JValue::Long(established_ecies_box.as_jlong()),
                    JValue::Object(&message_str),
                ],
            )?;
            established_ecies_box.leak();
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
            check_ptr(env, ptr)?;
            let message = InitialMessage::decode(&message.try_to_string(env)?)
                .map_err(|e| throw_ecies_error(env, e))?;
            let ecies = unsafe { Box::from_raw(ptr as *mut Ecies) };

            let creation_result = ecies
                .establish_inbound_channel(&message)
                .map_err(|e| throw_ecies_error(env, e))?;
            let established_ecies_box = RawBox::new(creation_result.ecies);
            let plaintext_bytes = env.byte_array_from_slice(&creation_result.message)?;
            let result = env.new_object(
                ECIES_INBOUND_CREATION_RESULT,
                jni_sig!((ptr: long, plaintext: byte[]) -> void),
                &[
                    JValue::Long(established_ecies_box.as_jlong()),
                    JValue::Object(&plaintext_bytes),
                ],
            )?;
            established_ecies_box.leak();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecies_new_generates_public_key() {
        let ecies = Ecies::new();
        let public_key = ecies.public_key();
        assert!(!public_key.to_base64().is_empty());
    }

    #[test]
    fn test_ecies_with_info_custom_prefix() {
        let ecies = Ecies::with_info("CUSTOM_INFO");
        let public_key = ecies.public_key();
        assert!(!public_key.to_base64().is_empty());
    }

    #[test]
    fn test_ecies_new_and_with_info_produce_different_keys() {
        let ecies1 = Ecies::new();
        let ecies2 = Ecies::new();
        assert_ne!(
            ecies1.public_key().to_base64(),
            ecies2.public_key().to_base64(),
            "Two random Ecies instances should have different keys"
        );
    }

    #[test]
    fn test_ecies_establish_outbound_channel() {
        let alice = Ecies::new();
        let bob = Ecies::new();

        let plaintext = b"Hello ECIES!";

        let result = alice
            .establish_outbound_channel(bob.public_key(), plaintext)
            .expect("Should establish outbound channel");

        assert!(!result.message.encode().is_empty());
    }

    #[test]
    fn test_ecies_establish_inbound_channel() {
        let alice = Ecies::new();
        let bob = Ecies::new();

        let plaintext = b"Hello ECIES!";

        let outbound = alice
            .establish_outbound_channel(bob.public_key(), plaintext)
            .expect("Should establish outbound channel");

        let inbound = bob
            .establish_inbound_channel(&outbound.message)
            .expect("Should establish inbound channel");

        assert_eq!(
            inbound.message, plaintext,
            "Decrypted plaintext should match the original"
        );
    }

    #[test]
    fn test_ecies_outbound_inbound_check_codes_match() {
        let alice = Ecies::new();
        let bob = Ecies::new();

        let plaintext = b"Check code test";

        let outbound = alice
            .establish_outbound_channel(bob.public_key(), plaintext)
            .expect("Should establish outbound channel");

        let inbound = bob
            .establish_inbound_channel(&outbound.message)
            .expect("Should establish inbound channel");

        assert_eq!(
            outbound.ecies.check_code(),
            inbound.ecies.check_code(),
            "Check codes should match on both sides"
        );
    }

    #[test]
    fn test_ecies_with_info_establishes_channel() {
        let alice = Ecies::with_info("CUSTOM_APP_INFO");
        let bob = Ecies::with_info("CUSTOM_APP_INFO");

        let plaintext = b"Custom info test";

        let outbound = alice
            .establish_outbound_channel(bob.public_key(), plaintext)
            .expect("Should establish outbound channel");

        let inbound = bob
            .establish_inbound_channel(&outbound.message)
            .expect("Should establish inbound channel");

        assert_eq!(inbound.message, plaintext);
    }

    #[test]
    fn test_ecies_initial_message_encode_decode_roundtrip() {
        let alice = Ecies::new();
        let bob = Ecies::new();

        let plaintext = b"Roundtrip test";

        let outbound = alice
            .establish_outbound_channel(bob.public_key(), plaintext)
            .expect("Should establish outbound channel");

        let encoded = outbound.message.encode();
        let decoded = InitialMessage::decode(&encoded).expect("Should decode initial message");

        assert_eq!(decoded.public_key, outbound.message.public_key);
        assert_eq!(decoded.ciphertext, outbound.message.ciphertext);
    }

    use jni::Env;
    use jni::objects::{JByteArray, JObject};
    use jni::sys::jlong;
    use jni::{jni_sig, jni_str};

    use crate::helpers::get_jvm;

    use super::Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeEstablishInboundChannel as native_establish_inbound;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeEstablishOutboundChannel as native_establish_outbound;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeFree as native_free_export;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeNew as native_new;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativePublicKey as native_public_key;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_Ecies_nativeWithInfo as native_with_info;

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_jni_creation_public_key_and_free() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let ptr = call(env, |unowned, class| native_new(unowned, class));
                let public_key = call(env, |unowned, class| native_public_key(unowned, class, ptr));
                let public_key = JString::from_raw(env, public_key).try_to_string(env)?;
                assert!(!public_key.is_empty());

                let info = env.new_string("JNI ECIES info")?;
                let info_ptr = call(env, |unowned, class| native_with_info(unowned, class, info));
                let info_public_key = call(env, |unowned, class| {
                    native_public_key(unowned, class, info_ptr)
                });
                let info_public_key = JString::from_raw(env, info_public_key).try_to_string(env)?;
                assert!(!info_public_key.is_empty());

                call(env, |unowned, class| {
                    native_free_export(unowned, class, ptr)
                });
                call(env, |unowned, class| {
                    native_free_export(unowned, class, info_ptr)
                });
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_jni_outbound_inbound_creation_results_and_accessors() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let alice_ptr = call(env, |unowned, class| native_new(unowned, class));
                let bob_ptr = call(env, |unowned, class| native_new(unowned, class));
                let alice_key = call(env, |unowned, class| {
                    native_public_key(unowned, class, alice_ptr)
                });
                let alice_key = JString::from_raw(env, alice_key).try_to_string(env)?;
                let bob_key = call(env, |unowned, class| {
                    native_public_key(unowned, class, bob_ptr)
                });
                let bob_key = JString::from_raw(env, bob_key).try_to_string(env)?;

                let their_key = env.new_string(&bob_key)?;
                let initial_plaintext = env.byte_array_from_slice(b"initial JNI plaintext")?;
                let outbound_raw = call(env, |unowned, class| {
                    native_establish_outbound(
                        unowned,
                        class,
                        alice_ptr,
                        their_key,
                        initial_plaintext,
                    )
                });
                assert!(!outbound_raw.is_null());
                assert!(!env.exception_check());
                let outbound = JObject::from_raw(env, outbound_raw);
                let initial_message = env
                    .call_method(
                        &outbound,
                        jni_str!("initialMessage"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                let initial_message =
                    JString::from_raw(env, initial_message.into_raw()).try_to_string(env)?;
                assert!(!initial_message.is_empty());
                let outbound_ecies = env
                    .call_method(
                        &outbound,
                        jni_str!("establishedEcies"),
                        jni_sig!(() -> io.github.fherbreteau.vodozemac.ecies.EstablishedEcies),
                        &[],
                    )?
                    .l()?;
                let outbound_public_key = env
                    .call_method(
                        &outbound_ecies,
                        jni_str!("publicKey"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                assert_eq!(
                    JString::from_raw(env, outbound_public_key.into_raw()).try_to_string(env)?,
                    alice_key
                );

                let initial_message = env.new_string(&initial_message)?;
                let inbound_raw = call(env, |unowned, class| {
                    native_establish_inbound(unowned, class, bob_ptr, initial_message)
                });
                assert!(!inbound_raw.is_null());
                assert!(!env.exception_check());
                let inbound = JObject::from_raw(env, inbound_raw);
                let plaintext = env
                    .call_method(&inbound, jni_str!("plaintext"), jni_sig!(() -> byte[]), &[])?
                    .l()?;
                assert_eq!(
                    env.convert_byte_array(JByteArray::from_raw(env, plaintext.as_raw()))?,
                    b"initial JNI plaintext"
                );
                let inbound_ecies = env
                    .call_method(
                        &inbound,
                        jni_str!("establishedEcies"),
                        jni_sig!(() -> io.github.fherbreteau.vodozemac.ecies.EstablishedEcies),
                        &[],
                    )?
                    .l()?;
                let inbound_public_key = env
                    .call_method(
                        &inbound_ecies,
                        jni_str!("publicKey"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                assert_eq!(
                    JString::from_raw(env, inbound_public_key.as_raw()).try_to_string(env)?,
                    bob_key
                );

                let outbound_code = env
                    .call_method(
                        &outbound_ecies,
                        jni_str!("checkCode"),
                        jni_sig!(() -> io.github.fherbreteau.vodozemac.ecies.CheckCode),
                        &[],
                    )?
                    .l()?;
                let inbound_code = env
                    .call_method(
                        &inbound_ecies,
                        jni_str!("checkCode"),
                        jni_sig!(() -> io.github.fherbreteau.vodozemac.ecies.CheckCode),
                        &[],
                    )?
                    .l()?;
                let outbound_bytes = env
                    .call_method(
                        &outbound_code,
                        jni_str!("asBytes"),
                        jni_sig!(() -> byte[]),
                        &[],
                    )?
                    .l()?;
                let inbound_bytes = env
                    .call_method(
                        &inbound_code,
                        jni_str!("asBytes"),
                        jni_sig!(() -> byte[]),
                        &[],
                    )?
                    .l()?;
                assert_eq!(
                    env.convert_byte_array(JByteArray::from_raw(env, outbound_bytes.as_raw()))?,
                    env.convert_byte_array(JByteArray::from_raw(env, inbound_bytes.as_raw()))?
                );
                let digit = env
                    .call_method(
                        &outbound_code,
                        jni_str!("toDigit"),
                        jni_sig!(() -> int),
                        &[],
                    )?
                    .i()?;
                assert!((0..=99).contains(&digit));

                env.call_method(
                    &outbound_ecies,
                    jni_str!("close"),
                    jni_sig!(() -> void),
                    &[],
                )?;
                env.call_method(&inbound_ecies, jni_str!("close"), jni_sig!(() -> void), &[])?;
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_jni_invalid_keys_messages_and_pointers_throw() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let ptr = call(env, |unowned, class| native_new(unowned, class));
                let invalid_key = env.new_string("not-a-curve25519-key")?;
                let plaintext = env.byte_array_from_slice(b"plaintext")?;
                let result = call(env, |unowned, class| {
                    native_establish_outbound(unowned, class, ptr, invalid_key, plaintext)
                });
                assert!(result.is_null());
                assert!(env.exception_check());
                env.exception_clear();
                call(env, |unowned, class| {
                    native_free_export(unowned, class, ptr)
                });

                let ptr = call(env, |unowned, class| native_new(unowned, class));
                let invalid_message = env.new_string("not-an-ecies-message")?;
                let result = call(env, |unowned, class| {
                    native_establish_inbound(unowned, class, ptr, invalid_message)
                });
                assert!(result.is_null());
                assert!(env.exception_check());
                env.exception_clear();
                call(env, |unowned, class| {
                    native_free_export(unowned, class, ptr)
                });

                let key = env.new_string("invalid")?;
                let plaintext = env.byte_array_from_slice(b"plaintext")?;
                let result = call(env, |unowned, class| {
                    native_establish_outbound(unowned, class, 0, key, plaintext)
                });
                assert!(result.is_null());
                assert!(env.exception_check());
                env.exception_clear();

                let message = env.new_string("invalid")?;
                let result = call(env, |unowned, class| {
                    native_establish_inbound(unowned, class, 0, message)
                });
                assert!(result.is_null());
                assert!(env.exception_check());
                env.exception_clear();

                let key = call(env, |unowned, class| native_public_key(unowned, class, 0));
                assert!(key.is_null());
                assert!(env.exception_check());
                env.exception_clear();
                let zero: jlong = 0;
                call(env, |unowned, class| {
                    native_free_export(unowned, class, zero)
                });
                assert!(env.exception_check());
                env.exception_clear();
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
