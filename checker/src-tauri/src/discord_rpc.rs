use discord_rich_presence::{
    activity::{self, Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::time::{SystemTime, UNIX_EPOCH};

const DISCORD_APP_ID: &str = "com.cs2checker.app";

const STATUSES: &[&str] = &[
    "Checking CS2 players...",
    "Hunting cheaters...",
    "Scanning memories...",
    "Analyzing browser history...",
    "Trust Factor: calculating...",
    "VAC-wave incoming...",
];

pub fn start() {
    let mut client = match DiscordIpcClient::new(DISCORD_APP_ID) {
        Ok(c) => c,
        Err(e) => {
            log::warn!("Failed to create Discord IPC client: {}", e);
            return;
        }
    };

    if let Err(e) = client.connect() {
        log::warn!("Failed to connect to Discord: {}", e);
        return;
    }

    log::info!("Discord RPC started successfully with meme statuses!");

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let mut status_index = 0;

    loop {
        let status = STATUSES[status_index % STATUSES.len()];

        let activity = Activity::new()
            .state(status)
            .details("CS2Checker v2.0")
            .assets(
                Assets::new()
                    .large_image("logo")
                    .large_text("CS2Checker"),
            )
            .timestamps(Timestamps::new().start(start_time));

        if let Err(e) = client.set_activity(activity) {
            log::warn!("Failed to set Discord activity: {}", e);
            break;
        }

        status_index += 1;
        std::thread::sleep(std::time::Duration::from_secs(30));
    }

    let _ = client.close();
}

