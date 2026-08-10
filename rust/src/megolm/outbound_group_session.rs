use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jint, jlong, jstring};
use vodozemac::megolm::{GroupSession, GroupSessionPickle};

use crate::errors::throw_pickle_error;
use crate::helpers::{megolm_session_config_from_version, wrap};

// ============================================================================
// Megolm: OutboundGroupSession (wraps vodozemac::megolm::GroupSession)
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
    version: jint,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let config = megolm_session_config_from_version(version)?;

        let session = Box::new(GroupSession::new(config));
        Ok(Box::into_raw(session) as jlong)
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
        let session = unsafe { &*(ptr as *const GroupSession) };

        let session_id = session.session_id();
        let jni_string = env.new_string(session_id)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeMessageIndex(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let outcome = env.with_env(|_env| -> Result<jint, jni::errors::Error> {
        let session = unsafe { &*(ptr as *const GroupSession) };

        Ok(session.message_index() as jint)
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
        let session = unsafe { &*(ptr as *const GroupSession) };

        let session_key = session.session_key().to_base64();
        let result = env.new_string(session_key)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    plaintext: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let session = unsafe { &mut *(ptr as *mut GroupSession) };
        let plaintext_bytes = env.convert_byte_array(&plaintext)?;

        let message = session.encrypt(&plaintext_bytes).to_base64();
        let result = env.new_string(message)?;
        Ok(result.into_raw())
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
        let session = unsafe { &*(ptr as *const GroupSession) };

        let pickle_data = session.pickle();
        let json_string =
            serde_json::to_string(&pickle_data).map_err(|e| throw_pickle_error(env, e))?;
        let jni_string = env.new_string(json_string)?;
        Ok(jni_string.into_raw())
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
        let session = unsafe { &*(ptr as *const GroupSession) };
        let key = wrap(env.convert_byte_array(key)?)?;

        let encrypted = session.pickle().encrypt(&key);
        let jni_string = env.new_string(encrypted)?;
        Ok(jni_string.into_raw())
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
        let pickle_str: String = pickle_data.to_string();

        let pickle_data: GroupSessionPickle =
            serde_json::from_str(&pickle_str).map_err(|e| throw_pickle_error(env, e))?;
        let session = Box::new(GroupSession::from_pickle(pickle_data));
        Ok(Box::into_raw(session) as jlong)
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
        let pickle_str: String = pickle_data.to_string();
        let key = wrap(env.convert_byte_array(key)?)?;

        let pickle_data = GroupSessionPickle::from_encrypted(&pickle_str, &key)
            .map_err(|e| throw_pickle_error(env, e))?;
        let session = Box::new(GroupSession::from_pickle(pickle_data));
        Ok(Box::into_raw(session) as jlong)
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
        let pickle_str: String = pickle_data.to_string();
        let pickle_key = env.convert_byte_array(&pickle_key)?;

        let session = GroupSession::from_libolm_pickle(&pickle_str, &pickle_key)
            .map_err(|e| throw_pickle_error(env, e))?;
        let session = Box::new(session);
        Ok(Box::into_raw(session) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut GroupSession);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vodozemac::megolm::SessionConfig as MegolmSessionConfig;

    #[test]
    fn test_megolm_session_config_version_1() {
        let result = megolm_session_config_from_version(1);
        assert!(
            result.is_ok(),
            "Megolm version 1 should produce a valid SessionConfig"
        );
    }

    #[test]
    fn test_megolm_session_config_version_2() {
        let result = megolm_session_config_from_version(2);
        assert!(
            result.is_ok(),
            "Megolm version 2 should produce a valid SessionConfig"
        );
    }

    #[test]
    fn test_megolm_session_config_invalid_version() {
        assert!(megolm_session_config_from_version(0).is_err());
        assert!(megolm_session_config_from_version(3).is_err());
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
