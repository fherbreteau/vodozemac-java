use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jint, jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::ecies::{EstablishedEcies, Message};

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
            let bytes: JByteArray = env.byte_array_from_slice(check_code.as_bytes())?;
            let digit = jint::from(check_code.to_digit());
            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/ecies/CheckCode"),
                jni_sig!((bytes: byte[], digit: int) -> void),
                &[JValue::Object(&bytes), JValue::Int(digit)],
            )?;
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
            let message =
                Message::decode(&message.to_string()).map_err(|e| throw_ecies_error(env, e))?;

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

        assert_eq!(alice.check_code().to_digit(), bob.check_code().to_digit());
        assert!((0..=99).contains(&alice.check_code().to_digit()));
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
}
