use reqwest::Client;
use std::path::{Path, PathBuf};
use std::io::Write;
use tauri::{AppHandle, Emitter};
use crate::models::state::DownloadProgress;
use crate::services::api_service;

const USER_AGENT: &str = "CS2Checker-Launcher/2.0";
const MAX_RETRIES: u32 = 3;

pub async fn download_checker(app: &AppHandle, dest_dir: &Path) -> Result<(), String> {
    let dest_path = dest_dir.join("cs2checker.exe");
    let temp_path = dest_dir.join("temp").join("cs2checker.exe.tmp");

    std::fs::create_dir_all(dest_dir.join("temp")).ok();

    let website_url = api_service::get_checker_download_url().await?;
    log::info!("Website download URL: {}", website_url);

    match download_file(app, &website_url, &temp_path).await {
        Ok(_) => {
            std::fs::rename(&temp_path, &dest_path)
                .map_err(|e| format!("Failed to move checker: {}", e))?;
            return Ok(());
        }
        Err(e) => {
            log::warn!("Website download failed: {}", e);
        }
    }

    log::info!("Starting fallback download for: cs2checker.exe");
    match download_from_github(app, "cs2checker.exe", &temp_path).await {
        Ok(_) => {
            std::fs::rename(&temp_path, &dest_path)
                .map_err(|e| format!("Failed to move checker: {}", e))?;
            Ok(())
        }
        Err(e) => {
            log::error!("GitHub download failed: {}", e);
            Err(format!("Failed to download checker.exe from all sources: {}", e))
        }
    }
}

pub async fn download_tools(app: &AppHandle, dest_dir: &Path) -> Result<(), String> {
    let zip_path = dest_dir.join("temp").join("Tools.zip");
    let tools_dir = dest_dir.join("Tools");

    std::fs::create_dir_all(dest_dir.join("temp")).ok();

    let website_url = api_service::get_tools_download_url().await?;
    log::info!("Website download URL: {}", website_url);

    let download_result = match download_file(app, &website_url, &zip_path).await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::warn!("Website download failed: {}", e);
            log::info!("Starting fallback download for: Tools.zip");
            download_from_github(app, "Tools.zip", &zip_path).await
                .map_err(|e| {
                    log::error!("GitHub download failed: {}", e);
                    format!("Failed to download Tools.zip from all sources: {}", e)
                })
        }
    };

    download_result?;

    let file = std::fs::File::open(&zip_path)
        .map_err(|e| format!("Failed to open Tools.zip: {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip: {}", e))?;
    archive.extract(&dest_dir)
        .map_err(|e| format!("Failed to extract Tools.zip: {}", e))?;

    std::fs::remove_file(&zip_path).ok();
    std::fs::remove_dir_all(dest_dir.join("temp")).ok();

    Ok(())
}

async fn download_file(app: &AppHandle, url: &str, dest: &Path) -> Result<(), String> {
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| e.to_string())?;

    for attempt in 1..=MAX_RETRIES {
        log::info!("Download attempt {} for: {}", attempt, url);

        let response = client.get(url).send().await.map_err(|e| e.to_string())?;
        log::info!("Download response status: {}", response.status());

        if !response.status().is_success() {
            continue;
        }

        let total_size = response.content_length().unwrap_or(0);
        log::info!("Starting file download: {}", url);

        let mut file = std::fs::File::create(dest).map_err(|e| e.to_string())?;
        let bytes = response.bytes().await.map_err(|e| e.to_string())?;

        file.write_all(&bytes).map_err(|e| e.to_string())?;

        log::info!("File download completed: {}", url);

        app.emit("progress-update", DownloadProgress {
            total_size,
            downloaded: total_size,
            speed: 0.0,
            eta: 0,
        }).ok();

        return Ok(());
    }

    Err(format!("Download failed after {} attempts", MAX_RETRIES))
}

async fn download_from_github(app: &AppHandle, asset_name: &str, dest: &Path) -> Result<(), String> {
    let release = api_service::get_github_release().await?;

    let asset = release.assets.iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| format!("Asset '{}' not found in release", asset_name))?;

    download_file(app, &asset.browser_download_url, dest).await
}

