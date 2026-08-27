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

            let _ = public_key
                .verify(&message, &signature)
                .map_err(|e| throw_signature_error(env, e));

            Ok(true)
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

pub(crate) fn to_java_curve25519<'local>(
    env: &mut Env<'local>,
    public_key: &Curve25519PublicKey,
) -> Result<JObject<'local>, jni::errors::Error> {
    let curve25519_str = env.new_string(public_key.to_base64())?;
    env.new_object(
        jni_str!("io/github/fherbreteau/vodozemac/types/Curve25519PublicKey"),
        jni_sig!((base64: java.lang.String) -> void),
        &[JValue::Object(&curve25519_str)],
    )
}

pub(crate) fn to_java_ed25519<'local>(
    env: &mut Env<'local>,
    public_key: &Ed25519PublicKey,
) -> Result<JObject<'local>, jni::errors::Error> {
    let curve25519_str = env.new_string(public_key.to_base64())?;
    env.new_object(
        jni_str!("io/github/fherbreteau/vodozemac/types/Ed25519PublicKey"),
        jni_sig!((base64: java.lang.String) -> void),
        &[JValue::Object(&curve25519_str)],
    )
}

pub(crate) fn to_java_signature<'local>(
    env: &mut Env<'local>,
    signature: &Ed25519Signature,
) -> Result<JObject<'local>, jni::errors::Error> {
    let signature_str = env.new_string(signature.to_base64())?;
    env.new_object(
        jni_str!("io/github/fherbreteau/vodozemac/types/Ed25519Signature"),
        jni_sig!((base64: java.lang.String) -> void),
        &[JValue::Object(&signature_str)],
    )
}
