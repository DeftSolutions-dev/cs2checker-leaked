use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherState {
    pub auto_update: bool,
    pub close_after_launch: bool,
    pub show_progress: bool,
    pub always_on_top: bool,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub total_size: u64,
    pub downloaded: u64,
    pub speed: f64,
    pub eta: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherInfo {
    pub version: String,
    pub app_name: String,
    pub checker_exists: bool,
    pub tools_exist: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub assets: Vec<GitHubAsset>,
    pub published_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationData {
    pub branding: Branding,
    pub assets: Assets,
    pub features: Features,
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

