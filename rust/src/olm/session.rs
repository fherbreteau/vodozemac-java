use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jboolean, jint, jlong, jobject, jstring};
use jni::{EnvUnowned, JValue, jni_sig, jni_str};
use vodozemac::base64_encode;
use vodozemac::olm::{OlmMessage, Session, SessionPickle};

use crate::errors::{throw_decryption_error, throw_generic_error, throw_pickle_error};
use crate::helpers::{
    box_to_jlong, catch_panic, check_ptr, from_json, json_to_jstring, native_free,
    string_to_jstring, wrap,
};
use crate::types::to_java_curve25519;

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
            let session_id = env.new_string(session_keys.session_id())?;
            let identity_key = to_java_curve25519(env, &(session_keys.identity_key))?;
            let base_key = to_java_curve25519(env, &(session_keys.base_key))?;
            let one_time_key = to_java_curve25519(env, &(session_keys.one_time_key))?;
            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/olm/SessionKeys"),
                jni_sig!((sessionId: java.lang.String, identityKey: io.github.fherbreteau.vodozemac.types.Curve25519PublicKey, baseKey: io.github.fherbreteau.vodozemac.types.Curve25519PublicKey, oneTimeKey: io.github.fherbreteau.vodozemac.types.Curve25519PublicKey) -> void),
                &[JValue::Object(&session_id), JValue::Object(&identity_key), JValue::Object(&base_key), JValue::Object(&one_time_key)],
            )?;
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
                .map_err(|e| throw_generic_error(env, e))?;
            let (message_type, ciphertext) = olm_message.to_parts();
            let body = env.new_string(base64_encode(ciphertext))?;
            let result = env.new_object(
                jni_str!("io/github/fherbreteau/vodozemac/olm/OlmMessage"),
                jni_sig!((messageType: int, body: java.lang.String) -> void),
                &[JValue::Int(message_type as i32), JValue::Object(&body)],
            )?;
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
            let message_str: String = message.to_string();

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
            let pickle_str: String = pickle_data.to_string();

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
            let pickle_str: String = pickle_data.to_string();
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
            let pickle_str: String = pickle_data.to_string();
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
    use vodozemac::olm::{Account, SessionConfig};

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
}
