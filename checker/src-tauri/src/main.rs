#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod discord_rpc;
mod frontend;
mod modules;
mod security;
mod services;

use crate::frontend::commands::*;
use crate::security::protection;
use tauri::Manager;

fn main() {
    env_logger::init();

    protection::init();

    std::thread::spawn(|| {
        discord_rpc::start();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            scan_commands::scan_files,
            scan_commands::stop_scan,
            scan_commands::load_database,
            scan_commands::is_database_loaded,
            scan_commands::quick_folder_scan,
            steam_commands::get_steam_accounts,
            steam_commands::get_steam_status,
            steam_commands::is_steam_running,
            steam_commands::get_player_profile,
            memory_commands::scan_game_memory,
            memory_commands::scan_process_memory,
            memory_commands::scan_process_handles,
            memory_commands::full_game_scan,
            memory_commands::get_process_info,
            system::get_system_info,
            system::get_system_metrics,
            system::get_system_drives,
            system::check_security,
            system::usb::get_usb_history,
            system::get_recent_apps,
            game_commands::check_cs2,
            game_commands::find_cs2_process,
            prefetch_commands::open_file_location,
            tools_commands::launch_tool,
            tools_commands::check_tools_available,
            scan_commands::scan_browser_history,
            scan_commands::load_customization_from_dat,
            scan_commands::check_customization_exists,
            scan_commands::apply_customization_to_window,
            scan_commands::send_telegram_report,
            scan_commands::open_folder_in_explorer,
            scan_commands::exit,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::apply_mica;
                let _ = apply_mica(&window, None);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

