use crate::services::scanner::file_scanner::FileThreat;
use sha2::{Sha256, Digest};
use std::path::PathBuf;

const MIDNIGHT_HASH: &str = "0345b8892bc6e55c85e4683d8fa68c512b2ae2c79eb95a5826bc15032bdf14aa";

const MIDNIGHT_INDICATORS: &[&str] = &[
    "midnight",
    "midnight.im",
    "midnightcheat",
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

                if MIDNIGHT_INDICATORS.iter().any(|ind| name.contains(ind)) {
                    if entry.path().is_dir() {
                        threats.push(FileThreat {
                            file_path: entry.path().to_string_lossy().to_string(),
                            file_name: entry.file_name().to_string_lossy().to_string(),
                            threat_type: "midnight_folder".to_string(),
                            confidence: 0.85,
                            details: "Midnight cheat directory found".to_string(),
                            sha256: None,
                        });
                    } else if name.ends_with(".exe") || name.ends_with(".dll") {
                        let hash = compute_hash(&entry.path());
                        let is_known = hash.as_deref() == Some(MIDNIGHT_HASH);

                        threats.push(FileThreat {
                            file_path: entry.path().to_string_lossy().to_string(),
                            file_name: entry.file_name().to_string_lossy().to_string(),
                            threat_type: if is_known { "known_cheat" } else { "midnight_file" }.to_string(),
                            confidence: if is_known { 1.0 } else { 0.8 },
                            details: if is_known {
                                "Known Midnight binary (SHA256 match)".to_string()
                            } else {
                                "Midnight-related file".to_string()
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

