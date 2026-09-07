pub mod account;
pub mod session;

use jni::objects::JObject;
use jni::sys::jint;
use jni::{Env, JValue, jni_sig};
use vodozemac::base64_encode;
use vodozemac::olm::{IdentityKeys, OlmMessage, SessionKeys};

use crate::classes::{IDENTITY_KEYS, OLM_MESSAGE, SESSION_KEYS};
use crate::types::to_java_base64_value;

pub(crate) fn to_java_identity_keys<'local>(
    env: &mut Env<'local>,
    identity_keys: &IdentityKeys,
) -> Result<JObject<'local>, jni::errors::Error> {
    let ed25519 = to_java_base64_value(env, &identity_keys.ed25519)?;
    let curve25519 = to_java_base64_value(env, &identity_keys.curve25519)?;
    env.new_object(
        IDENTITY_KEYS,
        jni_sig!((ed25519: io.github.fherbreteau.vodozemac.types.Ed25519PublicKey, curve25519: io.github.fherbreteau.vodozemac.types.Curve25519PublicKey) -> void),
        &[JValue::Object(&ed25519), JValue::Object(&curve25519)],
    )
}

pub(crate) fn to_java_session_keys<'local>(
    env: &mut Env<'local>,
    session_keys: &SessionKeys,
) -> Result<JObject<'local>, jni::errors::Error> {
    let session_id = env.new_string(session_keys.session_id())?;
    let identity_key = to_java_base64_value(env, &session_keys.identity_key)?;
    let base_key = to_java_base64_value(env, &session_keys.base_key)?;
    let one_time_key = to_java_base64_value(env, &session_keys.one_time_key)?;
    env.new_object(
        SESSION_KEYS,
        jni_sig!((sessionId: java.lang.String, identityKey: io.github.fherbreteau.vodozemac.types.Curve25519PublicKey, baseKey: io.github.fherbreteau.vodozemac.types.Curve25519PublicKey, oneTimeKey: io.github.fherbreteau.vodozemac.types.Curve25519PublicKey) -> void),
        &[JValue::Object(&session_id), JValue::Object(&identity_key), JValue::Object(&base_key), JValue::Object(&one_time_key)],
    )
}

pub(crate) fn to_java_olm_message<'local>(
    env: &mut Env<'local>,
    message: &OlmMessage,
) -> Result<JObject<'local>, jni::errors::Error> {
    let (message_type, ciphertext) = message.to_parts();
    let body = env.new_string(base64_encode(ciphertext))?;
    env.new_object(
        OLM_MESSAGE,
        jni_sig!((messageType: int, body: java.lang.String) -> void),
        &[JValue::Int(message_type as jint), JValue::Object(&body)],
    )
}
