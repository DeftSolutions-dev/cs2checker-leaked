use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const STEAM_API_KEY: &str = "CS2_CHECKER_SECURE_KEY_2024_V1"; // Заменяется реальным ключом
const STEAM_API_BASE: &str = "https://api.steampowered.com";

const STEAM_PATHS: &[&str] = &[
    r"C:\Program Files (x86)\Steam",
    r"C:\Program Files\Steam",
    r"D:\Steam",
    r"E:\Steam",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamAccount {
    pub steam_id: String,
    pub account_name: String,
    pub persona_name: String,
    pub last_login: String,
    pub is_online: bool,
    pub avatar_url: String,
    pub vac_status: bool,
    pub cs2_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub steam_id: String,
    pub persona_name: String,
    pub profile_url: String,
    pub avatar: String,
    pub real_name: Option<String>,
    pub country_code: Option<String>,
    pub state_code: Option<String>,
    pub city_id: Option<u32>,
    pub time_created: Option<u64>,
    pub last_logoff: Option<u64>,
    pub persona_state: u32,
    pub primary_clan_id: Option<String>,
    pub game_extra_info: Option<String>,
    pub game_id: Option<String>,
}

pub fn get_steam_accounts() -> Vec<SteamAccount> {
    let mut accounts = Vec::new();

    for steam_path in STEAM_PATHS {
        let vdf_path = PathBuf::from(steam_path).join("config").join("loginusers.vdf");
        if vdf_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&vdf_path) {
                accounts.extend(parse_login_users_vdf(&content));
            }
        }
    }

    if accounts.is_empty() {
        if let Some(path) = get_steam_path_from_registry() {
            let vdf_path = PathBuf::from(&path).join("config").join("loginusers.vdf");
            if vdf_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&vdf_path) {
                    accounts.extend(parse_login_users_vdf(&content));
                }
            }
        }
    }

    accounts
}

fn parse_login_users_vdf(content: &str) -> Vec<SteamAccount> {
    let mut accounts = Vec::new();
    let mut current_id = String::new();
    let mut current_name = String::new();
    let mut current_persona = String::new();

    for line in content.lines() {
        let trimmed = line.trim().trim_matches('"');
        if trimmed.len() == 17 && trimmed.chars().all(|c| c.is_ascii_digit()) {
            if !current_id.is_empty() {
                accounts.push(SteamAccount {
                    steam_id: current_id.clone(),
                    account_name: current_name.clone(),
                    persona_name: current_persona.clone(),
                    last_login: String::new(),
                    is_online: false,
                    avatar_url: String::new(),
                    vac_status: false,
                    cs2_hours: 0.0,
                });
            }
            current_id = trimmed.to_string();
            current_name.clear();
            current_persona.clear();
        }

        if line.contains("\"AccountName\"") {
            current_name = extract_vdf_value(line);
        }
        if line.contains("\"PersonaName\"") {
            current_persona = extract_vdf_value(line);
        }
    }

    if !current_id.is_empty() {
        accounts.push(SteamAccount {
            steam_id: current_id,
            account_name: current_name,
            persona_name: current_persona,
            last_login: String::new(),
            is_online: false,
            avatar_url: String::new(),
            vac_status: false,
            cs2_hours: 0.0,
        });
    }

    accounts
}

fn extract_vdf_value(line: &str) -> String {
    let parts: Vec<&str> = line.split('"').collect();
    if parts.len() >= 4 {
        parts[3].to_string()
    } else {
        String::new()
    }
}

fn get_steam_path_from_registry() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        use winapi::um::winreg::*;
        use winapi::um::winnt::KEY_READ;
        use std::ptr;

        let subkey = "SOFTWARE\\Valve\\Steam\0".encode_utf16().collect::<Vec<u16>>();
        let mut hkey: winapi::shared::minwindef::HKEY = ptr::null_mut();

        unsafe {
            if RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                subkey.as_ptr(),
                0,
                KEY_READ,
                &mut hkey,
            ) == 0 {
                let value_name = "InstallPath\0".encode_utf16().collect::<Vec<u16>>();
                let mut buf: [u16; 260] = [0; 260];
                let mut buf_len: u32 = (buf.len() * 2) as u32;
                let mut reg_type: u32 = 0;

                if RegQueryValueExW(
                    hkey,
                    value_name.as_ptr(),
                    ptr::null_mut(),
                    &mut reg_type,
                    buf.as_mut_ptr() as *mut u8,
                    &mut buf_len,
                ) == 0 {
                    let path = String::from_utf16_lossy(
                        &buf[..buf.iter().position(|&c| c == 0).unwrap_or(0)]
                    );
                    RegCloseKey(hkey);
                    return Some(path);
                }
                RegCloseKey(hkey);
            }
        }
    }
    None
}

pub async fn get_player_profile(steam_id: &str) -> Result<PlayerProfile, String> {
    let client = Client::new();
    let url = format!(
        "{}/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
        STEAM_API_BASE, STEAM_API_KEY, steam_id
    );

    let response: serde_json::Value = client
        .get(&url)
        .header("User-Agent", "Cs2Checker/2.0")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let player = response["response"]["players"]
        .as_array()
        .and_then(|arr| arr.first())
        .ok_or("Player not found")?;

    Ok(PlayerProfile {
        steam_id: player["steamid"].as_str().unwrap_or("").to_string(),
        persona_name: player["personaname"].as_str().unwrap_or("").to_string(),
        profile_url: player["profileurl"].as_str().unwrap_or("").to_string(),
        avatar: player["avatarfull"].as_str().unwrap_or("").to_string(),
        real_name: player["realname"].as_str().map(String::from),
        country_code: player["loccountrycode"].as_str().map(String::from),
        state_code: player["locstatecode"].as_str().map(String::from),
        city_id: player["loccityid"].as_u64().map(|v| v as u32),
        time_created: player["timecreated"].as_u64(),
        last_logoff: player["lastlogoff"].as_u64(),
        persona_state: player["personastate"].as_u64().unwrap_or(0) as u32,
        primary_clan_id: player["primaryclanid"].as_str().map(String::from),
        game_extra_info: player["gameextrainfo"].as_str().map(String::from),
        game_id: player["gameid"].as_str().map(String::from),
    })
}

pub async fn get_vac_status(steam_id: &str) -> Result<bool, String> {
    let client = Client::new();
    let url = format!(
        "{}/ISteamUser/GetPlayerBans/v1/?key={}&steamids={}",
        STEAM_API_BASE, STEAM_API_KEY, steam_id
    );

    let response: serde_json::Value = client
        .get(&url)
        .header("User-Agent", "Cs2Checker/2.0")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let vac_banned = response["players"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|p| p["VACBanned"].as_bool())
        .unwrap_or(false);

    Ok(vac_banned)
}

pub async fn get_cs2_hours(steam_id: &str) -> Result<f64, String> {
    let client = Client::new();
    let url = format!(
        "{}/IPlayerService/GetOwnedGames/v1/?key={}&steamid={}&include_appinfo=1",
        STEAM_API_BASE, STEAM_API_KEY, steam_id
    );

    let response: serde_json::Value = client
        .get(&url)
        .header("User-Agent", "Cs2Checker/2.0")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let games = response["response"]["games"]
        .as_array()
        .ok_or("No games data")?;

    for game in games {
        if game["appid"].as_u64() == Some(730) {
            let minutes = game["playtime_forever"].as_u64().unwrap_or(0);
            return Ok(minutes as f64 / 60.0);
        }
    }

    Ok(0.0)
}

