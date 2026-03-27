use tauri::command;
use std::process::Command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbDevice {
    pub device_name: String,
    pub vendor_id: String,
    pub product_id: String,
    pub serial_number: String,
    pub last_connected: String,
}

#[command]
pub async fn get_usb_history() -> Result<Vec<UsbDevice>, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent().unwrap().to_path_buf();

    let usbdeview = exe_dir.join("Tools").join("USBDeview.exe");

    if !usbdeview.exists() {
        return Err("USBDeview.exe not found in Tools".to_string());
    }

    let temp_csv = std::env::temp_dir().join("cs2checker_usb.csv");

    Command::new(&usbdeview)
        .args(&["/scomma", &temp_csv.to_string_lossy()])
        .output()
        .map_err(|e| format!("Failed to run USBDeview: {}", e))?;

    let mut devices = Vec::new();

    if let Ok(content) = std::fs::read_to_string(&temp_csv) {
        for line in content.lines().skip(1) {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 5 {
                devices.push(UsbDevice {
                    device_name: fields[0].trim_matches('"').to_string(),
                    vendor_id: fields.get(6).unwrap_or(&"").trim_matches('"').to_string(),
                    product_id: fields.get(7).unwrap_or(&"").trim_matches('"').to_string(),
                    serial_number: fields.get(9).unwrap_or(&"").trim_matches('"').to_string(),
                    last_connected: fields.get(3).unwrap_or(&"").trim_matches('"').to_string(),
                });
            }
        }
    }

    let _ = std::fs::remove_file(&temp_csv);

    Ok(devices)
}

