use crate::slices::{CErrorStr, CSlice};
use crate::{CResult, boxed, free};
use jni::JNIEnv;
use jni::objects::JClass;
use macros::ffi;
use std::ptr::NonNull;
use std::str;
use vodozemac::pk_encryption::PkDecryption;
use vodozemac::{Curve25519PublicKey, Curve25519SecretKey};

pub fn register_jni(env: &mut JNIEnv, class: &JClass) -> jni::errors::Result<()> {
    env.register_native_methods(
        class,
        &[
            VODOZEMAC_PK_DECRYPTION_NEW_JNI.into(),
            VODOZEMAC_PK_DECRYPTION_FROM_KEY_JNI.into(),
            VODOZEMAC_PK_DECRYPTION_SECRET_KEY_JNI.into(),
            VODOZEMAC_PK_DECRYPTION_PUBLIC_KEY_JNI.into(),
            VODOZEMAC_PK_DECRYPTION_DECRYPT_JNI.into(),
            VODOZEMAC_PK_DECRYPTION_FREE_JNI.into(),
        ],
    )
}

#[ffi]
pub fn vodozemac_pk_decryption_new() -> NonNull<PkDecryption> {
    boxed(PkDecryption::new())
}

#[ffi]
pub fn vodozemac_pk_decryption_from_key(key: &Curve25519SecretKey) -> NonNull<PkDecryption> {
    boxed(PkDecryption::from_key(key.clone()))
}

#[ffi]
pub fn vodozemac_pk_decryption_secret_key(pk: &PkDecryption) -> NonNull<Curve25519SecretKey> {
    boxed(pk.secret_key().clone())
}

#[ffi]
pub fn vodozemac_pk_decryption_public_key(pk: &PkDecryption) -> NonNull<Curve25519PublicKey> {
    boxed(pk.public_key())
}

#[ffi]
#[sret]
pub fn vodozemac_pk_decryption_decrypt(
    pk: &PkDecryption,
    #[expand] ciphertext: &[u8],
    #[expand] mac: &[u8],
    ephemeral_key: &Curve25519PublicKey,
) -> CResult<CSlice<u8>, CErrorStr> {
    let message = vodozemac::pk_encryption::Message {
        ciphertext: ciphertext.to_vec(),
        mac: mac.to_vec(),
        ephemeral_key: *ephemeral_key,
    };
    pk.decrypt(&message)
        .map(Into::into)
        .map_err(Into::into)
        .into()
}

#[ffi]
pub fn vodozemac_pk_decryption_free(pk: NonNull<PkDecryption>) {
    free(pk)
}

#[ffi]
#[sret]
pub fn vodozemac_pk_decryption_from_libolm_pickle(
    #[expand] pickle: &[u8],
    #[expand] pickle_key: &[u8],
) -> CResult<NonNull<PkDecryption>, CErrorStr> {
    let pickle = str::from_utf8(pickle).expect("valid utf8");

    PkDecryption::from_libolm_pickle(pickle, pickle_key)
        .map(boxed)
        .map_err(Into::into)
        .into()
}