use jni::Env;
use jni::jni_str;
use jni::strings::{JNIStr, JNIString};
use vodozemac::sas::InvalidCount;
use vodozemac::sas::SasError;

fn throw(env: &mut Env, class: &JNIStr, message: &str) -> jni::errors::Error {
    let _ = env.throw_new(class, JNIString::from(message));
    jni::errors::Error::JavaException
}

pub(crate) fn throw_pickle_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/PickleException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_decryption_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/DecryptionException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_session_creation_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/SessionCreationException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_key_error<E: std::fmt::Display>(env: &mut Env, error: E) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/KeyException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_signature_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/SignatureException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_sas_error(env: &mut Env, error: SasError) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/SasException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_ecies_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/EciesException"),
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

pub(crate) fn throw_generic_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/exception/VodozemacException"),
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
