use crate::pk_encryption::Message;
use crate::{boxed, free};
use jni::JNIEnv;
use jni::objects::JClass;
use macros::ffi;
use std::ptr::NonNull;
use vodozemac::Curve25519PublicKey;
use vodozemac::pk_encryption::PkEncryption;

pub fn register_jni(env: &mut JNIEnv, class: &JClass) -> jni::errors::Result<()> {
    env.register_native_methods(
        class,
        &[
            VODOZEMAC_PK_ENCRYPTION_FROM_KEY_JNI.into(),
            VODOZEMAC_PK_ENCRYPTION_ENCRYPT_JNI.into(),
            VODOZEMAC_PK_ENCRYPTION_FREE_JNI.into(),
        ],
    )
}

#[ffi]
pub fn vodozemac_pk_encryption_from_key(key: &Curve25519PublicKey) -> NonNull<PkEncryption> {
    boxed(PkEncryption::from_key(*key))
}

#[ffi]
#[sret]
pub fn vodozemac_pk_encryption_encrypt(pk: &PkEncryption, #[expand] message: &[u8]) -> Message {
    pk.encrypt(message).into()
}

#[ffi]
pub fn vodozemac_pk_encryption_free(pk: NonNull<PkEncryption>) {
    free(pk)
}
