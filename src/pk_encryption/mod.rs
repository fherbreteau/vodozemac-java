use crate::slices::CSlice;
use crate::{boxed, AsUsize, Chain, ChainExact};
use jni::JNIEnv;
use jni::objects::JClass;
use std::ptr::NonNull;
use vodozemac::Curve25519PublicKey;

mod pk_decryption;

#[allow(clippy::module_inception)]
mod pk_encryption;

pub struct PkEncryptionJniClasses<'local, 'a> {
    pub pk_encryption: &'a JClass<'local>,
    pub pk_decryption: &'a JClass<'local>,
}

pub fn register_jni(
    env: &mut JNIEnv,
    PkEncryptionJniClasses {
        pk_encryption,
        pk_decryption,
    }: &PkEncryptionJniClasses,
) -> jni::errors::Result<()> {
    pk_encryption::register_jni(env, pk_encryption)?;
    pk_decryption::register_jni(env, pk_decryption)?;

    Ok(())
}

#[repr(C)]
pub struct Message {
    pub ciphertext: CSlice<u8>,
    pub mac: CSlice<u8>,
    pub ephemeral_key: NonNull<Curve25519PublicKey>,
}

impl AsUsize for Message {
    type IntoIter = Chain<
        Chain<
            <CSlice<u8> as AsUsize>::IntoIter,
            <CSlice<u8> as AsUsize>::IntoIter
        >,
        <NonNull<Curve25519PublicKey> as AsUsize>::IntoIter
    >;

    fn as_usize(&self) -> Self::IntoIter {
        self.ciphertext.as_usize()
            .chain_exact(self.mac.as_usize())
            .chain_exact(self.ephemeral_key.as_usize())
    }
}

impl From<vodozemac::pk_encryption::Message> for Message {
    fn from(value: vodozemac::pk_encryption::Message) -> Self {
        Self {
            ciphertext: value.ciphertext.into(),
            mac: value.mac.into(),
            ephemeral_key: boxed(value.ephemeral_key),
        }
    }
}
