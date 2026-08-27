use std::mem::forget;

use jni::objects::{JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::Curve25519PublicKey;
use vodozemac::sas::{EstablishedSas, Sas};

use crate::errors::throw_key_error;
use crate::helpers::{box_to_jlong, catch_panic, check_ptr, native_free, string_to_jstring};

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
            string_to_jstring(env, public_key)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sas_new_generates_public_key() {
        let sas = Sas::new();
        let public_key = sas.public_key();
        assert!(!public_key.to_base64().is_empty());
    }

    #[test]
    fn test_sas_public_key_is_43_chars() {
        let sas = Sas::new();
        let public_key = sas.public_key().to_base64();
        assert_eq!(public_key.len(), 43);
    }

    #[test]
    fn test_sas_diffie_hellman_establishes_shared_secret() {
        let alice = Sas::new();
        let bob = Sas::new();

        let alice_public = alice.public_key();
        let bob_public = bob.public_key();

        let alice_established = alice
            .diffie_hellman(bob_public)
            .expect("Alice DH should succeed");
        let bob_established = bob
            .diffie_hellman(alice_public)
            .expect("Bob DH should succeed");

        let alice_bytes = alice_established.bytes("TEST_INFO");
        let bob_bytes = bob_established.bytes("TEST_INFO");

        assert_eq!(
            alice_bytes, bob_bytes,
            "Both sides should derive the same SAS bytes"
        );
    }

    #[test]
    fn test_sas_diffie_hellman_different_keys_produce_different_bytes() {
        let alice = Sas::new();
        let bob1 = Sas::new();
        let bob2 = Sas::new();

        let established1 = alice
            .diffie_hellman(bob1.public_key())
            .expect("DH with bob1 should succeed");
        let bytes1 = established1.bytes("TEST_INFO");

        let established2 = bob2
            .diffie_hellman(bob1.public_key())
            .expect("DH with bob1 from bob2 should succeed");
        let bytes2 = established2.bytes("TEST_INFO");

        assert_ne!(
            bytes1, bytes2,
            "Different key pairs should produce different SAS bytes"
        );
    }

    #[test]
    fn test_sas_established_public_keys() {
        let alice = Sas::new();
        let bob = Sas::new();

        let alice_public = alice.public_key();
        let bob_public = bob.public_key();

        let alice_established = alice
            .diffie_hellman(bob_public)
            .expect("Alice DH should succeed");

        assert_eq!(alice_established.our_public_key(), alice_public);
        assert_eq!(alice_established.their_public_key(), bob_public);
    }
}
