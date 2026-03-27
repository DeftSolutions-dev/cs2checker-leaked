use aes::cipher::{KeyIvInit, StreamCipher};
use aes::Aes256;
use serde::{Deserialize, Serialize};

type Aes256Ctr = ctr::Ctr128BE<Aes256>;

const ENCRYPTION_KEY: &[u8; 32] = b"CS2_CHECKER_SECURE_KEY_2024_V1\x00\x00";
const ENCRYPTION_KEY_HEX: &str = "9D93EA929B9AE9929C99E898EF9C92EE9D9CE8989A9C9BEEEEEE9A929BEA9A92";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationData {
    pub branding: Branding,
    pub assets: Assets,
    pub features: Features,
    pub theme: Option<ThemeColors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branding {
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "discordUrl")]
    pub discord_url: String,
    #[serde(rename = "websiteUrl")]
    pub website_url: String,
    #[serde(rename = "telegramUrl")]
    pub telegram_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assets {
    pub logo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "showVersion")]
    pub show_version: bool,
    #[serde(rename = "showDiscordButton")]
    pub show_discord_button: bool,
    #[serde(rename = "showWebsiteButton")]
    pub show_website_button: bool,
    #[serde(rename = "showTelegramButton")]
    pub show_telegram_button: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    #[serde(rename = "primaryColor")]
    pub primary_color: Option<String>,
    #[serde(rename = "secondaryColor")]
    pub secondary_color: Option<String>,
    #[serde(rename = "accentColor")]
    pub accent_color: Option<String>,
    #[serde(rename = "surfaceColor")]
    pub surface_color: Option<String>,
    #[serde(rename = "sidebarColor")]
    pub sidebar_color: Option<String>,
    #[serde(rename = "headerColor")]
    pub header_color: Option<String>,
    #[serde(rename = "textColor")]
    pub text_color: Option<String>,
    #[serde(rename = "mutedTextColor")]
    pub muted_text_color: Option<String>,
    #[serde(rename = "headingColor")]
    pub heading_color: Option<String>,
    #[serde(rename = "borderColor")]
    pub border_color: Option<String>,
    #[serde(rename = "dividerColor")]
    pub divider_color: Option<String>,
    #[serde(rename = "successColor")]
    pub success_color: Option<String>,
    #[serde(rename = "warningColor")]
    pub warning_color: Option<String>,
    #[serde(rename = "errorColor")]
    pub error_color: Option<String>,
    #[serde(rename = "infoColor")]
    pub info_color: Option<String>,
    #[serde(rename = "dangerColor")]
    pub danger_color: Option<String>,
    #[serde(rename = "safeColor")]
    pub safe_color: Option<String>,
    #[serde(rename = "scanningColor")]
    pub scanning_color: Option<String>,
    #[serde(rename = "discordButtonColor")]
    pub discord_button_color: Option<String>,
    #[serde(rename = "websiteButtonColor")]
    pub website_button_color: Option<String>,
    #[serde(rename = "telegramButtonColor")]
    pub telegram_button_color: Option<String>,
    #[serde(rename = "buttonGlowColor")]
    pub button_glow_color: Option<String>,
}

pub fn load_customization(dat_path: &str) -> Result<CustomizationData, String> {
    let data = std::fs::read(dat_path)
        .map_err(|e| format!("Failed to read customization.dat: {}", e))?;

    let decrypted = decrypt_data(&data)?;

    let customization: CustomizationData = serde_json::from_slice(&decrypted)
        .map_err(|e| format!("Failed to parse customization JSON: {}", e))?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    log::info!("[{}] Customization loaded successfully", timestamp);

    Ok(customization)
}

fn decrypt_data(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 16 {
        return Err("Encryption failed: data too short".to_string());
    }

    let iv = &data[..16];
    let ciphertext = &data[16..];

    let mut buf = ciphertext.to_vec();
    let mut cipher = Aes256Ctr::new(ENCRYPTION_KEY.into(), iv.into());
    cipher.apply_keystream(&mut buf);

    Ok(buf)
}

pub fn check_customization_exists(base_dir: &str) -> bool {
    let path = std::path::Path::new(base_dir).join("customization.dat");
    path.exists()
}

