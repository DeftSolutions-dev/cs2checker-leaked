use crate::services::scanner::file_scanner::FileThreat;
use sha2::{Sha256, Digest};
use std::path::PathBuf;

const XONE_HASH: &str = "31227a46942215f6c45113f9edad4d81f51628faa355a91a0b9ec00a81606ab3";

const XONE_INDICATORS: &[&str] = &[
    "xone",
    "x-one",
    "x_one",
];

pub fn detect() -> Vec<FileThreat> {
    let mut threats = Vec::new();

    let search_dirs = get_search_dirs();

    for base_dir in &search_dirs {
        let base = PathBuf::from(base_dir);
        if !base.exists() {
            continue;
        }

        if let Ok(entries) = std::fs::read_dir(&base) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_lowercase();

                if XONE_INDICATORS.iter().any(|ind| name.contains(ind)) {
                    if entry.path().is_dir() {
                        threats.push(FileThreat {
                            file_path: entry.path().to_string_lossy().to_string(),
                            file_name: entry.file_name().to_string_lossy().to_string(),
                            threat_type: "xone_folder".to_string(),
                            confidence: 0.85,
                            details: "Xone cheat directory found".to_string(),
                            sha256: None,
                        });
                    } else if name.ends_with(".exe") || name.ends_with(".dll") {
                        let hash = compute_hash(&entry.path());
                        let is_known = hash.as_deref() == Some(XONE_HASH);

                        threats.push(FileThreat {
                            file_path: entry.path().to_string_lossy().to_string(),
                            file_name: entry.file_name().to_string_lossy().to_string(),
                            threat_type: if is_known { "known_cheat" } else { "xone_file" }.to_string(),
                            confidence: if is_known { 1.0 } else { 0.8 },
                            details: if is_known {
                                "Known Xone binary (SHA256 match)".to_string()
                            } else {
                                "Xone-related file".to_string()
                            },
                            sha256: hash,
                        });
                    }
                }
            }
        }
    }

    threats
}

fn compute_hash(path: &PathBuf) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Some(hex::encode(hasher.finalize()))
}

fn get_search_dirs() -> Vec<String> {
    let mut dirs = Vec::new();
    if let Ok(profile) = std::env::var("USERPROFILE") {
        dirs.push(format!("{}\\Desktop", profile));
        dirs.push(format!("{}\\Downloads", profile));
        dirs.push(format!("{}\\AppData\\Local", profile));
        dirs.push(format!("{}\\AppData\\Roaming", profile));
    }
    dirs
}

