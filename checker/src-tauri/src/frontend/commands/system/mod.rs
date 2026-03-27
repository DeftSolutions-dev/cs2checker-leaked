pub mod usb;

use tauri::command;
use crate::services::system_info;
use crate::services::game_analyzer;
use crate::security::protection;

#[command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    let os_install_date = system_info::get_os_install_date();
    let screens = system_info::get_screen_info();

    Ok(serde_json::json!({
        "os_install_date": os_install_date,
        "screens": screens,
        "is_vm": protection::check_vm(),
    }))
}

#[command]
pub async fn get_system_metrics() -> Result<system_info::SystemMetrics, String> {
    Ok(tokio::task::spawn_blocking(|| {
        system_info::get_system_metrics()
    }).await.map_err(|e| e.to_string())?)
}

#[command]
pub async fn get_system_drives() -> Result<Vec<system_info::DriveInfo>, String> {
    Ok(tokio::task::spawn_blocking(|| {
        system_info::get_system_drives()
    }).await.map_err(|e| e.to_string())?)
}

#[command]
pub async fn check_security() -> Result<serde_json::Value, String> {
    let hooked_apis = protection::check_nt_hooks();
    let is_vm = protection::check_vm();

    Ok(serde_json::json!({
        "is_vm": is_vm,
        "hooked_apis": hooked_apis,
        "hooks_detected": !hooked_apis.is_empty(),
    }))
}

#[command]
pub async fn get_recent_apps() -> Result<Vec<game_analyzer::RecentApp>, String> {
    Ok(tokio::task::spawn_blocking(|| {
        game_analyzer::get_recent_apps_from_bam()
    }).await.map_err(|e| e.to_string())?)
}

