use super::*;
use highfleet_mod_api::ffi::HfmStr;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

#[derive(Default)]
struct Calls {
    stage_count: AtomicU32,
    patch_call_count: AtomicU32,
    install_count: AtomicU32,
    discard_count: AtomicU32,
    install_status: AtomicU32,
}

fn table(calls: &'static Calls) -> HfmPatchApiV1 {
    HfmPatchApiV1 {
        abi_version: HFM_PATCH_ABI_VERSION_1,
        struct_size: size_of::<HfmPatchApiV1>() as u32,
        context: std::ptr::from_ref(calls).cast_mut().cast(),
        session_create: Some(create),
        session_discard: Some(discard),
        session_overwrite: Some(overwrite),
        session_install: Some(install),
    }
}

unsafe extern "C" fn create(_: *mut c_void, output: *mut HfmPatchSessionId) -> u32 {
    unsafe { output.write(12) };
    HFM_PATCH_OK
}

unsafe extern "C" fn discard(context: *mut c_void, id: HfmPatchSessionId) -> u32 {
    if id != 12 {
        return HFM_PATCH_INVALID_ARGUMENT;
    }
    unsafe { &*context.cast::<Calls>() }
        .discard_count
        .fetch_add(1, Relaxed);
    HFM_PATCH_OK
}

unsafe extern "C" fn overwrite(
    context: *mut c_void,
    id: HfmPatchSessionId,
    name: HfmStr,
    address: usize,
    bytes: *const u8,
    len: usize,
) -> u32 {
    if id != 12
        || address != 0x1000
        || unsafe { std::slice::from_raw_parts(name.ptr, name.len) } != b"overwrite"
        || unsafe { std::slice::from_raw_parts(bytes, len) } != [42]
    {
        return HFM_PATCH_INVALID_ARGUMENT;
    }
    unsafe { &*context.cast::<Calls>() }
        .stage_count
        .fetch_add(1, Relaxed);
    HFM_PATCH_OK
}

unsafe extern "C" fn install(context: *mut c_void, id: HfmPatchSessionId) -> u32 {
    if id != 12 {
        return HFM_PATCH_INVALID_ARGUMENT;
    }
    let calls = unsafe { &*context.cast::<Calls>() };
    calls.install_count.fetch_add(1, Relaxed);
    calls.install_status.load(Relaxed)
}

extern "C" fn callback() {}

unsafe extern "C" fn patch_call(
    context: *mut c_void,
    id: HfmPatchSessionId,
    name: HfmStr,
    address: usize,
    size: usize,
    bridge: *const HfmCallBridgeV1,
    output: *mut usize,
) -> u32 {
    let bridge = unsafe { &*bridge };
    if id != 12
        || address != 0x1000
        || size != 5
        || unsafe { std::slice::from_raw_parts(name.ptr, name.len) } != b"patch_call"
        || bridge.callback != callback as *const () as usize
        || bridge.stack_alignment != HFM_PATCH_STACK_OFFSET_8
        || bridge.flags != HFM_PATCH_FLAGS_KEEP_PRODUCED
        || bridge.keep_return != HFM_PATCH_RETURN_RAX
        || bridge.before_call.len != 1
        || unsafe { bridge.before_call.ptr.read() } != 0x90
        || bridge.after_call.len != 0
    {
        return HFM_PATCH_INVALID_ARGUMENT;
    }
    unsafe { &*context.cast::<Calls>() }
        .patch_call_count
        .fetch_add(1, Relaxed);
    unsafe { output.write(0x2000) };
    HFM_PATCH_OK
}

fn mock() -> (&'static Calls, Host) {
    let calls = Box::leak(Box::new(Calls::default()));
    let api = HfmPatchApiWithCallV1 {
        base: HfmPatchApiV1 {
            struct_size: size_of::<HfmPatchApiWithCallV1>() as u32,
            ..table(calls)
        },
        session_patch_call: Some(patch_call),
    };
    let host = unsafe { Host::from_table(std::ptr::from_ref(&api).cast()) }.unwrap();
    (calls, host)
}

#[test]
fn new_without_setup_reports_the_missing_service() {
    assert!(matches!(PatchSession::new(), Err(PatchError::MissingHost)));
}

#[test]
fn an_incompatible_table_version_is_rejected() {
    let mut api = table(Box::leak(Box::new(Calls::default())));
    api.abi_version += 1;
    let slot = OnceLock::new();
    assert_eq!(unsafe { register_host(&slot, &api) }, HFM_PATCH_FAILED);
    assert!(matches!(
        PatchSession::from_slot(&slot),
        Err(PatchError::UnsupportedAbi { .. })
    ));
}

#[test]
fn setup_keeps_a_copy_of_the_host_table() {
    let calls = Box::leak(Box::new(Calls::default()));
    let slot = OnceLock::new();
    let mut api = table(calls);
    assert_eq!(unsafe { register_host(&slot, &api) }, HFM_PATCH_OK);
    api.session_create = None;
    let session = PatchSession::from_slot(&slot).unwrap();
    unsafe { session.install_permanently() }.unwrap();
    assert!(api.session_create.is_none());
    assert_eq!(calls.install_count.load(Relaxed), 1);
    assert_eq!(calls.discard_count.load(Relaxed), 0);
}

#[test]
fn a_header_only_table_is_rejected_as_too_small() {
    #[repr(C, align(8))]
    struct Header([u32; 2]);
    let header = Header([HFM_PATCH_ABI_VERSION_1, 8]);
    assert!(matches!(
        unsafe { Host::from_table(std::ptr::from_ref(&header).cast()) },
        Err(PatchError::TableTooSmall { .. })
    ));
}

#[test]
fn a_missing_callback_is_rejected() {
    let mut api = table(Box::leak(Box::new(Calls::default())));
    api.session_install = None;
    assert!(matches!(
        unsafe { Host::from_table(&api) },
        Err(PatchError::MissingCallback("session_install"))
    ));
}

#[test]
fn overwrite_forwards_the_request_to_its_host() {
    let (calls, host) = mock();
    let mut session = PatchSession::from_host(host).unwrap();

    let patch = unsafe { session.overwrite(0x1000, &[42]) }.unwrap();

    assert_eq!(calls.stage_count.load(Relaxed), 1);
    assert_eq!(patch.trampoline_address(), None);
}

#[test]
fn patch_call_forwards_bridge_options_and_returns_the_trampoline_address() {
    let (calls, host) = mock();
    let mut session = PatchSession::from_host(host).unwrap();
    let patch = unsafe {
        session.patch_call(
            0x1000,
            5,
            CallBridge {
                callback: callback as *const (),
                incoming_stack: StackAlignment::Offset8,
                flags: FlagPolicy::KeepProduced,
                keep_return: KeepReturn::Rax,
                before_call: &[0x90],
                after_call: &[],
            },
        )
    }
    .unwrap();
    assert_eq!(patch.trampoline_address(), Some(0x2000));
    assert_eq!(calls.patch_call_count.load(Relaxed), 1);
}

#[test]
fn an_overwrite_only_table_can_still_create_a_session() {
    let calls = Box::leak(Box::new(Calls::default()));
    let host = unsafe { Host::from_table(&table(calls)) }.unwrap();
    let session = PatchSession::from_host(host).unwrap();
    assert!(session.host.patch_call.is_none());
    drop(session);
    assert_eq!(calls.discard_count.load(Relaxed), 1);
}

#[test]
fn dropping_a_pending_session_discards_it_without_installing() {
    let (calls, host) = mock();
    drop(PatchSession::from_host(host).unwrap());
    assert_eq!(calls.discard_count.load(Relaxed), 1);
    assert_eq!(calls.install_count.load(Relaxed), 0);
}

#[test]
fn installation_failure_does_not_discard_the_consumed_handle_again() {
    let (calls, host) = mock();
    calls.install_status.store(HFM_PATCH_OVERLAP, Relaxed);
    let session = PatchSession::from_host(host).unwrap();
    assert!(matches!(
        unsafe { session.install_permanently() },
        Err(PatchError::Overlap)
    ));
    assert_eq!(calls.install_count.load(Relaxed), 1);
    assert_eq!(calls.discard_count.load(Relaxed), 0);
}
