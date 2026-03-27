
const SKIP_DIRS: &[&str] = &[
    "windows", "program files", "program files (x86)", "programdata",
    "system32", "syswow64", "winsxs", "perflogs", "recovery",
    "$recycle.bin", "system volume information", "msocache",
    "intel", "amd", "nvidia", "temp", "inetpub", "logs",
    "prefetch", "servicing", "all users", "default user",
    "microsoft", "microsoft corporation", "windowsapps",
    "packages", "systemapps", "windows defender", "windows security",
    ".git", "node_modules", "__pycache__",
];

const SUSPICIOUS_PATTERNS: &[&str] = &[
    "inject", "loader", "cheat", "hack", "bypass",
    "neverlose", "onetap", "gamesense", "aimware", "midnight",
    "nixware", "interium", "fatality", "skeet", "ev0lve",
    "spirthack", "iniuria", "osiris", "exloader", "memesense",
    "xone", "enigma", "en1gma", "sharkhack", "extrim",
    "aimstar", "aimmy", "ezfrags", "ratpoison", "charlatan",
    "weave", "legendware", "primordial", "pandora", "cryptic",
    "compkiller", "ekknod", "lunapaste", "whiskey",
    "s1mpleinternal", "zrk", "vac_bypass", "vac-bypass",
];

pub fn should_skip_directory(dir_name: &str) -> bool {
    let lower = dir_name.to_lowercase();
    SKIP_DIRS.iter().any(|skip| lower == *skip)
}

pub fn is_suspicious_filename(filename: &str) -> bool {
    let lower = filename.to_lowercase();

    if !lower.ends_with(".exe") && !lower.ends_with(".dll") && !lower.ends_with(".sys") {
        return false;
    }

    SUSPICIOUS_PATTERNS.iter().any(|pattern| lower.contains(pattern))
}

