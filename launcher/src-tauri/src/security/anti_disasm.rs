use std::process::Command;

const DEBUGGER_PROCESSES: &[&str] = &[
    "x64dbg.exe",
    "x32dbg.exe",
    "ollydbg.exe",
    "windbg.exe",
    "ida.exe",
    "ida64.exe",
    "cheatengine.exe",
];

const _DECOY_FILE_PATHS: &[&str] = &[
    r"C:\Windows\System32\config.dat",
    r"C:\ProgramData\license.key",
    r"C:\Users\Public\settings.ini",
];

const _DECOY_REGISTRY_KEYS: &[&str] = &[
    r"HKEY_LOCAL_MACHINE\SOFTWARE\FakeApp\License",
    r"HKEY_CURRENT_USER\Software\DecoyProgram\Settings",
    r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DummyService",
];

const _DECOY_URLS: &[&str] = &[
    "https://fake-api.example.com/auth",
    "https://decoy-server.net/validate",
    "https://dummy-endpoint.org/check",
];

pub fn init() {
    #[cfg(not(debug_assertions))]
    {
        if is_debugger_present() {
            log::error!("Security violation detected");
            std::process::exit(1);
        }

        if check_debugger_processes() {
            log::error!("Security violation detected");
            std::process::exit(1);
        }
    }
}

fn is_debugger_present() -> bool {
    #[cfg(target_os = "windows")]
    {
        extern "system" {
            fn IsDebuggerPresent() -> i32;
        }
        unsafe { IsDebuggerPresent() != 0 }
    }
    #[cfg(not(target_os = "windows"))]
    false
}

fn check_debugger_processes() -> bool {
    let output = Command::new("tasklist")
        .args(&["/fo", "csv", "/nh"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        for process in DEBUGGER_PROCESSES {
            if stdout.contains(&process.to_lowercase()) {
                return true;
            }
        }
    }

    false
}

#[allow(dead_code)]
fn fake_operation_decoy() {
    let _ = _DECOY_FILE_PATHS;
    let _ = _DECOY_REGISTRY_KEYS;
    let _ = _DECOY_URLS;
}

