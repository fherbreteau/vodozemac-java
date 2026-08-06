use jni::EnvUnowned;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jboolean, jlong, jobject, jstring};
use vodozemac::olm::{OlmMessage, Session, SessionPickle};

use crate::helpers::wrap;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut Session);
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeSessionId(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let session = unsafe { &*(ptr as *const Session) };
        let session_id = session.session_id();
        let jni_string = env.new_string(session_id)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeHasReceivedMessage(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    let outcome = env.with_env(|_env| -> Result<jboolean, jni::errors::Error> {
        let session = unsafe { &*(ptr as *const Session) };
        Ok(session.has_received_message())
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
        let session = unsafe { &mut *(ptr as *mut Session) };
        let plaintext_bytes = env.convert_byte_array(&plaintext)?;
        let olm_message = session.encrypt(&plaintext_bytes);
        let json_string =
            serde_json::to_string(&olm_message).map_err(|_e| jni::errors::Error::JavaException)?;
        let jni_string = env.new_string(json_string)?;
        Ok(jni_string.into_raw())
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
        let session = unsafe { &mut *(ptr as *mut Session) };
        let message_str: String = message.to_string();
        let olm_message: OlmMessage =
            serde_json::from_str(&message_str).map_err(|_e| jni::errors::Error::JavaException)?;
        let plaintext = session
            .decrypt(&olm_message)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let plaintext_bytes = env.byte_array_from_slice(&plaintext)?;
        Ok(plaintext_bytes.into_raw())
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
        let session = unsafe { &*(ptr as *const Session) };
        let pickle_data = session.pickle();
        let json_string =
            serde_json::to_string(&pickle_data).map_err(|_e| jni::errors::Error::JavaException)?;
        let jni_string = env.new_string(json_string)?;
        Ok(jni_string.into_raw())
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
        let session = unsafe { &*(ptr as *const Session) };
        let key = wrap(env.convert_byte_array(key).unwrap());
        let pickle_data = session.pickle();
        let encrypted = pickle_data.encrypt(&key);
        let jni_string = env.new_string(encrypted)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_olm_OlmSession_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let pickle_data: SessionPickle =
            serde_json::from_str(&pickle_str).map_err(|_e| jni::errors::Error::JavaException)?;
        let session = Box::new(Session::from_pickle(pickle_data));
        Ok(Box::into_raw(session) as jlong)
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
        let pickle_str: String = pickle_data.to_string();
        let key = wrap(env.convert_byte_array(key).unwrap());
        let pickle_data: SessionPickle = SessionPickle::from_encrypted(&pickle_str, &key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let session = Box::new(Session::from_pickle(pickle_data));
        Ok(Box::into_raw(session) as jlong)
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
        let pickle_str: String = pickle_data.to_string();
        let pickle_key = env.convert_byte_array(&pickle_key)?;
        let session = Session::from_libolm_pickle(&pickle_str, &pickle_key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let session = Box::new(session);
        Ok(Box::into_raw(session) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::PICKLE_KEY;
    use vodozemac::olm::{Account, SessionConfig};

    #[test]
    fn test_session_pickle_roundtrip() {
        let alice = Account::new();
        let mut bob = Account::new();
        let _ = bob.generate_one_time_keys(1);
        let bob_keys = bob.one_time_keys();
        let bob_identity = bob.curve25519_key();
        let bob_one_time = *bob_keys.values().next().unwrap();
        let session =
            alice.create_outbound_session(SessionConfig::version_2(), bob_identity, bob_one_time);

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

        let mut alice_session =
            alice.create_outbound_session(SessionConfig::version_1(), bob_identity, bob_one_time);
        let plaintext = b"Hello from test";
        let olm_message = alice_session.encrypt(plaintext);

        assert!(!alice_session.has_received_message());
        let pre_key_message = match olm_message {
            OlmMessage::PreKey(pk) => pk,
            OlmMessage::Normal(_) => panic!("First message should be pre-key"),
        };
        let decrypted = bob
            .create_inbound_session(alice.curve25519_key(), &pre_key_message)
            .expect("Should create inbound session");
        assert_eq!(decrypted.plaintext, plaintext);
    }
}
