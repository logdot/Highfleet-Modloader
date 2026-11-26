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

pub const HFM_PATCH_STACK_ALIGNED_16: u32 = 0;
pub const HFM_PATCH_STACK_OFFSET_8: u32 = 1;
pub const HFM_PATCH_FLAGS_PRESERVE: u32 = 0;
pub const HFM_PATCH_FLAGS_KEEP_PRODUCED: u32 = 1;
pub const HFM_PATCH_RETURN_NEITHER: u32 = 0;
pub const HFM_PATCH_RETURN_RAX: u32 = 1;
pub const HFM_PATCH_RETURN_XMM0: u32 = 2;

/// An immutable byte range borrowed during a host call. Null is allowed only
/// for an empty range.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HfmPatchBytes {
    pub ptr: *const u8,
    pub len: usize,
}

impl HfmPatchBytes {
    pub const EMPTY: Self = Self {
        ptr: core::ptr::null(),
        len: 0,
    };
}

impl From<&[u8]> for HfmPatchBytes {
    fn from(bytes: &[u8]) -> Self {
        Self {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
        }
    }
}

/// A Windows x64 callback bridge. The callback must not take stack-passed
/// arguments or unwind out of the bridge, and must remain live through
/// process exit if installed. The direction flag must be clear on entry.
///
/// `stack_alignment` describes RSP at the patch site. The snippets are
/// verbatim, position-independent, straight-line instructions that must fall
/// through with RSP unchanged without modifying the bridge's saved state.
/// `after_call` runs before register restoration.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HfmCallBridgeV1 {
    pub callback: usize,
    pub stack_alignment: u32,
    pub flags: u32,
    pub keep_return: u32,
    pub before_call: HfmPatchBytes,
    pub after_call: HfmPatchBytes,
}

/// Stages a callback and a jump to `address + size`, without replaying the
/// replaced instructions. The source span must remain readable through
/// install and contain complete instructions with no incoming branches into
/// its interior. On success, writes the trampoline address to the supplied
/// live, aligned, writable output pointer. Do not use the trampoline address
/// unless and until session installation succeeds.
pub type HfmPatchSessionPatchCallV1 = unsafe extern "C" fn(
    context: *mut c_void,
    session: HfmPatchSessionId,
    name: HfmStr,
    address: usize,
    size: usize,
    bridge: *const HfmCallBridgeV1,
    output_trampoline: *mut usize,
) -> u32;

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
}

/// The optional callback extension to the v1 overwrite service.
///
/// Read the extension only if the prefix's `struct_size` is large enough.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HfmPatchApiWithCallV1 {
    pub base: HfmPatchApiV1,
    /// Stages a call bridge, then resumes after the replaced span.
    pub session_patch_call: Option<HfmPatchSessionPatchCallV1>,
}

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
        assert_eq!(size_of::<HfmPatchApiV1>(), 8 + pointer_size * 5);
        assert_eq!(offset_of!(HfmPatchApiWithCallV1, base), 0);
        assert_eq!(
            offset_of!(HfmPatchApiWithCallV1, session_patch_call),
            size_of::<HfmPatchApiV1>()
        );
    }
}
