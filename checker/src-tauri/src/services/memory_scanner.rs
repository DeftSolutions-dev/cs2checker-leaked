use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
use winapi::um::{
    handleapi::CloseHandle,
    memoryapi::{ReadProcessMemory, VirtualQueryEx},
    processthreadsapi::OpenProcess,
    psapi::{EnumProcessModules, GetModuleFileNameExW},
    tlhelp32::*,
    winnt::*,
};

const MEMORY_SIGNATURES: &[&str] = &[
    "aimbot", "wallhack", "esp", "triggerbot", "bhop", "speedhack",
    "norecoil", "nospread", "injector", "dll_inject", "process_inject",
    "vac_bypass", "eac_bypass", "be_bypass", "faceit_bypass",
    "cr4ck", "k3y", "g3n", "p4tch", "h4ck", "ch34t",
    "deadbeef", "cafebabe", "anticheat", "anti_cheat", "anti-cheat",
    "hook", "detour", "patch", "memory_patch", "dll_hijack",
    "process_hollow", "manual_map",
];

const CHEAT_GUI_STRINGS: &[&str] = &[
    "unload_popup",
    "XONE",
    "##temp_button_add##",
    "Removals",
    "##main_window##",
    "watermark",
    "##Tooltip_Hidden",
    "Unload Osiris",
    "Spectator List",
    "INTERIUM",
    "hook/##",
];

const WHITELISTED_PROCESSES: &[&str] = &[
    "csrss.exe", "winlogon.exe", "services.exe", "svchost.exe", "lsass.exe",
    "dwm.exe", "explorer.exe", "steam.exe", "steamwebhelper.exe", "steamservice.exe",
    "origin.exe", "epicgameslauncher.exe", "battle.net.exe", "uplay.exe", "upc.exe",
    "nvcontainer.exe", "nvdisplay.container.exe", "radeonsoft.exe", "radeonsoftware.exe",
    "amdrsserv.exe", "discord.exe", "discordptb.exe", "spotify.exe",
    "chrome.exe", "firefox.exe", "msedge.exe", "mssense.exe", "msmpeng.exe",
    "avastui.exe", "avgui.exe",
];

const WHITELISTED_DLLS: &[&str] = &[
    "gameoverlayrenderer64.dll", "gameoverlayrenderer.dll",
    "steamclient64.dll", "steamclient.dll",
    "discord_voice.node", "discord_game_sdk.dll",
    "nvd3dumx.dll", "nvspcap64.dll", "nvngx.dll", "nvcuda.dll",
    "amdxc64.dll", "atig6pxx.dll",
    "clrjit.dll", "coreclr.dll", "mscorwks.dll", "mscoree.dll",
    "nvshadowplay.dll", "nvcplugin.dll",
    "avghookx.dll", "avghook.dll",
    "snxhk.dll", "snxhk64.dll",
    "hmpalert.dll",
    "ksde.dll", "kloehk.dll",
    "sbiedll.dll",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAnomaly {
    pub anomaly_type: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryScanResult {
    pub total_processes_scanned: u32,
    pub anomalies_found: u32,
    pub anomalies: Vec<MemoryAnomaly>,
    pub verdict: String,
    pub modules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryString {
    pub content: String,
    pub matched_signature: String,
    pub module_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub total_strings: u32,
    pub suspicious_strings: u32,
    pub scan_time_ms: u64,
    pub strings: Vec<MemoryString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleInfo {
    pub handle_value: u64,
    pub process_id: u32,
    pub process_name: String,
    pub process_path: String,
    pub access_mask: u32,
    pub is_suspicious: bool,
    pub suspicious_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleScanResult {
    pub total_handles: u32,
    pub suspicious_handles: u32,
    pub handles: Vec<HandleInfo>,
}

fn protection_to_string(protect: u32) -> String {
    match protect & 0xFF {
        0x01 => "NOACCESS".to_string(),
        0x02 => "READONLY".to_string(),
        0x04 => "READWRITE".to_string(),
        0x08 => "WRITECOPY".to_string(),
        0x10 => "EXECUTE".to_string(),
        0x20 => "EXECUTE_READ".to_string(),
        0x40 => "EXECUTE_READWRITE".to_string(),
        0x80 => "EXECUTE_WRITECOPY".to_string(),
        _ => format!("unknown(0x{:X})", protect),
    }
}

pub fn scan_game_memory(pid: u32) -> MemoryScanResult {
    let mut result = MemoryScanResult {
        total_processes_scanned: 1,
        anomalies_found: 0,
        anomalies: Vec::new(),
        verdict: "clean".to_string(),
        modules: Vec::new(),
    };

    #[cfg(target_os = "windows")]
    {
        unsafe {
            let process = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                0,
                pid,
            );

            if process.is_null() {
                result.verdict = "error: cannot open process".to_string();
                return result;
            }

            let mut modules: [winapi::shared::minwindef::HMODULE; 1024] = [std::ptr::null_mut(); 1024];
            let mut needed: u32 = 0;

            if EnumProcessModules(
                process,
                modules.as_mut_ptr(),
                std::mem::size_of_val(&modules) as u32,
                &mut needed,
            ) != 0 {
                let count = (needed as usize) / std::mem::size_of::<winapi::shared::minwindef::HMODULE>();

                for i in 0..count {
                    let mut name: [u16; 260] = [0; 260];
                    GetModuleFileNameExW(process, modules[i], name.as_mut_ptr(), 260);
                    let module_name = String::from_utf16_lossy(
                        &name[..name.iter().position(|&c| c == 0).unwrap_or(0)]
                    );

                    let file_name = module_name.rsplit('\\').next().unwrap_or("").to_lowercase();

                    if !WHITELISTED_DLLS.iter().any(|w| file_name == w.to_lowercase()) {
                        result.modules.push(module_name.clone());
                    }
                }
            }

            let mut address: usize = 0;
            let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();

            while VirtualQueryEx(
                process,
                address as *const _,
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            ) != 0 {
                if mbi.Protect == PAGE_EXECUTE_READWRITE && mbi.Type == MEM_PRIVATE {
                    result.anomalies.push(MemoryAnomaly {
                        anomaly_type: "RWX_PRIVATE".to_string(),
                        details: format!(
                            "Private RWX memory at 0x{:X}, size {} KB, protection: {}",
                            address,
                            mbi.RegionSize / 1024,
                            protection_to_string(mbi.Protect)
                        ),
                    });
                    result.anomalies_found += 1;
                }

                if (mbi.Protect & PAGE_READWRITE != 0 || mbi.Protect & PAGE_READONLY != 0)
                    && mbi.State == MEM_COMMIT
                    && mbi.RegionSize < 10 * 1024 * 1024
                {
                    let mut buffer = vec![0u8; mbi.RegionSize];
                    let mut bytes_read = 0;

                    if ReadProcessMemory(
                        process,
                        address as *const _,
                        buffer.as_mut_ptr() as *mut _,
                        mbi.RegionSize,
                        &mut bytes_read,
                    ) != 0 {
                        let content = String::from_utf8_lossy(&buffer[..bytes_read]);

                        for sig in MEMORY_SIGNATURES {
                            if content.contains(sig) {
                                result.anomalies.push(MemoryAnomaly {
                                    anomaly_type: "SUSPICIOUS_STRING".to_string(),
                                    details: format!(
                                        "Found '{}' in memory at 0x{:X} (protection: {})",
                                        sig,
                                        address,
                                        protection_to_string(mbi.Protect)
                                    ),
                                });
                                result.anomalies_found += 1;
                            }
                        }

                        for gui_str in CHEAT_GUI_STRINGS {
                            if content.contains(gui_str) {
                                result.anomalies.push(MemoryAnomaly {
                                    anomaly_type: "CHEAT_GUI_STRING".to_string(),
                                    details: format!(
                                        "Found cheat GUI string '{}' at 0x{:X}",
                                        gui_str, address
                                    ),
                                });
                                result.anomalies_found += 1;
                            }
                        }
                    }
                }

                address += mbi.RegionSize;
            }

            CloseHandle(process);
        }
    }

    if result.anomalies_found > 5 {
        result.verdict = "suspicious".to_string();
    } else if result.anomalies_found > 0 {
        result.verdict = "warning".to_string();
    }

    result
}

pub fn scan_process_memory(pid: u32) -> ScanResult {
    let start = std::time::Instant::now();
    let mut result = ScanResult {
        total_strings: 0,
        suspicious_strings: 0,
        scan_time_ms: 0,
        strings: Vec::new(),
    };

    result.scan_time_ms = start.elapsed().as_millis() as u64;
    result
}

