use jni::EnvUnowned;
use jni::objects::JClass;
use jni::objects::JString;
use jni::sys::{jint, jobject};
use jni::{JValue, jni_sig, jni_str};
use vodozemac::base64_encode;
use vodozemac::megolm::MegolmMessage;

use crate::errors::throw_decode_error;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_MegolmMessage_nativeFromBase64(
    mut env: EnvUnowned,
    _class: JClass,
    base64: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        let base64_str: String = base64.to_string();
        let message =
            MegolmMessage::from_base64(&base64_str).map_err(|e| throw_decode_error(env, e))?;

        let wire_base64 = message.to_base64();
        let ciphertext = env.new_string(base64_encode(message.ciphertext()))?;
        let mac = env.new_string(base64_encode(message.mac()))?;
        let signature = env.new_string(message.signature().to_base64())?;
        let wire_base64_str = env.new_string(wire_base64)?;

        let result = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/megolm/MegolmMessage"),
            jni_sig!((base64: java.lang.String, ciphertext: java.lang.String, messageIndex: int, mac: java.lang.String, signature: java.lang.String) -> void),
            &[
                JValue::Object(&wire_base64_str.into()),
                JValue::Object(&ciphertext.into()),
                JValue::Int(message.message_index() as jint),
                JValue::Object(&mac.into()),
                JValue::Object(&signature.into()),
            ],
        )?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
