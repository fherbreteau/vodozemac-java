use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong, jobject, jstring};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use std::collections::HashMap;
use vodozemac::megolm::{
    GroupSession, GroupSessionPickle, InboundGroupSession, InboundGroupSessionPickle,
    MegolmMessage, SessionConfig as MegolmSessionConfig, SessionKey,
};
use vodozemac::olm::{Account, Session, SessionConfig};
use vodozemac::olm::{AccountPickle, OlmMessage, SessionPickle};
use vodozemac::{Curve25519PublicKey, KeyId};

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let account = Box::new(Account::new());
        Ok(Box::into_raw(account) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeIdentityKeys(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };

        let identity_key = account.identity_keys();
        let ed25519 = env.new_string(identity_key.ed25519.to_base64())?;
        let curve25519 = env.new_string(identity_key.curve25519.to_base64())?;

        let to_java = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/account/IdentityKeys"),
            jni_sig!((ed25519: java.lang.String, curve25519: java.lang.String) -> void),
            &[JValue::Object(&ed25519), JValue::Object(&curve25519)],
        )?;
        Ok(to_java.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeCurve25519Key(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let key = account.curve25519_key().to_base64();
        let jni_string = env.new_string(key)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeEd25519Key(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let key = account.ed25519_key().to_base64();
        let jni_string = env.new_string(key)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeMaxNumberOfOneTimeKeys(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<i64, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let max_number_of_one_time_keys = account.max_number_of_one_time_keys();
        let result = jlong::try_from(max_number_of_one_time_keys)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        Ok(result)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn session_config_from_version(version: jint) -> Result<SessionConfig, jni::errors::Error> {
    match version {
        1 => Ok(SessionConfig::version_1()),
        2 => Ok(SessionConfig::version_2()),
        _ => Err(jni::errors::Error::JavaException),
    }
}

fn megolm_session_config_from_version(
    version: jint,
) -> Result<MegolmSessionConfig, jni::errors::Error> {
    match version {
        1 => Ok(MegolmSessionConfig::version_1()),
        2 => Ok(MegolmSessionConfig::version_2()),
        _ => Err(jni::errors::Error::JavaException),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeCreateOutboundSession(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    session_version: jint,
    identity_key: JString,
    one_time_key: JString,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };

        let session_config = session_config_from_version(session_version)?;

        let decoded_identity_key = Curve25519PublicKey::from_base64(&identity_key.to_string())
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let decoded_one_time_key = Curve25519PublicKey::from_base64(&one_time_key.to_string())
            .map_err(|_e| jni::errors::Error::JavaException)?;

        let session = account.create_outbound_session(
            session_config,
            decoded_identity_key,
            decoded_one_time_key,
        );
        Ok(Box::into_raw(Box::new(session)) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeCreateInboundSession(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    their_identity_key: JString,
    pre_key_message: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };
        let their_identity_key = Curve25519PublicKey::from_base64(&their_identity_key.to_string())
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let olm_message: OlmMessage = serde_json::from_str(&pre_key_message.to_string())
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let pre_key_message = match olm_message {
            OlmMessage::PreKey(pk) => pk,
            OlmMessage::Normal(_) => {
                return Err(jni::errors::Error::JavaException);
            }
        };
        let result = account
            .create_inbound_session(their_identity_key, &pre_key_message)
            .map_err(|_e| jni::errors::Error::JavaException)?;

        let session_ptr = Box::into_raw(Box::new(result.session)) as jlong;
        let plaintext_bytes = env.byte_array_from_slice(&result.plaintext)?;

        let result = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/olm/InboundCreationResult"),
            jni_sig!((sessionPtr: long, plaintext: byte[]) -> void),
            &[JValue::Long(session_ptr), JValue::Object(&plaintext_bytes)],
        )?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeStoredOneTimeKeyCount(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<i64, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let stored_one_time_key_count = account.stored_one_time_key_count();
        let result = jlong::try_from(stored_one_time_key_count)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        Ok(result)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn curve25519_keys_to_arraylist<'local>(
    env: &mut Env<'local>,
    keys: &[Curve25519PublicKey],
) -> Result<JObject<'local>, jni::errors::Error> {
    let array_list = env.new_object(jni_str!("java/util/ArrayList"), jni_sig!(() -> void), &[])?;

    for key in keys {
        let key_str = env.new_string(key.to_base64())?;
        env.call_method(
            &array_list,
            jni_str!("add"),
            jni_sig!((e: java.lang.Object) -> jboolean),
            &[JValue::Object(&key_str)],
        )?;
    }

    Ok(array_list)
}

fn one_time_key_result_to_java<'local>(
    env: &mut Env<'local>,
    result: vodozemac::olm::OneTimeKeyGenerationResult,
) -> Result<JObject<'local>, jni::errors::Error> {
    let created_list = curve25519_keys_to_arraylist(env, &result.created)?;
    let removed_list = curve25519_keys_to_arraylist(env, &result.removed)?;

    env.new_object(
        jni_str!("io/github/fherbreteau/vodozemac/account/OneTimeKeyGenerationResult"),
        jni_sig!((created: java.util.List, removed: java.util.List) -> void),
        &[JValue::Object(&created_list), JValue::Object(&removed_list)],
    )
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeGenerateOneTimeKeys<
    'local,
>(
    mut env: EnvUnowned<'local>,
    _class: JClass,
    ptr: jlong,
    count: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };
        let count = usize::try_from(count).map_err(|_e| jni::errors::Error::JavaException)?;

        let result = account.generate_one_time_keys(count);

        let java_result = one_time_key_result_to_java(env, result)?;

        Ok(java_result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn key_map_to_java_map<'local>(
    env: &mut Env<'local>,
    keys: &HashMap<KeyId, Curve25519PublicKey>,
) -> Result<JObject<'local>, jni::errors::Error> {
    let hash_map = env.new_object(jni_str!("java/util/HashMap"), jni_sig!(() -> void), &[])?;

    for (key_id, public_key) in keys {
        let key_str = env.new_string(key_id.to_base64())?;
        let value_str = env.new_string(public_key.to_base64())?;

        env.call_method(
            &hash_map,
            jni_str!("put"),
            jni_sig!((k:java.lang.Object, v:java.lang.Object) -> java.lang.Object),
            &[JValue::Object(&key_str), JValue::Object(&value_str)],
        )?;
    }

    Ok(hash_map)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeOneTimeKeys(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let keys = account.one_time_keys();

        let java_map = key_map_to_java_map(env, &keys)?;
        Ok(java_map.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeGenerateFallbackKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };

        let result = account.generate_fallback_key();

        match result {
            Some(key) => {
                let key_str = env.new_string(key.to_base64())?;
                Ok(key_str.into_raw() as jobject)
            }
            None => Ok(std::ptr::null_mut()),
        }
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeFallbackKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let keys = account.fallback_key();

        let java_map = key_map_to_java_map(env, &keys)?;
        Ok(java_map.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeForgetFallbackKey(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    let outcome = env.with_env(|_env| -> Result<jboolean, jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };
        let forgot = account.forget_fallback_key();
        Ok(forgot)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeMarkKeysAsPublished(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    let outcome = env.with_env(|_env| -> Result<(), jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };
        account.mark_keys_as_published();
        Ok(())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeSign(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    message: JString,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let msg: String = message.to_string();
        let signature = account.sign(&msg).to_base64();
        let jni_string = env.new_string(signature)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativePickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let pickle_data = account.pickle();
        let json_string =
            serde_json::to_string(&pickle_data).map_err(|_e| jni::errors::Error::JavaException)?;
        let jni_string = env.new_string(json_string)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeEncryptedPickle(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    key: JByteArray,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let pickle = account.pickle();
        let key = wrap(env.convert_byte_array(key).unwrap());
        let encrypted = pickle.encrypt(&key);
        let jni_string = env.new_string(encrypted)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let pickle_data: AccountPickle =
            serde_json::from_str(&pickle_str).map_err(|_e| jni::errors::Error::JavaException)?;
        let account = Box::new(Account::from_pickle(pickle_data));
        Ok(Box::into_raw(account) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeEncryptedUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let key = wrap(env.convert_byte_array(key).unwrap());
        let pickle_data = AccountPickle::from_encrypted(&pickle_str, &key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let account = Box::new(Account::from_pickle(pickle_data));
        Ok(Box::into_raw(account) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeUnpickleLegacy(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    pickle_key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let pickle_key = env.convert_byte_array(&pickle_key).unwrap();
        let from_olm = Account::from_libolm_pickle(&pickle_str, &pickle_key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let account = Box::new(from_olm);
        Ok(Box::into_raw(account) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn wrap<T>(v: Vec<T>) -> [T; 32] {
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!("Expected a Vec of length {} but it was {}", 32, v.len())
    })
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeFromDehydratedDevice(
    mut env: EnvUnowned,
    _class: JClass,
    ciphertext: JString,
    nonce: JString,
    key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let ciphertext_str: String = ciphertext.to_string();
        let nonce_str: String = nonce.to_string();
        let key = wrap(env.convert_byte_array(key).unwrap());
        let dehydrated = Account::from_dehydrated_device(&ciphertext_str, &nonce_str, &key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let account = Box::new(dehydrated);
        Ok(Box::into_raw(account) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeToDehydratedDevice(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    key: JByteArray,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let key = wrap(env.convert_byte_array(key).unwrap());
        let pickle_data = account
            .to_dehydrated_device(&key)
            .map_err(|_e| jni::errors::Error::JavaException)?;

        let ciphertext = env.new_string(pickle_data.ciphertext)?;
        let nonce = env.new_string(pickle_data.nonce)?;

        let dehydrated_device = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/account/DehydratedDeviceResult"),
            jni_sig!((ciphertext: java.lang.String, nonce: java.lang.String) -> void),
            &[JValue::Object(&ciphertext), JValue::Object(&nonce)],
        )?;

        Ok(dehydrated_device.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut Account);
    }
}

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
        let jni_string = env.new_string(session_key)?;
        Ok(jni_string.into_raw())
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
        let message = session.encrypt(&plaintext_bytes);
        let base64_string = message.to_base64();
        let jni_string = env.new_string(base64_string)?;
        Ok(jni_string.into_raw())
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
            serde_json::to_string(&pickle_data).map_err(|_e| jni::errors::Error::JavaException)?;
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
        let pickle = session.pickle();
        let key = wrap(env.convert_byte_array(key).unwrap());
        let encrypted = pickle.encrypt(&key);
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
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let pickle_data: GroupSessionPickle =
            serde_json::from_str(&pickle_str).map_err(|_e| jni::errors::Error::JavaException)?;
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
        let key = wrap(env.convert_byte_array(key).unwrap());
        let pickle_data = GroupSessionPickle::from_encrypted(&pickle_str, &key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
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
            .map_err(|_e| jni::errors::Error::JavaException)?;
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

// ============================================================================
// Megolm: InboundGroupSession (wraps vodozemac::megolm::InboundGroupSession)
// ============================================================================

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
    session_key: JString,
    version: jint,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let config = megolm_session_config_from_version(version)?;
        let session_key = SessionKey::from_base64(&session_key.to_string())
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let session = Box::new(InboundGroupSession::new(&session_key, config));
        Ok(Box::into_raw(session) as jlong)
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
        let session = unsafe { &*(ptr as *const InboundGroupSession) };
        let session_id = session.session_id();
        let jni_string = env.new_string(session_id)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFirstKnownIndex(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let outcome = env.with_env(|_env| -> Result<jint, jni::errors::Error> {
        let session = unsafe { &*(ptr as *const InboundGroupSession) };
        Ok(session.first_known_index() as jint)
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
        let session = unsafe { &mut *(ptr as *mut InboundGroupSession) };
        let message_str: String = message.to_string();
        let megolm_message = MegolmMessage::from_base64(&message_str)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let decrypted = session
            .decrypt(&megolm_message)
            .map_err(|_e| jni::errors::Error::JavaException)?;

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
        let session = unsafe { &*(ptr as *const InboundGroupSession) };
        let pickle_data = session.pickle();
        let json_string =
            serde_json::to_string(&pickle_data).map_err(|_e| jni::errors::Error::JavaException)?;
        let jni_string = env.new_string(json_string)?;
        Ok(jni_string.into_raw())
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
        let session = unsafe { &*(ptr as *const InboundGroupSession) };
        let pickle = session.pickle();
        let key = wrap(env.convert_byte_array(key).unwrap());
        let encrypted = pickle.encrypt(&key);
        let jni_string = env.new_string(encrypted)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let pickle_data: InboundGroupSessionPickle =
            serde_json::from_str(&pickle_str).map_err(|_e| jni::errors::Error::JavaException)?;
        let session = Box::new(InboundGroupSession::from_pickle(pickle_data));
        Ok(Box::into_raw(session) as jlong)
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
        let pickle_str: String = pickle_data.to_string();
        let key = wrap(env.convert_byte_array(key).unwrap());
        let pickle_data = InboundGroupSessionPickle::from_encrypted(&pickle_str, &key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let session = Box::new(InboundGroupSession::from_pickle(pickle_data));
        Ok(Box::into_raw(session) as jlong)
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
        let pickle_str: String = pickle_data.to_string();
        let pickle_key = env.convert_byte_array(&pickle_key)?;
        let session = InboundGroupSession::from_libolm_pickle(&pickle_str, &pickle_key)
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let session = Box::new(session);
        Ok(Box::into_raw(session) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_InboundGroupSession_nativeFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut InboundGroupSession);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PICKLE_KEY: [u8; 32] = [0u8; 32];

    #[test]
    fn test_pickle_key_is_32_bytes() {
        assert_eq!(PICKLE_KEY.len(), 32);
        assert!(PICKLE_KEY.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_session_config_version_1() {
        let result = session_config_from_version(1);
        assert!(
            result.is_ok(),
            "Version 1 should produce a valid SessionConfig"
        );
    }

    #[test]
    fn test_session_config_version_2() {
        let result = session_config_from_version(2);
        assert!(
            result.is_ok(),
            "Version 2 should produce a valid SessionConfig"
        );
    }

    #[test]
    fn test_session_config_invalid_version() {
        let result = session_config_from_version(0);
        assert!(result.is_err(), "Version 0 should produce an error");

        let result = session_config_from_version(3);
        assert!(result.is_err(), "Version 3 should produce an error");

        let result = session_config_from_version(-1);
        assert!(result.is_err(), "Negative version should produce an error");
    }

    fn get_jvm() -> jni::JavaVM {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let classpath = std::env::current_dir()
                .expect("Failed to get current dir")
                .parent()
                .expect("Failed to get project root")
                .join("target/classes");
            let args = jni::InitArgsBuilder::new()
                .option(format!("-Djava.class.path={}", classpath.to_string_lossy()))
                .build()
                .expect("Failed to create JVM init args");
            let _ = jni::JavaVM::new(args).expect("Failed to create JVM");
        });
        jni::JavaVM::singleton().expect("JVM should be initialized")
    }

    #[test]
    fn test_curve25519_keys_to_arraylist_empty() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let keys: Vec<Curve25519PublicKey> = vec![];
            let list = curve25519_keys_to_arraylist(env, &keys)?;

            let size = env
                .call_method(&list, jni_str!("size"), jni_sig!(() -> jint), &[])?
                .i()?;
            assert_eq!(size, 0, "Empty ArrayList should have size 0");
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_curve25519_keys_to_arraylist_with_keys() {
        let jvm = get_jvm();
        let mut account = Account::new();
        let result = account.generate_one_time_keys(2);
        let keys: Vec<Curve25519PublicKey> = result.created;
        assert_eq!(keys.len(), 2, "Should have generated 2 keys");

        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let list = curve25519_keys_to_arraylist(env, &keys)?;

            let size = env
                .call_method(&list, jni_str!("size"), jni_sig!(() -> jint), &[])?
                .i()?;
            assert_eq!(size, 2, "ArrayList should contain 2 keys");
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_key_map_to_java_map_empty() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let keys: HashMap<KeyId, Curve25519PublicKey> = HashMap::new();
            let map = key_map_to_java_map(env, &keys)?;

            let size = env
                .call_method(&map, jni_str!("size"), jni_sig!(() -> jint), &[])?
                .i()?;
            assert_eq!(size, 0, "Empty HashMap should have size 0");
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_key_map_to_java_map_with_keys() {
        let jvm = get_jvm();
        let mut account = Account::new();
        let _ = account.generate_one_time_keys(1);
        let keys = account.one_time_keys();
        assert!(!keys.is_empty(), "Should have at least one one-time key");

        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let map = key_map_to_java_map(env, &keys)?;

            let size = env
                .call_method(&map, jni_str!("size"), jni_sig!(() -> jint), &[])?
                .i()?;
            assert_eq!(
                size as usize,
                keys.len(),
                "HashMap size should match key count"
            );
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_one_time_key_result_to_java() {
        let jvm = get_jvm();
        let mut account = Account::new();
        let result = account.generate_one_time_keys(3);
        assert_eq!(result.created.len(), 3, "Should have generated 3 keys");
        assert_eq!(
            result.removed.len(),
            0,
            "No keys should be removed on first generation"
        );

        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let java_result = one_time_key_result_to_java(env, result)?;
            assert!(
                !java_result.is_null(),
                "Java result object should not be null"
            );
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_generate_fallback_key_returns_none_first_call() {
        let mut account = Account::new();
        let result = account.generate_fallback_key();
        assert!(
            result.is_none(),
            "First fallback key generation should return None"
        );
    }

    #[test]
    fn test_account_pickling_roundtrip() {
        let account = Account::new();
        let original_curve25519 = account.curve25519_key().to_base64();
        let original_ed25519 = account.ed25519_key().to_base64();

        let pickle = account.pickle();
        let json = serde_json::to_string(&pickle).expect("Should serialize pickle to JSON");
        let restored: AccountPickle =
            serde_json::from_str(&json).expect("Should deserialize pickle from JSON");
        let restored_account = Account::from_pickle(restored);

        assert_eq!(
            restored_account.curve25519_key().to_base64(),
            original_curve25519,
            "Curve25519 key should survive pickling roundtrip"
        );
        assert_eq!(
            restored_account.ed25519_key().to_base64(),
            original_ed25519,
            "Ed25519 key should survive pickling roundtrip"
        );
    }

    #[test]
    fn test_dehydrated_device_roundtrip() {
        let account = Account::new();
        let original_curve25519 = account.curve25519_key().to_base64();
        let original_ed25519 = account.ed25519_key().to_base64();

        let dehydrated = account
            .to_dehydrated_device(&PICKLE_KEY)
            .expect("Should create dehydrated device");

        let restored =
            Account::from_dehydrated_device(&dehydrated.ciphertext, &dehydrated.nonce, &PICKLE_KEY)
                .expect("Should restore from dehydrated device");

        assert_eq!(
            restored.curve25519_key().to_base64(),
            original_curve25519,
            "Curve25519 key should survive dehydrated device roundtrip"
        );
        assert_eq!(
            restored.ed25519_key().to_base64(),
            original_ed25519,
            "Ed25519 key should survive dehydrated device roundtrip"
        );
    }

    #[test]
    fn test_account_sign_and_verify_keys() {
        let account = Account::new();
        let message = "Hello Vodozemac!";
        let signature = account.sign(message);
        let signature_b64 = signature.to_base64();
        assert!(!signature_b64.is_empty(), "Signature should not be empty");

        let ed25519_key = account.ed25519_key();
        let public_key = vodozemac::Ed25519PublicKey::from_base64(&ed25519_key.to_base64())
            .expect("Should parse Ed25519 public key");
        assert!(
            public_key.verify(message.as_bytes(), &signature).is_ok(),
            "Signature should verify against the message"
        );
    }

    #[test]
    fn test_max_number_of_one_time_keys() {
        let account = Account::new();
        let max = account.max_number_of_one_time_keys();
        assert!(max > 0, "Max one-time keys should be positive");
    }

    #[test]
    fn test_stored_one_time_key_count() {
        let account = Account::new();
        assert_eq!(
            account.stored_one_time_key_count(),
            0,
            "New account should have 0 stored one-time keys"
        );
    }

    #[test]
    fn test_one_time_key_generation_and_retrieval() {
        let mut account = Account::new();
        let result = account.generate_one_time_keys(5);
        assert_eq!(result.created.len(), 5, "Should generate 5 keys");
        assert_eq!(result.removed.len(), 0, "No keys should be removed");

        let keys = account.one_time_keys();
        assert_eq!(keys.len(), 5, "Should have 5 unpublished one-time keys");

        account.mark_keys_as_published();
        let keys = account.one_time_keys();
        assert!(
            keys.is_empty(),
            "No keys should remain unpublished after mark_keys_as_published"
        );
    }

    #[test]
    fn test_fallback_key_generation_and_forgetting() {
        let mut account = Account::new();

        let result = account.generate_fallback_key();
        assert!(result.is_none(), "First fallback key should return None");

        let keys = account.fallback_key();
        assert_eq!(keys.len(), 1, "Should have 1 unpublished fallback key");

        account.mark_keys_as_published();
        let keys = account.fallback_key();
        assert!(
            keys.is_empty(),
            "Fallback key should be published after mark_keys_as_published"
        );

        let forgot = account.forget_fallback_key();
        assert!(
            !forgot,
            "forget_fallback_key should return false when no previously used key exists"
        );
    }

    #[test]
    fn test_create_outbound_session() {
        let alice = Account::new();
        let mut bob = Account::new();

        let _ = bob.generate_one_time_keys(1);
        let bob_keys = bob.one_time_keys();
        assert!(!bob_keys.is_empty(), "Bob should have one-time keys");

        let bob_identity = bob.curve25519_key();
        let bob_one_time = *bob_keys.values().next().unwrap();

        let session =
            alice.create_outbound_session(SessionConfig::version_2(), bob_identity, bob_one_time);
        let _ = session;
    }

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
        let result = megolm_session_config_from_version(0);
        assert!(result.is_err(), "Megolm version 0 should produce an error");

        let result = megolm_session_config_from_version(3);
        assert!(result.is_err(), "Megolm version 3 should produce an error");
    }

    #[test]
    fn test_megolm_group_session_encrypt_and_decrypt() {
        let mut outbound = GroupSession::new(MegolmSessionConfig::version_2());

        let session_id = outbound.session_id();
        assert!(!session_id.is_empty(), "Session ID should not be empty");

        assert_eq!(
            outbound.message_index(),
            0,
            "Initial message index should be 0"
        );

        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());

        assert_eq!(
            inbound.session_id(),
            session_id,
            "Inbound and outbound session IDs should match"
        );

        assert_eq!(
            inbound.first_known_index(),
            0,
            "First known index should be 0"
        );

        let plaintext = "Hello Megolm!";
        let message = outbound.encrypt(plaintext);

        assert_eq!(
            outbound.message_index(),
            1,
            "Message index should increment after encrypt"
        );

        let decrypted = inbound
            .decrypt(&message)
            .expect("Should decrypt Megolm message");

        assert_eq!(
            decrypted.plaintext,
            plaintext.as_bytes(),
            "Decrypted plaintext should match original"
        );
        assert_eq!(
            decrypted.message_index, 0,
            "Message index in decrypted message should be 0"
        );
    }

    #[test]
    fn test_megolm_group_session_pickle_roundtrip() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_1());
        let original_session_id = outbound.session_id();

        let pickle = outbound.pickle();
        let json = serde_json::to_string(&pickle).expect("Should serialize to JSON");
        let restored: GroupSessionPickle =
            serde_json::from_str(&json).expect("Should deserialize from JSON");
        let restored_session = GroupSession::from_pickle(restored);

        assert_eq!(
            restored_session.session_id(),
            original_session_id,
            "Session ID should survive pickle roundtrip"
        );
    }

    #[test]
    fn test_megolm_group_session_encrypted_pickle_roundtrip() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let original_session_id = outbound.session_id();

        let pickle = outbound.pickle();
        let encrypted = pickle.encrypt(&PICKLE_KEY);
        let restored = GroupSessionPickle::from_encrypted(&encrypted, &PICKLE_KEY)
            .expect("Should decrypt pickle");
        let restored_session = GroupSession::from_pickle(restored);

        assert_eq!(
            restored_session.session_id(),
            original_session_id,
            "Session ID should survive encrypted pickle roundtrip"
        );
    }

    #[test]
    fn test_megolm_inbound_group_session_pickle_roundtrip() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_1());
        let session_key = outbound.session_key();
        let inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_1());
        let original_session_id = inbound.session_id();

        let pickle = inbound.pickle();
        let json = serde_json::to_string(&pickle).expect("Should serialize to JSON");
        let restored: InboundGroupSessionPickle =
            serde_json::from_str(&json).expect("Should deserialize from JSON");
        let restored_inbound = InboundGroupSession::from_pickle(restored);

        assert_eq!(
            restored_inbound.session_id(),
            original_session_id,
            "Session ID should survive pickle roundtrip"
        );
    }

    #[test]
    fn test_megolm_inbound_group_session_encrypted_pickle_roundtrip() {
        let outbound = GroupSession::new(MegolmSessionConfig::version_2());
        let session_key = outbound.session_key();
        let inbound = InboundGroupSession::new(&session_key, MegolmSessionConfig::version_2());
        let original_session_id = inbound.session_id();

        let pickle = inbound.pickle();
        let encrypted = pickle.encrypt(&PICKLE_KEY);
        let restored = InboundGroupSessionPickle::from_encrypted(&encrypted, &PICKLE_KEY)
            .expect("Should decrypt pickle");
        let restored_inbound = InboundGroupSession::from_pickle(restored);

        assert_eq!(
            restored_inbound.session_id(),
            original_session_id,
            "Session ID should survive encrypted pickle roundtrip"
        );
    }
}
