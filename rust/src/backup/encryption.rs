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

#[cfg(test)]
mod tests {
    use vodozemac::pk_encryption::PkDecryption;

    use super::*;

    #[test]
    fn test_pk_encryption_from_key() {
        let decryption = PkDecryption::new();
        let public_key = decryption.public_key();
        let encryption = PkEncryption::from_key(public_key);
        let _ = encryption;
    }

    #[test]
    fn test_pk_encryption_from_curve25519_public_key() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from(decryption.public_key());
        let _ = encryption;
    }

    #[test]
    fn test_pk_encryption_encrypt() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from_key(decryption.public_key());

        let plaintext = b"It's a secret to everybody";
        let message = encryption
            .encrypt(plaintext)
            .expect("Should encrypt message");

        assert!(!message.ciphertext.is_empty());
        assert!(!message.mac.is_empty());
    }

    #[test]
    fn test_pk_encryption_encrypt_decrypt_roundtrip() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from_key(decryption.public_key());

        let plaintext = b"It's a secret to everybody";
        let message = encryption
            .encrypt(plaintext)
            .expect("Should encrypt message");

        let decrypted = decryption
            .decrypt(&message)
            .expect("Should decrypt message");

        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_pk_encryption_different_messages() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from_key(decryption.public_key());

        let msg1 = encryption.encrypt(b"Message 1").expect("Should encrypt");
        let msg2 = encryption.encrypt(b"Message 2").expect("Should encrypt");

        assert_ne!(
            msg1.ciphertext, msg2.ciphertext,
            "Ciphertexts should differ"
        );

        let decrypted1 = decryption.decrypt(&msg1).expect("Should decrypt msg1");
        let decrypted2 = decryption.decrypt(&msg2).expect("Should decrypt msg2");

        assert_eq!(decrypted1, b"Message 1");
        assert_eq!(decrypted2, b"Message 2");
    }

    #[test]
    fn test_pk_encryption_from_pk_decryption() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from(&decryption);

        let plaintext = b"Test from decryption";
        let message = encryption.encrypt(plaintext).expect("Should encrypt");
        let decrypted = decryption.decrypt(&message).expect("Should decrypt");

        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_pk_encryption_empty_plaintext() {
        let decryption = PkDecryption::new();
        let encryption = PkEncryption::from_key(decryption.public_key());

        let message = encryption
            .encrypt(b"")
            .expect("Should encrypt empty plaintext");
        let decrypted = decryption.decrypt(&message).expect("Should decrypt");

        assert!(decrypted.is_empty());
    }
}
