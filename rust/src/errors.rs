use jni::Env;
use jni::jni_str;
use jni::strings::{JNIStr, JNIString};
use vodozemac::sas::InvalidCount;
use vodozemac::sas::SasError;

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

throw_typed!(
    throw_pickle_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/PickleException")
);
throw_typed!(
    throw_decryption_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/DecryptionException")
);
throw_typed!(
    throw_encryption_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/EncryptionException")
);
throw_typed!(
    throw_session_creation_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/SessionCreationException")
);
throw_typed!(
    throw_key_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/KeyException")
);
throw_typed!(
    throw_signature_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/SignatureException")
);
throw_typed!(
    throw_ecies_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/EciesException")
);
throw_typed!(
    throw_generic_error,
    jni_str!("io/github/fherbreteau/vodozemac/exception/ConversionException")
);

pub(crate) fn throw_sas_error(env: &mut Env, error: SasError) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/SasException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_invalid_count_error(env: &mut Env, error: InvalidCount) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/SasException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_megolm_decryption_error(
    env: &mut Env,
    error: vodozemac::megolm::DecryptionError,
) -> jni::errors::Error {
    match &error {
        vodozemac::megolm::DecryptionError::Signature(e) => throw_signature_error(env, e),
        _ => throw_decryption_error(env, error),
    }
}

pub(crate) fn throw_session_key_decode_error(
    env: &mut Env,
    error: vodozemac::megolm::SessionKeyDecodeError,
) -> jni::errors::Error {
    match &error {
        vodozemac::megolm::SessionKeyDecodeError::Signature(e) => throw_signature_error(env, e),
        _ => throw_key_error(env, error),
    }
}

pub(crate) fn throw_decode_error(
    env: &mut Env,
    error: vodozemac::DecodeError,
) -> jni::errors::Error {
    match &error {
        vodozemac::DecodeError::Signature(e) => throw_signature_error(env, e),
        _ => throw_decryption_error(env, error),
    }
}
