use std::{ffi::CString, fs, path::PathBuf};

use libloading::{Library, Symbol};
use log::{warn, error};

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
            println!("Found mod: {}", path.display());
            mods.push(path);
        }
    }

    Ok(mods)
}

pub fn load_mods(version: String) {
    let mods = find_mods().unwrap_or_else(|e| {
        error!("Failed to find mods: {}", e);
        Vec::new()
    });

    for mod_path in mods {
        println!("Loading mod: {}", mod_path.display());

        load_mod(&mod_path, &version);
    }
}

fn load_mod(path: &PathBuf, version: &String) {
    unsafe {
        let library = match Library::new(path) {
            Ok(library) => {
                println!("Loaded mod: {}", path.display());
                library
            }
            Err(e) => {
                error!("Failed to load mod: {}", e);
                return;
            }
        };

        match library.get::<unsafe extern fn(&String) -> bool>(b"version") {
            Ok(version_func) => {
                if !version_func(&version) {
                    warn!("Mod doesn't support game version: {}", path.display());
                    warn!("This may cause crashes or other issues");
                }
            },
            Err(e) => {
                warn!("No version function: {}", e);
            }
        };

        std::mem::forget(library);
    }
}
