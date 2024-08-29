use std::{fs, path::PathBuf};

use libloading::{Library, Symbol};
use log::{warn, error};

const MOD_FOLDER: &str = "./Modloader/mods";
const CONFIG_FOLDER: &str = "./Modloader/config";

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

pub fn load_mods() {
    let mods = find_mods().unwrap_or_else(|e| {
        error!("Failed to find mods: {}", e);
        Vec::new()
    });

    for mod_path in mods {
        println!("Loading mod: {}", mod_path.display());

        load_mod(&mod_path);
    }
}

fn load_mod(path: &PathBuf) {
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

        match library.get::<unsafe extern fn()>(b"version") {
            Ok(version) => {
                println!("Found version function");
                version();
            },
            Err(e) => {
                eprint!("Failed to get version function: {}", e);
                warn!("No version function: {}", e);
            }
        };

        std::mem::forget(library);
    }
}
