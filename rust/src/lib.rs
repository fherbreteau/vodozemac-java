use jni::EnvUnowned;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use vodozemac::olm::Account;

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_VodozemacAccount_nativeNew(
    mut env: EnvUnowned,
    _class: JClass,
) -> jlong {
    let outcome = env.with_env(|_env| -> Result<jlong, jni::errors::Error> {
        let account = Box::new(Account::new());
        Ok(Box::into_raw(account) as jlong)
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_VodozemacAccount_nativeCurve25519Key(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let key = account.curve25519_key().to_base64();
        let jni_string = env.new_string(key)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_VodozemacAccount_nativeEd25519Key(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let key = account.ed25519_key().to_base64();
        let jni_string = env.new_string(key)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_VodozemacAccount_nativeSign(
    mut env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
    message: JString,
) -> jstring {
    let outcome = env.with_env(|env| -> Result<jstring, jni::errors::Error> {
        let account = unsafe { &*(ptr as *const Account) };
        let msg: String = message.to_string();
        let signature = account.sign(&msg).to_base64();
        let jni_string = env.new_string(signature)?;
        Ok(jni_string.into_raw())
    });
    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_fherbreteau_vodozemac_VodozemacAccount_nativeFree(
    _env: EnvUnowned,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut Account);
    }
}
