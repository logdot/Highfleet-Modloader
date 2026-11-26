use std::{error::Error, fs::File, io};
use log::{info, warn};
use sha2::{Sha256, Digest};

/// The version passed to mods when the running executable is not recognised.
pub const UNKNOWN_VERSION: &str = "Unknown";

struct Version {
    name: &'static str,
    sha256: &'static str,
}

const VERSIONS: [Version; 3] = [
    Version {
        name: "Steam 1.151",
        sha256: "1efb42b192bcbb94fd557352c79b2329f74c4c5026fadc40ca21ba3e77e3f200",
    },
    Version {
        name: "Steam 1.163",
        sha256: "15bd7600581e6018088cdd57c9de61453ab14f935295237516e34ee4d2689432",
    },
    Version {
        name: "Gog 1.163",
        sha256: "ef3b88e3d5d89eac1600d66e9c005c9563f1fe97e492c7964f31c9d71c5fd104"
    }
];

fn get_sha256() -> Result<String, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    let mut file = File::open(std::env::current_exe()?)?;

    io::copy(&mut file, &mut hasher)?;
    let hash_bytes = hasher.finalize();

    Ok(format!("{:x}", hash_bytes))
}

fn check_version(sha256: String, versions: &[Version]) -> Result<String, Box<dyn Error>> {
    for version in versions {
        if sha256 == version.sha256 {
            return Ok(version.name.to_string());
        }
    }

    Err("Unknown version".into())
}

fn get_version() -> Result<String, Box<dyn Error>> {
    let sha256 = get_sha256()?;
    let version = check_version(sha256, &VERSIONS)?;

    Ok(version)
}

/// Identifies the running executable for mods' `version` exports.
pub fn startup_version() -> String {
    match get_version() {
        Ok(version) => {
            info!("Highfleet version: {version}");
            version
        }
        Err(e) => {
            warn!("Error fetching version: {e}");
            UNKNOWN_VERSION.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_the_running_executable_even_when_its_build_is_unknown() {
        let executable = std::fs::read(std::env::current_exe().unwrap()).unwrap();
        let expected = format!("{:x}", Sha256::digest(executable));

        assert_eq!(get_sha256().unwrap(), expected);
    }
}
