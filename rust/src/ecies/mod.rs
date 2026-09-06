#[allow(clippy::module_inception)]
pub mod ecies;
pub mod established_ecies;

use jni::objects::{JByteArray, JObject};
use jni::sys::jint;
use jni::{Env, JValue, jni_sig};
use vodozemac::ecies::CheckCode;

use crate::classes::CHECK_CODE;

pub(crate) fn to_java_check_code<'local>(
    env: &mut Env<'local>,
    check_code: &CheckCode,
) -> Result<JObject<'local>, jni::errors::Error> {
    let bytes: JByteArray = env.byte_array_from_slice(check_code.as_bytes())?;
    let digit = jint::from(check_code.to_digit());
    env.new_object(
        CHECK_CODE,
        jni_sig!((bytes: byte[], digit: int) -> void),
        &[JValue::Object(&bytes), JValue::Int(digit)],
    )
}
