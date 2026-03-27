use tauri::command;
use crate::modules::game_check;

#[command]
pub async fn check_cs2() -> Result<game_check::GameProcess, String> {
    Ok(game_check::check_cs2_running())
}

#[command]
pub async fn find_cs2_process() -> Result<Option<u32>, String> {
    Ok(game_check::find_cs2_process())
}

