use jni::objects::{JByteArray, JClass, JIntArray, JObjectArray, JString};
use jni::sys::{jint, jlong, jobject, jstring};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::sas::{EstablishedSas, Mac, SasBytes};

use crate::errors::{throw_generic_error, throw_invalid_count_error, throw_sas_error};
use crate::helpers::{catch_panic, check_ptr, native_free};

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
        catch_panic(env, |env| {
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
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
            let info = info.to_string();
            let count = usize::try_from(count).map_err(|e| throw_generic_error(env, e))?;

            let bytes = established_sas
                .bytes_raw(&info, count)
                .map_err(|e| throw_invalid_count_error(env, e))?;

            let result = to_byte_array(env, &bytes)?;
            Ok(result.into_raw())
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
            let input = input.to_string();
            let info = info.to_string();

            let mac = established_sas.calculate_mac(&input, &info);
            let mac_str = env.new_string(mac.to_base64())?;
            Ok(mac_str.into_raw())
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
            let input = input.to_string();
            let info = info.to_string();

            let mac = established_sas.calculate_mac_invalid_base64(&input, &info);
            let mac_str = env.new_string(mac)?;
            Ok(mac_str.into_raw())
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let established_sas = unsafe { &*(ptr as *const EstablishedSas) };
            let input = input.to_string();
            let info = info.to_string();
            let mac =
                Mac::from_base64(&mac.to_string()).map_err(|e| throw_generic_error(env, e))?;

            established_sas
                .verify_mac(&input, &info, &mac)
                .map_err(|e| throw_sas_error(env, e))?;
            Ok(())
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let established_sas = unsafe { &*(ptr as *const EstablishedSas) };

            let our_public_key = established_sas.our_public_key().to_base64();
            let result = env.new_string(&our_public_key)?;
            Ok(result.into_raw())
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let established_sas = unsafe { &*(ptr as *const EstablishedSas) };

            let their_public_key = established_sas.their_public_key().to_base64();
            let result = env.new_string(&their_public_key)?;
            Ok(result.into_raw())
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use vodozemac::sas::Sas;

    fn establish_sas_pair() -> (EstablishedSas, EstablishedSas) {
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

        (alice_established, bob_established)
    }

    #[test]
    fn test_established_sas_bytes_match() {
        let (alice, bob) = establish_sas_pair();

        let alice_bytes = alice.bytes("SAS_INFO");
        let bob_bytes = bob.bytes("SAS_INFO");

        assert_eq!(
            alice_bytes, bob_bytes,
            "Both sides should derive the same SasBytes"
        );
    }

    #[test]
    fn test_established_sas_emoji_indices_match() {
        let (alice, bob) = establish_sas_pair();

        let alice_emojis = alice.bytes("SAS_INFO").emoji_indices();
        let bob_emojis = bob.bytes("SAS_INFO").emoji_indices();

        assert_eq!(alice_emojis, bob_emojis);
    }

    #[test]
    fn test_established_sas_decimals_match() {
        let (alice, bob) = establish_sas_pair();

        let alice_decimals = alice.bytes("SAS_INFO").decimals();
        let bob_decimals = bob.bytes("SAS_INFO").decimals();

        assert_eq!(alice_decimals, bob_decimals);
    }

    #[test]
    fn test_established_sas_raw_bytes() {
        let (alice, bob) = establish_sas_pair();

        let alice_raw = alice
            .bytes_raw("SAS_INFO", 32)
            .expect("Should generate 32 bytes");
        let bob_raw = bob
            .bytes_raw("SAS_INFO", 32)
            .expect("Should generate 32 bytes");

        assert_eq!(alice_raw, bob_raw);
        assert_eq!(alice_raw.len(), 32);
    }

    #[test]
    fn test_established_sas_bytes_raw_different_info_produces_different_bytes() {
        let (alice, _) = establish_sas_pair();

        let bytes1 = alice.bytes("INFO_1");
        let bytes2 = alice.bytes("INFO_2");

        assert_ne!(
            bytes1, bytes2,
            "Different info strings should produce different bytes"
        );
    }

    #[test]
    fn test_established_sas_calculate_mac() {
        let (alice, bob) = establish_sas_pair();

        let message = "ed25519:BOB_DEVICE";
        let info = "MATRIX_KEY_VERIFICATION_MAC";

        let alice_mac = alice.calculate_mac(message, info);
        let bob_mac = bob.calculate_mac(message, info);

        assert_eq!(
            alice_mac.to_base64(),
            bob_mac.to_base64(),
            "MACs should match"
        );
        assert!(!alice_mac.to_base64().is_empty());
    }

    #[test]
    fn test_established_sas_verify_mac_valid() {
        let (alice, bob) = establish_sas_pair();

        let message = "ed25519:BOB_DEVICE";
        let info = "MATRIX_KEY_VERIFICATION_MAC";

        let alice_mac = alice.calculate_mac(message, info);

        bob.verify_mac(message, info, &alice_mac)
            .expect("Bob should verify Alice's MAC");
    }

    #[test]
    fn test_established_sas_verify_mac_invalid() {
        let (alice, _) = establish_sas_pair();

        let message = "ed25519:BOB_DEVICE";
        let info = "MATRIX_KEY_VERIFICATION_MAC";

        let invalid_mac = Mac::from_slice(&[
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 0, 1,
        ]);

        alice
            .verify_mac(message, info, &invalid_mac)
            .expect_err("Should fail to verify invalid MAC");
    }

    #[test]
    fn test_established_sas_mac_from_base64_roundtrip() {
        let (alice, _) = establish_sas_pair();

        let message = "test message";
        let info = "test info";

        let mac = alice.calculate_mac(message, info);
        let mac_base64 = mac.to_base64();

        let restored = Mac::from_base64(&mac_base64).expect("Should decode MAC from base64");

        alice
            .verify_mac(message, info, &restored)
            .expect("Should verify restored MAC");
    }

    #[test]
    fn test_established_sas_calculate_mac_invalid_base64() {
        let (alice, _) = establish_sas_pair();

        let mac = alice.calculate_mac("", "");
        let invalid = alice.calculate_mac_invalid_base64("", "");

        assert_ne!(
            mac.to_base64(),
            invalid,
            "Invalid base64 MAC should differ from valid MAC"
        );
    }

    #[test]
    fn test_established_sas_our_public_key() {
        let sas = Sas::new();
        let public_key = sas.public_key();

        let established = sas
            .diffie_hellman(Sas::new().public_key())
            .expect("DH should succeed");

        assert_eq!(established.our_public_key(), public_key);
    }

    #[test]
    fn test_established_sas_their_public_key() {
        let alice = Sas::new();
        let bob = Sas::new();
        let bob_public = bob.public_key();

        let established = alice.diffie_hellman(bob_public).expect("DH should succeed");

        assert_eq!(established.their_public_key(), bob_public);
    }

    #[test]
    fn test_sas_bytes_as_bytes() {
        let (alice, _) = establish_sas_pair();
        let bytes = alice.bytes("INFO");
        assert_eq!(bytes.as_bytes().len(), 6);
    }
}
