use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use rayon::prelude::*;

use super::detectors;
use super::file_scanner;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSettings {
    pub intensity: u32,            // 1-3
    pub use_icon_analysis: bool,
    pub use_signature_analysis: bool,
    pub selected_drives: Vec<String>,
    pub max_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub files_scanned: u64,
    pub threats_found: u32,
    pub current_path: String,
    pub is_complete: bool,
}

static SCAN_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn start_scan(
    settings: ScanSettings,
    stop_flag: Arc<AtomicBool>,
) -> Vec<file_scanner::FileThreat> {
    SCAN_RUNNING.store(true, Ordering::SeqCst);

    let mut all_threats = Vec::new();

    for drive in &settings.selected_drives {
        if stop_flag.load(Ordering::SeqCst) {
            break;
        }

        let threats = file_scanner::scan_drive(
            drive,
            settings.max_depth,
            settings.use_signature_analysis,
            &stop_flag,
        );

        all_threats.extend(threats);
    }

    if !stop_flag.load(Ordering::SeqCst) {
        all_threats.extend(detectors::exloader_detector::detect());
        all_threats.extend(detectors::memesense_detector::detect());
        all_threats.extend(detectors::midnight_detector::detect());
        all_threats.extend(detectors::xone_detector::detect());
    }

    SCAN_RUNNING.store(false, Ordering::SeqCst);
    all_threats
}

pub fn stop_scan(stop_flag: &Arc<AtomicBool>) {
    stop_flag.store(true, Ordering::SeqCst);
}

pub fn is_scanning() -> bool {
    SCAN_RUNNING.load(Ordering::SeqCst)
}

