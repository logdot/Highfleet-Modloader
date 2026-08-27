//! Raw, C-compatible host API definitions.
//!
//! These definitions intentionally contain no Rust references, slices, trait
//! objects, enums, strings, or ownership-bearing values.

use core::ffi::c_void;

/// Version implemented by [`HfmHostApiV1`].
pub const HFM_HOST_ABI_VERSION_1: u32 = 1;

/// Setup completed successfully.
pub const HFM_STATUS_OK: u32 = 0;
/// The host API pointer was null.
pub const HFM_STATUS_NULL_API: u32 = 1;
/// The host API uses an unsupported ABI version.
pub const HFM_STATUS_UNSUPPORTED_ABI: u32 = 2;
/// The supplied host API is smaller than the requested ABI version.
pub const HFM_STATUS_API_TOO_SMALL: u32 = 3;
/// A required callback was null.
pub const HFM_STATUS_MISSING_CALLBACK: u32 = 4;
/// The mod already installed a logger.
pub const HFM_STATUS_LOGGER_ALREADY_SET: u32 = 5;
/// Setup panicked before it could return normally.
pub const HFM_STATUS_PANIC: u32 = u32::MAX;

/// Error log level.
pub const HFM_LOG_LEVEL_ERROR: u32 = 1;
/// Warning log level.
pub const HFM_LOG_LEVEL_WARN: u32 = 2;
/// Informational log level.
pub const HFM_LOG_LEVEL_INFO: u32 = 3;
/// Debug log level.
pub const HFM_LOG_LEVEL_DEBUG: u32 = 4;
/// Trace log level.
pub const HFM_LOG_LEVEL_TRACE: u32 = 5;

/// A borrowed UTF-8 string passed across the host boundary.
///
/// The bytes are only valid for the duration of the callback receiving this
/// value. A null pointer is valid only when `len` is zero. The receiver must
/// not retain or free the pointer.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HfmStr {
    /// Pointer to the first UTF-8 byte.
    pub ptr: *const u8,
    /// Number of bytes, excluding any null terminator.
    pub len: usize,
}

impl HfmStr {
    /// Creates an empty string with no backing allocation.
    pub const fn empty() -> Self {
        Self {
            ptr: core::ptr::null(),
            len: 0,
        }
    }
}

impl From<&str> for HfmStr {
    /// Borrows a Rust string for use during an immediate FFI call.
    fn from(value: &str) -> Self {
        Self {
            ptr: value.as_ptr(),
            len: value.len(),
        }
    }
}

impl Default for HfmStr {
    fn default() -> Self {
        Self::empty()
    }
}

/// A single, already-formatted log record.
///
/// The mod owns every referenced string. All strings cease to be valid when
/// the `log_write` callback returns.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HfmLogRecordV1 {
    /// Size of this record in bytes, for forward-compatible extension.
    pub struct_size: u32,
    /// One of the `HFM_LOG_LEVEL_*` constants.
    pub level: u32,
    /// Original [`log`] target selected by the mod.
    pub target: HfmStr,
    /// Formatted log message.
    pub message: HfmStr,
    /// Rust module path, or an empty string when unavailable.
    pub module_path: HfmStr,
    /// Source filename, or an empty string when unavailable.
    pub file: HfmStr,
    /// One-based source line, or zero when unavailable.
    pub line: u32,
    /// Must be zero.
    pub reserved: u32,
}

/// Returns nonzero when a record at this level and target should be formatted.
pub type HfmLogEnabledV1 =
    unsafe extern "C" fn(context: *mut c_void, level: u32, target: HfmStr) -> u8;

/// Writes one log record synchronously.
///
/// The host must copy any data it needs before this function returns.
pub type HfmLogWriteV1 = unsafe extern "C" fn(context: *mut c_void, record: *const HfmLogRecordV1);

/// Flushes any buffered log output.
pub type HfmLogFlushV1 = unsafe extern "C" fn(context: *mut c_void);

/// Version 1 of the services supplied by the Highfleet modloader.
///
/// The mod copies this table during setup; the table itself does not need to
/// remain alive. `context`, and the code behind every callback, must remain
/// valid and safe to call from any mod thread for as long as the mod can log.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HfmHostApiV1 {
    /// Must be [`HFM_HOST_ABI_VERSION_1`].
    pub abi_version: u32,
    /// Size of this table in bytes.
    pub struct_size: u32,
    /// Opaque, loader-owned context identifying this mod.
    pub context: *mut c_void,
    /// Optional filter callback. When null, all levels are considered enabled.
    pub log_enabled: Option<HfmLogEnabledV1>,
    /// Required record callback.
    pub log_write: Option<HfmLogWriteV1>,
    /// Optional flush callback.
    pub log_flush: Option<HfmLogFlushV1>,
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, offset_of, size_of};

    use super::*;

    #[test]
    fn ffi_layout_is_pointer_width_independent() {
        let pointer_size = size_of::<*const ()>();

        assert_eq!(offset_of!(HfmStr, ptr), 0);
        assert_eq!(offset_of!(HfmStr, len), pointer_size);
        assert_eq!(size_of::<HfmStr>(), pointer_size * 2);
        assert_eq!(align_of::<HfmStr>(), pointer_size);

        assert_eq!(offset_of!(HfmLogRecordV1, struct_size), 0);
        assert_eq!(offset_of!(HfmLogRecordV1, level), 4);
        assert_eq!(offset_of!(HfmLogRecordV1, target), 8);
        assert_eq!(offset_of!(HfmLogRecordV1, message), 8 + pointer_size * 2);
        assert_eq!(
            offset_of!(HfmLogRecordV1, module_path),
            8 + pointer_size * 4
        );
        assert_eq!(offset_of!(HfmLogRecordV1, file), 8 + pointer_size * 6);
        assert_eq!(offset_of!(HfmLogRecordV1, line), 8 + pointer_size * 8);
        assert_eq!(size_of::<HfmLogRecordV1>(), 16 + pointer_size * 8);

        assert_eq!(offset_of!(HfmHostApiV1, abi_version), 0);
        assert_eq!(offset_of!(HfmHostApiV1, struct_size), 4);
        assert_eq!(offset_of!(HfmHostApiV1, context), 8);
        assert_eq!(offset_of!(HfmHostApiV1, log_enabled), 8 + pointer_size);
        assert_eq!(offset_of!(HfmHostApiV1, log_write), 8 + pointer_size * 2);
        assert_eq!(offset_of!(HfmHostApiV1, log_flush), 8 + pointer_size * 3);
        assert_eq!(size_of::<HfmHostApiV1>(), 8 + pointer_size * 4);
    }

    #[test]
    fn optional_callbacks_have_nullable_pointer_layout() {
        assert_eq!(
            size_of::<Option<HfmLogEnabledV1>>(),
            size_of::<HfmLogEnabledV1>()
        );
        assert_eq!(
            size_of::<Option<HfmLogWriteV1>>(),
            size_of::<HfmLogWriteV1>()
        );
        assert_eq!(
            size_of::<Option<HfmLogFlushV1>>(),
            size_of::<HfmLogFlushV1>()
        );
    }
}
