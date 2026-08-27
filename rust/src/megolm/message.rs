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

        let ciphertext = env.new_string(base64_encode(message.ciphertext()))?;
        let mac = env.new_string(base64_encode(message.mac()))?;
        let signature = env.new_string(message.signature().to_base64())?;
        let base64 = env.new_string(message.to_base64())?;

        let result = env.new_object(
            jni_str!("io/github/fherbreteau/vodozemac/megolm/MegolmMessage"),
            jni_sig!((base64: java.lang.String, ciphertext: java.lang.String, messageIndex: int, mac: java.lang.String, signature: java.lang.String) -> void),
            &[
                JValue::Object(&base64),
                JValue::Object(&ciphertext),
                JValue::Int(message.message_index() as jint),
                JValue::Object(&mac),
                JValue::Object(&signature),
            ],
        )?;
        Ok(result.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use vodozemac::megolm::GroupSession;

    use super::*;

    #[test]
    fn test_megolm_message_from_base64_roundtrip() {
        let mut outbound = GroupSession::new(vodozemac::megolm::SessionConfig::version_2());
        let plaintext = "Hello Megolm!";
        let message = outbound.encrypt(plaintext);

        let base64 = message.to_base64();
        let restored = MegolmMessage::from_base64(&base64).expect("Should decode from base64");

        assert_eq!(restored.message_index(), message.message_index());
        assert_eq!(restored.ciphertext(), message.ciphertext());
        assert_eq!(restored.mac(), message.mac());
    }

    #[test]
    fn test_megolm_message_from_base64_version_1() {
        let mut outbound = GroupSession::new(vodozemac::megolm::SessionConfig::version_1());
        let plaintext = "Version 1 message";
        let message = outbound.encrypt(plaintext);

        let base64 = message.to_base64();
        let restored = MegolmMessage::from_base64(&base64).expect("Should decode from base64");

        assert_eq!(restored.message_index(), message.message_index());
        assert_eq!(restored.ciphertext(), message.ciphertext());
    }

    #[test]
    fn test_megolm_message_message_index() {
        let mut outbound = GroupSession::new(vodozemac::megolm::SessionConfig::version_2());

        let msg0 = outbound.encrypt("first");
        assert_eq!(msg0.message_index(), 0);

        let msg1 = outbound.encrypt("second");
        assert_eq!(msg1.message_index(), 1);

        let msg2 = outbound.encrypt("third");
        assert_eq!(msg2.message_index(), 2);
    }

    #[test]
    fn test_megolm_message_ciphertext() {
        let mut outbound = GroupSession::new(vodozemac::megolm::SessionConfig::version_2());
        let message = outbound.encrypt("test ciphertext");

        assert!(!message.ciphertext().is_empty());
    }

    #[test]
    fn test_megolm_message_mac() {
        let mut outbound = GroupSession::new(vodozemac::megolm::SessionConfig::version_2());
        let message = outbound.encrypt("test mac");

        assert!(!message.mac().is_empty());
    }

    #[test]
    fn test_megolm_message_signature() {
        let mut outbound = GroupSession::new(vodozemac::megolm::SessionConfig::version_2());
        let message = outbound.encrypt("test signature");

        assert!(!message.signature().to_base64().is_empty());
    }

    #[test]
    fn test_megolm_message_to_bytes_and_from_bytes() {
        let mut outbound = GroupSession::new(vodozemac::megolm::SessionConfig::version_2());
        let message = outbound.encrypt("bytes test");

        let bytes = message.to_bytes();
        let restored = MegolmMessage::from_bytes(&bytes).expect("Should decode from bytes");

        assert_eq!(restored.message_index(), message.message_index());
        assert_eq!(restored.ciphertext(), message.ciphertext());
    }

    #[test]
    fn test_megolm_message_from_base64_invalid_input() {
        let result = MegolmMessage::from_base64("not_valid_base64!!!");
        assert!(result.is_err(), "Invalid base64 should produce an error");
    }

    #[test]
    fn test_megolm_message_encrypt_and_decrypt_via_roundtrip() {
        use vodozemac::megolm::{InboundGroupSession, SessionConfig};

        let mut outbound = GroupSession::new(SessionConfig::version_2());
        let session_key = outbound.session_key();
        let mut inbound = InboundGroupSession::new(&session_key, SessionConfig::version_2());

        let plaintext = "Roundtrip test";
        let message = outbound.encrypt(plaintext);

        let base64 = message.to_base64();
        let restored = MegolmMessage::from_base64(&base64).expect("Should decode from base64");

        let decrypted = inbound.decrypt(&restored).expect("Should decrypt");
        assert_eq!(decrypted.plaintext, plaintext.as_bytes());
    }
}
