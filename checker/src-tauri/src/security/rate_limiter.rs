use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

lazy_static::lazy_static! {
    static ref RATE_LIMITS: Mutex<HashMap<String, Vec<Instant>>> = Mutex::new(HashMap::new());
}

const DEFAULT_MAX_REQUESTS: usize = 10;
const DEFAULT_WINDOW_SECS: u64 = 60;

pub fn check_rate_limit(action: &str) -> bool {
    check_rate_limit_custom(action, DEFAULT_MAX_REQUESTS, DEFAULT_WINDOW_SECS)
}

pub fn check_rate_limit_custom(action: &str, max_requests: usize, window_secs: u64) -> bool {
    let mut limits = RATE_LIMITS.lock().unwrap();
    let window = Duration::from_secs(window_secs);
    let now = Instant::now();

    let timestamps = limits.entry(action.to_string()).or_insert_with(Vec::new);

    timestamps.retain(|t| now.duration_since(*t) < window);

    if timestamps.len() >= max_requests {
        return false; // Rate limit exceeded
    }

    timestamps.push(now);
    true
}

