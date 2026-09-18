pub mod keypair;

use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::jboolean;
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::{Curve25519PublicKey, Ed25519PublicKey, Ed25519Signature};

use crate::errors::{throw_key_error, throw_signature_error};
use crate::helpers::catch_panic;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519PublicKey_nativeValidate(
    mut env: EnvUnowned,
    _class: JClass,
    base64: JString,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        catch_panic(env, |env| {
            let base64 = base64.try_to_string(env)?;

            let _ = Ed25519PublicKey::from_base64(&base64).map_err(|e| throw_key_error(env, e))?;
            Ok(())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519PublicKey_nativeVerify(
    mut env: EnvUnowned,
    _class: JClass,
    public_key: JString,
    message: JByteArray,
    signature: JString,
) -> jboolean {
    let outcome = env.with_env(|env| -> Result<jboolean, jni::errors::Error> {
        catch_panic(env, |env| {
            let public_key = public_key.try_to_string(env)?;
            let message = env.convert_byte_array(message)?;
            let signature = signature.try_to_string(env)?;

            let public_key =
                Ed25519PublicKey::from_base64(&public_key).map_err(|e| throw_key_error(env, e))?;
            let signature = Ed25519Signature::from_base64(&signature)
                .map_err(|e| throw_signature_error(env, e))?;

            let result = public_key.verify(&message, &signature);

            match result {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Ed25519Signature_nativeValidate(
    mut env: EnvUnowned,
    _class: JClass,
    base64: JString,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        catch_panic(env, |env| {
            let base64 = base64.try_to_string(env)?;

            let _ =
                Ed25519Signature::from_base64(&base64).map_err(|e| throw_signature_error(env, e));
            Ok(())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_types_Curve25519PublicKey_nativeValidate(
    mut env: EnvUnowned,
    _class: JClass,
    base64: JString,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        catch_panic(env, |env| {
            let base64 = base64.try_to_string(env)?;
            let _ = Curve25519PublicKey::from_base64(&base64).map_err(|e| throw_key_error(env, e));
            Ok(())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

pub(crate) trait JniBase64Value {
    const JNI_CLASS: &jni::strings::JNIStr;

    fn to_base64(&self) -> String;
}

impl JniBase64Value for Curve25519PublicKey {
    const JNI_CLASS: &jni::strings::JNIStr =
        jni_str!("io/github/fherbreteau/vodozemac/types/Curve25519PublicKey");

    fn to_base64(&self) -> String {
        Curve25519PublicKey::to_base64(self)
    }
}

impl JniBase64Value for Ed25519PublicKey {
    const JNI_CLASS: &jni::strings::JNIStr =
        jni_str!("io/github/fherbreteau/vodozemac/types/Ed25519PublicKey");

    fn to_base64(&self) -> String {
        Ed25519PublicKey::to_base64(self)
    }
}

impl JniBase64Value for Ed25519Signature {
    const JNI_CLASS: &jni::strings::JNIStr =
        jni_str!("io/github/fherbreteau/vodozemac/types/Ed25519Signature");

    fn to_base64(&self) -> String {
        Ed25519Signature::to_base64(self)
    }
}

pub(crate) fn to_java_base64_value<'local, T: JniBase64Value>(
    env: &mut Env<'local>,
    value: &T,
) -> Result<JObject<'local>, jni::errors::Error> {
    let encoded = env.new_string(value.to_base64())?;
    env.new_object(
        T::JNI_CLASS,
        jni_sig!((base64: java.lang.String) -> void),
        &[JValue::Object(&encoded)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::get_jvm;

    use super::Java_io_github_fherbreteau_vodozemac_types_Curve25519PublicKey_nativeValidate as curve25519_validate;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519PublicKey_nativeValidate as ed25519_key_validate;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519PublicKey_nativeVerify as ed25519_verify;
    use super::Java_io_github_fherbreteau_vodozemac_types_Ed25519Signature_nativeValidate as ed25519_signature_validate;

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_jni_validate_accepts_valid_keys() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let key = env.new_string("NnTo+WL1n6ZjGN1EdHKtrYMRKAlrNUlxrZLtX0hDkbs")?;
                call(env, |unowned, class| {
                    ed25519_key_validate(unowned, class, key)
                });
                assert!(!env.exception_check(), "A valid Ed25519 key should validate");

                let signature = env.new_string(
                    "SucffO/oXYCEPa2lSLPiutmbbN+F3fKMd4Bps8ONOQJ/QjjwlpuXL/ag0kfa9vC0LeH0b+Y7/Qy+83jpExuUCQ",
                )?;
                call(env, |unowned, class| {
                    ed25519_signature_validate(unowned, class, signature)
                });
                assert!(!env.exception_check(), "A valid signature should validate");

                let curve_key = env.new_string("WHKTK+K7GSjf83JuPfGV0KAZjxQU/3HKOb0DD1MaOm4")?;
                call(env, |unowned, class| {
                    curve25519_validate(unowned, class, curve_key)
                });
                assert!(!env.exception_check(), "A valid Curve25519 key should validate");
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_jni_validate_rejects_invalid_keys() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let invalid = env.new_string("invalid")?;

                call(env, |unowned, class| {
                    ed25519_key_validate(unowned, class, invalid)
                });
                assert!(env.exception_check(), "An invalid Ed25519 key should throw");
                env.exception_clear();

                let invalid = env.new_string("invalid")?;
                call(env, |unowned, class| {
                    ed25519_signature_validate(unowned, class, invalid)
                });
                assert!(env.exception_check(), "An invalid signature should throw");
                env.exception_clear();

                let invalid = env.new_string("invalid")?;
                call(env, |unowned, class| {
                    curve25519_validate(unowned, class, invalid)
                });
                assert!(
                    env.exception_check(),
                    "An invalid Curve25519 key should throw"
                );
                env.exception_clear();
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_jni_verify_signatures() {
        use vodozemac::Ed25519Keypair;

        let keypair = Ed25519Keypair::new();
        let key_base64 = keypair.public_key().to_base64();
        let signature_base64 = keypair.sign(b"Hello Matrix").to_base64();

        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let key = env.new_string(&key_base64)?;
                let signature = env.new_string(&signature_base64)?;
                let message = env.byte_array_from_slice(b"Hello Matrix")?;
                let verified = call(env, |unowned, class| {
                    ed25519_verify(unowned, class, key, message, signature)
                });
                assert!(verified, "A matching signature should verify");

                let key = env.new_string(&key_base64)?;
                let signature = env.new_string(&signature_base64)?;
                let message = env.byte_array_from_slice(b"other message")?;
                let verified = call(env, |unowned, class| {
                    ed25519_verify(unowned, class, key, message, signature)
                });
                assert!(
                    !verified,
                    "A signature over another message should not verify"
                );

                let key = env.new_string(&key_base64)?;
                let invalid = env.new_string("not-a-signature")?;
                let message = env.byte_array_from_slice(b"Hello Matrix")?;
                call(env, |unowned, class| {
                    ed25519_verify(unowned, class, key, message, invalid)
                });
                assert!(
                    env.exception_check(),
                    "An undecodable signature should throw"
                );
                env.exception_clear();
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
