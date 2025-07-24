use crate::megolm::InboundGroupSession;
use crate::slices::{CErrorStr, CSlice};
use crate::{AsUsize, CResult, Chain, ChainExact, boxed, free, pickle, unpickle};
use jni::JNIEnv;
use jni::objects::JClass;
use macros::ffi;
use parking_lot::{RwLock, RwLockWriteGuard};
use std::ptr::NonNull;
use std::{array, str};
use vodozemac::megolm;
use vodozemac::megolm::{
    ExportedSessionKey, InboundGroupSessionPickle, MegolmMessage, SessionConfig, SessionKey,
    SessionOrdering,
};

pub fn register_jni(env: &mut JNIEnv, class: &JClass) -> jni::errors::Result<()> {
    env.register_native_methods(
        class,
        &[
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_NEW_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_FREE_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_IMPORT_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_SESSION_ID_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_CONNECTED_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_COMPARE_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_MERGE_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_FIRST_KNOWN_INDEX_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_ADVANCE_TO_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_DECRYPT_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_EXPORT_AT_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_EXPORT_AT_FIRST_KNOWN_INDEX_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_PICKLE_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_FROM_PICKLE_JNI.into(),
            VODOZEMAC_MEGOLM_INBOUND_GROUP_SESSION_FROM_LIBOLM_PICKLE_JNI.into(),
        ],
    )
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_new(
    key: &SessionKey,
    session_config: &SessionConfig,
) -> NonNull<InboundGroupSession> {
    boxed(RwLock::new(megolm::InboundGroupSession::new(
        key,
        *session_config,
    )))
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_free(
    inbound_group_session: NonNull<InboundGroupSession>,
) {
    free(inbound_group_session)
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_import(
    session_key: &ExportedSessionKey,
    session_config: &SessionConfig,
) -> NonNull<InboundGroupSession> {
    boxed(RwLock::new(megolm::InboundGroupSession::import(
        session_key,
        *session_config,
    )))
}

#[ffi]
#[sret]
pub fn vodozemac_megolm_inbound_group_session_session_id(
    inbound_group_session: &InboundGroupSession,
) -> CSlice<u8> {
    inbound_group_session.read().session_id().into()
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_connected(
    inbound_group_session: &InboundGroupSession,
    other: &InboundGroupSession,
) -> u32 {
    if (&raw const *inbound_group_session) == (&raw const *other) {
        return true.into();
    }

    // TODO: fix vodozemac, this does not need to be mutable
    let (mut this, mut other) = lock_ordered(inbound_group_session, other);

    this.connected(&mut other).into()
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_compare(
    inbound_group_session: &InboundGroupSession,
    other: &InboundGroupSession,
) -> u32 {
    if (&raw const *inbound_group_session) == (&raw const *other) {
        return 0; // Connected
    }

    // TODO: fix vodozemac, this does not need to be mutable
    let (mut this, mut other) = lock_ordered(inbound_group_session, other);

    match this.compare(&mut other) {
        SessionOrdering::Equal => 0,
        SessionOrdering::Better => 1,
        SessionOrdering::Worse => 2,
        SessionOrdering::Unconnected => 3,
    }
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_merge(
    inbound_group_session: &InboundGroupSession,
    other: &InboundGroupSession,
) -> Option<NonNull<InboundGroupSession>> {
    if (&raw const *inbound_group_session) == (&raw const *other) {
        return Some(boxed(RwLock::new(
            megolm::InboundGroupSession::from_pickle(inbound_group_session.read().pickle()),
        )));
    }

    // TODO: fix vodozemac, this should not need to be mutable
    let (mut this, mut other) = lock_ordered(inbound_group_session, other);

    this.merge(&mut other).map(RwLock::new).map(boxed)
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_first_known_index(
    inbound_group_session: &InboundGroupSession,
) -> u32 {
    inbound_group_session.read().first_known_index()
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_advance_to(
    inbound_group_session: &InboundGroupSession,
    index: u32,
) -> u32 {
    inbound_group_session.write().advance_to(index).into()
}

#[repr(C)]
pub struct DecryptedMessage {
    plaintext: CSlice<u8>,
    message_index: usize,
}

impl AsUsize for DecryptedMessage {
    type IntoIter = Chain<<CSlice<u8> as AsUsize>::IntoIter, array::IntoIter<usize, 1>>;

    fn as_usize(&self) -> Self::IntoIter {
        self.plaintext
            .as_usize()
            .chain_exact([self.message_index].into_iter())
    }
}

impl From<megolm::DecryptedMessage> for DecryptedMessage {
    fn from(value: megolm::DecryptedMessage) -> Self {
        Self {
            plaintext: value.plaintext.into(),
            message_index: value.message_index as usize,
        }
    }
}

#[ffi]
#[sret]
pub fn vodozemac_megolm_inbound_group_session_decrypt(
    inbound_group_session: &InboundGroupSession,
    message: &MegolmMessage,
) -> CResult<DecryptedMessage, CErrorStr> {
    inbound_group_session
        .write()
        .decrypt(message)
        .map(Into::into)
        .map_err(Into::into)
        .into()
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_export_at(
    inbound_group_session: &InboundGroupSession,
    index: u32,
) -> Option<NonNull<ExportedSessionKey>> {
    inbound_group_session.write().export_at(index).map(boxed)
}

#[ffi]
pub fn vodozemac_megolm_inbound_group_session_export_at_first_known_index(
    inbound_group_session: &InboundGroupSession,
) -> NonNull<ExportedSessionKey> {
    boxed(inbound_group_session.read().export_at_first_known_index())
}

#[ffi]
#[sret]
pub fn vodozemac_megolm_inbound_group_session_pickle(
    inbound_group_session: &InboundGroupSession,
    pickle_key: Option<&[u8; 32]>,
) -> CSlice<u8> {
    pickle!(inbound_group_session, pickle_key, RwLock::read)
}

#[ffi]
#[sret]
pub fn vodozemac_megolm_inbound_group_session_from_pickle(
    #[expand] ciphertext: &[u8],
    pickle_key: Option<&[u8; 32]>,
) -> CResult<NonNull<InboundGroupSession>, CErrorStr> {
    unpickle!(ciphertext, pickle_key, InboundGroupSessionPickle, megolm::InboundGroupSession, RwLock::new)
}

fn lock_ordered<'a, T>(
    a: &'a RwLock<T>,
    b: &'a RwLock<T>,
) -> (RwLockWriteGuard<'a, T>, RwLockWriteGuard<'a, T>) {
    let ptr_a = &raw const *a;
    let ptr_b = &raw const *b;

    if ptr_a < ptr_b {
        let left = a.write();
        let right = b.write();

        (left, right)
    } else {
        let left = b.write();
        let right = a.write();

        (right, left)
    }
}

#[ffi]
#[sret]
pub fn vodozemac_megolm_inbound_group_session_from_libolm_pickle(
    #[expand] pickle: &[u8],
    #[expand] pickle_key: &[u8],
) -> CResult<NonNull<InboundGroupSession>, CErrorStr> {
    let pickle = str::from_utf8(pickle).expect("valid utf8");

    megolm::InboundGroupSession::from_libolm_pickle(pickle, pickle_key)
        .map(RwLock::new)
        .map(boxed)
        .map_err(Into::into)
        .into()
}
