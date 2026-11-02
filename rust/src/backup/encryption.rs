use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jlong, jobject};
use vodozemac::Curve25519PublicKey;
use vodozemac::pk_encryption::PkEncryption;

use super::to_java_pk_message;
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
            let public_key = Curve25519PublicKey::from_base64(&key.try_to_string(env)?)
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

            let result = encryption
                .encrypt(&data)
                .map_err(|e| throw_encryption_error(env, e))?;
            let result = to_java_pk_message(env, &result)?;
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
    use super::*;
    use crate::helpers::get_jvm;
    use jni::objects::JObject;
    use jni::{Env, jni_sig, jni_str};
    use vodozemac::pk_encryption::PkDecryption;

    use super::Java_io_github_fherbreteau_vodozemac_backup_PkEncryption_nativeEncrypt as native_encrypt;
    use super::Java_io_github_fherbreteau_vodozemac_backup_PkEncryption_nativeFree as native_free;

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

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
    use super::Java_io_github_fherbreteau_vodozemac_backup_PkEncryption_nativeFromKey as native_from_key;

    #[test]
    fn test_jni_encryption_lifecycle() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let secret_key = vodozemac::Curve25519SecretKey::new();
                let key = env.new_string(Curve25519PublicKey::from(&secret_key).to_base64())?;
                let ptr = call(env, |unowned, class| native_from_key(unowned, class, key));
                assert_ne!(ptr, 0);

                let plaintext = env.byte_array_from_slice(b"JNI backup encryption")?;
                let message = call(env, |unowned, class| {
                    native_encrypt(unowned, class, ptr, plaintext)
                });
                assert!(!message.is_null());
                let message = JObject::from_raw(env, message);
                let ciphertext = env
                    .call_method(
                        &message,
                        jni_str!("ciphertext"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                let mac = env
                    .call_method(
                        &message,
                        jni_str!("mac"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                let ephemeral_key = env
                    .call_method(
                        &message,
                        jni_str!("ephemeralKey"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                assert!(!ciphertext.is_null());
                assert!(!mac.is_null());
                assert!(!ephemeral_key.is_null());

                call(env, |unowned, class| native_free(unowned, class, ptr));

                let invalid = env.new_string("invalid")?;
                let invalid_ptr = call(env, |unowned, class| {
                    native_from_key(unowned, class, invalid)
                });
                assert_eq!(invalid_ptr, 0);
                assert!(env.exception_check());
                env.exception_clear();

                let wrong_size = env.new_string(vodozemac::base64_encode([1u8; 31]))?;
                let wrong_size_ptr = call(env, |unowned, class| {
                    native_from_key(unowned, class, wrong_size)
                });
                assert_eq!(wrong_size_ptr, 0);
                assert!(env.exception_check());
                env.exception_clear();
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
