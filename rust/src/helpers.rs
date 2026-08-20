use jni::Env;
use jni::sys::{jint, jlong};
use vodozemac::megolm::SessionConfig as MegolmSessionConfig;
use vodozemac::olm::SessionConfig as OlmSessionConfig;

use crate::errors::throw_generic_error;

pub(crate) fn wrap<T>(v: Vec<T>) -> Result<[T; 32], jni::errors::Error> {
    v.try_into().map_err(|_v| jni::errors::Error::JavaException)
}

pub(crate) fn olm_session_config_from_version(
    env: &mut Env,
    version: jint,
) -> Result<OlmSessionConfig, jni::errors::Error> {
    match version {
        1 => Ok(OlmSessionConfig::version_1()),
        2 => Ok(OlmSessionConfig::version_2()),
        _ => Err(throw_generic_error(
            env,
            format!("Invalid session config version: {version}"),
        )),
    }
}

pub(crate) fn megolm_session_config_from_version(
    env: &mut Env,
    version: jint,
) -> Result<MegolmSessionConfig, jni::errors::Error> {
    match version {
        1 => Ok(MegolmSessionConfig::version_1()),
        2 => Ok(MegolmSessionConfig::version_2()),
        _ => Err(throw_generic_error(
            env,
            format!("Invalid session config version: {version}"),
        )),
    }
}

pub(crate) fn check_ptr(env: &mut Env, ptr: jlong) -> Result<(), jni::errors::Error> {
    if ptr == 0 {
        return Err(throw_generic_error(env, "Null native pointer"));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) const PICKLE_KEY: [u8; 32] = [0u8; 32];

#[cfg(test)]
pub(crate) fn get_jvm() -> jni::JavaVM {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let classpath = std::env::current_dir()
            .expect("Failed to get current dir")
            .parent()
            .expect("Failed to get project root")
            .join("target/classes");
        let args = jni::InitArgsBuilder::new()
            .option(format!("-Djava.class.path={}", classpath.to_string_lossy()))
            .build()
            .expect("Failed to create JVM init args");
        let _ = jni::JavaVM::new(args).expect("Failed to create JVM");
    });
    jni::JavaVM::singleton().expect("JVM should be initialized")
}
