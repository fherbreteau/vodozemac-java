use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong, jobject, jstring};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::megolm::{
    ExportedSessionKey, InboundGroupSession, InboundGroupSessionPickle, MegolmMessage, SessionKey,
    SessionOrdering,
};

use super::to_java_decrypted_message;
use crate::classes::{JAVA_LONG, SESSION_ORDERING};
use crate::errors::{
    throw_conversion_error, throw_decode_error, throw_megolm_decryption_error, throw_pickle_error,
    throw_session_key_decode_error,
};
use crate::helpers::{
    RawBox, box_to_jlong, catch_panic, check_ptr, from_json, json_to_jstring,
    megolm_session_config_from_version, native_free, string_to_jstring, wrap,
};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
    session_key: JString,
    version: jint,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let config = megolm_session_config_from_version(env, version)?;
            let session_key = SessionKey::from_base64(&session_key.try_to_string(env)?)
                .map_err(|e| throw_session_key_decode_error(env, e))?;

            let session = InboundGroupSession::new(&session_key, config);
            Ok(box_to_jlong(session))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeSessionId(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const InboundGroupSession) };

            let session_id = session.session_id();
            string_to_jstring(env, session_id)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFirstKnownIndex(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const InboundGroupSession) };

            Ok(session.first_known_index() as jlong)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeDecrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    message: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
            let message_str = message.try_to_string(env)?;
            let megolm_message =
                MegolmMessage::from_base64(&message_str).map_err(|e| throw_decode_error(env, e))?;

            let decrypted = session
                .decrypt(&megolm_message)
                .map_err(|e| throw_megolm_decryption_error(env, e))?;

            let result =
                to_java_decrypted_message(env, &decrypted.plaintext, decrypted.message_index)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativePickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const InboundGroupSession) };

            json_to_jstring(env, &session.pickle())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeEncryptedPickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    key: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const InboundGroupSession) };
            let key = wrap(env, env.convert_byte_array(key)?)?;

            let encrypted = session.pickle().encrypt(&key);
            string_to_jstring(env, encrypted)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeExportAt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    index: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
            let index = u32::try_from(index).map_err(|e| throw_conversion_error(env, e))?;

            let result = session.export_at(index);
            match result {
                Some(key) => string_to_jstring(env, key.to_base64()),
                None => Ok(std::ptr::null_mut()),
            }
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeExportAtFirstKnownIndex(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };

            let result = session.export_at_first_known_index();
            string_to_jstring(env, result.to_base64())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeAdvanceTo(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    index: jlong,
) -> jboolean {
    let outcome = env.with_env(|env| -> Result<jboolean, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
            let index = u32::try_from(index).map_err(|e| throw_conversion_error(env, e))?;

            let result = session.advance_to(index);
            Ok(result as jboolean)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn check_self_pointer(
    env: &mut Env,
    ptr: jlong,
    other_ptr: jlong,
) -> Result<(), jni::errors::Error> {
    if ptr == other_ptr {
        return Err(throw_conversion_error(
            env,
            "Cannot compare a session with itself",
        ));
    }
    Ok(())
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeConnected(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    other_ptr: jlong,
) -> jboolean {
    let outcome = env.with_env(|env| -> Result<jboolean, jni::errors::Error> {
        catch_panic(env, |env| {
            check_self_pointer(env, ptr, other_ptr)?;
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
            check_ptr(env, other_ptr)?;
            let other_session = unsafe { &mut *(other_ptr as *mut InboundGroupSession) };

            let result = session.connected(other_session);
            Ok(result as jboolean)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn session_ordering_to_jobject<'local>(
    env: &mut Env<'local>,
    ordering: SessionOrdering,
) -> Result<JObject<'local>, jni::errors::Error> {
    let name = match ordering {
        SessionOrdering::Equal => jni_str!("EQUAL"),
        SessionOrdering::Better => jni_str!("BETTER"),
        SessionOrdering::Worse => jni_str!("WORSE"),
        SessionOrdering::Unconnected => jni_str!("UNCONNECTED"),
    };
    env.get_static_field(
        SESSION_ORDERING,
        name,
        jni_sig!(io.github.fherbreteau.vodozemac.megolm.SessionOrdering),
    )
    .and_then(|value| value.l())
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeCompare(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    other_ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_self_pointer(env, ptr, other_ptr)?;
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
            check_ptr(env, other_ptr)?;
            let other_session = unsafe { &mut *(other_ptr as *mut InboundGroupSession) };

            let result = session.compare(other_session);
            let ordering = session_ordering_to_jobject(env, result)
                .map_err(|e| throw_conversion_error(env, e))?;
            Ok(ordering.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeMerge(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    other_ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_self_pointer(env, ptr, other_ptr)?;
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
            check_ptr(env, other_ptr)?;
            let other_session = unsafe { &mut *(other_ptr as *mut InboundGroupSession) };

            let result = session.merge(other_session);
            match result {
                Some(new_session) => {
                    let new_box = RawBox::new(new_session);
                    let result = env.new_object(
                        JAVA_LONG,
                        jni_sig!((long) -> void),
                        &[JValue::Long(new_box.as_jlong())],
                    )?;
                    new_box.leak();
                    Ok(result.into_raw())
                }
                None => Ok(std::ptr::null_mut()),
            }
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<InboundGroupSession>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str = pickle_data.try_to_string(env)?;
            let pickle_data: InboundGroupSessionPickle = from_json(env, &pickle_str)?;

            Ok(box_to_jlong(InboundGroupSession::from_pickle(pickle_data)))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeEncryptedUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str = pickle_data.try_to_string(env)?;
            let key = wrap(env, env.convert_byte_array(key)?)?;
            let pickle_data = InboundGroupSessionPickle::from_encrypted(&pickle_str, &key)
                .map_err(|e| throw_pickle_error(env, e))?;

            Ok(box_to_jlong(InboundGroupSession::from_pickle(pickle_data)))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeUnpickleLegacy(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    pickle_key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str = pickle_data.try_to_string(env)?;
            let pickle_key = env.convert_byte_array(pickle_key)?;
            let session = InboundGroupSession::from_libolm_pickle(&pickle_str, &pickle_key)
                .map_err(|e| throw_pickle_error(env, e))?;

            Ok(box_to_jlong(session))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeImport(
    mut env: EnvUnowned,
    _class: JClass,
    session_key: JString,
    version: jint,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let config = megolm_session_config_from_version(env, version)?;
            let session_str = session_key.try_to_string(env)?;
            let exported_session = ExportedSessionKey::from_base64(&session_str)
                .map_err(|e| throw_session_key_decode_error(env, e))?;

            let session = InboundGroupSession::import(&exported_session, config);
            Ok(box_to_jlong(session))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::PICKLE_KEY;
    use crate::megolm::outbound_group_session::{
        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncrypt,
        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeFree,
        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeNew,
        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionKey,
    };
    use vodozemac::megolm::{GroupSession, SessionConfig as MegolmSessionConfig};

    #[test]
    fn test_megolm_inbound_encrypt_and_decrypt() {
        let mut outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert_eq!(inbound.session_id(), outbound.session_id());
        assert_eq!(inbound.first_known_index(), 0);

        let plaintext = "Hello Megolm!";
        let message = outbound.encrypt(plaintext);
        let decrypted = inbound.decrypt(&message).expect("Should decrypt");

        assert_eq!(decrypted.plaintext, plaintext.as_bytes());
        assert_eq!(decrypted.message_index, 0);
    }

    #[test]
    fn test_megolm_inbound_pickle_roundtrip() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_1());
        let session_key = outbound.session_key();
        let inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_1());
        let original_id = inbound.session_id();

        let pickle = inbound.pickle();
        let json = serde_json::to_string(&pickle).expect("Should serialize");
        let restored: InboundGroupSessionPickle =
            serde_json::from_str(&json).expect("Should deserialize");
        let restored_inbound = InboundGroupSession::from_pickle(restored);

        assert_eq!(restored_inbound.session_id(), original_id);
    }

    #[test]
    fn test_megolm_inbound_encrypted_pickle_roundtrip() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let original_id = inbound.session_id();

        let pickle = inbound.pickle();
        let encrypted = pickle.encrypt(&PICKLE_KEY);
        let restored = InboundGroupSessionPickle::from_encrypted(&encrypted, &PICKLE_KEY)
            .expect("Should decrypt");
        let restored_inbound = InboundGroupSession::from_pickle(restored);

        assert_eq!(restored_inbound.session_id(), original_id);
    }

    #[test]
    fn test_megolm_inbound_export_at_first_known_index() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        let exported = inbound.export_at_first_known_index();
        assert!(!exported.to_base64().is_empty());
    }

    #[test]
    fn test_megolm_inbound_export_at_specific_index() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert!(
            inbound.export_at(5).is_some(),
            "Should be able to export at index 5"
        );
        assert!(
            inbound.export_at(0).is_some(),
            "Should be able to export at index 0"
        );
    }

    #[test]
    fn test_megolm_inbound_export_at_after_advance() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert!(inbound.advance_to(10));
        assert!(
            inbound.export_at(10).is_some(),
            "Should export at first known index 10"
        );
        assert!(
            inbound.export_at(5).is_none(),
            "Should not export at index below first known"
        );
    }

    #[test]
    fn test_megolm_inbound_import_from_export() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        let exported = inbound.export_at_first_known_index();
        let imported = InboundGroupSession::import(&exported, MegolmSessionConfig::version_2());

        assert_eq!(imported.session_id(), inbound.session_id());
        assert_eq!(imported.first_known_index(), 0);
    }

    #[test]
    fn test_megolm_inbound_import_from_export_at_index() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        let exported = inbound.export_at(10).expect("Should export at index 10");
        let imported = InboundGroupSession::import(&exported, MegolmSessionConfig::version_2());

        assert_eq!(imported.session_id(), inbound.session_id());
        assert_eq!(imported.first_known_index(), 10);
    }

    #[test]
    fn test_megolm_inbound_connected_same_session() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let mut session2 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert!(
            session1.connected(&mut session2),
            "Sessions from the same outbound should be connected"
        );
    }

    #[test]
    fn test_megolm_inbound_connected_different_sessions() {
        let outbound1 = GroupSession::new(MegolmSessionConfig::version_2());
        let outbound2 = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key1 = outbound1.session_key();
        let session_key2 = outbound2.session_key();
        let mut session1 =
            InboundGroupSession::new(&session_key1, MegolmSessionConfig::version_2());
        let mut session2 =
            InboundGroupSession::new(&session_key2, MegolmSessionConfig::version_2());

        assert!(
            !session1.connected(&mut session2),
            "Sessions from different outbound should not be connected"
        );
    }

    #[test]
    fn test_megolm_inbound_connected_after_advance() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let mut session2 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        session2.advance_to(10);

        assert!(
            session1.connected(&mut session2),
            "Sessions at different indices should still be connected"
        );
    }

    #[test]
    fn test_megolm_inbound_compare_equal() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let mut session2 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert_eq!(
            session1.compare(&mut session2),
            SessionOrdering::Equal,
            "Same index sessions should compare as Equal"
        );
    }

    #[test]
    fn test_megolm_inbound_compare_better() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let mut session2 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        session2.advance_to(10);

        assert_eq!(
            session1.compare(&mut session2),
            SessionOrdering::Better,
            "Lower index session should compare as Better"
        );
    }

    #[test]
    fn test_megolm_inbound_compare_worse() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let mut session2 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        session1.advance_to(10);

        assert_eq!(
            session1.compare(&mut session2),
            SessionOrdering::Worse,
            "Higher index session should compare as Worse"
        );
    }

    #[test]
    fn test_megolm_inbound_compare_unconnected() {
        let outbound1 = GroupSession::new(MegolmSessionConfig::version_2());
        let outbound2 = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key1 = outbound1.session_key();
        let session_key2 = outbound2.session_key();
        let mut session1 =
            InboundGroupSession::new(&session_key1, MegolmSessionConfig::version_2());
        let mut session2 =
            InboundGroupSession::new(&session_key2, MegolmSessionConfig::version_2());

        assert_eq!(
            session1.compare(&mut session2),
            SessionOrdering::Unconnected,
            "Different sessions should compare as Unconnected"
        );
    }

    #[test]
    fn test_megolm_inbound_merge_equal() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let mut session2 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        let merged = session1
            .merge(&mut session2)
            .expect("Should merge equal sessions");

        assert_eq!(merged.first_known_index(), 0);
    }

    #[test]
    fn test_megolm_inbound_merge_better_with_worse() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let mut session2 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        session2.advance_to(10);

        let merged = session1
            .merge(&mut session2)
            .expect("Should merge connected sessions");

        assert_eq!(
            merged.first_known_index(),
            0,
            "Merged session should have lower index"
        );
    }

    #[test]
    fn test_megolm_inbound_merge_unconnected_returns_none() {
        let outbound1 = GroupSession::new(MegolmSessionConfig::version_2());
        let outbound2 = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key1 = outbound1.session_key();
        let session_key2 = outbound2.session_key();
        let mut session1 =
            InboundGroupSession::new(&session_key1, MegolmSessionConfig::version_2());
        let mut session2 =
            InboundGroupSession::new(&session_key2, MegolmSessionConfig::version_2());

        assert!(
            session1.merge(&mut session2).is_none(),
            "Unconnected sessions should not merge"
        );
    }

    #[test]
    fn test_megolm_inbound_merge_preserves_verification() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut session1 = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        let exported = session1.export_at(10).expect("Should export at index 10");
        let mut session2 = InboundGroupSession::import(&exported, MegolmSessionConfig::version_2());

        let merged = session2
            .merge(&mut session1)
            .expect("Should merge connected sessions");

        assert_eq!(
            merged.first_known_index(),
            0,
            "Merged session should use the better ratchet"
        );
    }

    #[test]
    fn test_megolm_inbound_advance_to() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert_eq!(inbound.first_known_index(), 0);

        assert!(inbound.advance_to(10));
        assert_eq!(inbound.first_known_index(), 10);

        assert!(!inbound.advance_to(5), "Should not advance to lower index");
        assert_eq!(inbound.first_known_index(), 10);

        assert!(!inbound.advance_to(10), "Should not advance to same index");
        assert_eq!(inbound.first_known_index(), 10);

        assert!(inbound.advance_to(20));
        assert_eq!(inbound.first_known_index(), 20);
    }

    #[test]
    fn test_megolm_inbound_advance_to_then_decrypt_fails_for_old_messages() {
        let mut outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        let msg0 = outbound.encrypt("first");
        let _msg1 = outbound.encrypt("second");
        let msg2 = outbound.encrypt("third");

        let decrypted0 = inbound.decrypt(&msg0).expect("Should decrypt message 0");
        assert_eq!(decrypted0.plaintext, b"first");

        inbound.advance_to(2);

        assert!(
            inbound.decrypt(&msg0).is_err(),
            "Should not decrypt message with index below first known index"
        );

        let decrypted2 = inbound.decrypt(&msg2).expect("Should decrypt message 2");
        assert_eq!(decrypted2.plaintext, b"third");
    }

    #[test]
    fn test_megolm_inbound_decrypt_multiple_messages() {
        let mut outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        for i in 0..5 {
            let plaintext = format!("Message {i}");
            let message = outbound.encrypt(&plaintext);
            let decrypted = inbound.decrypt(&message).expect("Should decrypt");
            assert_eq!(decrypted.plaintext, plaintext.as_bytes());
            assert_eq!(decrypted.message_index, i);
        }
    }

    #[test]
    fn test_megolm_inbound_session_id_matches_outbound() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert_eq!(inbound.session_id(), outbound.session_id());
    }

    #[test]
    fn test_megolm_inbound_import_preserves_session_id() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        for i in 0..3 {
            let exported = inbound.export_at(i as u32).expect("Should export");
            let imported = InboundGroupSession::import(&exported, MegolmSessionConfig::version_2());
            assert_eq!(
                imported.session_id(),
                inbound.session_id(),
                "Imported session should have same ID at index {i}"
            );
            assert_eq!(imported.first_known_index(), i as u32);
        }
    }

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_jni_inbound_session_lifecycle_and_error_paths() {
        use jni::objects::{JByteArray, JObject};
        use jni::{jni_sig, jni_str};

        let jvm = crate::helpers::get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let outbound_ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeNew(
                        unowned, class, 2,
                    )
                });
                let session_key = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionKey(
                        unowned, class, outbound_ptr,
                    )
                });
                let session_key = JString::from_raw(env, session_key).try_to_string(env)?;
                let session_key_java = env.new_string(&session_key)?;
                let inbound_ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeNew(
                        unowned,
                        class,
                        session_key_java,
                        2,
                    )
                });
                assert_ne!(inbound_ptr, 0);

                let session_id = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeSessionId(
                        unowned, class, inbound_ptr,
                    )
                });
                let session_id = JString::from_raw(env, session_id).try_to_string(env)?;
                assert!(!session_id.is_empty());
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFirstKnownIndex(
                            unowned, class, inbound_ptr,
                        )
                    }),
                    0
                );

                let plaintext = env.byte_array_from_slice(b"JNI inbound plaintext")?;
                let message = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncrypt(
                        unowned, class, outbound_ptr, plaintext,
                    )
                });
                let message = JObject::from_raw(env, message);
                let wire_message = env
                    .call_method(&message, jni_str!("toString"), jni_sig!(() -> java.lang.String), &[])?
                    .l()?;
                let wire_message = JString::from_raw(env, wire_message.as_raw()).try_to_string(env)?;
                let wire_message_java = env.new_string(&wire_message)?;
                let decrypted = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeDecrypt(
                        unowned,
                        class,
                        inbound_ptr,
                        wire_message_java,
                    )
                });
                let decrypted = JObject::from_raw(env, decrypted);
                let decrypted_plaintext = env
                    .call_method(&decrypted, jni_str!("plaintext"), jni_sig!(() -> byte[]), &[])?
                    .l()?;
                let decrypted_plaintext = JByteArray::from_raw(env, decrypted_plaintext.as_raw());
                assert_eq!(env.convert_byte_array(decrypted_plaintext)?, b"JNI inbound plaintext");
                assert_eq!(
                    env.call_method(&decrypted, jni_str!("messageIndex"), jni_sig!(() -> long), &[])?
                        .j()?,
                    0
                );

                let pickle = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativePickle(
                        unowned, class, inbound_ptr,
                    )
                });
                let pickle = JString::from_raw(env, pickle).try_to_string(env)?;
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                let encrypted_pickle = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeEncryptedPickle(
                        unowned, class, inbound_ptr, key,
                    )
                });
                let encrypted_pickle = JString::from_raw(env, encrypted_pickle).try_to_string(env)?;

                let exported = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeExportAt(
                        unowned, class, inbound_ptr, 0,
                    )
                });
                let exported = JString::from_raw(env, exported).try_to_string(env)?;
                assert!(!exported.is_empty());
                let first_export = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeExportAtFirstKnownIndex(
                        unowned, class, inbound_ptr,
                    )
                });
                let first_export = JString::from_raw(env, first_export).try_to_string(env)?;
                assert_eq!(first_export, exported);
                assert!(call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeAdvanceTo(
                        unowned, class, inbound_ptr, 5,
                    )
                }));
                assert!(!call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeAdvanceTo(
                        unowned, class, inbound_ptr, 2,
                    )
                }));
                assert!(call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeExportAt(
                        unowned, class, inbound_ptr, 0,
                    )
                })
                .is_null());
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeExportAt(
                            unowned, class, inbound_ptr, -1,
                        )
                    }),
                    std::ptr::null_mut()
                );
                assert!(env.exception_check());
                env.exception_clear();

                let exported_java = env.new_string(&exported)?;
                let imported_ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeImport(
                        unowned, class, exported_java, 2,
                    )
                });
                let pickle_java = env.new_string(&pickle)?;
                let unpickled_ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeUnpickle(
                        unowned, class, pickle_java,
                    )
                });
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                let encrypted_java = env.new_string(&encrypted_pickle)?;
                let encrypted_unpickled_ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeEncryptedUnpickle(
                        unowned, class, encrypted_java, key,
                    )
                });
                assert_ne!(imported_ptr, 0);
                assert_ne!(unpickled_ptr, 0);
                assert_ne!(encrypted_unpickled_ptr, 0);

                for (left, right, expected) in [
                    (imported_ptr, unpickled_ptr, "EQUAL"),
                    (imported_ptr, inbound_ptr, "BETTER"),
                ] {
                    let ordering = call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeCompare(
                            unowned, class, left, right,
                        )
                    });
                    let ordering = JObject::from_raw(env, ordering);
                    let name = env
                        .call_method(
                            &ordering,
                            jni_str!("name"),
                            jni_sig!(() -> java.lang.String),
                            &[],
                        )?
                        .l()?;
                    let name = JString::from_raw(env, name.as_raw()).try_to_string(env)?;
                    assert_eq!(name, expected);
                }

                assert!(call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeConnected(
                        unowned, class, inbound_ptr, imported_ptr,
                    )
                }));
                let ordering = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeCompare(
                        unowned, class, inbound_ptr, imported_ptr,
                    )
                });
                let ordering = JObject::from_raw(env, ordering);
                let ordering_name = env
                    .call_method(&ordering, jni_str!("name"), jni_sig!(() -> java.lang.String), &[])?
                    .l()?;
                let ordering_name = JString::from_raw(env, ordering_name.as_raw()).try_to_string(env)?;
                assert_eq!(ordering_name, "WORSE");
                let boxed_long = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeMerge(
                        unowned, class, inbound_ptr, imported_ptr,
                    )
                });
                let boxed_long = JObject::from_raw(env, boxed_long);
                let merged_ptr = env
                    .call_method(&boxed_long, jni_str!("longValue"), jni_sig!(() -> long), &[])?
                    .j()?;
                assert_ne!(merged_ptr, 0);

                let bad_message = env.new_string("not a megolm message")?;
                assert!(call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeDecrypt(
                        unowned,
                        class,
                        inbound_ptr,
                        bad_message,
                    )
                })
                .is_null());
                assert!(env.exception_check());
                env.exception_clear();
                let bad_session_key = env.new_string("invalid session key")?;
                let invalid_session = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeNew(
                        unowned,
                        class,
                        bad_session_key,
                        2,
                    )
                });
                assert_eq!(invalid_session, 0);
                assert!(env.exception_check());
                env.exception_clear();

                let other_outbound_ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeNew(
                        unowned, class, 2,
                    )
                });
                let other_key = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionKey(
                        unowned, class, other_outbound_ptr,
                    )
                });
                let other_key = JString::from_raw(env, other_key).try_to_string(env)?;
                let other_key = env.new_string(&other_key)?;
                let other_ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeNew(
                        unowned, class, other_key, 2,
                    )
                });
                assert!(!call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeConnected(
                        unowned, class, inbound_ptr, other_ptr,
                    )
                }));
                assert!(call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeMerge(
                        unowned, class, inbound_ptr, other_ptr,
                    )
                })
                .is_null());
                let unconnected_ordering = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeCompare(
                        unowned, class, inbound_ptr, other_ptr,
                    )
                });
                let unconnected_ordering = JObject::from_raw(env, unconnected_ordering);
                let unconnected_name = env
                    .call_method(
                        &unconnected_ordering,
                        jni_str!("name"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                let unconnected_name =
                    JString::from_raw(env, unconnected_name.as_raw()).try_to_string(env)?;
                assert_eq!(unconnected_name, "UNCONNECTED");
                assert!(call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeCompare(
                        unowned, class, inbound_ptr, inbound_ptr,
                    )
                })
                .is_null());
                assert!(env.exception_check());
                env.exception_clear();

                let bad_key = env.new_string("invalid key")?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeNew(
                            unowned, class, bad_key, 2,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let bad_pickle = env.new_string("invalid pickle")?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeUnpickle(
                            unowned, class, bad_pickle,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let bad_pickle = env.new_string("invalid encrypted pickle")?;
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeEncryptedUnpickle(
                            unowned, class, bad_pickle, key,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let bad_legacy_pickle = env.new_string("invalid legacy pickle")?;
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeUnpickleLegacy(
                            unowned, class, bad_legacy_pickle, key,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let invalid_version_key = env.new_string(&exported)?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeImport(
                            unowned, class, invalid_version_key, 0,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let bad_export = env.new_string("invalid export")?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeImport(
                            unowned, class, bad_export, 2,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                for ptr in [outbound_ptr, other_outbound_ptr] {
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeFree(
                            unowned, class, ptr,
                        )
                    });
                }
                for ptr in [inbound_ptr, imported_ptr, unpickled_ptr, encrypted_unpickled_ptr, other_ptr] {
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFree(
                            unowned, class, ptr,
                        )
                    });
                }
                call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFree(
                        unowned, class, merged_ptr,
                    )
                });
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
