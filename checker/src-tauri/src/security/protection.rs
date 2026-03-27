use std::process::Command;

#[cfg(target_os = "windows")]
use winapi::um::debugapi::{IsDebuggerPresent, CheckRemoteDebuggerPresent};
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::GetCurrentProcess;

const RE_TOOLS: &[&str] = &[
    "x64dbg.exe",
    "x32dbg.exe",
    "ida.exe",
    "ida64.exe",
    "ollydbg.exe",
    "windbg.exe",
    "ghidra.exe",
    "processhacker.exe",
    "cheatengine-x86_64.exe",
    "cheatengine.exe",
    "dnspy.exe",
    "de4dot.exe",
    "ilspy.exe",
];

const VM_BIOS_NAMES: &[&str] = &[
    "vmware",
    "virtualbox",
    "virtual",
    "qemu",
    "kvm",
];

const VM_MAC_PREFIXES: &[&str] = &[
    "00:05:69", // VMware
    "00:0C:29", // VMware
    "00:1C:14", // VMware
    "00:50:56", // VMware
    "08:00:27", // VirtualBox
    "00:15:5D", // Hyper-V
];

const HOOK_PATTERNS: &[&str] = &[
    "JMP_REL32",
    "JMP_SHORT",
    "JMP_ABS_INDIRECT",
    "MOV_RAX_ADDR_JMP",
    "PUSH_RET",
    "RET_PATCH",
];

const MONITORED_NT_APIS: &[&str] = &[
    "NtReadVirtualMemory",
    "NtWriteVirtualMemory",
    "NtProtectVirtualMemory",
    "NtOpenProcess",
    "NtCreateThreadEx",
    "NtQueueApcThread",
    "NtSetContextThread",
    "NtSuspendThread",
    "NtResumeThread",
    "NtQueryVirtualMemory",
];

pub fn init() {
    #[cfg(not(debug_assertions))]
    {
        if check_debugger() {
            log::error!("Debugger detected");
            std::process::exit(1);
        }

        if check_remote_debugger() {
            log::error!("Remote debugger detection");
            std::process::exit(1);
        }

        if check_nt_global_flag() {
            log::error!("NtGlobalFlag check failed");
            std::process::exit(1);
        }

        if check_analysis_tools() {
            log::error!("Analysis tools detected");
            std::process::exit(1);
        }
    }
}

fn check_debugger() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe { IsDebuggerPresent() != 0 }
    }
    #[cfg(not(target_os = "windows"))]
    false
}

fn check_remote_debugger() -> bool {
    #[cfg(target_os = "windows")]
    {
        let mut debugger_present: i32 = 0;
        unsafe {
            CheckRemoteDebuggerPresent(GetCurrentProcess(), &mut debugger_present);
        }
        debugger_present != 0
    }
    #[cfg(not(target_os = "windows"))]
    false
}

fn check_nt_global_flag() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let peb: *const u8;
            std::arch::asm!(
                "mov {}, gs:[0x60]",
                out(reg) peb,
            );
            if peb.is_null() {
                return false;
            }
            let flags = *(peb.add(0xBC) as *const u32);
            (flags & 0x70) != 0
        }
    }
    #[cfg(not(target_os = "windows"))]
    false
}

fn check_analysis_tools() -> bool {
    let output = Command::new("tasklist")
        .args(&["/fo", "csv", "/nh"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        for tool in RE_TOOLS {
            if stdout.contains(&tool.to_lowercase()) {
                return true;
            }
        }
    }

    false
}

pub fn check_vm() -> bool {
    if let Ok(output) = Command::new("reg")
        .args(&["query", r"HKLM\HARDWARE\DESCRIPTION\System\BIOS", "/v", "SystemProductName"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        for name in VM_BIOS_NAMES {
            if stdout.contains(name) {
                return true;
            }
        }
    }

    if let Ok(output) = Command::new("wmic")
        .args(&["computersystem", "get", "model,manufacturer"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if stdout.contains("virtual machine") || stdout.contains("hyper-v") {
            return true;
        }
    }

    if let Ok(output) = Command::new("getmac").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for prefix in VM_MAC_PREFIXES {
            if stdout.contains(prefix) {
                return true;
            }
        }
    }

    false
}

pub fn check_nt_hooks() -> Vec<String> {
    let mut hooked = Vec::new();

    #[cfg(target_os = "windows")]
    {
        let ntdll = unsafe {
            winapi::um::libloaderapi::GetModuleHandleA(b"ntdll.dll\0".as_ptr() as *const i8)
        };

        if ntdll.is_null() {
            return hooked;
        }

        for api_name in MONITORED_NT_APIS {
            let func_name = std::ffi::CString::new(*api_name).unwrap();
            let addr = unsafe {
                winapi::um::libloaderapi::GetProcAddress(ntdll, func_name.as_ptr())
            };

            if !addr.is_null() {
                let bytes = unsafe { std::slice::from_raw_parts(addr as *const u8, 8) };

                if bytes[0] == 0xE9 { // JMP rel32
                    hooked.push(format!("{}: JMP_REL32", api_name));
                } else if bytes[0] == 0xEB { // JMP short
                    hooked.push(format!("{}: JMP_SHORT", api_name));
                } else if bytes[0] == 0xFF && bytes[1] == 0x25 { // JMP [addr]
                    hooked.push(format!("{}: JMP_ABS_INDIRECT", api_name));
                } else if bytes[0] == 0x48 && bytes[1] == 0xB8 { // MOV RAX, addr
                    hooked.push(format!("{}: MOV_RAX_ADDR_JMP", api_name));
                } else if bytes[0] == 0x68 { // PUSH addr
                    hooked.push(format!("{}: PUSH_RET", api_name));
                } else if bytes[0] == 0xC3 { // RET at start
                    hooked.push(format!("{}: RET_PATCH", api_name));
                }
            }
        }
    }

    hooked
}

