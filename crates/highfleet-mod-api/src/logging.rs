use std::mem::size_of;
use std::sync::OnceLock;

use log::{Level, Log, Metadata, Record};
use thiserror::Error;

use crate::ffi::{
    HfmHostApiV1, HfmLogRecordV1, HfmStr, HFM_HOST_ABI_VERSION_1, HFM_LOG_LEVEL_DEBUG,
    HFM_LOG_LEVEL_ERROR, HFM_LOG_LEVEL_INFO, HFM_LOG_LEVEL_TRACE, HFM_LOG_LEVEL_WARN,
    HFM_STATUS_API_TOO_SMALL, HFM_STATUS_LOGGER_ALREADY_SET, HFM_STATUS_MISSING_CALLBACK,
    HFM_STATUS_NULL_API, HFM_STATUS_UNSUPPORTED_ABI,
};

static LOGGER: OnceLock<HostLogger> = OnceLock::new();

/// An error encountered while installing the modloader logger.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum InstallError {
    /// The host API pointer was null.
    #[error("the host API pointer is null")]
    NullApi,
    /// The host implements a different ABI version.
    #[error("the host API version is unsupported")]
    UnsupportedAbi,
    /// The host table is too small to contain version 1.
    #[error("the host API table is too small")]
    ApiTooSmall,
    /// The host did not provide the required write callback.
    #[error("the host API is missing the log_write callback")]
    MissingCallback,
    /// This mod already installed a global logger.
    #[error("this mod already installed a logger")]
    LoggerAlreadySet,
}

impl InstallError {
    /// Returns the stable status code used by `highfleet_mod_setup_v1`.
    pub const fn status_code(self) -> u32 {
        match self {
            Self::NullApi => HFM_STATUS_NULL_API,
            Self::UnsupportedAbi => HFM_STATUS_UNSUPPORTED_ABI,
            Self::ApiTooSmall => HFM_STATUS_API_TOO_SMALL,
            Self::MissingCallback => HFM_STATUS_MISSING_CALLBACK,
            Self::LoggerAlreadySet => HFM_STATUS_LOGGER_ALREADY_SET,
        }
    }
}

/// Installs a [`log::Log`] implementation backed by modloader callbacks.
///
/// The host table is copied. Its callback context and callback code must
/// nevertheless remain valid for the remainder of the mod's logging lifetime.
/// The callbacks must be thread-safe because [`log`] can invoke them from any
/// thread.
///
/// # Safety
///
/// `host` must be null or point to readable memory containing at least its
/// `abi_version` and `struct_size` fields. When it declares a complete version
/// 1 table, the entire [`HfmHostApiV1`] must be readable and properly aligned.
pub unsafe fn install_logger(host: *const HfmHostApiV1) -> Result<(), InstallError> {
    let host = unsafe { copy_and_validate_api(host)? };
    let logger = LOGGER.get_or_init(|| HostLogger { host });

    log::set_logger(logger).map_err(|_| InstallError::LoggerAlreadySet)?;

    // Leave the facade open to every level. The host's enabled callback owns
    // dynamic filtering and can change it without reaching into this DLL.
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}

unsafe fn copy_and_validate_api(host: *const HfmHostApiV1) -> Result<HfmHostApiV1, InstallError> {
    if host.is_null() {
        return Err(InstallError::NullApi);
    }

    // Read the common two-field header before accessing the rest of the table.
    let abi_version = unsafe { core::ptr::addr_of!((*host).abi_version).read() };
    let struct_size = unsafe { core::ptr::addr_of!((*host).struct_size).read() };

    if abi_version != HFM_HOST_ABI_VERSION_1 {
        return Err(InstallError::UnsupportedAbi);
    }
    if struct_size < size_of::<HfmHostApiV1>() as u32 {
        return Err(InstallError::ApiTooSmall);
    }

    let host = unsafe { host.read() };
    if host.log_write.is_none() {
        return Err(InstallError::MissingCallback);
    }

    Ok(host)
}

struct HostLogger {
    host: HfmHostApiV1,
}

// The host contract requires its opaque context and callbacks to remain valid
// and thread-safe for the mod's entire logging lifetime.
unsafe impl Send for HostLogger {}
unsafe impl Sync for HostLogger {}

impl Log for HostLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        let Some(enabled) = self.host.log_enabled else {
            return true;
        };

        unsafe {
            enabled(
                self.host.context,
                level_to_ffi(metadata.level()),
                HfmStr::from(metadata.target()),
            ) != 0
        }
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = record.args().to_string();
        let ffi_record = HfmLogRecordV1 {
            struct_size: size_of::<HfmLogRecordV1>() as u32,
            level: level_to_ffi(record.level()),
            target: HfmStr::from(record.target()),
            message: HfmStr::from(message.as_str()),
            module_path: record.module_path().map(HfmStr::from).unwrap_or_default(),
            file: record.file().map(HfmStr::from).unwrap_or_default(),
            line: record.line().unwrap_or(0),
            reserved: 0,
        };

        // Validation during installation guarantees that this is present.
        let write = self.host.log_write.expect("validated log_write callback");
        unsafe {
            write(self.host.context, &ffi_record);
        }
    }

    fn flush(&self) {
        if let Some(flush) = self.host.log_flush {
            unsafe {
                flush(self.host.context);
            }
        }
    }
}

const fn level_to_ffi(level: Level) -> u32 {
    match level {
        Level::Error => HFM_LOG_LEVEL_ERROR,
        Level::Warn => HFM_LOG_LEVEL_WARN,
        Level::Info => HFM_LOG_LEVEL_INFO,
        Level::Debug => HFM_LOG_LEVEL_DEBUG,
        Level::Trace => HFM_LOG_LEVEL_TRACE,
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use std::sync::Mutex;

    use log::Level;

    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct CapturedRecord {
        level: u32,
        target: String,
        message: String,
        module_path: String,
        file: String,
        line: u32,
    }

    struct Capture {
        enabled: bool,
        records: Mutex<Vec<CapturedRecord>>,
        flushes: Mutex<u32>,
    }

    unsafe extern "C" fn enabled(context: *mut c_void, _level: u32, _target: HfmStr) -> u8 {
        let capture = unsafe { &*(context.cast::<Capture>()) };
        capture.enabled.into()
    }

    unsafe extern "C" fn write(context: *mut c_void, record: *const HfmLogRecordV1) {
        let capture = unsafe { &*(context.cast::<Capture>()) };
        let record = unsafe { &*record };
        capture.records.lock().unwrap().push(CapturedRecord {
            level: record.level,
            target: unsafe { copy_string(record.target) },
            message: unsafe { copy_string(record.message) },
            module_path: unsafe { copy_string(record.module_path) },
            file: unsafe { copy_string(record.file) },
            line: record.line,
        });
    }

    unsafe extern "C" fn flush(context: *mut c_void) {
        let capture = unsafe { &*(context.cast::<Capture>()) };
        *capture.flushes.lock().unwrap() += 1;
    }

    unsafe fn copy_string(value: HfmStr) -> String {
        if value.len == 0 {
            return String::new();
        }
        let bytes = unsafe { std::slice::from_raw_parts(value.ptr, value.len) };
        std::str::from_utf8(bytes).unwrap().to_owned()
    }

    fn host_for(capture: &mut Capture) -> HfmHostApiV1 {
        HfmHostApiV1 {
            abi_version: HFM_HOST_ABI_VERSION_1,
            struct_size: size_of::<HfmHostApiV1>() as u32,
            context: (capture as *mut Capture).cast(),
            log_enabled: Some(enabled),
            log_write: Some(write),
            log_flush: Some(flush),
        }
    }

    #[test]
    fn forwards_formatted_records_and_source_metadata() {
        let mut capture = Capture {
            enabled: true,
            records: Mutex::new(Vec::new()),
            flushes: Mutex::new(0),
        };
        let logger = HostLogger {
            host: host_for(&mut capture),
        };
        logger.log(
            &Record::builder()
                .args(format_args!("value = {}", 42))
                .level(Level::Warn)
                .target("qol::config")
                .module_path(Some("qol::config"))
                .file(Some("src/config.rs"))
                .line(Some(27))
                .build(),
        );
        logger.flush();

        assert_eq!(
            *capture.records.lock().unwrap(),
            vec![CapturedRecord {
                level: HFM_LOG_LEVEL_WARN,
                target: "qol::config".to_owned(),
                message: "value = 42".to_owned(),
                module_path: "qol::config".to_owned(),
                file: "src/config.rs".to_owned(),
                line: 27,
            }]
        );
        assert_eq!(*capture.flushes.lock().unwrap(), 1);
    }

    #[test]
    fn disabled_records_are_not_formatted_or_written() {
        let mut capture = Capture {
            enabled: false,
            records: Mutex::new(Vec::new()),
            flushes: Mutex::new(0),
        };
        let logger = HostLogger {
            host: host_for(&mut capture),
        };
        let arguments = format_args!("discarded");
        let record = Record::builder().args(arguments).level(Level::Info).build();

        logger.log(&record);

        assert!(capture.records.lock().unwrap().is_empty());
    }

    #[test]
    fn rejects_invalid_api_headers_without_installing() {
        assert_eq!(
            unsafe { copy_and_validate_api(core::ptr::null()) }.unwrap_err(),
            InstallError::NullApi
        );

        let mut too_old = HfmHostApiV1 {
            abi_version: 99,
            struct_size: size_of::<HfmHostApiV1>() as u32,
            context: core::ptr::null_mut(),
            log_enabled: None,
            log_write: Some(write),
            log_flush: None,
        };
        assert_eq!(
            unsafe { copy_and_validate_api(&too_old) }.unwrap_err(),
            InstallError::UnsupportedAbi
        );

        too_old.abi_version = HFM_HOST_ABI_VERSION_1;
        too_old.struct_size = 8;
        assert_eq!(
            unsafe { copy_and_validate_api(&too_old) }.unwrap_err(),
            InstallError::ApiTooSmall
        );
    }
}
