use tauri::{AppHandle, Emitter};
use crate::services::api_service;
use crate::services::download_service;
use crate::models::state::DownloadProgress;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

const FILE_FRESHNESS_DAYS: u64 = 7;

pub async fn launch(app: AppHandle) -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .unwrap()
        .to_path_buf();

    let checker_path = exe_dir.join("cs2checker.exe");
    let tools_path = exe_dir.join("Tools");

    if needs_download(&checker_path) {
        log::info!("Need to download checker.exe");
        app.emit("progress-update", DownloadProgress {
            total_size: 0,
            downloaded: 0,
            speed: 0.0,
            eta: 0,
        }).ok();

        download_service::download_checker(&app, &exe_dir).await
            .map_err(|e| format!("Failed to download checker.exe from all sources: {}", e))?;
    }

    if !tools_path.exists() {
        log::info!("Need to download Tools");
        download_service::download_tools(&app, &exe_dir).await
            .map_err(|e| format!("Failed to download Tools.zip from all sources: {}", e))?;
    }

    Command::new(&checker_path)
        .current_dir(&exe_dir)
        .spawn()
        .map_err(|e| format!("Failed to launch cs2checker: {}", e))?;

    Ok(())
}

fn needs_download(path: &PathBuf) -> bool {
    if !path.exists() {
        log::info!("Checking if file needs download: {:?} - not found", path);
        return true;
    }

    if let Ok(metadata) = std::fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            let age = SystemTime::now()
                .duration_since(modified)
                .unwrap_or_default();
            let days = age.as_secs() / 86400;

            if days >= FILE_FRESHNESS_DAYS {
                log::info!("{} days old (>= {}), download needed", days, FILE_FRESHNESS_DAYS);
                return true;
            } else {
                log::info!("{} days old (< {}), download not needed", days, FILE_FRESHNESS_DAYS);
                return false;
            }
        }
    }

    true
}

