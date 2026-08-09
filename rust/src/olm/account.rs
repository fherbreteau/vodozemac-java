use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong, jobject, jstring};
use jni::{Env, EnvUnowned, JValue, jni_sig, jni_str};
use std::collections::HashMap;
use vodozemac::olm::{Account, AccountPickle, OlmMessage};
use vodozemac::{Curve25519PublicKey, KeyId};

use crate::errors::{throw_generic_error, throw_key_error, throw_pickle_error, throw_session_creation_error};
use crate::helpers::{olm_session_config_from_version, wrap};

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
        let identity_keys = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/account/IdentityKeys"),
            jni_sig!((ed25519: java.lang.String, curve25519: java.lang.String) -> void),
            &[JValue::Object(&ed25519), JValue::Object(&curve25519)],
        )?;
        Ok(identity_keys.into_raw())
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
    let outcome = env.with_env(|env| -> Result<i64, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };

        let max_number_of_one_time_keys = account.max_number_of_one_time_keys();
        let result = jlong::try_from(max_number_of_one_time_keys)
            .map_err(|e| throw_generic_error(env, e))?;
        Ok(result)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
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
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };
        let session_config = olm_session_config_from_version(session_version)?;
        let decoded_identity_key = Curve25519PublicKey::from_base64(&identity_key.to_string())
            .map_err(|e| throw_key_error(env, e))?;
        let decoded_one_time_key = Curve25519PublicKey::from_base64(&one_time_key.to_string())
            .map_err(|e| throw_key_error(env, e))?;

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
    session_version: jint,
    their_identity_key: JString,
    pre_key_message: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let account = unsafe { &mut *(ptr as *mut Account) };
        let session_config = olm_session_config_from_version(session_version)?;
        let their_identity_key = Curve25519PublicKey::from_base64(&their_identity_key.to_string())
            .map_err(|e| throw_key_error(env, e))?;
        let olm_message: OlmMessage = serde_json::from_str(&pre_key_message.to_string())
            .map_err(|e| throw_session_creation_error(env, e))?;

        let pre_key_message = match olm_message {
            OlmMessage::PreKey(pk) => pk,
            OlmMessage::Normal(_) => {
                return Err(throw_session_creation_error(env, "Expected a pre-key message but got a normal message"));
            }
        };
        let result = account
            .create_inbound_session(session_config, their_identity_key, &pre_key_message)
            .map_err(|e| throw_session_creation_error(env, e))?;

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
    let outcome = env.with_env(|env| -> Result<i64, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };

        let stored_one_time_key_count = account.stored_one_time_key_count();
        let result = jlong::try_from(stored_one_time_key_count)
            .map_err(|e| throw_generic_error(env, e))?;
        Ok(result)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn key_vec_to_java_list<'local>(
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

fn key_generation_to_result<'local>(
    env: &mut Env<'local>,
    result: vodozemac::olm::OneTimeKeyGenerationResult,
) -> Result<JObject<'local>, jni::errors::Error> {
    let created_list = key_vec_to_java_list(env, &result.created)?;
    let removed_list = key_vec_to_java_list(env, &result.removed)?;

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
        let count = usize::try_from(count).map_err(|e| throw_generic_error(env, e))?;

        let result = account.generate_one_time_keys(count);
        let result = key_generation_to_result(env, result)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

fn key_map_to_result<'local>(
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
        let result = key_map_to_result(env, &keys)?;
        Ok(result.into_raw())
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
        let result = key_map_to_result(env, &keys)?;
        Ok(result.into_raw())
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
        let json_string = serde_json::to_string(&pickle_data)
            .map_err(|e| throw_pickle_error(env, e))?;
        let result = env.new_string(json_string)?;
        Ok(result.into_raw())
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
        let key = wrap(env.convert_byte_array(key)?)?;
        let encrypted = pickle.encrypt(&key);
        let result = env.new_string(encrypted)?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_account_Account_nativeUnpickle(
    mut env: EnvUnowned,
    _class: JClass,
    pickle_data: JString,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let pickle_str: String = pickle_data.to_string();

        let pickle_data: AccountPickle = serde_json::from_str(&pickle_str)
            .map_err(|e| throw_pickle_error(env, e))?;
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
        let key = wrap(env.convert_byte_array(key)?)?;

        let pickle_data = AccountPickle::from_encrypted(&pickle_str, &key)
            .map_err(|e| throw_pickle_error(env, e))?;
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
        let pickle_key = env.convert_byte_array(&pickle_key)?;

        let from_olm = Account::from_libolm_pickle(&pickle_str, &pickle_key)
            .map_err(|e| throw_pickle_error(env, e))?;
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
    key: JByteArray,
) -> jlong {
    let outcome = env.with_env(|env| -> Result<jlong, jni::errors::Error> {
        let ciphertext_str: String = ciphertext.to_string();
        let nonce_str: String = nonce.to_string();
        let key = wrap(env.convert_byte_array(key)?)?;

        let dehydrated = Account::from_dehydrated_device(&ciphertext_str, &nonce_str, &key)
            .map_err(|e| throw_pickle_error(env, e))?;
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
        let key = wrap(env.convert_byte_array(key)?)?;

        let pickle_data = account
            .to_dehydrated_device(&key)
            .map_err(|e| throw_pickle_error(env, e))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{PICKLE_KEY, get_jvm};
    use vodozemac::olm::SessionConfig;

    #[test]
    fn test_pickle_key_is_32_bytes() {
        assert_eq!(PICKLE_KEY.len(), 32);
        assert!(PICKLE_KEY.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_session_config_version_1() {
        let result = olm_session_config_from_version(1);
        assert!(
            result.is_ok(),
            "Version 1 should produce a valid SessionConfig"
        );
    }

    #[test]
    fn test_session_config_version_2() {
        let result = olm_session_config_from_version(2);
        assert!(
            result.is_ok(),
            "Version 2 should produce a valid SessionConfig"
        );
    }

    #[test]
    fn test_session_config_invalid_version() {
        let result = olm_session_config_from_version(0);
        assert!(result.is_err(), "Version 0 should produce an error");

        let result = olm_session_config_from_version(3);
        assert!(result.is_err(), "Version 3 should produce an error");

        let result = olm_session_config_from_version(-1);
        assert!(result.is_err(), "Negative version should produce an error");
    }

    #[test]
    fn test_curve25519_keys_to_arraylist_empty() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let keys: Vec<Curve25519PublicKey> = vec![];
            let list = key_vec_to_java_list(env, &keys)?;
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
            let list = key_vec_to_java_list(env, &keys)?;
            let size = env
                .call_method(&list, jni_str!("size"), jni_sig!(() -> jint), &[])?
                .i()?;
            assert_eq!(size, 2, "ArrayList should contain 2 keys");
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_key_map_to_result_empty() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let keys: HashMap<KeyId, Curve25519PublicKey> = HashMap::new();
            let map = key_map_to_result(env, &keys)?;
            let size = env
                .call_method(&map, jni_str!("size"), jni_sig!(() -> jint), &[])?
                .i()?;
            assert_eq!(size, 0, "Empty HashMap should have size 0");
            Ok(())
        })
        .expect("JVM test failed");
    }

    #[test]
    fn test_key_map_to_result_with_keys() {
        let jvm = get_jvm();
        let mut account = Account::new();
        let _ = account.generate_one_time_keys(1);
        let keys = account.one_time_keys();
        assert!(!keys.is_empty(), "Should have at least one one-time key");

        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let map = key_map_to_result(env, &keys)?;
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

        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            let java_result = key_generation_to_result(env, result)?;
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
            original_curve25519
        );
        assert_eq!(restored_account.ed25519_key().to_base64(), original_ed25519);
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

        assert_eq!(restored.curve25519_key().to_base64(), original_curve25519);
        assert_eq!(restored.ed25519_key().to_base64(), original_ed25519);
    }

    #[test]
    fn test_account_sign_and_verify_keys() {
        let account = Account::new();
        let message = "Hello Vodozemac!";
        let signature = account.sign(message);
        assert!(
            !signature.to_base64().is_empty(),
            "Signature should not be empty"
        );

        let ed25519_key = account.ed25519_key();
        let public_key = vodozemac::Ed25519PublicKey::from_base64(&ed25519_key.to_base64())
            .expect("Should parse Ed25519 public key");
        assert!(public_key.verify(message.as_bytes(), &signature).is_ok());
    }

    #[test]
    fn test_max_number_of_one_time_keys() {
        let account = Account::new();
        assert!(account.max_number_of_one_time_keys() > 0);
    }

    #[test]
    fn test_stored_one_time_key_count() {
        let account = Account::new();
        assert_eq!(account.stored_one_time_key_count(), 0);
    }

    #[test]
    fn test_one_time_key_generation_and_retrieval() {
        let mut account = Account::new();
        let result = account.generate_one_time_keys(5);
        assert_eq!(result.created.len(), 5);
        assert_eq!(result.removed.len(), 0);

        let keys = account.one_time_keys();
        assert_eq!(keys.len(), 5);

        account.mark_keys_as_published();
        assert!(account.one_time_keys().is_empty());
    }

    #[test]
    fn test_fallback_key_generation_and_forgetting() {
        let mut account = Account::new();
        assert!(account.generate_fallback_key().is_none());
        assert_eq!(account.fallback_key().len(), 1);
        account.mark_keys_as_published();
        assert!(account.fallback_key().is_empty());
        assert!(!account.forget_fallback_key());
    }

    #[test]
    fn test_create_outbound_session() {
        let alice = Account::new();
        let mut bob = Account::new();
        let _ = bob.generate_one_time_keys(1);
        let bob_keys = bob.one_time_keys();
        assert!(!bob_keys.is_empty());
        let bob_identity = bob.curve25519_key();
        let bob_one_time = *bob_keys.values().next().unwrap();
        let session =
            alice.create_outbound_session(SessionConfig::version_2(), bob_identity, bob_one_time);
        let _ = session;
    }
}
