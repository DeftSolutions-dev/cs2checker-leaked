use serde::{Deserialize, Serialize};
use std::process::Command;

const BAM_REGISTRY_PATHS: &[&str] = &[
    r"SYSTEM\CurrentControlSet\Services\bam\State\UserSettings",
    r"SYSTEM\CurrentControlSet\Services\bam\UserSettings",
];

const USER_ASSIST_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentApp {
    pub name: String,
    pub path: String,
    pub last_run: String,
    pub source: String, // "bam", "userassist", "prefetch"
}

const SOFTWARE_WHITELIST: &[&str] = &[
    "Microsoft Corporation",
    "Microsoft Windows",
    "NVIDIA Corporation",
    "Intel Corporation",
    "AMD Inc.",
    "Realtek Semiconductor Corp.",
    "Qualcomm Atheros",
    "Google LLC",
    "Adobe Inc.",
    "Valve Corporation",
    "Epic Games Inc.",
    "Blizzard Entertainment Inc.",
    "Electronic Arts Inc.",
    "Ubisoft Entertainment SA",
    "Riot Games Inc.",
    "7-Zip",
    "WinRAR GmbH",
    "Piriform Ltd",
    "CCleaner Ltd",
];

pub fn get_recent_apps_from_bam() -> Vec<RecentApp> {
    let mut apps = Vec::new();

    for path in BAM_REGISTRY_PATHS {
        if let Ok(output) = Command::new("reg")
            .args(&["query", &format!("HKLM\\{}", path), "/s"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.contains("\\Device\\") || trimmed.contains(".exe") {
                    if let Some(exe_path) = extract_exe_path(trimmed) {
                        let name = exe_path.rsplit('\\').next().unwrap_or("").to_string();
                        apps.push(RecentApp {
                            name,
                            path: exe_path,
                            last_run: String::new(),
                            source: "bam".to_string(),
                        });
                    }
                }
            }
        }
    }

    apps
}

fn extract_exe_path(line: &str) -> Option<String> {
    if let Some(start) = line.find("\\Device\\") {
        let path = &line[start..];
        if let Some(end) = path.find(".exe") {
            return Some(path[..end + 4].to_string());
        }
    }
    if let Some(start) = line.find("C:\\") {
        let path = &line[start..];
        if let Some(end) = path.find(".exe") {
            return Some(path[..end + 4].to_string());
        }
    }
    None
}

pub fn is_suspicious_app(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    let cheat_keywords = [
        "cheat", "hack", "inject", "loader", "bypass",
        "neverlose", "onetap", "gamesense", "aimware", "midnight",
        "nixware", "interium", "fatality", "skeet", "ev0lve",
        "exloader", "memesense", "xone", "enigma",
    ];

    cheat_keywords.iter().any(|kw| name_lower.contains(kw))
}

