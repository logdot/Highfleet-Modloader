//! Rust client for the Modloader's provisional shared Patchy service.
//!
//! Alias this package as `patchy` in the mod's manifest and invoke
//! [`export_patchy!`] once. This crate never links the local patch engine.

use highfleet_mod_api::patching::{
    HfmPatchApiV1, HfmPatchSessionCreateV1, HfmPatchSessionDetourV1, HfmPatchSessionDiscardV1,
    HfmPatchSessionId, HfmPatchSessionInstallV1, HfmPatchSessionOverwriteV1, HfmTrampolineFixupV1,
    HfmTrampolinePlanV1, HFM_PATCH_ABI_VERSION_1, HFM_PATCH_FAILED, HFM_PATCH_FIXUP_RELATIVE,
    HFM_PATCH_FIXUP_REPLAY, HFM_PATCH_FIXUP_RESUME, HFM_PATCH_INVALID_ARGUMENT, HFM_PATCH_OK,
    HFM_PATCH_OVERLAP, HFM_PATCH_SOURCE_CHANGED,
};
pub use patchy_code::{
    CallBridge, Condition, DetourSite, FlagPolicy, KeepReturn, Label, StackAlignment, Trampoline,
};
use patchy_code::{TrampolineFixup, TrampolinePlan};
use std::{ffi::c_void, mem::size_of, num::NonZeroU64, sync::OnceLock};

mod process_module;
pub use process_module::ProcessModule;

pub type Result<T = ()> = std::result::Result<T, PatchError>;

#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PatchError {
    #[error("the Modloader did not supply the shared Patchy service")]
    MissingHost,
    #[error("the Modloader supplied a null Patchy table")]
    NullTable,
    #[error("unsupported Modloader Patchy ABI version {version}")]
    UnsupportedAbi { version: u32 },
    #[error("Modloader Patchy table is too small ({size} bytes; need {minimum})")]
    TableTooSmall { size: u32, minimum: u32 },
    #[error("the Modloader supplied a null Patchy context")]
    MissingContext,
    #[error("the Modloader Patchy table is missing {0}")]
    MissingCallback(&'static str),
    #[error("the Modloader returned an invalid Patchy session ID")]
    InvalidSession,
    #[error("the Modloader did not return a trampoline address for the staged detour")]
    MissingTrampoline,
    #[error("the Modloader rejected invalid Patchy arguments")]
    InvalidArgument,
    #[error("a patch overlaps another patch; see the Modloader log")]
    Overlap,
    #[error("patch source bytes changed after preparation; see the Modloader log")]
    SourceChanged,
    #[error("the Modloader Patchy operation failed (status {0}); if installing, writes may have occurred")]
    HostFailure(u32),
    #[error(transparent)]
    Builder(std::sync::Arc<patchy_code::PatchError>),
}

impl From<patchy_code::PatchError> for PatchError {
    fn from(error: patchy_code::PatchError) -> Self {
        Self::Builder(error.into())
    }
}

#[derive(Clone, Copy)]
struct Host {
    context: *mut c_void,
    create: HfmPatchSessionCreateV1,
    discard: HfmPatchSessionDiscardV1,
    overwrite: HfmPatchSessionOverwriteV1,
    install: HfmPatchSessionInstallV1,
    detour: HfmPatchSessionDetourV1,
}

// SAFETY: The host ABI keeps the opaque context and callbacks live for the
// process and supports calls from any thread. The client never dereferences it.
unsafe impl Send for Host {}
// SAFETY: As above, sharing the table only shares thread-safe host callbacks.
unsafe impl Sync for Host {}

impl Host {
    /// A non-null pointer must provide two readable, aligned `u32` fields.
    /// If they report a supported version and sufficient size, the pointer
    /// must be aligned and valid for a full `HfmPatchApiV1`, and its context
    /// and callbacks must satisfy the host ABI for process lifetime.
    unsafe fn from_table(table: *const HfmPatchApiV1) -> Result<Self> {
        if table.is_null() {
            return Err(PatchError::NullTable);
        }
        let version = unsafe { (&raw const (*table).abi_version).read() };
        if version != HFM_PATCH_ABI_VERSION_1 {
            return Err(PatchError::UnsupportedAbi { version });
        }
        let size = unsafe { (&raw const (*table).struct_size).read() };
        let minimum = size_of::<HfmPatchApiV1>() as u32;
        if size < minimum {
            return Err(PatchError::TableTooSmall { size, minimum });
        }
        let api = unsafe { table.read() };
        if api.context.is_null() {
            return Err(PatchError::MissingContext);
        }
        Ok(Self {
            context: api.context,
            create: api
                .session_create
                .ok_or(PatchError::MissingCallback("session_create"))?,
            discard: api
                .session_discard
                .ok_or(PatchError::MissingCallback("session_discard"))?,
            overwrite: api
                .session_overwrite
                .ok_or(PatchError::MissingCallback("session_overwrite"))?,
            install: api
                .session_install
                .ok_or(PatchError::MissingCallback("session_install"))?,
            detour: api
                .session_detour
                .ok_or(PatchError::MissingCallback("session_detour"))?,
        })
    }
}

static HOST: OnceLock<Result<Host>> = OnceLock::new();

#[doc(hidden)]
pub use highfleet_mod_api::patching::HfmPatchApiV1 as __HostTable;

/// Exports the shared Patchy setup hook. Invoke once at the mod crate root.
#[macro_export]
macro_rules! export_patchy {
    () => {
        /// Receives the Modloader's shared Patchy capability.
        ///
        /// # Safety
        /// The pointer and callbacks must satisfy the Modloader's Patchy setup ABI.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn highfleet_mod_setup_patchy_v1(
            table: *const $crate::__HostTable,
        ) -> u32 {
            unsafe { $crate::__setup_patchy(table) }
        }
    };
}

#[doc(hidden)]
/// # Safety
/// The pointer and callbacks must satisfy the Modloader's Patchy setup ABI.
pub unsafe fn __setup_patchy(table: *const HfmPatchApiV1) -> u32 {
    register_host(&HOST, table)
}

// The caller upholds Host::from_table's contract.
unsafe fn register_host(slot: &OnceLock<Result<Host>>, table: *const HfmPatchApiV1) -> u32 {
    let host = unsafe { Host::from_table(table) };
    let accepted = host.is_ok();
    if slot.set(host).is_ok() && accepted {
        HFM_PATCH_OK
    } else {
        HFM_PATCH_FAILED
    }
}

fn check_status(status: u32) -> Result {
    match status {
        HFM_PATCH_OK => Ok(()),
        HFM_PATCH_INVALID_ARGUMENT => Err(PatchError::InvalidArgument),
        HFM_PATCH_OVERLAP => Err(PatchError::Overlap),
        HFM_PATCH_SOURCE_CHANGED => Err(PatchError::SourceChanged),
        other => Err(PatchError::HostFailure(other)),
    }
}

fn encode_fixups(plan: &TrampolinePlan) -> Vec<HfmTrampolineFixupV1> {
    plan.parts()
        .1
        .iter()
        .map(|&fixup| {
            let (kind, offset, argument) = match fixup {
                TrampolineFixup::Replay { offset, size } => (HFM_PATCH_FIXUP_REPLAY, offset, size),
                TrampolineFixup::Relative {
                    offset,
                    destination,
                } => (HFM_PATCH_FIXUP_RELATIVE, offset, destination),
                TrampolineFixup::Resume { offset } => (HFM_PATCH_FIXUP_RESUME, offset, 0),
            };
            HfmTrampolineFixupV1 {
                kind,
                reserved: 0,
                offset,
                argument,
            }
        })
        .collect()
}

/// A pending batch, owned by the Modloader. Dropping it discards staged work.
pub struct PatchSession {
    host: Host,
    id: Option<NonZeroU64>,
}

impl PatchSession {
    /// Creates a session once the loader has called the patch setup export.
    /// Returns an error if the host is absent or incompatible.
    pub fn new() -> Result<Self> {
        Self::from_slot(&HOST)
    }

    fn from_slot(slot: &OnceLock<Result<Host>>) -> Result<Self> {
        match slot.get() {
            Some(Ok(host)) => Self::from_host(*host),
            Some(Err(error)) => Err(error.clone()),
            None => Err(PatchError::MissingHost),
        }
    }

    fn from_host(host: Host) -> Result<Self> {
        let mut id: HfmPatchSessionId = 0;
        check_status(unsafe { (host.create)(host.context, &mut id) })?;
        let id = NonZeroU64::new(id).ok_or(PatchError::InvalidSession)?;
        Ok(Self { host, id: Some(id) })
    }

    /// Stages an in-place overwrite without installing it.
    ///
    /// # Safety
    /// The destination range must be readable through installation and valid
    /// to overwrite. If executed, `bytes` must be valid code at that address.
    pub unsafe fn overwrite(&mut self, address: usize, bytes: &[u8]) -> Result<Patch> {
        unsafe { self.overwrite_named("overwrite", address, bytes) }
    }

    /// Like [`Self::overwrite`], with a name for the loader's diagnostics.
    ///
    /// # Safety
    /// The range and replacement must satisfy [`Self::overwrite`].
    pub unsafe fn overwrite_named(
        &mut self,
        name: &str,
        address: usize,
        bytes: &[u8],
    ) -> Result<Patch> {
        let id = self.id.expect("live session").get();
        check_status(unsafe {
            (self.host.overwrite)(
                self.host.context,
                id,
                name.into(),
                address,
                bytes.as_ptr(),
                bytes.len(),
            )
        })?;
        Ok(Patch { trampoline: None })
    }

    /// Stages a callback followed by a jump to `address + size`, without
    /// replaying the replaced instructions.
    ///
    /// # Safety
    /// The span must stay readable through install, contain whole instructions,
    /// and have no incoming jumps into its interior. The callback must use the
    /// Windows x64 ABI without stack-passed arguments, must not unwind out of
    /// the bridge, and must stay live for the process if installed. The
    /// declared alignment must match RSP at the site, and the direction flag
    /// must be clear. Both snippets must be position-independent, straight-line
    /// code that falls through without changing RSP or the bridge's saved
    /// state and is valid for the machine state in which it runs.
    pub unsafe fn patch_call(
        &mut self,
        address: usize,
        size: usize,
        bridge: CallBridge<'_>,
    ) -> Result<Patch> {
        if bridge.callback.is_null() {
            return Err(PatchError::InvalidArgument);
        }
        unsafe {
            self.stage_detour("patch_call", address, size, |site| {
                let mut code = Trampoline::new();
                code.call(bridge);
                site.resume(&mut code);
                Ok(code)
            })
        }
    }

    /// Builds a custom trampoline locally, then sends the compiled plan to
    /// the loader. The host captures the source bytes and resolves placement.
    /// No closure crosses the ABI.
    ///
    /// # Safety
    /// The source span must remain readable through installation, consist of
    /// whole instructions, and have no incoming branches into its interior.
    /// The generated code must leave valid machine state on every exit.
    /// Any replay must satisfy [`DetourSite::replay_verbatim`], and referenced
    /// callbacks and data must stay live through process exit if installed.
    pub unsafe fn detour_trampoline<F>(
        &mut self,
        address: usize,
        size: usize,
        build: F,
    ) -> Result<Patch>
    where
        F: FnOnce(DetourSite) -> Result<Trampoline>,
    {
        unsafe { self.stage_detour("detour", address, size, build) }
    }

    unsafe fn stage_detour(
        &mut self,
        name: &str,
        address: usize,
        size: usize,
        build: impl FnOnce(DetourSite) -> Result<Trampoline>,
    ) -> Result<Patch> {
        if address == 0 || size < 5 || address.checked_add(size).is_none() {
            return Err(PatchError::InvalidArgument);
        }
        let plan = build(DetourSite::new(size))?.into_plan()?;
        let fixups = encode_fixups(&plan);
        let request = HfmTrampolinePlanV1 {
            template: plan.parts().0.into(),
            fixups: fixups.as_ptr(),
            fixup_count: fixups.len(),
        };
        let id = self.id.expect("live session").get();
        let mut output = 0;
        check_status(unsafe {
            (self.host.detour)(
                self.host.context,
                id,
                name.into(),
                address,
                size,
                &request,
                &mut output,
            )
        })?;
        Patch::from_trampoline(output)
    }

    /// Installs every staged patch. The host consumes the session even on error.
    ///
    /// # Safety
    /// All staged source ranges must remain valid, and no other thread may
    /// execute a patch source while it is being written.
    pub unsafe fn install_permanently(mut self) -> Result {
        let id = self.id.take().expect("live session").get();
        check_status(unsafe { (self.host.install)(self.host.context, id) })
    }
}

impl Drop for PatchSession {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            // SAFETY: This is a live session issued by the same host.
            unsafe { (self.host.discard)(self.host.context, id.get()) };
        }
    }
}

/// Describes a prepared patch without owning the loader's session.
pub struct Patch {
    trampoline: Option<usize>,
}

impl Patch {
    fn from_trampoline(address: usize) -> Result<Self> {
        let trampoline = (address != 0)
            .then_some(address)
            .ok_or(PatchError::MissingTrampoline)?;
        Ok(Self {
            trampoline: Some(trampoline),
        })
    }

    /// Returns the trampoline address if the patch has one.
    ///
    /// Do not use the address unless and until session installation succeeds.
    pub fn trampoline_address(&self) -> Option<usize> {
        self.trampoline
    }
}

#[cfg(test)]
mod tests;
