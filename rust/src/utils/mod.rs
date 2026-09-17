use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jobject, jstring};
use vodozemac::{base64_decode, base64_encode};

use crate::errors::throw_conversion_error;
use crate::helpers::{catch_panic, string_to_jstring};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_Vodozemac_nativeBase64Encode(
    mut env: EnvUnowned,
    _class: JClass,
    src: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            let src = env.convert_byte_array(src)?;

            let dst = base64_encode(src);
            string_to_jstring(env, dst)
        })
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
        catch_panic(env, |env| {
            let src = src.try_to_string(env)?;

            let dst = base64_decode(&src).map_err(|e| throw_conversion_error(env, e))?;
            let result = env.byte_array_from_slice(&dst)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_Vodozemac_nativeVersion(
    mut env: EnvUnowned,
    _class: JClass,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            let version = env.new_string(vodozemac::VERSION)?;
            Ok(version.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::get_jvm;
    use jni::Env;

    use super::Java_io_github_fherbreteau_vodozemac_Vodozemac_nativeBase64Decode as native_base64_decode;
    use super::Java_io_github_fherbreteau_vodozemac_Vodozemac_nativeBase64Encode as native_base64_encode;

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_jni_base64_roundtrip() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let byte_array = env.byte_array_from_slice(b"Hello Matrix")?;
                let encoded = call(env, |unowned, class| {
                    native_base64_encode(unowned, class, byte_array)
                });
                let encoded = JString::from_raw(env, encoded);
                let encoded = encoded.try_to_string(env)?;
                assert_eq!(encoded, "SGVsbG8gTWF0cml4");

                let src = env.new_string(&encoded)?;
                let decoded = call(env, |unowned, class| {
                    native_base64_decode(unowned, class, src)
                });
                let decoded = JByteArray::from_raw(env, decoded);
                let decoded = env.convert_byte_array(decoded)?;
                assert_eq!(decoded, b"Hello Matrix");

                let invalid = env.new_string("not base64 !!")?;
                call(env, |unowned, class| {
                    native_base64_decode(unowned, class, invalid)
                });
                assert!(
                    env.exception_check(),
                    "An invalid base64 string should throw"
                );
                env.exception_clear();
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
