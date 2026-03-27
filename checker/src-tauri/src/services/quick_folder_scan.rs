use serde::{Deserialize, Serialize};
use std::path::Path;

const CHEAT_FOLDER_NAMES: &[&str] = &[
    "sharkhack", "extrim", "osiris", "nixware", "xone", "naim",
    "interium", "en1gma", "enigma", "fatality", "onetap", "skeet",
    "com.swiftsoft", "ezfrags", "inuria", "iniuria", "spirthack",
    "ev0lve", "aimtux", "fuzion", "en1gma-tech",
];

const EXCLUDED_FOLDERS: &[&str] = &[
    "program files", "program files (x86)", "programdata", "system32",
    "syswow64", "winsxs", "perflogs", "recovery", "$recycle.bin",
    "system volume information", "msocache", "intel", "amd", "nvidia",
    "temp", "inetpub", "logs", "prefetch", "servicing", "all users",
    "default user", "microsoft", "microsoft corporation", "windowsapps",
    "packages", "systemapps", "windows defender", "windows security",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderThreat {
    pub folder_name: String,
    pub folder_path: String,
    pub matched_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickFolderScanResult {
    pub threats_found: u32,
    pub folders_scanned: u32,
    pub threats: Vec<FolderThreat>,
}

pub fn scan_all_drives() -> QuickFolderScanResult {
    let mut result = QuickFolderScanResult {
        threats_found: 0,
        folders_scanned: 0,
        threats: Vec::new(),
    };

    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        if Path::new(&drive).exists() {
            scan_directory(&drive, 0, 3, &mut result); // max_depth = 3
        }
    }

    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let special_dirs = [
            "Desktop", "Downloads", "Documents", "AppData\\Local",
            "AppData\\Roaming", "AppData\\Local\\Temp",
        ];

        for dir in &special_dirs {
            let path = format!("{}\\{}", user_profile, dir);
            if Path::new(&path).exists() {
                scan_directory(&path, 0, 4, &mut result);
            }
        }
    }

    result
}

fn scan_directory(path: &str, depth: u32, max_depth: u32, result: &mut QuickFolderScanResult) {
    if depth > max_depth {
        return;
    }

    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let entry_path = entry.path();
        if !entry_path.is_dir() {
            continue;
        }

        let folder_name = entry.file_name().to_string_lossy().to_lowercase();
        result.folders_scanned += 1;

        if EXCLUDED_FOLDERS.iter().any(|ex| folder_name == *ex) {
            continue;
        }

        for pattern in CHEAT_FOLDER_NAMES {
            if folder_name.contains(pattern) {
                result.threats.push(FolderThreat {
                    folder_name: entry.file_name().to_string_lossy().to_string(),
                    folder_path: entry_path.to_string_lossy().to_string(),
                    matched_pattern: pattern.to_string(),
                });
                result.threats_found += 1;
                break;
            }
        }

        scan_directory(&entry_path.to_string_lossy(), depth + 1, max_depth, result);
    }
}

