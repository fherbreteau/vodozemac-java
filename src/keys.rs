use crate::slices::CErrorStr;
use crate::{CResult, ZST, boxed, free};
use jni::JNIEnv;
use jni::objects::JClass;
use macros::ffi;
use std::ptr::NonNull;
use vodozemac::{
    Curve25519PublicKey, Curve25519SecretKey, Ed25519PublicKey, Ed25519SecretKey, Ed25519Signature,
};
use zeroize::Zeroize;

pub fn register_jni(env: &mut JNIEnv, class: &JClass) -> jni::errors::Result<()> {
    env.register_native_methods(
        class,
        &[
            VODOZEMAC_ED25519_PUBLIC_KEY_FROM_BYTES_JNI.into(),
            VODOZEMAC_ED25519_PUBLIC_KEY_TO_BYTES_JNI.into(),
            VODOZEMAC_ED25519_PUBLIC_KEY_VERIFY_JNI.into(),
            VODOZEMAC_ED25519_PUBLIC_KEY_FREE_JNI.into(),
            VODOZEMAC_ED25519_SECRET_KEY_NEW_JNI.into(),
            VODOZEMAC_ED25519_SECRET_KEY_TO_BYTES_JNI.into(),
            VODOZEMAC_ED25519_SECRET_KEY_FROM_BYTES_JNI.into(),
            VODOZEMAC_ED25519_SECRET_KEY_PUBLIC_KEY_JNI.into(),
            VODOZEMAC_ED25519_SECRET_KEY_SIGN_JNI.into(),
            VODOZEMAC_ED25519_SECRET_KEY_FREE_JNI.into(),
            VODOZEMAC_CURVE25519_PUBLIC_KEY_FROM_BYTES_JNI.into(),
            VODOZEMAC_CURVE25519_PUBLIC_KEY_TO_BYTES_JNI.into(),
            VODOZEMAC_CURVE25519_PUBLIC_KEY_FREE_JNI.into(),
            VODOZEMAC_CURVE25519_SECRET_KEY_NEW_JNI.into(),
            VODOZEMAC_CURVE25519_SECRET_KEY_FROM_BYTES_JNI.into(),
            VODOZEMAC_CURVE25519_SECRET_KEY_TO_BYTES_JNI.into(),
            VODOZEMAC_CURVE25519_SECRET_KEY_DIFFIE_HELLMAN_JNI.into(),
            VODOZEMAC_CURVE25519_SECRET_KEY_PUBLIC_KEY_JNI.into(),
            VODOZEMAC_CURVE25519_SECRET_KEY_FREE_JNI.into(),
            VODOZEMAC_ED25519_SIGNATURE_FROM_BYTES_JNI.into(),
            VODOZEMAC_ED25519_SIGNATURE_TO_BYTES_JNI.into(),
            VODOZEMAC_ED25519_SIGNATURE_FREE_JNI.into(),
        ],
    )
}

#[ffi]
pub fn vodozemac_ed25519_public_key_from_bytes(
    bytes: &[u8; 32],
) -> Option<NonNull<Ed25519PublicKey>> {
    Some(boxed(Ed25519PublicKey::from_slice(bytes).ok()?))
}

#[ffi]
pub fn vodozemac_ed25519_public_key_to_bytes(key: &Ed25519PublicKey, bytes: &mut [u8; 32]) {
    bytes.copy_from_slice(key.as_bytes())
}

#[ffi]
#[sret]
pub fn vodozemac_ed25519_public_key_verify(
    key: &Ed25519PublicKey,
    #[expand] message: &[u8],
    signature: &Ed25519Signature,
) -> CResult<ZST, CErrorStr> {
    key.verify(message, signature)
        .map(Into::into)
        .map_err(Into::into)
        .into()
}

#[ffi]
pub fn vodozemac_ed25519_public_key_free(key: NonNull<Ed25519PublicKey>) {
    free(key)
}

#[ffi]
pub fn vodozemac_ed25519_secret_key_new() -> NonNull<Ed25519SecretKey> {
    boxed(Ed25519SecretKey::new())
}

#[ffi]
pub fn vodozemac_ed25519_secret_key_to_bytes(key: &Ed25519SecretKey, bytes: &mut [u8; 32]) {
    let mut key = key.to_bytes();
    bytes.copy_from_slice(&*key);
    key.zeroize();
}

#[ffi]
pub fn vodozemac_ed25519_secret_key_from_bytes(bytes: &[u8; 32]) -> NonNull<Ed25519SecretKey> {
    boxed(Ed25519SecretKey::from_slice(bytes))
}

#[ffi]
pub fn vodozemac_ed25519_secret_key_public_key(
    key: &Ed25519SecretKey,
) -> NonNull<Ed25519PublicKey> {
    boxed(key.public_key())
}

#[ffi]
pub fn vodozemac_ed25519_secret_key_sign(
    key: &Ed25519SecretKey,
    #[expand] message: &[u8],
) -> NonNull<Ed25519Signature> {
    boxed(key.sign(message))
}

#[ffi]
pub fn vodozemac_ed25519_secret_key_free(
    key: NonNull<Ed25519SecretKey>,
) {
    free(key)
}

#[ffi]
pub fn vodozemac_curve25519_public_key_from_bytes(
    bytes: &[u8; 32],
) -> NonNull<Curve25519PublicKey> {
    boxed(Curve25519PublicKey::from_bytes(*bytes))
}

#[ffi]
pub fn vodozemac_curve25519_public_key_to_bytes(key: &Curve25519PublicKey, bytes: &mut [u8; 32]) {
    bytes.copy_from_slice(key.as_bytes())
}

#[ffi]
pub fn vodozemac_curve25519_public_key_free(key: NonNull<Curve25519PublicKey>) {
    free(key)
}

#[ffi]
pub fn vodozemac_curve25519_secret_key_new() -> NonNull<Curve25519SecretKey> {
    boxed(Curve25519SecretKey::new())
}

#[ffi]
pub fn vodozemac_curve25519_secret_key_from_bytes(
    bytes: &[u8; 32],
) -> NonNull<Curve25519SecretKey> {
    boxed(Curve25519SecretKey::from_slice(bytes))
}

#[ffi]
pub fn vodozemac_curve25519_secret_key_to_bytes(key: &Curve25519SecretKey, bytes: &mut [u8; 32]) {
    let mut key = key.to_bytes();
    bytes.copy_from_slice(&*key);
    key.zeroize();
}

#[ffi]
pub fn vodozemac_curve25519_secret_key_diffie_hellman(
    key: &Curve25519SecretKey,
    their_public_key: &Curve25519PublicKey,
    secret: &mut [u8; 32],
) -> u32 {
    let shared_secret = key.diffie_hellman(their_public_key);
    secret.copy_from_slice(shared_secret.as_bytes());
    shared_secret.was_contributory().into()
}

#[ffi]
pub fn vodozemac_curve25519_secret_key_public_key(
    key: &Curve25519SecretKey,
) -> NonNull<Curve25519PublicKey> {
    boxed(Curve25519PublicKey::from(key))
}

#[ffi]
pub fn vodozemac_curve25519_secret_key_free(key: NonNull<Curve25519SecretKey>) {
    free(key)
}

#[ffi]
pub fn vodozemac_ed25519_signature_from_bytes(bytes: &[u8; 64]) -> NonNull<Ed25519Signature> {
    // TODO: fix in vodozemac
    boxed(Ed25519Signature::from_slice(bytes).expect("input is 64 bytes"))
}

#[ffi]
pub fn vodozemac_ed25519_signature_to_bytes(signature: &Ed25519Signature, bytes: &mut [u8; 64]) {
    bytes.copy_from_slice(&signature.to_bytes())
}

#[ffi]
pub fn vodozemac_ed25519_signature_free(signature: NonNull<Ed25519Signature>) {
    free(signature)
}
