use std::{ffi::CString, fs, os::raw::c_char, path::PathBuf};

use libloading::Library;
use log::{debug, error, info, warn};

use crate::{host_logging, host_patchy};

const MOD_FOLDER: &str = "./Modloader/mods";
const CONFIG_FOLDER: &str = "./Modloader/config";

pub fn create_folders() {
    if !PathBuf::from(MOD_FOLDER).exists() {
        fs::create_dir_all(MOD_FOLDER).unwrap_or_else(|e| {
            error!("Failed to create mods folder: {}", e);
        });
    }

    if !PathBuf::from(CONFIG_FOLDER).exists() {
        fs::create_dir_all(CONFIG_FOLDER).unwrap_or_else(|e| {
            error!("Failed to create config folder: {}", e);
        });
    }
}

fn find_mods() -> std::io::Result<Vec<PathBuf>> {
    let folder = fs::read_dir(MOD_FOLDER)?;

    let mut mods = Vec::new();

    for path in folder {
        let path = path?;
        let path = path.path();

        if path.is_file() && path.extension().unwrap_or_default() == "dll" {
            info!("Found mod: {}", path.display());
            mods.push(path);
        }
    }

    Ok(mods)
}

pub fn load_mods(version: &str) {
    let mods = find_mods().unwrap_or_else(|e| {
        error!("Failed to find mods: {}", e);
        Vec::new()
    });

    for mod_path in mods {
        info!("Loading mod: {}", mod_path.display());

        load_mod(&mod_path, version);
    }
}

fn load_mod(path: &PathBuf, version: &str) {
    unsafe {
        let library = match Library::new(path) {
            Ok(library) => {
                debug!("Loaded mod: {}", path.display());
                library
            }
            Err(e) => {
                error!("Failed to load mod: {}", e);
                return;
            }
        };

        host_logging::setup_mod_logging(&library, path);
        host_patchy::setup_mod_patching(&library, path);

        match library.get::<unsafe extern "C" fn(*const c_char) -> bool>(b"version") {
            Ok(version_func) => {
                let cstr = CString::new(version).unwrap();
                if !version_func(cstr.as_ptr()) {
                    warn!(
                        "Mod doesn't support game version {version}: {}. This may cause crashes or other issues",
                        path.display()
                    );
                }
            }
            Err(e) => {
                warn!("No version function: {}", e);
            }
        };

        match library.get::<unsafe extern "C" fn() -> bool>(b"init") {
            Ok(init) => {
                if !init() {
                    error!("Failed to initialize mod: {}", path.display());
                }
            }
            Err(e) => {
                warn!("No init function: {}", e);
            }
        };

        std::mem::forget(library);
    }
}

#[cfg(test)]
mod tests {
    use crate::versioning::{startup_version, UNKNOWN_VERSION};

    #[test]
    #[ignore = "requires a built patchy-test-mod DLL; run with PATCHY_TEST_MOD set"]
    fn unknown_build_still_initializes_a_patchy_mod_that_declines_the_version() {
        let path = std::path::PathBuf::from(
            std::env::var_os("PATCHY_TEST_MOD").expect("PATCHY_TEST_MOD must name the fixture DLL"),
        )
        .canonicalize()
        .unwrap();
        // SAFETY: This test controls the fixture DLL and its export signature.
        unsafe {
            let library = libloading::Library::new(&path).unwrap();
            let target = library
                .get::<unsafe extern "C" fn() -> u8>(b"patchy_test_target")
                .unwrap();
            let callback_result = library
                .get::<unsafe extern "C" fn() -> u32>(b"patchy_test_callback_result")
                .unwrap();
            let version = startup_version();
            assert_eq!(
                version, UNKNOWN_VERSION,
                "the test runner is not a known game build"
            );
            assert_eq!(target(), 7);
            assert_eq!(callback_result(), 8);
            super::load_mod(&path, &version);
            assert_eq!(target(), 42);
            assert_eq!(callback_result(), 41);
        }
    }
}
