use highfleet_mod_api::{
    ffi::HfmStr,
    patching::{HfmPatchApiV1, HFM_PATCH_ABI_VERSION_1, HFM_PATCH_FAILED, HFM_PATCH_OK},
};
use std::{
    ffi::{c_char, CStr},
    mem::size_of,
    sync::atomic::{AtomicBool, Ordering},
};

static mut API: Option<HfmPatchApiV1> = None;
static mut TARGET: u8 = 7;
static SAW_UNKNOWN_AFTER_SETUP: AtomicBool = AtomicBool::new(false);

// The fixture uses these exports sequentially on a single loader thread.
#[unsafe(no_mangle)]
unsafe extern "C" fn highfleet_mod_setup_patchy_v1(api: *const HfmPatchApiV1) -> u32 {
    if api.is_null()
        || unsafe { (*api).abi_version } != HFM_PATCH_ABI_VERSION_1
        || unsafe { (*api).struct_size } < size_of::<HfmPatchApiV1>() as u32
    {
        return HFM_PATCH_FAILED;
    }
    unsafe { (&raw mut API).write(Some(api.read())) };
    HFM_PATCH_OK
}

/// Declines every version to show that a rejected version keeps the service.
#[unsafe(no_mangle)]
unsafe extern "C" fn version(version: *const c_char) -> bool {
    let set_up = unsafe { (&raw const API).read() }.is_some();
    let unknown = !version.is_null() && unsafe { CStr::from_ptr(version) }.to_bytes() == b"Unknown";
    SAW_UNKNOWN_AFTER_SETUP.store(set_up && unknown, Ordering::Relaxed);
    false
}

/// Patches only when the loader offered the service, then reported `Unknown`.
#[unsafe(no_mangle)]
unsafe extern "C" fn init() -> bool {
    if !SAW_UNKNOWN_AFTER_SETUP.load(Ordering::Relaxed) {
        return false;
    }
    let Some(api) = (unsafe { (&raw const API).read() }) else {
        return false;
    };
    let (Some(create), Some(discard), Some(overwrite), Some(install)) = (
        api.session_create,
        api.session_discard,
        api.session_overwrite,
        api.session_install,
    ) else {
        return false;
    };
    let mut session = 0;
    if unsafe { create(api.context, &mut session) } != HFM_PATCH_OK {
        return false;
    }
    let replacement = 42u8;
    let status = unsafe {
        overwrite(
            api.context,
            session,
            HfmStr::from("fixture overwrite"),
            (&raw mut TARGET) as usize,
            &replacement,
            1,
        )
    };
    if status != HFM_PATCH_OK {
        unsafe { discard(api.context, session) };
        return false;
    }
    unsafe { install(api.context, session) == HFM_PATCH_OK }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn patchy_test_target() -> u8 {
    unsafe { (&raw const TARGET).read_volatile() }
}
