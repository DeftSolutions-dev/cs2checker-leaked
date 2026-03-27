use serde::{Deserialize, Serialize};
use std::process::Command;

#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::*;
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;

const CS2_PROCESS_NAME: &str = "cs2.exe";
const STEAM_PROCESS_NAME: &str = "steam.exe";

const CS2_PATHS: &[&str] = &[
    r"C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive",
    r"C:\Program Files\Steam\steamapps\common\Counter-Strike Global Offensive",
    r"D:\Steam\steamapps\common\Counter-Strike Global Offensive",
    r"E:\Steam\steamapps\common\Counter-Strike Global Offensive",
    r"C:\Program Files (x86)\Steam\steamapps\common\CS2",
    r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProcess {
    pub is_running: bool,
    pub thread_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub base_address: String,
    pub size_kb: u64,
}

pub fn check_cs2_running() -> GameProcess {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return GameProcess { is_running: false, thread_count: 0 };
            }

            let mut entry: PROCESSENTRY32W = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    let name = String::from_utf16_lossy(
                        &entry.szExeFile[..entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)]
                    );

                    if name.to_lowercase() == CS2_PROCESS_NAME {
                        CloseHandle(snapshot);
                        return GameProcess {
                            is_running: true,
                            thread_count: entry.cntThreads,
                        };
                    }

                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
        }
    }

    GameProcess { is_running: false, thread_count: 0 }
}

pub fn find_cs2_process() -> Option<u32> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
                return None;
            }

            let mut entry: PROCESSENTRY32W = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    let name = String::from_utf16_lossy(
                        &entry.szExeFile[..entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(0)]
                    );

                    if name.to_lowercase() == CS2_PROCESS_NAME {
                        CloseHandle(snapshot);
                        return Some(entry.th32ProcessID);
                    }

                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
        }
    }

    None
}

pub fn is_steam_running() -> bool {
    let output = Command::new("tasklist")
        .args(&["/FI", &format!("IMAGENAME eq {}", STEAM_PROCESS_NAME)])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.to_lowercase().contains(STEAM_PROCESS_NAME)
        }
        Err(_) => false,
    }
}

pub fn find_cs2_installation() -> Option<String> {
    for path in CS2_PATHS {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}

