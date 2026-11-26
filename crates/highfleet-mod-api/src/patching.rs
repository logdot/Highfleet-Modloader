//! Provisional C ABI for the loader-owned Patchy service.
//!
//! A mod exports `highfleet_mod_setup_patchy_v1` to receive the table after
//! logging setup and before `version` and `init`. It is offered regardless of
//! the detected game version or the mod's `version` answer.
//!
//! Setup returns [`HFM_PATCH_OK`] to accept the service; any other value
//! declines it and is logged by the loader. The table is borrowed during
//! setup and a mod can copy it when accepting. The loader, its context, and
//! any mod referenced by an installed patch must stay loaded until process
//! exit.

use crate::ffi::HfmStr;
use core::ffi::c_void;

pub const HFM_PATCH_ABI_VERSION_1: u32 = 1;
pub const HFM_PATCH_OK: u32 = 0;
pub const HFM_PATCH_INVALID_ARGUMENT: u32 = 1;
pub const HFM_PATCH_OVERLAP: u32 = 2;
pub const HFM_PATCH_SOURCE_CHANGED: u32 = 3;
/// Failure without a guarantee that no writes occurred. Do not blindly retry.
pub const HFM_PATCH_FAILED: u32 = 4;

/// A loader-issued session ID. Zero is never issued and IDs are never reused,
/// so a stale, unknown, or another mod's ID is rejected with
/// [`HFM_PATCH_INVALID_ARGUMENT`] without affecting any session.
pub type HfmPatchSessionId = u64;

pub type HfmPatchSetupV1 = unsafe extern "C" fn(*const HfmPatchApiV1) -> u32;
pub type HfmPatchSessionCreateV1 =
    unsafe extern "C" fn(context: *mut c_void, output: *mut HfmPatchSessionId) -> u32;
pub type HfmPatchSessionDiscardV1 =
    unsafe extern "C" fn(context: *mut c_void, session: HfmPatchSessionId) -> u32;
pub type HfmPatchSessionOverwriteV1 = unsafe extern "C" fn(
    context: *mut c_void,
    session: HfmPatchSessionId,
    name: HfmStr,
    address: usize,
    bytes: *const u8,
    len: usize,
) -> u32;
pub type HfmPatchSessionInstallV1 =
    unsafe extern "C" fn(context: *mut c_void, session: HfmPatchSessionId) -> u32;

/// An immutable byte range borrowed during a host call. Null is allowed only
/// for an empty range.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HfmPatchBytes {
    pub ptr: *const u8,
    pub len: usize,
}

impl From<&[u8]> for HfmPatchBytes {
    fn from(bytes: &[u8]) -> Self {
        Self {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
        }
    }
}

/// A process-lifetime capability bound to the receiving mod.
///
/// `context` must be the value from this table. Sessions belong to the context
/// that created them. Calls may come from any thread; the loader serializes
/// staging and installation through this service. An install consumes its
/// session before writing.
/// Requests borrow strings, descriptors, and bytes only for the duration of
/// the call. Keep that memory readable and unchanged until the call returns.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HfmPatchApiV1 {
    pub abi_version: u32,
    pub struct_size: u32,
    pub context: *mut c_void,
    /// On success writes a new nonzero session ID to a live, aligned, writable output.
    pub session_create: Option<HfmPatchSessionCreateV1>,
    /// Consumes a session without writing its pending patches.
    pub session_discard: Option<HfmPatchSessionDiscardV1>,
    /// Stages a write under a nonempty UTF-8 name. The source must remain
    /// readable through install and the replacement must be valid at the site.
    pub session_overwrite: Option<HfmPatchSessionOverwriteV1>,
    /// Consumes a session belonging to this context, even if install fails.
    /// No other thread may execute a patch source while it is being written.
    pub session_install: Option<HfmPatchSessionInstallV1>,
    /// Stages a trampoline and returns its address.
    pub session_detour: Option<HfmPatchSessionDetourV1>,
}

/// Copies the displaced source span verbatim. Relocating instructions would
/// require a different fixup kind, not a change in meaning of this one.
pub const HFM_PATCH_FIXUP_REPLAY: u32 = 1;
pub const HFM_PATCH_FIXUP_RELATIVE: u32 = 2;
pub const HFM_PATCH_FIXUP_RESUME: u32 = 3;

/// A fixup in a machine code template. `offset` is the start of a replay slot
/// or of a four-byte displacement field at the end of an instruction.
/// `argument` is the replay slot length, the relative target address, or zero
/// for resume, respectively. `reserved` must be zero.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HfmTrampolineFixupV1 {
    pub kind: u32,
    pub reserved: u32,
    pub offset: usize,
    pub argument: usize,
}

/// A borrowed trampoline plan with internal labels already resolved.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HfmTrampolinePlanV1 {
    /// Nonempty machine code template, at most one executable page.
    pub template: HfmPatchBytes,
    /// A live, aligned array of `fixup_count` records. Ignored when empty.
    pub fixups: *const HfmTrampolineFixupV1,
    pub fixup_count: usize,
}

/// Stages a detour. The source span must stay readable through installation,
/// contain whole instructions, and have no incoming branches into its
/// interior. Every trampoline exit must leave valid machine state. Replay
/// copies whole instructions verbatim and must be valid where placed.
/// Referenced code and data must stay live for any installed patch.
///
/// On success, writes the trampoline address to a live, aligned, writable
/// output pointer. Do not use that address unless installation succeeds.
pub type HfmPatchSessionDetourV1 = unsafe extern "C" fn(
    context: *mut c_void,
    session: HfmPatchSessionId,
    name: HfmStr,
    address: usize,
    size: usize,
    plan: *const HfmTrampolinePlanV1,
    output_trampoline: *mut usize,
) -> u32;

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn patch_table_has_a_versioned_c_prefix() {
        let pointer_size = size_of::<*const ()>();
        assert_eq!(offset_of!(HfmPatchApiV1, abi_version), 0);
        assert_eq!(offset_of!(HfmPatchApiV1, struct_size), 4);
        assert_eq!(offset_of!(HfmPatchApiV1, context), 8);
        assert_eq!(offset_of!(HfmPatchApiV1, session_create), 8 + pointer_size);
        assert_eq!(
            offset_of!(HfmPatchApiV1, session_detour),
            8 + pointer_size * 5
        );
        assert_eq!(size_of::<HfmPatchApiV1>(), 8 + pointer_size * 6);
    }

    #[test]
    fn trampoline_records_have_the_expected_c_layout() {
        let pointer_size = size_of::<usize>();
        assert_eq!(offset_of!(HfmTrampolineFixupV1, kind), 0);
        assert_eq!(offset_of!(HfmTrampolineFixupV1, reserved), 4);
        assert_eq!(offset_of!(HfmTrampolineFixupV1, offset), 8);
        assert_eq!(offset_of!(HfmTrampolineFixupV1, argument), 8 + pointer_size);
        assert_eq!(size_of::<HfmTrampolineFixupV1>(), 8 + 2 * pointer_size);
        assert_eq!(offset_of!(HfmTrampolinePlanV1, template), 0);
        assert_eq!(offset_of!(HfmTrampolinePlanV1, fixups), 2 * pointer_size);
        assert_eq!(
            offset_of!(HfmTrampolinePlanV1, fixup_count),
            3 * pointer_size
        );
        assert_eq!(size_of::<HfmTrampolinePlanV1>(), 4 * pointer_size);
    }
}
