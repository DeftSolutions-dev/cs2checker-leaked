use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserHistoryEntry {
    pub url: String,
    pub title: String,
    pub visit_count: i64,
    pub last_visit_time: String,
    pub browser: String,
    pub matched_domain: Option<String>,
}

const CHEAT_DOMAINS: &[&str] = &[
    "midnight.im",
    "neverlose.cc",
    "interium.ooo",
    "gamesense.pub",
    "aimware.net",
    "onetap.com",
    "vredux.com",
    "nixware.cc",
    "shark-hack.ru",
    "ww38.shark-hack.ru",
    "phoenix-hack.com",
    "unknowncheats.me",
    "mpgh.net",
    "elitepvpers.com",
    "ev0lve.xyz",
    "iniuria.us",
    "spirthack.me",
    "osiris.solutions",
    "ratpoison.cc",
];

const CHEAT_SEARCH_TERMS: &[&str] = &[
    "cs2 cheat",
    "cs2 hack",
    "counter strike 2 cheat",
    "counter strike 2 hack",
    "buy cs2 cheat",
    "free cs2 cheat",
    "cs2 aimbot",
    "cs2 wallhack",
    "cs2 esp",
    "cs2 triggerbot",
    "cs go cheat",
    "csgo hack",
    "neverlose",
    "midnight cheat",
    "onetap cheat",
    "gamesense",
    "aimware",
];

struct BrowserProfile {
    name: &'static str,
    history_path: &'static str,
}

const BROWSER_PROFILES: &[BrowserProfile] = &[
    BrowserProfile { name: "Chrome", history_path: r"Google\Chrome\User Data\Default\History" },
    BrowserProfile { name: "Brave", history_path: r"BraveSoftware\Brave-Browser\User Data\Default\History" },
    BrowserProfile { name: "Edge", history_path: r"Microsoft\Edge\User Data\Default\History" },
    BrowserProfile { name: "Opera", history_path: r"Opera Software\Opera Stable\History" },
    BrowserProfile { name: "Opera GX", history_path: r"Opera Software\Opera GX Stable\History" },
    BrowserProfile { name: "Vivaldi", history_path: r"Vivaldi\User Data\Default\History" },
    BrowserProfile { name: "Yandex Browser", history_path: r"Yandex\YandexBrowser\User Data\Default\History" },
    BrowserProfile { name: "Chromium", history_path: r"Chromium\User Data\Default\History" },
    BrowserProfile { name: "Torch Browser", history_path: r"Torch\User Data\Default\History" },
    BrowserProfile { name: "CocCoc Browser", history_path: r"CocCoc\Browser\User Data\Default\History" },
];

pub fn scan_all_browsers() -> Vec<BrowserHistoryEntry> {
    let mut results = Vec::new();
    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let app_data = std::env::var("APPDATA").unwrap_or_default();

    for profile in BROWSER_PROFILES {
        let db_path = PathBuf::from(&local_app_data).join(profile.history_path);
        if let Ok(entries) = scan_chromium_history(&db_path, profile.name) {
            results.extend(entries);
        }
    }

    let firefox_profiles = PathBuf::from(&app_data).join(r"Mozilla\Firefox\Profiles");
    if firefox_profiles.exists() {
        if let Ok(entries) = std::fs::read_dir(&firefox_profiles) {
            for entry in entries.flatten() {
                let places_db = entry.path().join("places.sqlite");
                if places_db.exists() {
                    if let Ok(history) = scan_firefox_history(&places_db) {
                        results.extend(history);
                    }
                }
            }
        }
    }

    let tor_path = PathBuf::from(&local_app_data)
        .join(r"TorBrowser\profile.default\places.sqlite");
    if tor_path.exists() {
        if let Ok(entries) = scan_firefox_history(&tor_path) {
            for mut entry in entries {
                entry.browser = "Tor Browser".to_string();
                results.push(entry);
            }
        }
    }

    let waterfox_profiles = PathBuf::from(&app_data).join(r"Waterfox\Profiles");
    if waterfox_profiles.exists() {
        if let Ok(entries) = std::fs::read_dir(&waterfox_profiles) {
            for entry in entries.flatten() {
                let places_db = entry.path().join("places.sqlite");
                if places_db.exists() {
                    if let Ok(mut history) = scan_firefox_history(&places_db) {
                        for h in &mut history {
                            h.browser = "Waterfox".to_string();
                        }
                        results.extend(history);
                    }
                }
            }
        }
    }

    let palemoon_profiles = PathBuf::from(&app_data).join(r"Moonchild Productions\Pale Moon\Profiles");
    if palemoon_profiles.exists() {
        if let Ok(entries) = std::fs::read_dir(&palemoon_profiles) {
            for entry in entries.flatten() {
                let places_db = entry.path().join("places.sqlite");
                if places_db.exists() {
                    if let Ok(mut history) = scan_firefox_history(&places_db) {
                        for h in &mut history {
                            h.browser = "Pale Moon".to_string();
                        }
                        results.extend(history);
                    }
                }
            }
        }
    }

    results
}

fn scan_chromium_history(db_path: &PathBuf, browser_name: &str) -> Result<Vec<BrowserHistoryEntry>, String> {
    if !db_path.exists() {
        return Ok(Vec::new());
    }

    let temp_path = std::env::temp_dir().join(format!("cs2checker_{}.db", browser_name));
    std::fs::copy(db_path, &temp_path).map_err(|e| e.to_string())?;

    let conn = Connection::open(&temp_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT url, title, visit_count, last_visit_time FROM urls ORDER BY last_visit_time DESC LIMIT 10000")
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    let rows = stmt.query_map([], |row| {
        Ok(BrowserHistoryEntry {
            url: row.get(0)?,
            title: row.get(1)?,
            visit_count: row.get(2)?,
            last_visit_time: row.get::<_, i64>(3)?.to_string(),
            browser: browser_name.to_string(),
            matched_domain: None,
        })
    }).map_err(|e| e.to_string())?;

    for row in rows.flatten() {
        let url_lower = row.url.to_lowercase();

        for domain in CHEAT_DOMAINS {
            if url_lower.contains(domain) {
                let mut entry = row.clone();
                entry.matched_domain = Some(domain.to_string());
                results.push(entry);
                break;
            }
        }

        for term in CHEAT_SEARCH_TERMS {
            if url_lower.contains(&term.replace(' ', "+")) || url_lower.contains(&term.replace(' ', "%20")) {
                let mut entry = row.clone();
                entry.matched_domain = Some(format!("search: {}", term));
                results.push(entry);
                break;
            }
        }
    }

    let _ = std::fs::remove_file(&temp_path);

    Ok(results)
}

fn scan_firefox_history(db_path: &PathBuf) -> Result<Vec<BrowserHistoryEntry>, String> {
    if !db_path.exists() {
        return Ok(Vec::new());
    }

    let temp_path = std::env::temp_dir().join("cs2checker_firefox.db");
    std::fs::copy(db_path, &temp_path).map_err(|e| e.to_string())?;

    let conn = Connection::open(&temp_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT url, title, visit_count, last_visit_date FROM moz_places ORDER BY last_visit_date DESC LIMIT 10000")
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();

    let rows = stmt.query_map([], |row| {
        Ok(BrowserHistoryEntry {
            url: row.get(0)?,
            title: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            visit_count: row.get(2)?,
            last_visit_time: row.get::<_, Option<i64>>(3)?.unwrap_or(0).to_string(),
            browser: "Firefox".to_string(),
            matched_domain: None,
        })
    }).map_err(|e| e.to_string())?;

    for row in rows.flatten() {
        let url_lower = row.url.to_lowercase();
        for domain in CHEAT_DOMAINS {
            if url_lower.contains(domain) {
                let mut entry = row.clone();
                entry.matched_domain = Some(domain.to_string());
                results.push(entry);
                break;
            }
        }
    }

    let _ = std::fs::remove_file(&temp_path);
    Ok(results)
}

