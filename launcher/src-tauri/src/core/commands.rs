use tauri::{command, AppHandle, Emitter};
use crate::core::launcher;
use crate::models::state::{CustomizationData, LauncherInfo};
use crate::services::api_service;

#[command]
pub async fn load_customization(_app: AppHandle) -> Result<CustomizationData, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .unwrap()
        .to_path_buf();
    let dat_path = exe_dir.join("customization.dat");

    if !dat_path.exists() {
        return Err("customization.dat not found".to_string());
    }

    let data = std::fs::read(&dat_path)
        .map_err(|e| format!("Failed to read customization.dat: {}", e))?;

    let decrypted = crate::security::polymorphic::decrypt_customization(&data)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    let customization: CustomizationData = serde_json::from_slice(&decrypted)
        .map_err(|e| format!("Failed to parse customization: {}", e))?;

    Ok(customization)
}

#[command]
pub async fn start_launch_process(app: AppHandle) -> Result<(), String> {
    launcher::launch(app).await
}

#[command]
pub async fn get_launcher_info(app: AppHandle) -> Result<LauncherInfo, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .unwrap()
        .to_path_buf();

    let checker_path = exe_dir.join("cs2checker.exe");
    let tools_path = exe_dir.join("Tools");

    Ok(LauncherInfo {
        version: "1.0.0".to_string(),
        app_name: "DETI00YKH CHECKER".to_string(),
        checker_exists: checker_path.exists(),
        tools_exist: tools_path.exists(),
    })
}

#[command]
pub async fn check_api_status() -> Result<bool, String> {
    api_service::check_status().await
}

#[command]
pub async fn open_url(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| format!("Failed to open URL: {}", e))
}

#[command]
pub async fn open_cs2checker_folder() -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .unwrap()
        .to_path_buf();

    open::that(&exe_dir).map_err(|e| format!("Failed to open folder: {}", e))
}

