use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong, jobject, jstring};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use std::collections::HashMap;
use vodozemac::olm::{Account, SessionConfig};
use vodozemac::olm::{AccountPickle, PreKeyMessage};
use vodozemac::{Curve25519PublicKey, KeyId};

const PICKLE_KEY: [u8; 32] = [0u8; 32];

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
        let pre_key_message = PreKeyMessage::from_base64(&pre_key_message.to_string())
            .map_err(|_e| jni::errors::Error::JavaException)?;
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
            jni_sig!((e: java.lang.Object) -> jint),
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
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeUnpickleLegacy(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
    pickle_key: JString,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();
        let pickle_key_str: String = pickle_key.to_string();
        let from_olm = Account::from_libolm_pickle(&pickle_str, pickle_key_str.as_bytes())
            .map_err(|_e| jni::errors::Error::JavaException)?;
        let account = Box::new(from_olm);
        Ok(Box::into_raw(account) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeFromDehydratedDevice(
    mut env: EnvUnowned,
    _class: JClass,
    ciphertext: JString,
    nonce: JString,
    key: JString,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let ciphertext_str: String = ciphertext.to_string();
        let nonce_str: String = nonce.to_string();
        let _key_str: String = key.to_string();
        // TODO use the inputed key => to transform to an array of 32 u8
        let dehydrated = Account::from_dehydrated_device(&ciphertext_str, &nonce_str, &PICKLE_KEY)
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
    key: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let _key_str = key.to_string();
        // TODO use the inputed key => to transform to an array of 32 u8
        let pickle_data = account
            .to_dehydrated_device(&PICKLE_KEY)
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
