use tauri::command;
use std::path::PathBuf;
use std::process::Command;

const AVAILABLE_TOOLS: &[&str] = &[
    "BrowserDownloadsView.exe",
    "CachedProgramsList.exe",
    "ExecutedProgramsList.exe",
    "JournalTrace.exe",
    "LastActivityView.exe",
    "PreviousFilesRecovery.exe",
    "SystemInformer.exe",
    "USBDeview.exe",
    "WinPrefetchView.exe",
    "everything.exe",
    "shellbag_analyzer_cleaner.exe",
];

#[command]
pub async fn launch_tool(tool_name: String) -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent().unwrap().to_path_buf();

    let tool_path = exe_dir.join("Tools").join(&tool_name);

    if !tool_path.exists() {
        return Err(format!("Tool not found: {}", tool_name));
    }

    Command::new("cmd")
        .args(&["/C", &tool_path.to_string_lossy()])
        .current_dir(exe_dir.join("Tools"))
        .spawn()
        .map_err(|e| format!("Failed to launch {}: {}", tool_name, e))?;

    Ok(())
}

#[command]
pub async fn check_tools_available() -> Result<Vec<String>, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent().unwrap().to_path_buf();

    let tools_dir = exe_dir.join("Tools");

    let available = AVAILABLE_TOOLS.iter()
        .filter(|tool| tools_dir.join(tool).exists())
        .map(|s| s.to_string())
        .collect();

    Ok(available)
}

