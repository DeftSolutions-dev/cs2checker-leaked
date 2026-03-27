use crate::services::scanner::file_scanner::FileThreat;
use sha2::{Sha256, Digest};
use std::path::PathBuf;

const MEMESENSE_HASH: &str = "2e737dfb6b404d6fb5760359d33ab98c2c7462bedc70b81e1fafd8fa27ea45e5";

const MEMESENSE_INDICATORS: &[&str] = &[
    "memesense",
    "meme-sense",
    "MeMeSense",
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

                if MEMESENSE_INDICATORS.iter().any(|ind| name.contains(&ind.to_lowercase())) {
                    if entry.path().is_dir() {
                        threats.push(FileThreat {
                            file_path: entry.path().to_string_lossy().to_string(),
                            file_name: entry.file_name().to_string_lossy().to_string(),
                            threat_type: "memesense_folder".to_string(),
                            confidence: 0.85,
                            details: "Memesense directory found".to_string(),
                            sha256: None,
                        });
                    } else if name.ends_with(".exe") || name.ends_with(".dll") {
                        let hash = compute_hash(&entry.path());
                        let is_known = hash.as_deref() == Some(MEMESENSE_HASH);

                        threats.push(FileThreat {
                            file_path: entry.path().to_string_lossy().to_string(),
                            file_name: entry.file_name().to_string_lossy().to_string(),
                            threat_type: if is_known { "known_cheat" } else { "memesense_file" }.to_string(),
                            confidence: if is_known { 1.0 } else { 0.8 },
                            details: if is_known {
                                "Known Memesense binary (SHA256 match)".to_string()
                            } else {
                                "Memesense-related file".to_string()
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

