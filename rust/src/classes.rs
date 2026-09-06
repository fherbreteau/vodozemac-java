use jni::jni_str;
use jni::strings::JNIStr;

pub(crate) const OLM_SESSION: &JNIStr = jni_str!("io/github/fherbreteau/vodozemac/olm/OlmSession");
pub(crate) const OLM_INBOUND_CREATION_RESULT: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/olm/InboundCreationResult");
pub(crate) const OLM_MESSAGE: &JNIStr = jni_str!("io/github/fherbreteau/vodozemac/olm/OlmMessage");
pub(crate) const SESSION_KEYS: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/olm/SessionKeys");
pub(crate) const IDENTITY_KEYS: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/account/IdentityKeys");
pub(crate) const ONE_TIME_KEY_GENERATION_RESULT: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/account/OneTimeKeyGenerationResult");
pub(crate) const DEHYDRATED_DEVICE_RESULT: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/account/DehydratedDeviceResult");
pub(crate) const MEGOLM_MESSAGE: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/megolm/MegolmMessage");
pub(crate) const DECRYPTED_MESSAGE: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/megolm/DecryptedMessage");
pub(crate) const SESSION_ORDERING: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/megolm/SessionOrdering");
pub(crate) const SAS_BYTES: &JNIStr = jni_str!("io/github/fherbreteau/vodozemac/sas/SasBytes");
pub(crate) const ESTABLISHED_SAS: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/sas/EstablishedSas");
pub(crate) const CHECK_CODE: &JNIStr = jni_str!("io/github/fherbreteau/vodozemac/ecies/CheckCode");
pub(crate) const ECIES_OUTBOUND_CREATION_RESULT: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/ecies/OutboundCreationResult");
pub(crate) const ECIES_INBOUND_CREATION_RESULT: &JNIStr =
    jni_str!("io/github/fherbreteau/vodozemac/ecies/InboundCreationResult");
pub(crate) const PK_MESSAGE: &JNIStr = jni_str!("io/github/fherbreteau/vodozemac/backup/PkMessage");
pub(crate) const JAVA_LONG: &JNIStr = jni_str!("java/lang/Long");
pub(crate) const JAVA_ARRAY_LIST: &JNIStr = jni_str!("java/util/ArrayList");
pub(crate) const JAVA_HASH_MAP: &JNIStr = jni_str!("java/util/HashMap");
