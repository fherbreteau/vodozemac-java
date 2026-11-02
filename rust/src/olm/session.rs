use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jboolean, jint, jlong, jobject, jstring};
use vodozemac::olm::{OlmMessage, Session, SessionPickle};

use super::{to_java_olm_message, to_java_session_keys};
use crate::errors::{throw_decryption_error, throw_encryption_error, throw_pickle_error};
use crate::helpers::{
    box_to_jlong, catch_panic, check_ptr, from_json, json_to_jstring, native_free,
    string_to_jstring, wrap,
};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeFree(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|env| -> Result<(), jni::errors::Error> {
        native_free::<Session>(env, ptr);
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeSessionId(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const Session) };

            let session_id = session.session_id();
            string_to_jstring(env, session_id)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeSessionKeys(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const Session) };

            let session_keys = session.session_keys();
            let result = to_java_session_keys(env, &session_keys)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeSessionConfig(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let outcome = env.with_env(|env| -> Result<jint, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const Session) };

            let session_config = session.session_config();
            Ok(session_config.version() as jint)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeHasReceivedMessage(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    let outcome = env.with_env(|env| -> Result<jboolean, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const Session) };

            Ok(session.has_received_message())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeEncrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    plaintext: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut Session) };
            let plaintext_bytes = env.convert_byte_array(plaintext)?;

            let olm_message = session
                .encrypt(&plaintext_bytes)
                .map_err(|e| throw_encryption_error(env, e))?;
            let result = to_java_olm_message(env, &olm_message)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeDecrypt(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    message: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &mut *(ptr as *mut Session) };
            let message_str = message.try_to_string(env)?;

            let olm_message: OlmMessage =
                serde_json::from_str(&message_str).map_err(|e| throw_decryption_error(env, e))?;
            let plaintext = session
                .decrypt(&olm_message)
                .map_err(|e| throw_decryption_error(env, e))?;
            let plaintext_bytes = env.byte_array_from_slice(&plaintext)?;
            Ok(plaintext_bytes.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativePickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const Session) };

            json_to_jstring(env, &session.pickle())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeEncryptedPickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    key: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        catch_panic(env, |env| {
            check_ptr(env, ptr)?;
            let session = unsafe { &*(ptr as *const Session) };
            let key = wrap(env, env.convert_byte_array(key)?)?;

            let encrypted = session.pickle().encrypt(&key);
            string_to_jstring(env, encrypted)
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str = pickle_data.try_to_string(env)?;

            let pickle_data: SessionPickle = from_json(env, &pickle_str)?;
            Ok(box_to_jlong(Session::from_pickle(pickle_data)))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeEncryptedUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str = pickle_data.try_to_string(env)?;
            let key = wrap(env, env.convert_byte_array(key)?)?;

            let pickle_data: SessionPickle = SessionPickle::from_encrypted(&pickle_str, &key)
                .map_err(|e| throw_pickle_error(env, e))?;
            Ok(box_to_jlong(Session::from_pickle(pickle_data)))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeUnpickleLegacy(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    pickle_key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        catch_panic(env, |env| {
            let pickle_str = pickle_data.try_to_string(env)?;
            let pickle_key = env.convert_byte_array(pickle_key)?;

            let session = Session::from_libolm_pickle(&pickle_str, &pickle_key)
                .map_err(|e| throw_pickle_error(env, e))?;
            Ok(box_to_jlong(session))
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{PICKLE_KEY, box_to_jlong, get_jvm};
    use jni::Env;
    use jni::objects::JObject;
    use jni::{jni_sig, jni_str};
    use vodozemac::olm::{Account, SessionConfig};

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        unsafe {
            f(
                EnvUnowned::from_raw(env.get_raw()),
                JClass::from_raw(env, std::ptr::null_mut()),
            )
        }
    }

    #[test]
    fn test_session_jni_lifecycle_and_operations() {
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeDecrypt as decrypt;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeEncrypt as encrypt;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeEncryptedPickle as encrypted_pickle;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeEncryptedUnpickle as encrypted_unpickle;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeHasReceivedMessage as has_received;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativePickle as pickle;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeSessionConfig as session_config;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeSessionId as session_id;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeSessionKeys as session_keys;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeUnpickle as unpickle;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeUnpickleLegacy as unpickle_legacy;

        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let alice = Account::new();
                let mut bob = Account::new();
                bob.generate_one_time_keys(1);
                let bob_key = *bob.one_time_keys().values().next().unwrap();
                let session = alice
                    .create_outbound_session(
                        SessionConfig::version_2(),
                        bob.curve25519_key(),
                        bob_key,
                    )
                    .expect("session creation should succeed");
                let ptr = box_to_jlong(session);

                let id = call(env, |unowned, class| session_id(unowned, class, ptr));
                let id = JString::from_raw(env, id).try_to_string(env)?;
                assert!(!id.is_empty());
                assert_eq!(
                    call(env, |unowned, class| session_config(unowned, class, ptr)),
                    2
                );
                assert!(!call(env, |unowned, class| has_received(
                    unowned, class, ptr
                )));
                let keys = call(env, |unowned, class| session_keys(unowned, class, ptr));
                assert!(!keys.is_null());
                let keys = JObject::from_raw(env, keys);
                let java_id = env
                    .call_method(
                        &keys,
                        jni_str!("sessionId"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                assert_eq!(
                    JString::from_raw(env, java_id.into_raw()).try_to_string(env)?,
                    id
                );
                for key in [
                    env.call_method(
                        &keys,
                        jni_str!("identityKey"),
                        jni_sig!(() -> io.github.fherbreteau.vodozemac.types.Curve25519PublicKey),
                        &[],
                    )?
                    .l()?,
                    env.call_method(
                        &keys,
                        jni_str!("baseKey"),
                        jni_sig!(() -> io.github.fherbreteau.vodozemac.types.Curve25519PublicKey),
                        &[],
                    )?
                    .l()?,
                    env.call_method(
                        &keys,
                        jni_str!("oneTimeKey"),
                        jni_sig!(() -> io.github.fherbreteau.vodozemac.types.Curve25519PublicKey),
                        &[],
                    )?
                    .l()?,
                ] {
                    assert!(!key.is_null());
                }

                let plaintext = b"session JNI lifecycle";
                let plaintext_array = env.byte_array_from_slice(plaintext)?;
                let message = call(env, |unowned, class| {
                    encrypt(unowned, class, ptr, plaintext_array)
                });
                assert!(!message.is_null());
                let message = JObject::from_raw(env, message);
                let serialized = env
                    .call_method(
                        &message,
                        jni_str!("toJson"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                let serialized =
                    JString::from_raw(env, serialized.into_raw()).try_to_string(env)?;
                let prekey: OlmMessage = serde_json::from_str(&serialized)
                    .expect("JNI message should serialize as Olm JSON");
                let prekey = match prekey {
                    OlmMessage::PreKey(message) => message,
                    OlmMessage::Normal(_) => panic!("first outbound message must be pre-key"),
                };
                let bob_session = bob
                    .create_inbound_session(
                        SessionConfig::version_2(),
                        alice.curve25519_key(),
                        &prekey,
                    )
                    .expect("inbound session creation should succeed")
                    .session;
                let bob_ptr = box_to_jlong(bob_session);
                let reply = env.byte_array_from_slice(b"JNI reply")?;
                let reply = call(env, |unowned, class| {
                    encrypt(unowned, class, bob_ptr, reply)
                });
                assert!(!reply.is_null());
                let reply = JObject::from_raw(env, reply);
                let serialized_reply = env
                    .call_method(
                        &reply,
                        jni_str!("toJson"),
                        jni_sig!(() -> java.lang.String),
                        &[],
                    )?
                    .l()?;
                let serialized_reply = JString::from_raw(env, serialized_reply.into_raw());
                let plain = call(env, |unowned, class| {
                    decrypt(unowned, class, ptr, serialized_reply)
                });
                assert!(!plain.is_null());
                assert_eq!(
                    env.convert_byte_array(jni::objects::JByteArray::from_raw(env, plain))?,
                    b"JNI reply"
                );
                assert!(call(env, |unowned, class| has_received(
                    unowned, class, ptr
                )));

                let pickle = call(env, |unowned, class| pickle(unowned, class, ptr));
                let pickle = JString::from_raw(env, pickle).try_to_string(env)?;
                let pickle_arg = env.new_string(&pickle)?;
                let restored = call(env, |unowned, class| unpickle(unowned, class, pickle_arg));
                assert_ne!(restored, 0);
                let key = env.byte_array_from_slice(&PICKLE_KEY)?;
                let encrypted = call(env, |unowned, class| {
                    encrypted_pickle(unowned, class, ptr, key)
                });
                let encrypted = JString::from_raw(env, encrypted).try_to_string(env)?;
                let encrypted_arg = env.new_string(&encrypted)?;
                let key = env.byte_array_from_slice(&PICKLE_KEY)?;
                let restored_encrypted = call(env, |unowned, class| {
                    encrypted_unpickle(unowned, class, encrypted_arg, key)
                });
                assert_ne!(restored_encrypted, 0);

                let pickle_key = env.byte_array_from_slice(b"DEFAULT_PICKLE_KEY")?;
                let legacy_pickle = env.new_string(concat!(
                    "OTC2YvFEfkLiOCiVcLH6Jk6Akmak7QKT8E9s6YpAsP/WoijQFg+RRODQrnHmSBQp",
                    "DnG7CCD8/qOykPihrs8FCJABqCpXqMoel4awkKZ60/YBCiFt9kGnSTR2RUezupKO",
                    "3vVWTx6nciMl9B23n8Ru2+v1LJSMZ4OBSKbj7LIESKYRQN8LKhIIrYfW6x7hoH5J",
                    "8NvcmrKBe66422tMBf0fTYV4576WBW+68gWi3Rg+J56e1JIzovpJ5NJmMX7D3Igg",
                    "1+CDr64SCyV2WtVC0r+tNPv+VbsxrnhoD3/TYyPWpqGrlWYk6ntM0w"
                ))?;
                let restored_legacy = call(env, |unowned, class| {
                    unpickle_legacy(unowned, class, legacy_pickle, pickle_key)
                });
                assert_ne!(restored_legacy, 0);

                for owned_ptr in [ptr, bob_ptr, restored, restored_encrypted, restored_legacy] {
                    call(env, |unowned, class| {
                        Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeFree(
                            unowned, class, owned_ptr,
                        )
                    });
                }
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_session_jni_errors_clear_exceptions() {
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeDecrypt as decrypt;
        use super::Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeUnpickle as unpickle;

        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let invalid_pickle = env.new_string("not-json")?;
                assert_eq!(
                    call(env, |unowned, class| unpickle(
                        unowned,
                        class,
                        invalid_pickle
                    )),
                    0
                );
                assert!(env.exception_check());
                env.exception_clear();

                let account = Account::new();
                let mut peer = Account::new();
                peer.generate_one_time_keys(1);
                let peer_key = *peer.one_time_keys().values().next().unwrap();
                let ptr = box_to_jlong(
                    account
                        .create_outbound_session(
                            SessionConfig::version_2(),
                            peer.curve25519_key(),
                            peer_key,
                        )
                        .expect("session creation should succeed"),
                );
                let invalid_message = env.new_string("not-json")?;
                assert!(
                    call(env, |unowned, class| decrypt(
                        unowned,
                        class,
                        ptr,
                        invalid_message
                    ))
                    .is_null()
                );
                assert!(env.exception_check());
                env.exception_clear();
                call(env, |unowned, class| {
                    Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeFree(
                        unowned, class, ptr,
                    )
                });
            }
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_session_pickle_roundtrip() {
        let alice = Account::new();
        let mut bob = Account::new();
        let _ = bob.generate_one_time_keys(1);
        let bob_keys = bob.one_time_keys();
        let bob_identity = bob.curve25519_key();
        let bob_one_time = *bob_keys.values().next().unwrap();
        let session = alice
            .create_outbound_session(SessionConfig::version_2(), bob_identity, bob_one_time)
            .expect("Should create the outbound session");

        let pickle = session.pickle();
        let json = serde_json::to_string(&pickle).expect("Should serialize to JSON");
        let restored: SessionPickle =
            serde_json::from_str(&json).expect("Should deserialize from JSON");
        let restored_session = Session::from_pickle(restored);

        assert_eq!(restored_session.session_id(), session.session_id());
    }

    #[test]
    fn test_session_encrypt_decrypt() {
        let alice = Account::new();
        let mut bob = Account::new();
        let _ = bob.generate_one_time_keys(1);
        let bob_keys = bob.one_time_keys();
        let bob_identity = bob.curve25519_key();
        let bob_one_time = *bob_keys.values().next().unwrap();

        let mut alice_session = alice
            .create_outbound_session(SessionConfig::version_1(), bob_identity, bob_one_time)
            .expect("Should create the outbound session");
        let plaintext = b"Hello from test";
        let olm_message = alice_session
            .encrypt(plaintext)
            .expect("Should encrypt the Pre Key message");

        assert!(!alice_session.has_received_message());
        let pre_key_message = match olm_message {
            OlmMessage::PreKey(pk) => pk,
            OlmMessage::Normal(_) => panic!("First message should be pre-key"),
        };
        let decrypted = bob
            .create_inbound_session(
                SessionConfig::version_1(),
                alice.curve25519_key(),
                &pre_key_message,
            )
            .expect("Should create inbound session");
        assert_eq!(decrypted.plaintext, plaintext);
    }

    #[test]
    fn test_session_encrypt_decrypt_with_v2_config() {
        let alice = Account::new();
        let mut bob = Account::new();
        let _ = bob.generate_one_time_keys(1);
        let bob_keys = bob.one_time_keys();
        let bob_identity = bob.curve25519_key();
        let bob_one_time = *bob_keys.values().next().unwrap();

        let mut alice_session = alice
            .create_outbound_session(SessionConfig::version_2(), bob_identity, bob_one_time)
            .expect("Should create the outbound session");
        assert_eq!(alice_session.session_config().version(), 2);

        let first = alice_session
            .encrypt(b"first message")
            .expect("Should encrypt the Pre Key message");
        let pre_key_message = match first {
            OlmMessage::PreKey(pk) => pk,
            OlmMessage::Normal(_) => panic!("First message should be pre-key"),
        };

        let mut bob_session = bob
            .create_inbound_session(
                SessionConfig::version_2(),
                alice.curve25519_key(),
                &pre_key_message,
            )
            .expect("Should create inbound session");
        assert_eq!(bob_session.session.session_config().version(), 2);

        let reply = bob_session
            .session
            .encrypt(b"reply")
            .expect("Should encrypt the reply");
        let decrypted_reply = alice_session
            .decrypt(&reply)
            .expect("Should decrypt the reply");
        assert_eq!(decrypted_reply, b"reply");

        let second = alice_session
            .encrypt(b"second message")
            .expect("Should encrypt a normal message");
        assert!(
            matches!(second, OlmMessage::Normal(_)),
            "Messages should be normal once a message has been received"
        );
        let decrypted = bob_session
            .session
            .decrypt(&second)
            .expect("Should decrypt the normal message");
        assert_eq!(decrypted, b"second message");
    }
}
