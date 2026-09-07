use jni::Env;
use jni::jni_str;
use jni::strings::{JNIStr, JNIString};
use vodozemac::sas::InvalidCount;
use vodozemac::sas::SasError;

const PICKLE_EXCEPTION: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/exception/PickleException");
const DECRYPTION_EXCEPTION: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/exception/DecryptionException");
const ENCRYPTION_EXCEPTION: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/exception/EncryptionException");
const SESSION_CREATION_EXCEPTION: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/exception/SessionCreationException");
const KEY_EXCEPTION: &JNIStr = jni_str!("io/github/fherbreteau/vodozemac/exception/KeyException");
const SIGNATURE_EXCEPTION: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/exception/SignatureException");
const ECIES_EXCEPTION: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/exception/EciesException");
const CONVERSION_EXCEPTION: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/exception/ConversionException");
const SAS_EXCEPTION: &JNIStr = jni_str!("io/github/fherbreteau/vodozemac/exception/SasException");

fn throw(env: &mut Env, class: &JNIStr, message: &str) -> jni::errors::Error {
    let _ = env.throw_new(class, JNIString::from(message));
    jni::errors::Error::JavaException
}

macro_rules! throw_typed {
    ($name:ident, $class:expr) => {
        pub(crate) fn $name<E: std::fmt::Display>(env: &mut Env, error: E) -> jni::errors::Error {
            throw(env, $class, &error.to_string())
        }
    };
}

throw_typed!(throw_pickle_error, PICKLE_EXCEPTION);
throw_typed!(throw_decryption_error, DECRYPTION_EXCEPTION);
throw_typed!(throw_encryption_error, ENCRYPTION_EXCEPTION);
throw_typed!(throw_session_creation_error, SESSION_CREATION_EXCEPTION);
throw_typed!(throw_key_error, KEY_EXCEPTION);
throw_typed!(throw_signature_error, SIGNATURE_EXCEPTION);
throw_typed!(throw_ecies_error, ECIES_EXCEPTION);
throw_typed!(throw_conversion_error, CONVERSION_EXCEPTION);

pub(crate) fn throw_sas_error(env: &mut Env, error: SasError) -> jni::errors::Error {
    throw(env, SAS_EXCEPTION, &error.to_string())
}

pub(crate) fn throw_invalid_count_error(env: &mut Env, error: InvalidCount) -> jni::errors::Error {
    throw(env, SAS_EXCEPTION, &error.to_string())
}

trait SignatureSplitError {
    fn signature_error(&self) -> Option<&vodozemac::SignatureError>;
}

impl SignatureSplitError for vodozemac::megolm::DecryptionError {
    fn signature_error(&self) -> Option<&vodozemac::SignatureError> {
        match self {
            Self::Signature(e) => Some(e),
            _ => None,
        }
    }
}

impl SignatureSplitError for vodozemac::megolm::SessionKeyDecodeError {
    fn signature_error(&self) -> Option<&vodozemac::SignatureError> {
        match self {
            Self::Signature(e) => Some(e),
            _ => None,
        }
    }
}

impl SignatureSplitError for vodozemac::DecodeError {
    fn signature_error(&self) -> Option<&vodozemac::SignatureError> {
        match self {
            Self::Signature(e) => Some(e),
            _ => None,
        }
    }
}

fn throw_with_signature<E: SignatureSplitError>(
    env: &mut Env,
    error: &E,
    fallback: impl FnOnce(&mut Env, &E) -> jni::errors::Error,
) -> jni::errors::Error {
    match error.signature_error() {
        Some(e) => throw_signature_error(env, e),
        None => fallback(env, error),
    }
}

pub(crate) fn throw_megolm_decryption_error(
    env: &mut Env,
    error: vodozemac::megolm::DecryptionError,
) -> jni::errors::Error {
    throw_with_signature(env, &error, |env, e| throw_decryption_error(env, e))
}

pub(crate) fn throw_session_key_decode_error(
    env: &mut Env,
    error: vodozemac::megolm::SessionKeyDecodeError,
) -> jni::errors::Error {
    throw_with_signature(env, &error, |env, e| throw_key_error(env, e))
}

pub(crate) fn throw_decode_error(
    env: &mut Env,
    error: vodozemac::DecodeError,
) -> jni::errors::Error {
    throw_with_signature(env, &error, |env, e| throw_decryption_error(env, e))
}
