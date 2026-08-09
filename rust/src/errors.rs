use jni::Env;
use jni::strings::{JNIString, JNIStr};
use jni::jni_str;

fn throw(env: &mut Env, class: &JNIStr, message: &str) -> jni::errors::Error {
    let _ = env.throw_new(class, JNIString::from(message));
    jni::errors::Error::JavaException
}

pub(crate) fn throw_pickle_error<E: std::fmt::Display>(env: &mut Env, error: E) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/PickleException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_decryption_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/DecryptionException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_session_creation_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/SessionCreationException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_key_error<E: std::fmt::Display>(env: &mut Env, error: E) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/KeyException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_signature_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/SignatureException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_generic_error<E: std::fmt::Display>(
    env: &mut Env,
    error: E,
) -> jni::errors::Error {
    throw(
        env,
        jni_str!("io/github/fherbreteau/vodozemac/VodozemacException"),
        &error.to_string(),
    )
}

pub(crate) fn throw_megolm_decryption_error(
    env: &mut Env,
    error: vodozemac::megolm::DecryptionError,
) -> jni::errors::Error {
    match &error {
        vodozemac::megolm::DecryptionError::Signature(_) => throw(
            env,
            jni_str!("io/github/fherbreteau/vodozemac/SignatureException"),
            &error.to_string(),
        ),
        _ => throw(
            env,
            jni_str!("io/github/fherbreteau/vodozemac/DecryptionException"),
            &error.to_string(),
        ),
    }
}
