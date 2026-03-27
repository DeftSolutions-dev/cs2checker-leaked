use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, DiskExt, NetworkExt, NetworksExt, System, SystemExt};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_used_gb: f64,
    pub memory_total_gb: f64,
    pub memory_percent: f32,
    pub disk_usage: f64,
    pub disk_activity: f64,
    pub network_usage: f64,
    pub network_down: f64,
    pub network_up: f64,
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    pub name: String,
    pub mount_point: String,
    pub drive_type: String,
    pub is_removable: bool,
    pub total_gb: f64,
    pub free_gb: f64,
}

pub fn get_system_metrics() -> SystemMetrics {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_total = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let memory_used = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let memory_percent = (memory_used / memory_total * 100.0) as f32;

    let mut total_disk = 0u64;
    let mut used_disk = 0u64;
    for disk in sys.disks() {
        total_disk += disk.total_space();
        used_disk += disk.total_space() - disk.available_space();
    }

    let disk_usage = if total_disk > 0 {
        used_disk as f64 / total_disk as f64 * 100.0
    } else {
        0.0
    };

    let mut network_down = 0f64;
    let mut network_up = 0f64;
    for (_, data) in sys.networks() {
        network_down += data.received() as f64;
        network_up += data.transmitted() as f64;
    }

    let temperature = get_temperature();

    SystemMetrics {
        cpu_usage,
        memory_used_gb: memory_used,
        memory_total_gb: memory_total,
        memory_percent,
        disk_usage,
        disk_activity: 0.0,
        network_usage: network_down + network_up,
        network_down: network_down / (1024.0 * 1024.0),
        network_up: network_up / (1024.0 * 1024.0),
        temperature,
    }
}

pub fn get_system_drives() -> Vec<DriveInfo> {
    let sys = System::new_all();
    let mut drives = Vec::new();

    for disk in sys.disks() {
        let total_gb = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let free_gb = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let is_removable = disk.is_removable();

        drives.push(DriveInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            drive_type: if is_removable {
                "USB Drive".to_string()
            } else {
                "Local Disk".to_string()
            },
            is_removable,
            total_gb,
            free_gb,
        });
    }

    drives
}

pub fn get_os_install_date() -> Option<String> {
    let output = Command::new("powershell")
        .args(&["-Command", "(Get-CimInstance Win32_OperatingSystem).InstallDate.ToString('dd.MM.yyyy')"])
        .output()
        .ok()?;

    let date = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if date.is_empty() { None } else { Some(date) }
}

fn get_temperature() -> Option<f64> {
    let output = Command::new("powershell")
        .args(&["-Command", "Get-WmiObject -Namespace root\\WMI -Class MSAcpi_ThermalZoneTemperature -ErrorAction SilentlyContinue | Select-Object -First 1 -ExpandProperty CurrentTemperature"])
        .output()
        .ok()?;

    let temp_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let raw_temp: f64 = temp_str.parse().ok()?;
    Some((raw_temp / 10.0) - 273.15)
}

pub fn get_screen_info() -> Vec<String> {
    let output = Command::new("powershell")
        .args(&["-Command", "[System.Windows.Forms.Screen]::AllScreens | ForEach-Object { $_.Bounds.Width.ToString() + 'x' + $_.Bounds.Height.ToString() }"])
        .output();

    match output {
        Ok(out) => {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
        Err(_) => Vec::new(),
    }
}

