use std::collections::HashMap;
use std::ffi::c_void;
use std::fs;
use std::io::Write;
use std::mem::size_of;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::OnceLock;

use flexi_logger::DeferredNow;
use highfleet_mod_api::ffi::{
    HfmHostApiV1, HfmLogRecordV1, HfmStr, HFM_HOST_ABI_VERSION_1, HFM_LOG_LEVEL_DEBUG,
    HFM_LOG_LEVEL_ERROR, HFM_LOG_LEVEL_INFO, HFM_LOG_LEVEL_TRACE, HFM_LOG_LEVEL_WARN,
    HFM_STATUS_OK,
};
use libloading::Library;
use log::{debug, error, warn, Level, Metadata, Record};
use serde::{Deserialize, Serialize};

const MOD_TARGET_PREFIX: &str = "highfleet_mod::";
const MAX_LOG_STRING_BYTES: usize = 1024 * 1024;
const LOGGING_CONFIG_PATH: &str = "./Modloader/config/logging.json";

type SetupModV1 = unsafe extern "C" fn(host: *const HfmHostApiV1) -> u32;

static LOGGING_CONFIG: OnceLock<LoggingConfig> = OnceLock::new();

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
enum ModLogLevel {
    Off = 0,
    Error = HFM_LOG_LEVEL_ERROR as u8,
    Warn = HFM_LOG_LEVEL_WARN as u8,
    Info = HFM_LOG_LEVEL_INFO as u8,
    Debug = HFM_LOG_LEVEL_DEBUG as u8,
    Trace = HFM_LOG_LEVEL_TRACE as u8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
struct LoggingConfig {
    default_level: ModLogLevel,
    mods: HashMap<String, ModLogLevel>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            default_level: ModLogLevel::Debug,
            mods: HashMap::new(),
        }
    }
}

impl LoggingConfig {
    fn level_for(&self, name: &str) -> ModLogLevel {
        self.mods
            .iter()
            .find_map(|(configured_name, level)| {
                configured_name.eq_ignore_ascii_case(name).then_some(*level)
            })
            .unwrap_or(self.default_level)
    }
}

struct ModLogContext {
    name: String,
    target: String,
    enabled: AtomicBool,
    max_level: AtomicU8,
}

impl ModLogContext {
    fn from_path(path: &Path) -> Self {
        let name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-mod");
        let name = sanitize_name(name);
        let level = logging_config().level_for(&name);

        Self {
            target: format!("{MOD_TARGET_PREFIX}{name}"),
            name,
            enabled: AtomicBool::new(level != ModLogLevel::Off),
            max_level: AtomicU8::new(level as u8),
        }
    }

    fn allows(&self, level: Level) -> bool {
        if !self.enabled.load(Ordering::Relaxed)
            || level_to_ffi(level) > u32::from(self.max_level.load(Ordering::Relaxed))
            || level > log::max_level()
        {
            return false;
        }

        log::logger().enabled(
            &Metadata::builder()
                .level(level)
                .target(&self.target)
                .build(),
        )
    }
}

pub fn load_config() {
    let config = match fs::read_to_string(LOGGING_CONFIG_PATH) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(config) => config,
            Err(error) => {
                error!(
                    "Failed to parse logging config at {}: {}. Using defaults",
                    LOGGING_CONFIG_PATH, error
                );
                LoggingConfig::default()
            }
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let config = LoggingConfig::default();
            match serde_json::to_string_pretty(&config)
                .map_err(|error| error.to_string())
                .and_then(|contents| {
                    fs::write(LOGGING_CONFIG_PATH, contents).map_err(|error| error.to_string())
                }) {
                Ok(()) => debug!("Created default logging config at {}", LOGGING_CONFIG_PATH),
                Err(error) => error!(
                    "Failed to create default logging config at {}: {}",
                    LOGGING_CONFIG_PATH, error
                ),
            }
            config
        }
        Err(error) => {
            error!(
                "Failed to read logging config at {}: {}. Using defaults",
                LOGGING_CONFIG_PATH, error
            );
            LoggingConfig::default()
        }
    };

    if LOGGING_CONFIG.set(config).is_err() {
        warn!("Logging config was already loaded; keeping the existing settings");
    }
}

fn logging_config() -> &'static LoggingConfig {
    LOGGING_CONFIG.get_or_init(LoggingConfig::default)
}

/// Installs the host logging API in a loaded mod when it exports the version 1
/// setup function.
///
/// # Safety
///
/// `library` must represent a loaded mod and must not be unloaded concurrently
/// while its setup function is being resolved and called.
pub unsafe fn setup_mod_logging(library: &Library, path: &Path) {
    let setup = match unsafe { library.get::<SetupModV1>(b"highfleet_mod_setup_v1") } {
        Ok(setup) => setup,
        Err(error) => {
            warn!(
                "Mod {} does not expose highfleet_mod_setup_v1; mod logging is unavailable: {}",
                path.display(),
                error
            );
            return;
        }
    };

    // The callbacks can be retained by the mod's global logger, so the context
    // intentionally has process lifetime, just like the loaded mod library.
    let context: &'static ModLogContext = Box::leak(Box::new(ModLogContext::from_path(path)));
    let host = HfmHostApiV1 {
        abi_version: HFM_HOST_ABI_VERSION_1,
        struct_size: size_of::<HfmHostApiV1>() as u32,
        context: (context as *const ModLogContext).cast_mut().cast(),
        log_enabled: Some(mod_log_enabled),
        log_write: Some(mod_log_write),
        log_flush: Some(mod_log_flush),
    };

    let status = unsafe { setup(&host) };
    if status == HFM_STATUS_OK {
        debug!("Configured modloader logging for {}", context.name);
    } else {
        context.enabled.store(false, Ordering::Relaxed);
        error!(
            "Mod {} rejected the version 1 host logging API with status {}",
            context.name, status
        );
    }
}

pub fn format_log(
    write: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record<'_>,
) -> Result<(), std::io::Error> {
    let origin = record
        .target()
        .strip_prefix(MOD_TARGET_PREFIX)
        .unwrap_or("modloader");

    write!(
        write,
        "{} [{}] {} {}",
        now.format("%Y-%m-%dT%H:%M:%S"),
        origin,
        record.level(),
        record.args()
    )
}

unsafe extern "C" fn mod_log_enabled(context: *mut c_void, level: u32, _target: HfmStr) -> u8 {
    std::panic::catch_unwind(|| unsafe {
        context_from_ptr(context)
            .zip(level_from_ffi(level))
            .is_some_and(|(context, level)| context.allows(level))
    })
    .unwrap_or(false)
    .into()
}

unsafe extern "C" fn mod_log_write(context: *mut c_void, record: *const HfmLogRecordV1) {
    let _ = std::panic::catch_unwind(|| unsafe {
        route_mod_record(context, record);
    });
}

unsafe extern "C" fn mod_log_flush(_context: *mut c_void) {
    let _ = std::panic::catch_unwind(|| {
        log::logger().flush();
    });
}

unsafe fn route_mod_record(context: *mut c_void, record: *const HfmLogRecordV1) {
    let Some(context) = (unsafe { context_from_ptr(context) }) else {
        return;
    };
    if record.is_null() {
        return;
    }

    let struct_size = unsafe { core::ptr::addr_of!((*record).struct_size).read() };
    if struct_size < size_of::<HfmLogRecordV1>() as u32 {
        return;
    }

    let record = unsafe { &*record };
    let Some(level) = level_from_ffi(record.level) else {
        return;
    };
    if !context.allows(level) {
        return;
    }

    let Some(message) = (unsafe { read_ffi_str(record, record.message) }) else {
        return;
    };
    let module_path =
        unsafe { read_ffi_str(record, record.module_path) }.filter(|value| !value.is_empty());
    let file = unsafe { read_ffi_str(record, record.file) }.filter(|value| !value.is_empty());
    let line = (record.line != 0).then_some(record.line);

    log::logger().log(
        &Record::builder()
            .args(format_args!("{}", message))
            .level(level)
            .target(&context.target)
            .module_path(module_path)
            .file(file)
            .line(line)
            .build(),
    );
}

unsafe fn context_from_ptr(context: *mut c_void) -> Option<&'static ModLogContext> {
    if context.is_null() {
        return None;
    }

    Some(unsafe { &*context.cast::<ModLogContext>() })
}

unsafe fn read_ffi_str(_record: &HfmLogRecordV1, value: HfmStr) -> Option<&str> {
    if value.len == 0 {
        return Some("");
    }
    if value.ptr.is_null() || value.len > MAX_LOG_STRING_BYTES {
        return None;
    }

    let bytes = unsafe { std::slice::from_raw_parts(value.ptr, value.len) };
    std::str::from_utf8(bytes).ok()
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|character| {
            if character.is_control() || matches!(character, '[' | ']') {
                '_'
            } else {
                character
            }
        })
        .collect()
}

const fn level_from_ffi(level: u32) -> Option<Level> {
    match level {
        HFM_LOG_LEVEL_ERROR => Some(Level::Error),
        HFM_LOG_LEVEL_WARN => Some(Level::Warn),
        HFM_LOG_LEVEL_INFO => Some(Level::Info),
        HFM_LOG_LEVEL_DEBUG => Some(Level::Debug),
        HFM_LOG_LEVEL_TRACE => Some(Level::Trace),
        _ => None,
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
    use super::*;

    #[test]
    fn sanitizes_names_that_could_break_the_log_prefix() {
        assert_eq!(sanitize_name("qol[beta]\n"), "qol_beta__");
    }

    #[test]
    fn maps_only_known_log_levels() {
        assert_eq!(level_from_ffi(HFM_LOG_LEVEL_ERROR), Some(Level::Error));
        assert_eq!(level_from_ffi(HFM_LOG_LEVEL_TRACE), Some(Level::Trace));
        assert_eq!(level_from_ffi(0), None);
        assert_eq!(level_from_ffi(u32::MAX), None);
    }

    #[test]
    fn resolves_mod_levels_case_insensitively() {
        let config: LoggingConfig = serde_json::from_str(
            r#"{
                "default_level": "warn",
                "mods": {
                    "Highfleet-QOL": "trace",
                    "disabled-mod": "off"
                }
            }"#,
        )
        .unwrap();

        assert_eq!(config.level_for("highfleet-qol"), ModLogLevel::Trace);
        assert_eq!(config.level_for("DISABLED-MOD"), ModLogLevel::Off);
        assert_eq!(config.level_for("another-mod"), ModLogLevel::Warn);
    }

    #[test]
    fn missing_config_fields_use_defaults() {
        let config: LoggingConfig = serde_json::from_str("{}").unwrap();

        assert_eq!(config.default_level, ModLogLevel::Debug);
        assert!(config.mods.is_empty());
    }

    #[test]
    fn formats_mod_name_and_level_centrally() {
        let mut output = Vec::new();
        let mut now = DeferredNow::new();
        let arguments = format_args!("configuration loaded");
        let record = Record::builder()
            .args(arguments)
            .level(Level::Info)
            .target("highfleet_mod::highfleet-qol")
            .build();

        format_log(&mut output, &mut now, &record).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.ends_with("[highfleet-qol] INFO configuration loaded"));
    }
}
