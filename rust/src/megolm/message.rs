use jni::EnvUnowned;
use jni::objects::JClass;
use jni::objects::JString;
use jni::sys::jobject;
use vodozemac::megolm::MegolmMessage;

use super::to_java_megolm_message;
use crate::errors::throw_decode_error;
use crate::helpers::catch_panic;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_megolm_MegolmMessage_nativeFromBase64(
    mut env: EnvUnowned,
    _class: JClass,
    base64: JString,
) -> jobject {
    let outcome = env.with_env(|env| -> Result<jobject, jni::errors::Error> {
        catch_panic(env, |env| {
            let base64_str = base64.try_to_string(env)?;
            let message =
                MegolmMessage::from_base64(&base64_str).map_err(|e| throw_decode_error(env, e))?;

            let result = to_java_megolm_message(env, &message)?;
            Ok(result.into_raw())
        })
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[cfg(test)]
mod tests {
    use vodozemac::megolm::GroupSession;

    use super::*;
    use jni::Env;

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
    use crate::helpers::get_jvm;

    use super::Java_io_github_fherbreteau_vodozemac_megolm_MegolmMessage_nativeFromBase64 as native_from_base64;

    unsafe fn call<R>(env: &mut Env, f: impl FnOnce(EnvUnowned, JClass) -> R) -> R {
        let unowned = unsafe { EnvUnowned::from_raw(env.get_raw()) };
        let class = unsafe { JClass::from_raw(env, std::ptr::null_mut()) };
        f(unowned, class)
    }

    #[test]
    fn test_jni_from_base64_invalid_throws() {
        let jvm = get_jvm();
        jvm.attach_current_thread(|env| -> Result<(), jni::errors::Error> {
            unsafe {
                let invalid = env.new_string("not-a-megolm-message")?;
                let result = call(env, |unowned, class| {
                    native_from_base64(unowned, class, invalid)
                });
                assert!(
                    result.is_null(),
                    "An invalid message should produce no object"
                );
                assert!(env.exception_check(), "An invalid message should throw");
                env.exception_clear();
            }
            Ok(())
        })
        .expect("JVM test failed");
    }
}
