pub mod inbound_group_session;
pub mod message;
pub mod outbound_group_session;

use jni::objects::JObject;
use jni::sys::jlong;
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
        jni_sig!((base64: java.lang.String, ciphertext: java.lang.String, messageIndex: long, mac: java.lang.String, signature: io.github.fherbreteau.vodozemac.types.Ed25519Signature) -> void),
        &[
            JValue::Object(&base64),
            JValue::Object(&ciphertext),
            JValue::Long(message.message_index() as jlong),
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
        jni_sig!((plaintext: byte[], messageIndex: long) -> void),
        &[
            JValue::Object(&plaintext_bytes),
            JValue::Long(message_index as jlong),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::get_jvm;
    use jni::{jni_sig, jni_str};

    fn encode_varint(mut value: u32) -> Vec<u8> {
        let mut encoded = Vec::new();
        loop {
            if value < 0x80 {
                encoded.push(value as u8);
                break;
            }
            encoded.push((value as u8 & 0x7f) | 0x80);
            value >>= 7;
        }
        encoded
    }

    fn megolm_message_with_index(message_index: u32) -> MegolmMessage {
        let mut bytes = vec![3, 0x08];
        bytes.extend(encode_varint(message_index));
        bytes.extend([0x12, 1, 0x41]);
        bytes.extend([0; 8 + 64]);
        MegolmMessage::from_bytes(&bytes).expect("Synthetic Megolm message should parse")
    }

    #[test]
    fn test_megolm_message_and_decrypted_message_indices_preserve_unsigned_range() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            for message_index in [i32::MAX as u32 + 1, u32::MAX] {
                let message = megolm_message_with_index(message_index);
                let java_message = to_java_megolm_message(env, &message)?;
                let java_message_index = env
                    .call_method(
                        &java_message,
                        jni_str!("messageIndex"),
                        jni_sig!(() -> long),
                        &[],
                    )?
                    .j()?;
                assert_eq!(java_message_index, message_index as jlong);

                let java_decrypted_message =
                    to_java_decrypted_message(env, b"test", message_index)?;
                let java_decrypted_index = env
                    .call_method(
                        &java_decrypted_message,
                        jni_str!("messageIndex"),
                        jni_sig!(() -> long),
                        &[],
                    )?
                    .j()?;
                assert_eq!(java_decrypted_index, message_index as jlong);
            }
            Ok(())
        })
        .expect("JVM-attached JNI value test failed");
    }
}
