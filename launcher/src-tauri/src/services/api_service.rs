use crate::models::state::GitHubRelease;
use reqwest::Client;

const API_KEY: &str = "d2b5ae9308d3278d541b37ff0cca11cf2c9296b101c12c156742a38b87813a4b";
const API_BASE_URL: &str = "https://cs2checker.ru/api";
const API_PUBLIC_URL: &str = "https://cs2checker.ru/api/public";
const GITHUB_OWNER: &str = "cr1stal12444";
const GITHUB_REPO: &str = "Cs2-checker";
const GITHUB_API_BASE: &str = "https://api.github.com/repos";
const USER_AGENT: &str = "CS2Checker-Launcher/2.0";

pub async fn check_status() -> Result<bool, String> {
    let client = build_client()?;

    let response = client
        .get(API_PUBLIC_URL)
        .header("X-API-Key", API_KEY)
        .send()
        .await
        .map_err(|e| format!("API check failed: {}", e))?;

    Ok(response.status().is_success())
}

pub async fn get_checker_download_url() -> Result<String, String> {
    Ok(format!("{}/download/cs2checker", API_BASE_URL))
}

pub async fn get_tools_download_url() -> Result<String, String> {
    Ok(format!("{}/download/tools", API_BASE_URL))
}

pub async fn get_github_release() -> Result<GitHubRelease, String> {
    let client = build_client()?;
    let url = format!("{}/{}/{}/releases/tags/1.0", GITHUB_API_BASE, GITHUB_OWNER, GITHUB_REPO);

    let response = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| format!("GitHub API request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned status: {}", response.status()));
    }

    let release: GitHubRelease = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub release: {}", e))?;

    Ok(release)
}

pub fn get_obfuscated_api_url() -> String {
    use base64::Engine;
    let encoded = "aHR0cHM6Ly9jczJjaGVja2VyLnJ1L2FwaS9wdWJsaWM=";
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .unwrap_or_default();
    String::from_utf8(decoded).unwrap_or_else(|_| API_PUBLIC_URL.to_string())
}

fn build_client() -> Result<Client, String> {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))
}

