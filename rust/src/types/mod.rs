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
            let base64 = base64.to_string();

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
            let public_key = public_key.to_string();
            let message = env.convert_byte_array(message)?;
            let signature = signature.to_string();

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
            let base64 = base64.to_string();

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
            let base64 = base64.to_string();
            let _ = Curve25519PublicKey::from_base64(&base64).map_err(|e| throw_key_error(env, e));
            Ok(())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

pub(crate) trait JniBase64Value {
    const JNI_CLASS: &'static jni::strings::JNIStr;

    fn to_base64(&self) -> String;
}

impl JniBase64Value for Curve25519PublicKey {
    const JNI_CLASS: &'static jni::strings::JNIStr =
        jni_str!("io/github/fherbreteau/vodozemac/types/Curve25519PublicKey");

    fn to_base64(&self) -> String {
        Curve25519PublicKey::to_base64(self)
    }
}

impl JniBase64Value for Ed25519PublicKey {
    const JNI_CLASS: &'static jni::strings::JNIStr =
        jni_str!("io/github/fherbreteau/vodozemac/types/Ed25519PublicKey");

    fn to_base64(&self) -> String {
        Ed25519PublicKey::to_base64(self)
    }
}

impl JniBase64Value for Ed25519Signature {
    const JNI_CLASS: &'static jni::strings::JNIStr =
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
