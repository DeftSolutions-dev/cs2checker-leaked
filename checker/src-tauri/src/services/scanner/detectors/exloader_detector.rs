use crate::services::scanner::file_scanner::FileThreat;
use std::path::PathBuf;

const EXLOADER_PATHS: &[&str] = &[
    "ExLoader",
    "exloader",
    "Ex-Loader",
];

const EXLOADER_FILES: &[&str] = &[
    "exloader.exe",
    "ex-loader.exe",
    "ExLoader.exe",
];

pub fn detect() -> Vec<FileThreat> {
    let mut threats = Vec::new();

    let search_dirs = get_search_dirs();

    for base_dir in &search_dirs {
        for folder_name in EXLOADER_PATHS {
            let path = PathBuf::from(base_dir).join(folder_name);
            if path.exists() {
                threats.push(FileThreat {
                    file_path: path.to_string_lossy().to_string(),
                    file_name: folder_name.to_string(),
                    threat_type: "exloader_folder".to_string(),
                    confidence: 0.9,
                    details: "ExLoader installation directory found".to_string(),
                    sha256: None,
                });

                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_lowercase();
                        if EXLOADER_FILES.iter().any(|f| name == f.to_lowercase()) {
                            threats.push(FileThreat {
                                file_path: entry.path().to_string_lossy().to_string(),
                                file_name: entry.file_name().to_string_lossy().to_string(),
                                threat_type: "exloader_exe".to_string(),
                                confidence: 1.0,
                                details: "ExLoader executable found".to_string(),
                                sha256: None,
                            });
                        }
                    }
                }
            }
        }
    }

    threats
}

fn get_search_dirs() -> Vec<String> {
    let mut dirs = Vec::new();
    if let Ok(profile) = std::env::var("USERPROFILE") {
        dirs.push(format!("{}\\Desktop", profile));
        dirs.push(format!("{}\\Downloads", profile));
        dirs.push(format!("{}\\Documents", profile));
        dirs.push(format!("{}\\AppData\\Local", profile));
        dirs.push(format!("{}\\AppData\\Roaming", profile));
    }
    if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
        dirs.push(appdata);
    }
    dirs.push("C:\\".to_string());
    dirs.push("D:\\".to_string());
    dirs
}

