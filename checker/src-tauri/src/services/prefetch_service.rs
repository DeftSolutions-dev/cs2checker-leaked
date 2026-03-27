use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const PREFETCH_DIR: &str = r"C:\Windows\Prefetch";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefetchEntry {
    pub filename: String,
    pub full_path: String,
    pub last_modified: String,
    pub matched_cheat: Option<String>,
}

const CHEAT_EXECUTABLES: &[&str] = &[
    "neverlose", "onetap", "gamesense", "aimware", "midnight",
    "nixware", "interium", "fatality", "skeet", "ev0lve",
    "spirthack", "iniuria", "osiris", "sharkhack", "extrim",
    "exloader", "memesense", "xone", "enigma", "en1gma",
    "aimstar", "aimmy", "ezfrags", "ratpoison", "charlatan",
    "weave", "legendware", "primordial", "pandora", "cryptic",
    "compkiller", "ekknod", "lunapaste", "whiskey",
    "s1mpleinternal", "zrk",
];

pub fn scan_prefetch() -> Vec<PrefetchEntry> {
    let mut results = Vec::new();
    let prefetch_path = PathBuf::from(PREFETCH_DIR);

    if !prefetch_path.exists() {
        return results;
    }

    let entries = match std::fs::read_dir(&prefetch_path) {
        Ok(e) => e,
        Err(_) => return results,
    };

    for entry in entries.flatten() {
        let filename = entry.file_name().to_string_lossy().to_lowercase();

        if !filename.ends_with(".pf") {
            continue;
        }

        let modified = entry.metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let datetime: chrono::DateTime<chrono::Local> = t.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_default();

        let matched = CHEAT_EXECUTABLES.iter()
            .find(|cheat| filename.contains(*cheat))
            .map(|s| s.to_string());

        if matched.is_some() {
            results.push(PrefetchEntry {
                filename: entry.file_name().to_string_lossy().to_string(),
                full_path: entry.path().to_string_lossy().to_string(),
                last_modified: modified,
                matched_cheat: matched,
            });
        }
    }

    results
}

