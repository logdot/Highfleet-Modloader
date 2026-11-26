//! Loader-owned sessions backed by the single Patchy engine in this process.
//!
//! Mods refer to sessions by loader-issued IDs, so a stale or forged ID is a
//! failed lookup rather than a dangling pointer. The engine gate serializes
//! Patchy's source reads during staging with its writes during installation.
//! Installation does not hold the session registry lock.

use crate::session_registry::SessionRegistry;
use highfleet_mod_api::{
    ffi::HfmStr,
    patching::{
        HfmPatchApiV1, HfmPatchSessionId, HfmPatchSetupV1, HFM_PATCH_ABI_VERSION_1,
        HFM_PATCH_FAILED, HFM_PATCH_INVALID_ARGUMENT, HFM_PATCH_OK, HFM_PATCH_OVERLAP,
        HFM_PATCH_SOURCE_CHANGED,
    },
};
use libloading::Library;
use log::{debug, error, warn};
use patchy::{PatchError, PatchSession};
use std::{
    ffi::c_void,
    mem::size_of,
    panic::{catch_unwind, AssertUnwindSafe},
    path::{Path, PathBuf},
    slice, str,
    sync::{Mutex, MutexGuard, PoisonError},
};

struct ModContext {
    path: PathBuf,
}

struct Session {
    owner: &'static ModContext,
    inner: PatchSession,
}

static SESSIONS: Mutex<SessionRegistry<Session>> = Mutex::new(SessionRegistry::new());
static SOURCE_ACCESS: Mutex<()> = Mutex::new(());

fn sessions() -> MutexGuard<'static, SessionRegistry<Session>> {
    SESSIONS.lock().unwrap_or_else(PoisonError::into_inner)
}

fn source_access() -> MutexGuard<'static, ()> {
    SOURCE_ACCESS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Sessions are owned by the address of the context they were created with;
/// the address is compared, never dereferenced.
fn owner_key(context: *mut c_void) -> usize {
    context as usize
}

/// Offers the capability to opt-in mods. An absent export is normal.
///
/// # Safety
/// The mod library must remain loaded for as long as a patch can refer to it.
pub unsafe fn setup_mod_patching(library: &Library, path: &Path) {
    let Ok(setup) = (unsafe { library.get::<HfmPatchSetupV1>(b"highfleet_mod_setup_patchy_v1") })
    else {
        return;
    };

    let status = unsafe { offer_patchy(path, *setup) };
    if status != HFM_PATCH_OK {
        warn!(
            "Mod {} declined the Patchy service (status {status})",
            path.display()
        );
    }
}

/// # Safety
/// The setup callback must accept the v1 table and must not unwind.
pub(crate) unsafe fn offer_patchy(path: &Path, setup: HfmPatchSetupV1) -> u32 {
    unsafe { setup(&patch_api(path)) }
}

/// Creates the table for one mod. Its context lives for the process because
/// the mod may keep the table.
fn patch_api(path: &Path) -> HfmPatchApiV1 {
    let context = Box::leak(Box::new(ModContext { path: path.into() }));
    HfmPatchApiV1 {
        abi_version: HFM_PATCH_ABI_VERSION_1,
        struct_size: size_of::<HfmPatchApiV1>() as u32,
        context: (context as *mut ModContext).cast(),
        session_create: Some(session_create),
        session_discard: Some(session_discard),
        session_overwrite: Some(session_overwrite),
        session_install: Some(session_install),
    }
}

fn run_ffi(operation: impl FnOnce() -> Result<(), u32>) -> u32 {
    match catch_unwind(AssertUnwindSafe(operation)) {
        Ok(Ok(())) => HFM_PATCH_OK,
        Ok(Err(status)) => status,
        Err(_) => HFM_PATCH_FAILED,
    }
}

fn patch_status(owner: &ModContext, operation: &str, err: PatchError) -> u32 {
    error!("Mod {}: {operation}: {err}", owner.path.display());
    match err {
        PatchError::OverlappingPatch { .. } => HFM_PATCH_OVERLAP,
        PatchError::SourceChanged { .. } => HFM_PATCH_SOURCE_CHANGED,
        _ => HFM_PATCH_FAILED,
    }
}

/// Removes a session so it is consumed whatever happens next. Another
/// context's ID is not found and therefore not consumed.
fn take_session(context: *mut c_void, id: HfmPatchSessionId) -> Result<Session, u32> {
    sessions()
        .take(owner_key(context), id)
        .ok_or(HFM_PATCH_INVALID_ARGUMENT)
}

unsafe extern "C" fn session_create(context: *mut c_void, output: *mut HfmPatchSessionId) -> u32 {
    run_ffi(|| {
        if output.is_null() {
            return Err(HFM_PATCH_INVALID_ARGUMENT);
        }
        // The table contract requires `context` to be the one it supplied.
        let owner =
            unsafe { context.cast::<ModContext>().as_ref() }.ok_or(HFM_PATCH_INVALID_ARGUMENT)?;
        let session = Session {
            owner,
            inner: PatchSession::new(),
        };
        let Some(id) = sessions().insert(owner_key(context), session) else {
            error!(
                "Mod {}: Patchy session IDs are exhausted",
                owner.path.display()
            );
            return Err(HFM_PATCH_FAILED);
        };
        unsafe { output.write(id) };
        Ok(())
    })
}

unsafe extern "C" fn session_discard(context: *mut c_void, session: HfmPatchSessionId) -> u32 {
    run_ffi(|| take_session(context, session).map(drop))
}

unsafe fn request_bytes<'a>(bytes: *const u8, len: usize) -> Result<&'a [u8], u32> {
    if bytes.is_null() || len == 0 || len > isize::MAX as usize {
        return Err(HFM_PATCH_INVALID_ARGUMENT);
    }
    Ok(unsafe { slice::from_raw_parts(bytes, len) })
}

unsafe extern "C" fn session_overwrite(
    context: *mut c_void,
    session: HfmPatchSessionId,
    name: HfmStr,
    address: usize,
    bytes: *const u8,
    len: usize,
) -> u32 {
    run_ffi(|| {
        let name = str::from_utf8(unsafe { request_bytes(name.ptr, name.len)? })
            .map_err(|_| HFM_PATCH_INVALID_ARGUMENT)?;
        let replacement = unsafe { request_bytes(bytes, len)? };
        if address == 0 || address.checked_add(len).is_none() {
            return Err(HFM_PATCH_INVALID_ARGUMENT);
        }

        let (owner, staged) = {
            // Both staging and installation touch live process memory. Lock
            // source access before the registry if both are needed.
            let _source = source_access();
            let mut sessions = sessions();
            let session = sessions
                .get_mut(owner_key(context), session)
                .ok_or(HFM_PATCH_INVALID_ARGUMENT)?;
            (session.owner, unsafe {
                session.inner.overwrite(address, replacement)
            })
        };
        staged.map_err(|err| patch_status(owner, name, err))?;
        debug!(
            "Mod {} prepared {name} at {address:#x}",
            owner.path.display()
        );
        Ok(())
    })
}

unsafe extern "C" fn session_install(context: *mut c_void, session: HfmPatchSessionId) -> u32 {
    run_ffi(|| {
        let Session { owner, inner } = take_session(context, session)?;
        let installed = {
            let _source = source_access();
            unsafe { inner.install_permanently() }
        };
        installed.map_err(|err| patch_status(owner, "install", err))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    const ORIGINAL: u8 = 7;
    const REPLACEMENT: u8 = 42;

    /// The table one mod would receive during setup.
    struct ModService(HfmPatchApiV1);

    impl ModService {
        fn new() -> Self {
            Self(patch_api(Path::new("test-mod.dll")))
        }

        fn create(&self) -> HfmPatchSessionId {
            let mut session = 0;
            // SAFETY: The table came from this service and the output is live.
            let status = unsafe { self.0.session_create.unwrap()(self.0.context, &mut session) };
            assert_eq!(status, HFM_PATCH_OK);
            session
        }

        fn overwrite(
            &self,
            session: HfmPatchSessionId,
            name: HfmStr,
            address: usize,
            bytes: *const u8,
            len: usize,
        ) -> u32 {
            // SAFETY: Every test address is either rejected before use or is
            // readable and writable memory leaked for the process lifetime.
            unsafe {
                self.0.session_overwrite.unwrap()(
                    self.0.context,
                    session,
                    name,
                    address,
                    bytes,
                    len,
                )
            }
        }

        fn overwrite_byte(&self, session: HfmPatchSessionId, target: *mut u8) -> u32 {
            self.overwrite(
                session,
                HfmStr::from("test overwrite"),
                target as usize,
                &REPLACEMENT,
                1,
            )
        }

        fn discard(&self, session: HfmPatchSessionId) -> u32 {
            // SAFETY: The table came from this service.
            unsafe { self.0.session_discard.unwrap()(self.0.context, session) }
        }

        fn install(&self, session: HfmPatchSessionId) -> u32 {
            // SAFETY: Test targets are data that no thread executes.
            unsafe { self.0.session_install.unwrap()(self.0.context, session) }
        }
    }

    /// Fresh bytes per test, because installed patches are permanent and
    /// process-wide.
    fn writable_bytes() -> *mut u8 {
        Box::leak(Box::new([ORIGINAL; 4])).as_mut_ptr()
    }

    fn read(target: *const u8) -> u8 {
        // SAFETY: `writable_bytes` memory is leaked and never freed.
        unsafe { target.read_volatile() }
    }

    unsafe extern "C" fn accept_patch_service(api: *const HfmPatchApiV1) -> u32 {
        // SAFETY: The loader supplies the table for the duration of setup.
        let api = unsafe { &*api };
        if api.abi_version != HFM_PATCH_ABI_VERSION_1 {
            return u32::MAX;
        }
        let (Some(create), Some(discard)) = (api.session_create, api.session_discard) else {
            return u32::MAX;
        };
        let mut session = 0;
        let status = unsafe { create(api.context, &mut session) };
        if status != HFM_PATCH_OK {
            return status;
        }
        unsafe { discard(api.context, session) }
    }

    #[test]
    fn patch_setup_offers_a_usable_session() {
        assert_eq!(
            // SAFETY: The in-process test export accepts this table.
            unsafe { offer_patchy(Path::new("test-mod.dll"), accept_patch_service) },
            HFM_PATCH_OK,
        );
    }

    #[test]
    fn installing_an_overwrite_writes_the_replacement() {
        let service = ModService::new();
        let target = writable_bytes();
        let session = service.create();

        assert_eq!(service.overwrite_byte(session, target), HFM_PATCH_OK);
        assert_eq!(service.install(session), HFM_PATCH_OK);
        assert_eq!(read(target), REPLACEMENT);
    }

    #[test]
    fn issued_session_ids_are_nonzero() {
        let service = ModService::new();

        assert_ne!(service.create(), 0);
    }

    #[test]
    fn create_rejects_a_null_output() {
        let service = ModService::new();

        // SAFETY: The table came from this service.
        let status =
            unsafe { service.0.session_create.unwrap()(service.0.context, ptr::null_mut()) };

        assert_eq!(status, HFM_PATCH_INVALID_ARGUMENT);
    }

    #[test]
    fn overwrite_rejects_invalid_names() {
        let service = ModService::new();
        let target = writable_bytes();
        let session = service.create();
        let invalid_utf8 = [0xff_u8];

        for name in [
            HfmStr::empty(),
            HfmStr {
                ptr: ptr::null(),
                len: 1,
            },
            HfmStr {
                ptr: invalid_utf8.as_ptr(),
                len: invalid_utf8.len(),
            },
        ] {
            assert_eq!(
                service.overwrite(session, name, target as usize, &REPLACEMENT, 1),
                HFM_PATCH_INVALID_ARGUMENT,
            );
        }
    }

    #[test]
    fn overwrite_rejects_invalid_replacement_ranges() {
        let service = ModService::new();
        let target = writable_bytes() as usize;
        let session = service.create();

        for (address, bytes, len) in [
            (target, ptr::null(), 1),
            (target, &REPLACEMENT as *const u8, 0),
            (0, &REPLACEMENT as *const u8, 1),
            (usize::MAX, &REPLACEMENT as *const u8, 1),
        ] {
            assert_eq!(
                service.overwrite(session, HfmStr::from("invalid"), address, bytes, len),
                HFM_PATCH_INVALID_ARGUMENT,
            );
        }
    }

    #[test]
    fn a_rejected_request_leaves_the_session_usable() {
        let service = ModService::new();
        let target = writable_bytes();
        let session = service.create();
        service.overwrite(session, HfmStr::empty(), target as usize, &REPLACEMENT, 1);

        assert_eq!(service.overwrite_byte(session, target), HFM_PATCH_OK);
        assert_eq!(service.install(session), HFM_PATCH_OK);
        assert_eq!(read(target), REPLACEMENT);
    }

    #[test]
    fn overlapping_writes_in_one_session_are_rejected() {
        let service = ModService::new();
        let target = writable_bytes();
        let session = service.create();
        let replacement = [REPLACEMENT; 2];
        let overwrite_two_bytes_at = |offset: usize| {
            service.overwrite(
                session,
                HfmStr::from("two bytes"),
                target as usize + offset,
                replacement.as_ptr(),
                replacement.len(),
            )
        };

        assert_eq!(overwrite_two_bytes_at(0), HFM_PATCH_OK);
        assert_eq!(overwrite_two_bytes_at(1), HFM_PATCH_OVERLAP);
    }

    #[test]
    fn another_mod_cannot_use_a_session() {
        let owner = ModService::new();
        let other = ModService::new();
        let target = writable_bytes();
        let session = owner.create();

        assert_eq!(
            other.overwrite_byte(session, target),
            HFM_PATCH_INVALID_ARGUMENT
        );
        assert_eq!(other.discard(session), HFM_PATCH_INVALID_ARGUMENT);
        assert_eq!(other.install(session), HFM_PATCH_INVALID_ARGUMENT);
        assert_eq!(read(target), ORIGINAL);
    }

    #[test]
    fn another_mods_rejected_calls_do_not_consume_the_session() {
        let owner = ModService::new();
        let other = ModService::new();
        let target = writable_bytes();
        let session = owner.create();
        other.discard(session);
        other.install(session);

        assert_eq!(owner.overwrite_byte(session, target), HFM_PATCH_OK);
        assert_eq!(owner.install(session), HFM_PATCH_OK);
        assert_eq!(read(target), REPLACEMENT);
    }

    fn assert_unknown(service: &ModService, session: HfmPatchSessionId) {
        let target = writable_bytes();
        assert_eq!(
            service.overwrite_byte(session, target),
            HFM_PATCH_INVALID_ARGUMENT
        );
        assert_eq!(service.install(session), HFM_PATCH_INVALID_ARGUMENT);
        assert_eq!(service.discard(session), HFM_PATCH_INVALID_ARGUMENT);
        assert_eq!(read(target), ORIGINAL);
    }

    #[test]
    fn never_issued_session_ids_are_rejected() {
        let service = ModService::new();

        for session in [0, HfmPatchSessionId::MAX] {
            assert_unknown(&service, session);
        }
    }

    #[test]
    fn discard_consumes_the_session() {
        let service = ModService::new();
        let session = service.create();

        assert_eq!(service.discard(session), HFM_PATCH_OK);
        assert_unknown(&service, session);
    }

    #[test]
    fn install_consumes_the_session() {
        let service = ModService::new();
        let session = service.create();
        service.overwrite_byte(session, writable_bytes());

        assert_eq!(service.install(session), HFM_PATCH_OK);
        assert_unknown(&service, session);
    }

    #[test]
    fn a_failed_install_consumes_the_session() {
        let service = ModService::new();
        let target = writable_bytes();
        let installed = service.create();
        service.overwrite_byte(installed, target);
        service.install(installed);
        let conflicting = service.create();
        service.overwrite_byte(conflicting, target);

        assert_eq!(service.install(conflicting), HFM_PATCH_OVERLAP);
        assert_unknown(&service, conflicting);
    }
}
