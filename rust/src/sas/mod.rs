pub mod established_sas;
#[allow(clippy::module_inception)]
pub mod sas;

use jni::objects::{JByteArray, JObject, JObjectArray, JString};
use jni::{Env, JValue, jni_sig};
use vodozemac::sas::SasBytes;

use crate::classes::SAS_BYTES;

fn to_decimal_array<'local>(
    env: &mut Env<'local>,
    bytes: &SasBytes,
) -> Result<JObjectArray<'local, JString<'local>>, jni::errors::Error> {
    let decimals_array = JObjectArray::<JString>::new(env, 3, JString::null())?;
    let (d1, d2, d3) = bytes.decimals();

    for (i, d) in [d1, d2, d3].iter().enumerate() {
        let jstr = env.new_string(d.to_string())?;
        decimals_array.set_element(env, i, &jstr)?;
    }
    Ok(decimals_array)
}

fn to_emoji_array<'local>(
    env: &mut Env<'local>,
    bytes: &SasBytes,
) -> Result<jni::objects::JIntArray<'local>, jni::errors::Error> {
    let emoji_array = env.new_int_array(7)?;
    let jints = bytes.emoji_indices().map(|b| b as i32);

    emoji_array.set_region(env, 0, &jints)?;
    Ok(emoji_array)
}

fn to_raw_byte_array<'local>(
    env: &mut Env<'local>,
    bytes: &SasBytes,
) -> Result<JByteArray<'local>, jni::errors::Error> {
    let bytes_array = env.new_byte_array(6)?;
    let jbytes = bytes.as_bytes().map(|b| b as i8);

    bytes_array.set_region(env, 0, &jbytes)?;
    Ok(bytes_array)
}

pub(crate) fn to_java_sas_bytes<'local>(
    env: &mut Env<'local>,
    bytes: &SasBytes,
) -> Result<JObject<'local>, jni::errors::Error> {
    let decimals = to_decimal_array(env, bytes)?;
    let emoji_indices = to_emoji_array(env, bytes)?;
    let raw_bytes = to_raw_byte_array(env, bytes)?;

    env.new_object(
        SAS_BYTES,
        jni_sig!((rawBytes: byte[], emojiIndices: int[], decimals: java.lang.String[]) -> void),
        &[
            JValue::Object(&raw_bytes),
            JValue::Object(&emoji_indices),
            JValue::Object(&decimals),
        ],
    )
}
