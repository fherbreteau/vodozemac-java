use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use vodozemac::ecies::{EstablishedEcies, Message};

use super::to_java_check_code;
use crate::errors::throw_ecies_error;
use crate::helpers::{catch_panic, check_ptr, native_free, string_to_jstring};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativePublicKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let ecies = unsafe { &*(ptr as *const EstablishedEcies) };

            let public_key = ecies.public_key().to_base64();
            string_to_jstring(env, public_key)
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let ecies = unsafe { &*(ptr as *const EstablishedEcies) };

            let check_code = ecies.check_code();
            let result = to_java_check_code(env, check_code)?;
            Ok(result.into_raw())
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let ecies = unsafe { &mut *(ptr as *mut EstablishedEcies) };
            let plaintext = env.convert_byte_array(plaintext)?;

            let message = ecies.encrypt(&plaintext).encode();
            string_to_jstring(env, message)
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let ecies = unsafe { &mut *(ptr as *mut EstablishedEcies) };
            let message = Message::decode(&message.try_to_string(env)?)
                .map_err(|e| throw_ecies_error(env, e))?;

            let plaintext = ecies
                .decrypt(&message)
                .map_err(|e| throw_ecies_error(env, e))?;
            let result = env.byte_array_from_slice(&plaintext)?;
            Ok(result.into_raw())
        })
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

#[cfg(test)]
mod tests {
    use vodozemac::ecies::Ecies;
    use vodozemac::hpke::DigitMode;

    use super::*;

    fn establish_ecies_pair() -> (EstablishedEcies, EstablishedEcies) {
        let alice = Ecies::new();
        let bob = Ecies::new();

        let outbound = alice
            .establish_outbound_channel(bob.public_key(), b"initial plaintext")
            .expect("Should establish outbound channel");

        let inbound = bob
            .establish_inbound_channel(&outbound.message)
            .expect("Should establish inbound channel");

        (outbound.ecies, inbound.ecies)
    }

    #[test]
    fn test_established_ecies_public_key() {
        let alice = Ecies::new();
        let bob = Ecies::new();

        let alice_public = alice.public_key();

        let outbound = alice
            .establish_outbound_channel(bob.public_key(), b"test")
            .expect("Should establish outbound channel");

        assert_eq!(outbound.ecies.public_key(), alice_public);
    }

    #[test]
    fn test_established_ecies_check_code_matches() {
        let (alice, bob) = establish_ecies_pair();

        assert_eq!(
            alice.check_code(),
            bob.check_code(),
            "Check codes should match on both sides"
        );
    }

    #[test]
    fn test_established_ecies_check_code_to_digit() {
        let (alice, bob) = establish_ecies_pair();

        assert_eq!(
            alice.check_code().to_digit(DigitMode::AllowLeadingZero),
            bob.check_code().to_digit(DigitMode::AllowLeadingZero)
        );
        assert!((0..=99).contains(&alice.check_code().to_digit(DigitMode::AllowLeadingZero)));
    }

    #[test]
    fn test_established_ecies_check_code_as_bytes() {
        let (alice, bob) = establish_ecies_pair();

        assert_eq!(alice.check_code().as_bytes(), bob.check_code().as_bytes());
        assert_eq!(alice.check_code().as_bytes().len(), 2);
    }

    #[test]
    fn test_established_ecies_encrypt_decrypt_roundtrip() {
        let (mut alice, mut bob) = establish_ecies_pair();

        let plaintext = b"Secret message";
        let encrypted = alice.encrypt(plaintext);
        let encoded = encrypted.encode();

        let decoded = Message::decode(&encoded).expect("Should decode message");
        let decrypted = bob.decrypt(&decoded).expect("Should decrypt message");

        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_established_ecies_encrypt_decrypt_multiple_messages() {
        let (mut alice, mut bob) = establish_ecies_pair();

        for i in 0..5 {
            let plaintext = format!("Message {i}");
            let encrypted = alice.encrypt(plaintext.as_bytes());
            let decrypted = bob.decrypt(&encrypted).expect("Should decrypt message");

            assert_eq!(decrypted, plaintext.as_bytes());
        }
    }

    #[test]
    fn test_established_ecies_encrypt_decrypt_both_directions() {
        let (mut alice, mut bob) = establish_ecies_pair();

        let plaintext_a = b"Alice to Bob";
        let encrypted_a = alice.encrypt(plaintext_a);
        let decrypted_a = bob
            .decrypt(&encrypted_a)
            .expect("Bob should decrypt Alice's message");
        assert_eq!(decrypted_a, plaintext_a);

        let plaintext_b = b"Bob to Alice";
        let encrypted_b = bob.encrypt(plaintext_b);
        let decrypted_b = alice
            .decrypt(&encrypted_b)
            .expect("Alice should decrypt Bob's message");
        assert_eq!(decrypted_b, plaintext_b);
    }

    #[test]
    fn test_established_ecies_decrypt_invalid_message_fails() {
        let (mut alice, _) = establish_ecies_pair();

        let bad_message = Message {
            ciphertext: vec![0xff, 0xfe, 0xfd, 0xfc, 0xfb],
        };

        alice
            .decrypt(&bad_message)
            .expect_err("Should fail to decrypt invalid message");
    }

    use jni::Env;
    use jni::objects::{JByteArray, JObject};
    use jni::sys::jlong;
    use jni::{jni_sig, jni_str};

    use crate::helpers::get_jvm;

    use super::Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeCheckCode as native_check_code;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeDecrypt as native_decrypt;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeEncrypt as native_encrypt;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativeFree as native_free_export;
    use super::Java_io_github_fherbreteau_vodozemac_ecies_EstablishedEcies_nativePublicKey as native_public_key;

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_jni_established_ecies_operations_and_free() {
        let jvm = get_jvm();
        let alice = Ecies::new();
        let bob = Ecies::new();
        let outbound = alice
            .establish_outbound_channel(bob.public_key(), b"initial")
            .expect("Should establish outbound channel");
        let inbound = bob
            .establish_inbound_channel(&outbound.message)
            .expect("Should establish inbound channel");
        let alice_key = outbound.ecies.public_key().to_base64();
        let alice_ptr = Box::into_raw(Box::new(outbound.ecies)) as jlong;
        let bob_ptr = Box::into_raw(Box::new(inbound.ecies)) as jlong;

        jvm.attach_current_thread(move |env| -> Result<(), jni::errors::Error> {
            unsafe {
                let public_key = call(env, |unowned, class| {
                    native_public_key(unowned, class, alice_ptr)
                });
                assert_eq!(
                    JString::from_raw(env, public_key).try_to_string(env)?,
                    alice_key
                );

                let code = call(env, |unowned, class| {
                    native_check_code(unowned, class, alice_ptr)
                });
                assert!(!code.is_null());
                let code = JObject::from_raw(env, code);
                let bytes = env
                    .call_method(&code, jni_str!("asBytes"), jni_sig!(() -> byte[]), &[])?
                    .l()?;
                assert_eq!(
                    env.convert_byte_array(JByteArray::from_raw(env, bytes.as_raw()))?
                        .len(),
                    2
                );
                let digit = env
                    .call_method(&code, jni_str!("toDigit"), jni_sig!(() -> int), &[])?
                    .i()?;
                assert!((0..=99).contains(&digit));

                let plaintext = env.byte_array_from_slice(b"JNI established message")?;
                let encrypted = call(env, |unowned, class| {
                    native_encrypt(unowned, class, alice_ptr, plaintext)
                });
                let encrypted = JString::from_raw(env, encrypted).try_to_string(env)?;
                assert!(!encrypted.is_empty());
                let encrypted = env.new_string(&encrypted)?;
                let decrypted = call(env, |unowned, class| {
                    native_decrypt(unowned, class, bob_ptr, encrypted)
                });
                assert!(!decrypted.is_null());
                assert_eq!(
                    env.convert_byte_array(JByteArray::from_raw(env, decrypted))?,
                    b"JNI established message"
                );
                assert!(!env.exception_check());

                call(env, |unowned, class| {
                    native_free_export(unowned, class, alice_ptr)
                });
                call(env, |unowned, class| {
                    native_free_export(unowned, class, bob_ptr)
                });
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_jni_established_ecies_invalid_message_and_pointers_throw() {
        let jvm = get_jvm();
        let (alice, bob) = establish_ecies_pair();
        let alice_ptr = Box::into_raw(Box::new(alice)) as jlong;
        let bob_ptr = Box::into_raw(Box::new(bob)) as jlong;

        jvm.attach_current_thread(move |env| -> Result<(), jni::errors::Error> {
            unsafe {
                let invalid = env.new_string("not-an-ecies-message")?;
                let result = call(env, |unowned, class| {
                    native_decrypt(unowned, class, bob_ptr, invalid)
                });
                assert!(result.is_null());
                assert!(env.exception_check());
                env.exception_clear();

                let public_key = call(env, |unowned, class| native_public_key(unowned, class, 0));
                assert!(public_key.is_null());
                assert!(env.exception_check());
                env.exception_clear();

                let check_code = call(env, |unowned, class| native_check_code(unowned, class, 0));
                assert!(check_code.is_null());
                assert!(env.exception_check());
                env.exception_clear();

                let plaintext = env.byte_array_from_slice(b"plaintext")?;
                let encrypted = call(env, |unowned, class| {
                    native_encrypt(unowned, class, 0, plaintext)
                });
                assert!(encrypted.is_null());
                assert!(env.exception_check());
                env.exception_clear();

                let invalid = env.new_string("invalid")?;
                let decrypted = call(env, |unowned, class| {
                    native_decrypt(unowned, class, 0, invalid)
                });
                assert!(decrypted.is_null());
                assert!(env.exception_check());
                env.exception_clear();

                call(env, |unowned, class| {
                    native_free_export(unowned, class, alice_ptr)
                });
                call(env, |unowned, class| {
                    native_free_export(unowned, class, bob_ptr)
                });
                call(env, |unowned, class| native_free_export(unowned, class, 0));
                assert!(env.exception_check());
                env.exception_clear();
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
