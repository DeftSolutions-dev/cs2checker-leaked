use tauri::command;
use crate::modules::game_check;
use crate::services::steam_service;

#[command]
pub async fn get_steam_accounts() -> Result<Vec<steam_service::SteamAccount>, String> {
    Ok(steam_service::get_steam_accounts())
}

#[command]
pub async fn get_steam_status() -> Result<bool, String> {
    Ok(game_check::is_steam_running())
}

#[command]
pub async fn is_steam_running() -> Result<bool, String> {
    Ok(game_check::is_steam_running())
}

#[command]
pub async fn get_player_profile(steam_id: String) -> Result<steam_service::PlayerProfile, String> {
    steam_service::get_player_profile(&steam_id).await
}

