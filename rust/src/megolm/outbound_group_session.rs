use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jint, jlong, jobject, jstring};
use vodozemac::megolm::{GroupSession, GroupSessionPickle};

use crate::errors::throw_pickle_error;
use crate::helpers::{
    box_to_jlong, catch_panic, check_ptr, from_json, json_to_jstring,
    megolm_session_config_from_version, native_free, string_to_jstring, wrap,
};
use crate::megolm::to_java_megolm_message;

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
            string_to_jstring(env, session_id)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeMessageIndex(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const GroupSession) };

            Ok(session.message_index() as jlong)
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
            string_to_jstring(env, session_key)
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
            let result = to_java_megolm_message(env, &message)?;
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
            let pickle_str = pickle_data.try_to_string(env)?;

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
            let pickle_str = pickle_data.try_to_string(env)?;
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
            let pickle_str = pickle_data.try_to_string(env)?;
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

    unsafe fn call<R>(env: &mut jni::Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_jni_outbound_session_lifecycle_and_pickle_errors() {
        use jni::objects::JObject;
        use jni::{jni_sig, jni_str};

        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let ptr = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeNew(
                        unowned, class, 2,
                    )
                });
                assert_ne!(ptr, 0);

                let session_id = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionId(
                        unowned, class, ptr,
                    )
                });
                let session_id = JString::from_raw(env, session_id).try_to_string(env)?;
                assert!(!session_id.is_empty());
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeMessageIndex(
                            unowned, class, ptr,
                        )
                    }),
                    0
                );
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionConfig(
                            unowned, class, ptr,
                        )
                    }),
                    2
                );
                let session_key = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeSessionKey(
                        unowned, class, ptr,
                    )
                });
                let session_key = JString::from_raw(env, session_key).try_to_string(env)?;
                assert!(!session_key.is_empty());

                let plaintext = env.byte_array_from_slice(b"JNI Megolm plaintext")?;
                let message = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncrypt(
                        unowned, class, ptr, plaintext,
                    )
                });
                let message = JObject::from_raw(env, message);
                let base64 = env
                    .call_method(&message, jni_str!("toString"), jni_sig!(() -> java.lang.String), &[])?
                    .l()?;
                let base64 = JString::from_raw(env, base64.as_raw()).try_to_string(env)?;
                assert!(!base64.is_empty());
                assert_eq!(
                    env.call_method(&message, jni_str!("messageIndex"), jni_sig!(() -> long), &[])?
                        .j()?,
                    0
                );
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeMessageIndex(
                            unowned, class, ptr,
                        )
                    }),
                    1
                );

                let pickle = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativePickle(
                        unowned, class, ptr,
                    )
                });
                let pickle = JString::from_raw(env, pickle).try_to_string(env)?;
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                let encrypted_pickle = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncryptedPickle(
                        unowned, class, ptr, key,
                    )
                });
                let encrypted_pickle =
                    JString::from_raw(env, encrypted_pickle).try_to_string(env)?;

                let pickle_string = env.new_string(&pickle)?;
                let restored = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeUnpickle(
                        unowned, class, pickle_string,
                    )
                });
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                let encrypted_string = env.new_string(&encrypted_pickle)?;
                let encrypted_restored = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncryptedUnpickle(
                        unowned, class, encrypted_string, key,
                    )
                });
                assert_ne!(restored, 0);
                assert_ne!(encrypted_restored, 0);

                let bad_json_pickle = env.new_string("invalid pickle")?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeUnpickle(
                            unowned, class, bad_json_pickle,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let bad_pickle = env.new_string("invalid pickle")?;
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeEncryptedUnpickle(
                            unowned, class, bad_pickle, key,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let bad_pickle = env.new_string("invalid legacy pickle")?;
                let key = env.byte_array_from_slice(&crate::helpers::PICKLE_KEY)?;
                assert_eq!(
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeUnpickleLegacy(
                            unowned, class, bad_pickle, key,
                        )
                    }),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let invalid = call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeNew(
                        unowned, class, 3,
                    )
                });
                assert_eq!(invalid, 0);
                assert!(env.exception_check());
                env.exception_clear();

                call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeFree(
                        unowned, class, restored,
                    )
                });
                call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeFree(
                        unowned, class, encrypted_restored,
                    )
                });
                call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_megolm_OutboundGroupSession_nativeFree(
                        unowned, class, ptr,
                    )
                });
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
