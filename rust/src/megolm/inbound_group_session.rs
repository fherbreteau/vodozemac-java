use std::mem::forget;

use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong, jobject, jstring};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::megolm::{
    ExportedSessionKey, InboundGroupSession, InboundGroupSessionPickle, MegolmMessage, SessionKey,
    SessionOrdering,
};

use crate::errors::{
    throw_decode_error, throw_generic_error, throw_megolm_decryption_error, throw_pickle_error,
    throw_session_key_decode_error,
};
use crate::helpers::{
    box_to_jlong, catch_panic, check_ptr, from_json, json_to_jstring,
    megolm_session_config_from_version, native_free, string_to_jstring, wrap,
};

// Megolm: InboundGroupSession (wraps vodozemac::megolm::InboundGroupSession)
// ============================================================================

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
            let session_key = SessionKey::from_base64(&session_key.to_string())
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
            let jni_string = env.new_string(session_id)?;
            Ok(jni_string.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFirstKnownIndex(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let outcome = env.with_env(|env| -> Result<jint, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const InboundGroupSession) };

            Ok(session.first_known_index() as jint)
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
            let message_str: String = message.to_string();
            let megolm_message =
                MegolmMessage::from_base64(&message_str).map_err(|e| throw_decode_error(env, e))?;

            let decrypted = session
                .decrypt(&megolm_message)
                .map_err(|e| throw_megolm_decryption_error(env, e))?;

            let plaintext_bytes = env.byte_array_from_slice(&decrypted.plaintext)?;
            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/megolm/DecryptedMessage"),
                jni_sig!((plaintext: byte[], messageIndex: int) -> void),
                &[
                    JValue::Object(&plaintext_bytes),
                    JValue::Int(decrypted.message_index as jint),
                ],
            )?;
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
    index: jint,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
            let index = u32::try_from(index).map_err(|e| throw_generic_error(env, e))?;

            let result = session.export_at(index);
            match result {
                Some(key) => {
                    let key_str = env.new_string(key.to_base64())?;
                    Ok(key_str.into_raw())
                }
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
            let key_str = env.new_string(result.to_base64())?;
            Ok(key_str.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeAdvanceTo(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    index: jint,
) -> jboolean {
    let outcome = env.with_env(|env| -> Result<jboolean, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };

            let result = session.advance_to(index as u32);
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
        return Err(throw_generic_error(
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
        jni_str!("io/github/fherbreteau/vodozemac/megolm/SessionOrdering"),
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
                .map_err(|e| throw_generic_error(env, e))?;
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
                    let new_box = Box::new(new_session);
                    let new_ptr = &*new_box as *const InboundGroupSession as jlong;
                    let result = env.new_object(
                        jni_str!("java/lang/Long"),
                        jni_sig!((long) -> void),
                        &[JValue::Long(new_ptr)],
                    )?;
                    forget(new_box);
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
            let pickle_str: String = pickle_data.to_string();
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
            let pickle_str: String = pickle_data.to_string();
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
            let pickle_str: String = pickle_data.to_string();
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
            let session_str: String = session_key.to_string();
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
}
