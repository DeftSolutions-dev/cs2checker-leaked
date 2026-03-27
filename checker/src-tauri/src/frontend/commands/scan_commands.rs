use tauri::command;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

use crate::services::browser_history;
use crate::services::customization_service;
use crate::services::quick_folder_scan;
use crate::services::scanner::file_scanner::FileThreat;
use crate::services::scanner::scanner_engine::{self, ScanSettings};
use crate::services::telegram_reporter;

lazy_static::lazy_static! {
    static ref STOP_FLAG: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref DATABASE_LOADED: Mutex<bool> = Mutex::new(false);
    static ref SCAN_RESULTS: Mutex<Vec<FileThreat>> = Mutex::new(Vec::new());
}

#[command]
pub async fn scan_files(settings: ScanSettings) -> Result<Vec<FileThreat>, String> {
    STOP_FLAG.store(false, std::sync::atomic::Ordering::SeqCst);
    let stop = STOP_FLAG.clone();
    let results = tokio::task::spawn_blocking(move || {
        scanner_engine::start_scan(settings, stop)
    }).await.map_err(|e| e.to_string())?;

    *SCAN_RESULTS.lock().unwrap() = results.clone();
    Ok(results)
}

#[command]
pub async fn stop_scan() -> Result<(), String> {
    scanner_engine::stop_scan(&STOP_FLAG);
    Ok(())
}

#[command]
pub async fn load_database() -> Result<(), String> {
    *DATABASE_LOADED.lock().unwrap() = true;
    Ok(())
}

#[command]
pub async fn is_database_loaded() -> Result<bool, String> {
    Ok(*DATABASE_LOADED.lock().unwrap())
}

#[command]
pub async fn quick_folder_scan() -> Result<quick_folder_scan::QuickFolderScanResult, String> {
    Ok(tokio::task::spawn_blocking(|| {
        quick_folder_scan::scan_all_drives()
    }).await.map_err(|e| e.to_string())?)
}

#[command]
pub async fn scan_browser_history() -> Result<Vec<browser_history::BrowserHistoryEntry>, String> {
    Ok(tokio::task::spawn_blocking(|| {
        browser_history::scan_all_browsers()
    }).await.map_err(|e| e.to_string())?)
}

#[command]
pub async fn load_customization_from_dat() -> Result<customization_service::CustomizationData, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent().unwrap().to_path_buf();
    let dat_path = exe_dir.join("customization.dat");
    customization_service::load_customization(&dat_path.to_string_lossy())
}

#[command]
pub async fn check_customization_exists() -> Result<bool, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent().unwrap().to_path_buf();
    Ok(customization_service::check_customization_exists(&exe_dir.to_string_lossy()))
}

#[command]
pub async fn apply_customization_to_window() -> Result<(), String> {
    Ok(())
}

#[command]
pub async fn send_telegram_report(
    bot_token: String,
    chat_id: String,
    steam_id: String,
    persona_name: String,
    browser_hits: usize,
    memory_anomalies: usize,
    folder_threats: usize,
    prefetch_hits: usize,
    verdict: String,
) -> Result<(), String> {
    log::info!("[COMMAND] send_telegram_report called");

    let config = telegram_reporter::TelegramConfig { bot_token, chat_id };
    let report = telegram_reporter::format_report(
        &steam_id, &persona_name,
        browser_hits, memory_anomalies, folder_threats, prefetch_hits,
        &verdict,
    );

    match telegram_reporter::send_report(&config, &report).await {
        Ok(_) => {
            log::info!("[COMMAND] Telegram report sent successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("[COMMAND] Telegram report failed: {}", e);
            Err(e)
        }
    }
}

#[command]
pub async fn open_folder_in_explorer(path: String) -> Result<(), String> {
    open::that(&path).map_err(|e| e.to_string())
}

#[command]
pub async fn exit() {
    std::process::exit(0);
}

