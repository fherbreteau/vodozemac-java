use jni::sys::jint;
use vodozemac::megolm::SessionConfig as MegolmSessionConfig;
use vodozemac::olm::SessionConfig as OlmSessionConfig;

pub(crate) fn wrap<T>(v: Vec<T>) -> [T; 32] {
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!("Expected a Vec of length {} but it was {}", 32, v.len())
    })
}

pub(crate) fn olm_session_config_from_version(
    version: jint,
) -> Result<OlmSessionConfig, jni::errors::Error> {
    match version {
        1 => Ok(OlmSessionConfig::version_1()),
        2 => Ok(OlmSessionConfig::version_2()),
        _ => Err(jni::errors::Error::JavaException),
    }
}

pub(crate) fn megolm_session_config_from_version(
    version: jint,
) -> Result<MegolmSessionConfig, jni::errors::Error> {
    match version {
        1 => Ok(MegolmSessionConfig::version_1()),
        2 => Ok(MegolmSessionConfig::version_2()),
        _ => Err(jni::errors::Error::JavaException),
    }
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
