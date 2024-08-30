use std::{fs, path::PathBuf};

use libloading::Library;
use log::{debug, error, info, warn, LevelFilter, Log};

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

pub fn load_mods(version: String) {
    let mods = find_mods().unwrap_or_else(|e| {
        error!("Failed to find mods: {}", e);
        Vec::new()
    });

    for mod_path in mods {
        info!("Loading mod: {}", mod_path.display());

        load_mod(&mod_path, &version);
    }
}

fn load_mod(path: &PathBuf, version: &String) {
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

        match library.get::<unsafe extern fn(&String) -> bool>(b"version") {
            Ok(version_func) => {
                if !version_func(version) {
                    error!("Mod doesn't support game version: {}", path.display());
                    error!("This may cause crashes or other issues");
                }
            },
            Err(e) => {
                warn!("No version function: {}", e);
            }
        };

        match library.get::<unsafe fn(&'static dyn Log, LevelFilter) -> bool>(b"setup_logger") {
            Ok(setup_logger) => {
                setup_logger(log::logger(), log::max_level());
            },
            Err(e) => {
                warn!("No setup_logger function: {}", e);
            }
        };

        std::mem::forget(library);
    }
}
