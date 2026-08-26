use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject, jstring};
use vodozemac::pk_encryption::{Message, PkDecryption};
use vodozemac::{Curve25519PublicKey, Curve25519SecretKey, base64_decode, base64_encode};

use crate::errors::{
    throw_decryption_error, throw_generic_error, throw_key_error, throw_pickle_error,
};
use crate::helpers::{box_to_jlong, catch_panic, check_ptr, native_free, string_to_jstring, wrap};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_backup_PkDecryption_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let pk_decryption = PkDecryption::new();

        Ok(box_to_jlong(pk_decryption))
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
        catch_panic(env, |env| {
            let bytes = base64_decode(key.to_string()).map_err(|e| throw_generic_error(env, e))?;
            let bytes: [u8; 32] = wrap(env, bytes)?;
            let secret_key = Curve25519SecretKey::from_slice(&bytes);

            let pk_decryption = PkDecryption::from_key(secret_key);

            Ok(box_to_jlong(pk_decryption))
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let pk_decryption = unsafe { &*(ptr as *const PkDecryption) };

            let secret_key = pk_decryption.secret_key().to_bytes().to_vec();
            let secret_key_base64 = base64_encode(secret_key);
            string_to_jstring(env, secret_key_base64)
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let pk_decryption = unsafe { &*(ptr as *const PkDecryption) };

            let public_key = pk_decryption.public_key().to_base64();
            string_to_jstring(env, public_key)
        })
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
        catch_panic(env, |env| {
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
        })
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
        catch_panic(env, |env| {
            let pickle_str: String = pickle_data.to_string();
            let pickle_key = env.convert_byte_array(pickle_key)?;

            let from_olm = PkDecryption::from_libolm_pickle(&pickle_str, &pickle_key)
                .map_err(|e| throw_pickle_error(env, e))?;
            Ok(box_to_jlong(from_olm))
        })
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
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let pk_decryption = unsafe { &*(ptr as *const PkDecryption) };
            let key = wrap(env, env.convert_byte_array(key)?)?;

            let pickle = pk_decryption
                .to_libolm_pickle(&key)
                .map_err(|e| throw_pickle_error(env, e))?;
            string_to_jstring(env, pickle)
        })
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
        native_free::<PkDecryption>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use crate::helpers::PICKLE_KEY;
    use vodozemac::Curve25519SecretKey;
    use vodozemac::pk_encryption::PkEncryption;

    use super::*;

    #[test]
    fn test_pk_decryption_new_generates_keys() {
        let decryption = PkDecryption::new();
        let public_key = decryption.public_key();
        assert!(!public_key.to_base64().is_empty());
        assert_eq!(decryption.secret_key().to_bytes().len(), 32);
    }

    #[test]
    fn test_pk_decryption_from_key() {
        let secret_key = Curve25519SecretKey::new();
        let original_bytes = secret_key.to_bytes();

        let decryption = PkDecryption::from_key(secret_key);
        let restored_bytes = decryption.secret_key().to_bytes();

        assert_eq!(original_bytes.as_ref(), restored_bytes.as_ref());
    }

    #[test]
    fn test_pk_decryption_from_key_consistent_public_key() {
        let secret_key = Curve25519SecretKey::new();
        let decryption1 = PkDecryption::from_key(secret_key);
        let public_key1 = decryption1.public_key();

        let secret_key2 =
            Curve25519SecretKey::from_slice(decryption1.secret_key().to_bytes().as_ref());
        let decryption2 = PkDecryption::from_key(secret_key2);
        let public_key2 = decryption2.public_key();

        assert_eq!(
            public_key1, public_key2,
            "Same secret key should produce same public key"
        );
    }

    #[test]
    fn test_pk_decryption_secret_key_to_bytes() {
        let decryption = PkDecryption::new();
        let bytes = decryption.secret_key().to_bytes();
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn test_pk_decryption_public_key() {
        let decryption = PkDecryption::new();
        let public_key = decryption.public_key();
        assert_eq!(public_key.to_base64().len(), 43);
    }

    #[test]
    fn test_pk_decryption_decrypt() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from_key(decryption.public_key());

        let plaintext = b"It's a secret to everybody";
        let message = encryption.encrypt(plaintext).expect("Should encrypt");

        let decrypted = decryption
            .decrypt(&message)
            .expect("Should decrypt message");

        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_pk_decryption_decrypt_multiple_messages() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from_key(decryption.public_key());

        for i in 0..5 {
            let plaintext = format!("Message {i}");
            let message = encryption
                .encrypt(plaintext.as_bytes())
                .expect("Should encrypt");
            let decrypted = decryption.decrypt(&message).expect("Should decrypt");
            assert_eq!(decrypted, plaintext.as_bytes());
        }
    }

    #[test]
    fn test_pk_decryption_message_from_base64_roundtrip() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from_key(decryption.public_key());

        let plaintext = b"Base64 roundtrip test";
        let message = encryption.encrypt(plaintext).expect("Should encrypt");

        let ciphertext_b64 = base64_encode(message.ciphertext.clone());
        let mac_b64 = base64_encode(message.mac.clone());
        let ephemeral_b64 = message.ephemeral_key.to_base64();

        let restored = Message::from_base64(&ciphertext_b64, &mac_b64, &ephemeral_b64)
            .expect("Should decode message from base64");

        let decrypted = decryption
            .decrypt(&restored)
            .expect("Should decrypt restored message");

        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_pk_decryption_libolm_pickle_roundtrip() {
        let decryption = PkDecryption::new();
        let original_public = decryption.public_key().to_base64();

        let pickle = decryption
            .to_libolm_pickle(&PICKLE_KEY)
            .expect("Should pickle");

        let restored =
            PkDecryption::from_libolm_pickle(&pickle, &PICKLE_KEY).expect("Should unpickle");

        assert_eq!(restored.public_key().to_base64(), original_public);
    }
}
