use tauri::command;
use crate::modules::game_check;
use crate::services::memory_scanner;

#[command]
pub async fn scan_game_memory() -> Result<memory_scanner::MemoryScanResult, String> {
    let pid = game_check::find_cs2_process()
        .ok_or("CS2 process not found")?;

    Ok(tokio::task::spawn_blocking(move || {
        memory_scanner::scan_game_memory(pid)
    }).await.map_err(|e| e.to_string())?)
}

#[command]
pub async fn scan_process_memory(pid: u32) -> Result<memory_scanner::ScanResult, String> {
    Ok(tokio::task::spawn_blocking(move || {
        memory_scanner::scan_process_memory(pid)
    }).await.map_err(|e| e.to_string())?)
}

#[command]
pub async fn scan_process_handles(pid: u32) -> Result<memory_scanner::HandleScanResult, String> {
    Ok(memory_scanner::HandleScanResult {
        total_handles: 0,
        suspicious_handles: 0,
        handles: Vec::new(),
    })
}

#[command]
pub async fn full_game_scan() -> Result<memory_scanner::MemoryScanResult, String> {
    let pid = game_check::find_cs2_process()
        .ok_or("CS2 process not found")?;

    Ok(tokio::task::spawn_blocking(move || {
        memory_scanner::scan_game_memory(pid)
    }).await.map_err(|e| e.to_string())?)
}

#[command]
pub async fn get_process_info(pid: u32) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "pid": pid,
        "is_running": true,
    }))
}

