use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::utils::fast_filters;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileThreat {
    pub file_path: String,
    pub file_name: String,
    pub threat_type: String,
    pub confidence: f32,
    pub details: String,
    pub sha256: Option<String>,
}

const KNOWN_CHEAT_HASHES: &[(&str, &str)] = &[
    ("2e737dfb6b404d6fb5760359d33ab98c2c7462bedc70b81e1fafdfa27ea45e5", "Memesense"),
    ("31227a46942215f6c45113f9edad4d81f51628faa355a91a0b9ec080a81606ab3", "Xone"),
    ("0345b8892bc6e55c85e4683d8fa68c512b2ae2c79eb95a5826bc15032bdf14aa", "Midnight"),
];

pub fn scan_drive(
    drive: &str,
    max_depth: u32,
    use_signatures: bool,
    stop_flag: &Arc<AtomicBool>,
) -> Vec<FileThreat> {
    let mut threats = Vec::new();
    scan_directory(Path::new(drive), 0, max_depth, use_signatures, stop_flag, &mut threats);
    threats
}

fn scan_directory(
    path: &Path,
    depth: u32,
    max_depth: u32,
    use_signatures: bool,
    stop_flag: &Arc<AtomicBool>,
    threats: &mut Vec<FileThreat>,
) {
    if depth > max_depth || stop_flag.load(Ordering::SeqCst) {
        return;
    }

    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        if stop_flag.load(Ordering::SeqCst) {
            return;
        }

        let entry_path = entry.path();

        if entry_path.is_dir() {
            let dir_name = entry.file_name().to_string_lossy().to_lowercase();

            if fast_filters::should_skip_directory(&dir_name) {
                continue;
            }

            scan_directory(&entry_path, depth + 1, max_depth, use_signatures, stop_flag, threats);
        } else if entry_path.is_file() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();

            if fast_filters::is_suspicious_filename(&file_name) {
                let mut threat = FileThreat {
                    file_path: entry_path.to_string_lossy().to_string(),
                    file_name: entry.file_name().to_string_lossy().to_string(),
                    threat_type: "suspicious_name".to_string(),
                    confidence: 0.6,
                    details: format!("Suspicious filename: {}", file_name),
                    sha256: None,
                };

                if use_signatures {
                    if let Ok(hash) = compute_sha256(&entry_path) {
                        for (known_hash, cheat_name) in KNOWN_CHEAT_HASHES {
                            if hash == *known_hash {
                                threat.threat_type = "known_cheat".to_string();
                                threat.confidence = 1.0;
                                threat.details = format!("Known cheat: {}", cheat_name);
                                threat.sha256 = Some(hash.clone());
                                break;
                            }
                        }
                        if threat.sha256.is_none() {
                            threat.sha256 = Some(hash);
                        }
                    }
                }

                threats.push(threat);
            }
        }
    }
}

fn compute_sha256(path: &Path) -> Result<String, std::io::Error> {
    let data = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Ok(hex::encode(hasher.finalize()))
}

