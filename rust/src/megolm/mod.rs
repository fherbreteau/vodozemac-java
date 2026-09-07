pub mod inbound_group_session;
pub mod message;
pub mod outbound_group_session;

use jni::objects::JObject;
use jni::sys::jint;
use jni::{Env, JValue, jni_sig};
use vodozemac::base64_encode;
use vodozemac::megolm::MegolmMessage;

use crate::classes::{DECRYPTED_MESSAGE, MEGOLM_MESSAGE};
use crate::types::to_java_base64_value;

pub(crate) fn to_java_megolm_message<'local>(
    env: &mut Env<'local>,
    message: &MegolmMessage,
) -> Result<JObject<'local>, jni::errors::Error> {
    let ciphertext = env.new_string(base64_encode(message.ciphertext()))?;
    let mac = env.new_string(base64_encode(message.mac()))?;
    let signature = to_java_base64_value(env, message.signature())?;
    let base64 = env.new_string(message.to_base64())?;

    env.new_object(
        MEGOLM_MESSAGE,
        jni_sig!((base64: java.lang.String, ciphertext: java.lang.String, messageIndex: int, mac: java.lang.String, signature: io.github.fherbreteau.vodozemac.types.Ed25519Signature) -> void),
        &[
            JValue::Object(&base64),
            JValue::Object(&ciphertext),
            JValue::Int(message.message_index() as jint),
            JValue::Object(&mac),
            JValue::Object(&signature),
        ],
    )
}

pub(crate) fn to_java_decrypted_message<'local>(
    env: &mut Env<'local>,
    plaintext: &[u8],
    message_index: u32,
) -> Result<JObject<'local>, jni::errors::Error> {
    let plaintext_bytes = env.byte_array_from_slice(plaintext)?;
    env.new_object(
        DECRYPTED_MESSAGE,
        jni_sig!((plaintext: byte[], messageIndex: int) -> void),
        &[
            JValue::Object(&plaintext_bytes),
            JValue::Int(message_index as jint),
        ],
    )
}
