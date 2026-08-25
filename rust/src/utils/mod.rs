use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jobject, jstring};
use vodozemac::{base64_decode, base64_encode};

use crate::errors::throw_generic_error;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_Vodozemac_nativeBase64Encode(
    mut env: EnvUnowned,
    _class: JClass,
    src: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let src = env.convert_byte_array(src)?;

        let dst = base64_encode(src);
        let result = env.new_string(&dst)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_Vodozemac_nativeBase64Decode(
    mut env: EnvUnowned,
    _class: JClass,
    src: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let src = src.to_string();

        let dst = base64_decode(&src).map_err(|e| throw_generic_error(env, e))?;
        let result = env.byte_array_from_slice(&dst)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_Vodozemac_nativeVersion(
    mut env: EnvUnowned,
    _class: JClass,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let version = env.new_string(vodozemac::VERSION)?;
        Ok(version.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
