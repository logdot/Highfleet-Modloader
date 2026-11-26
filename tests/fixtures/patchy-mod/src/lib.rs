use patchy::PatchSession;
use std::{
    ffi::{c_char, CStr},
    sync::atomic::{AtomicBool, Ordering},
};

patchy::export_patchy!();

static mut TARGET: u8 = 7;
static SAW_UNKNOWN_AFTER_SETUP: AtomicBool = AtomicBool::new(false);

#[cfg(all(windows, target_arch = "x86_64"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn patchy_test_callback_result() -> u32 {
    // First instruction: MOV EAX, 7 (five bytes). Continuation: ADD EAX, 1.
    core::arch::naked_asm!(".byte 0xb8, 7, 0, 0, 0", "add eax, 1", "ret");
}

#[cfg(all(windows, target_arch = "x86_64"))]
fn stage_callback(session: &mut PatchSession) -> patchy::Result {
    use patchy::{CallBridge, FlagPolicy, KeepReturn, StackAlignment};

    extern "C" fn replacement() -> u32 {
        40
    }

    // SAFETY: The fixture owns the naked function, whose entry has RSP % 16
    // == 8. Its first instruction is exactly five bytes and the callback
    // supplies the EAX value required by the continuation.
    unsafe {
        session.patch_call(
            patchy_test_callback_result as *const () as usize,
            5,
            CallBridge {
                callback: replacement as *const (),
                incoming_stack: StackAlignment::Offset8,
                flags: FlagPolicy::Preserve,
                keep_return: KeepReturn::Rax,
                before_call: &[],
                after_call: &[],
            },
        )?;
    }
    Ok(())
}

#[cfg(all(windows, target_arch = "x86_64"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn patchy_test_replay_result(_value: u32) -> u32 {
    core::arch::naked_asm!(
        ".byte 0x89, 0xc8",                   // MOV EAX, ECX
        ".byte 0x83, 0xc0, 0x02, 0x66, 0x90", // ADD EAX, 2; NOP (five-byte site)
        "add eax, 1",
        "ret",
    );
}

#[cfg(all(windows, target_arch = "x86_64"))]
fn stage_conditional_replay(session: &mut PatchSession) -> patchy::Result {
    use patchy::{Condition, Trampoline};

    let address = patchy_test_replay_result as *const () as usize + 2;
    // SAFETY: The fixture owns the source and only calls it before or after
    // installation. The displaced instructions can be replayed verbatim.
    unsafe {
        session.detour_trampoline_with(address, 5, |site| {
            let mut code = Trampoline::new();
            let skip = code.new_label();
            code.bytes(&[0x85, 0xc9]); // TEST ECX, ECX
            code.jump_if(Condition::NotEqual, skip);
            site.replay_verbatim(&mut code);
            code.bind(skip)?;
            site.resume(&mut code);
            Ok(code)
        })?;
    }
    Ok(())
}

/// Declines every version to show that a rejected version keeps the service.
#[unsafe(no_mangle)]
unsafe extern "C" fn version(version: *const c_char) -> bool {
    // Opening and dropping an empty session proves setup already ran.
    let set_up = PatchSession::new().is_ok();
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
    let Ok(mut session) = PatchSession::new() else {
        return false;
    };
    #[cfg(all(windows, target_arch = "x86_64"))]
    if stage_callback(&mut session)
        .and_then(|()| stage_conditional_replay(&mut session))
        .is_err()
    {
        return false;
    }
    // SAFETY: TARGET stays live and only this fixture touches it during init.
    unsafe {
        session
            .overwrite_named("fixture overwrite", (&raw mut TARGET) as usize, &[42])
            .and_then(|_| session.install_permanently())
            .is_ok()
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn patchy_test_target() -> u8 {
    unsafe { (&raw const TARGET).read_volatile() }
}
