//! Stable host API definitions and Rust adapters for Highfleet mods.
//!
//! The types in [`ffi`] are the only values that cross the dynamic-library
//! boundary. Rust mods can invoke [`export_logger!`] once and continue using
//! the standard macros from the [`log`] crate.

pub mod ffi;
mod logging;

pub use logging::{install_logger, InstallError};

/// Exports the version 1 host setup function and connects the [`log`] facade
/// in the mod to the modloader.
///
/// Invoke this macro exactly once in the root of a mod crate:
///
/// ```no_run
/// highfleet_mod_api::export_logger!();
///
/// fn initialize() {
///     log::info!("Logging through the Highfleet modloader");
/// }
/// ```
///
/// The generated export catches Rust panics so they cannot unwind into the
/// modloader. When an old modloader does not call the export, the mod still
/// loads normally but its [`log`] records are discarded.
#[macro_export]
macro_rules! export_logger {
    () => {
        /// Installs the logging callbacks supplied by the Highfleet modloader.
        ///
        /// # Safety
        ///
        /// `host` must point to a readable
        /// [`highfleet_mod_api::ffi::HfmHostApiV1`] for the duration of this
        /// call. The callback context and callback code must remain valid
        /// afterward, for as long as this mod can emit a log record.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn highfleet_mod_setup_v1(
            host: *const $crate::ffi::HfmHostApiV1,
        ) -> u32 {
            unsafe { $crate::__setup_logging_v1(host) }
        }
    };
}

/// Implements the exported setup function without exposing implementation
/// details through the macro expansion.
///
/// This is public because macros expanded in another crate must be able to
/// reach it. Mod code should use [`export_logger!`] instead.
#[doc(hidden)]
pub unsafe fn __setup_logging_v1(host: *const ffi::HfmHostApiV1) -> u32 {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    match catch_unwind(AssertUnwindSafe(|| unsafe { install_logger(host) })) {
        Ok(Ok(())) => ffi::HFM_STATUS_OK,
        Ok(Err(error)) => error.status_code(),
        Err(_) => ffi::HFM_STATUS_PANIC,
    }
}
