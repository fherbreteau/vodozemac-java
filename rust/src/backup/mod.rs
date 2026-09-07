pub mod decryption;
pub mod encryption;

use jni::objects::JObject;
use jni::{Env, JValue, jni_sig};
use vodozemac::base64_encode;
use vodozemac::pk_encryption::Message as EncryptedMessage;

use crate::classes::PK_MESSAGE;

pub(crate) fn to_java_pk_message<'local>(
    env: &mut Env<'local>,
    message: &EncryptedMessage,
) -> Result<JObject<'local>, jni::errors::Error> {
    let ciphertext = env.new_string(base64_encode(&message.ciphertext))?;
    let mac = env.new_string(base64_encode(&message.mac))?;
    let ephemeral_key = env.new_string(message.ephemeral_key.to_base64())?;
    env.new_object(
        PK_MESSAGE,
        jni_sig!((ciphertext: java.lang.String, mac: java.lang.String, ephemeralKey: java.lang.String) -> void),
        &[
            JValue::Object(&ciphertext),
            JValue::Object(&mac),
            JValue::Object(&ephemeral_key),
        ],
    )
}
