use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jint, jlong, jobject, jstring};
use jni::{JValue, jni_sig, jni_str};
use vodozemac::base64_encode;
use vodozemac::megolm::{GroupSession, GroupSessionPickle};

use crate::errors::throw_pickle_error;
use crate::helpers::{
    box_to_jlong, catch_panic, check_ptr, from_json, json_to_jstring,
    megolm_session_config_from_version, native_free, string_to_jstring, wrap,
};

// ============================================================================
// Megolm: OutboundGroupSession (wraps vodozemac::megolm::GroupSession)
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
    version: jint,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let config = megolm_session_config_from_version(env, version)?;

            let session = GroupSession::new(config);
            Ok(box_to_jlong(session))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionId(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const GroupSession) };

            let session_id = session.session_id();
            let jni_string = env.new_string(session_id)?;
            Ok(jni_string.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeMessageIndex(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let outcome = env.with_env(|env| -> Result<jint, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const GroupSession) };

            Ok(session.message_index() as jint)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const GroupSession) };

            let session_key = session.session_key().to_base64();
            let result = env.new_string(session_key)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionConfig(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let outcome = env.with_env(|env| -> Result<jint, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const GroupSession) };

            let session_config = session.session_config();
            Ok(session_config.version() as jint)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    plaintext: JByteArray,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut GroupSession) };
            let plaintext_bytes = env.convert_byte_array(plaintext)?;

            let message = session.encrypt(&plaintext_bytes);
            let ciphertext = env.new_string(base64_encode(message.ciphertext()))?;
            let mac = env.new_string(base64_encode(message.mac()))?;
            let signature = env.new_string(message.signature().to_base64())?;
            let base64 = env.new_string(message.to_base64())?;

            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/megolm/MegolmMessage"),
                jni_sig!((base64: java.lang.String, ciphertext: java.lang.String, messageIndex: int, mac: java.lang.String, signature: java.lang.String) -> void),
                &[
                    JValue::Object(&base64),
                    JValue::Object(&ciphertext),
                    JValue::Int(message.message_index() as jint),
                    JValue::Object(&mac),
                    JValue::Object(&signature),
                ],
            )?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativePickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const GroupSession) };

            json_to_jstring(env, &session.pickle())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncryptedPickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    key: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const GroupSession) };
            let key = wrap(env, env.convert_byte_array(key)?)?;

            let encrypted = session.pickle().encrypt(&key);
            string_to_jstring(env, encrypted)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str: String = pickle_data.to_string();

            let pickle_data: GroupSessionPickle = from_json(env, &pickle_str)?;
            Ok(box_to_jlong(GroupSession::from_pickle(pickle_data)))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncryptedUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str: String = pickle_data.to_string();
            let key = wrap(env, env.convert_byte_array(key)?)?;

            let pickle_data = GroupSessionPickle::from_encrypted(&pickle_str, &key)
                .map_err(|e| throw_pickle_error(env, e))?;
            Ok(box_to_jlong(GroupSession::from_pickle(pickle_data)))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeUnpickleLegacy(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    pickle_key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str: String = pickle_data.to_string();
            let pickle_key = env.convert_byte_array(pickle_key)?;

            let session = GroupSession::from_libolm_pickle(&pickle_str, &pickle_key)
                .map_err(|e| throw_pickle_error(env, e))?;
            Ok(box_to_jlong(session))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<GroupSession>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use crate::helpers::get_jvm;

    use super::*;
    use vodozemac::megolm::SessionConfig as MegolmSessionConfig;

    #[test]
    fn test_megolm_session_config_version_1() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let result = megolm_session_config_from_version(env, 1);
            assert!(
                result.is_ok(),
                "Megolm version 1 should produce a valid SessionConfig"
            );
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_megolm_session_config_version_2() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let result = megolm_session_config_from_version(env, 2);
            assert!(
                result.is_ok(),
                "Megolm version 2 should produce a valid SessionConfig"
            );
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_megolm_session_config_invalid_version() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let result = megolm_session_config_from_version(env, 0);
            assert!(result.is_err(), "Version 0 should produce an error");
            assert!(
                env.exception_check(),
                "a Java exception should have been thrown"
            );
            env.exception_clear();
            Ok(())
        })
        .expect("JVM test failed");
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let result = megolm_session_config_from_version(env, 3);
            assert!(result.is_err(), "Version 3 should produce an error");
            assert!(
                env.exception_check(),
                "a Java exception should have been thrown"
            );
            env.exception_clear();
            Ok(())
        })
        .expect("JVM test failed");
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let result = megolm_session_config_from_version(env, -1);
            assert!(result.is_err(), "Version -1 should produce an error");
            assert!(
                env.exception_check(),
                "a Java exception should have been thrown"
            );
            env.exception_clear();
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_megolm_group_session_encrypt() {
        let mut session = GroupSession::new(MegolmSessionConfig::version_2());
        assert!(!session.session_id().is_empty());
        assert_eq!(session.message_index(), 0);

        let message = session.encrypt("Hello Megolm");
        assert_eq!(session.message_index(), 1);
        assert!(!message.to_base64().is_empty());
    }

    #[test]
    fn test_megolm_group_session_pickle_roundtrip() {
        let session = GroupSession::new(MegolmSessionConfig::version_1());
        let original_id = session.session_id();
        let pickle = session.pickle();
        let json = serde_json::to_string(&pickle).expect("Should serialize");
        let restored: GroupSessionPickle = serde_json::from_str(&json).expect("Should deserialize");
        let restored_session = GroupSession::from_pickle(restored);
        assert_eq!(restored_session.session_id(), original_id);
    }

    #[test]
    fn test_megolm_group_session_encrypted_pickle_roundtrip() {
        use crate::helpers::PICKLE_KEY;
        let session = GroupSession::new(MegolmSessionConfig::version_2());
        let original_id = session.session_id();
        let pickle = session.pickle();
        let encrypted = pickle.encrypt(&PICKLE_KEY);
        let restored = GroupSessionPickle::from_encrypted(&encrypted, &PICKLE_KEY)
            .expect("Should decrypt pickle");
        let restored_session = GroupSession::from_pickle(restored);
        assert_eq!(restored_session.session_id(), original_id);
    }
}
