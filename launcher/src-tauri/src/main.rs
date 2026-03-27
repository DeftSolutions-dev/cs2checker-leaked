#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod models;
mod security;
mod services;

use crate::core::commands;
use crate::security::{anti_disasm, polymorphic};
use tauri::Manager;

fn main() {
    anti_disasm::init();
    polymorphic::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_customization,
            commands::start_launch_process,
            commands::get_launcher_info,
            commands::check_api_status,
            commands::open_url,
            commands::open_cs2checker_folder,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::apply_mica;
                let _ = apply_mica(&window, None);
            }

            #[cfg(not(debug_assertions))]
            {
                let js_code = r#"
                    ['log', 'debug', 'info', 'warn', 'error', 'assert', 'clear',
                     'count', 'countReset', 'dir', 'dirxml', 'group', 'groupCollapsed',
                     'groupEnd', 'profile', 'profileEnd', 'table', 'time', 'timeEnd',
                     'timeLog', 'timeStamp', 'trace'].forEach(function(method) {
                        window.console[method] = function() {};
                    });
                    setInterval(function() {
                        if (window.outerHeight - window.innerHeight > 200 ||
                            window.outerWidth - window.innerWidth > 200) {
                            window.__TAURI__.process.exit(1);
                        }
                    }, 1000);
                "#;
                let _ = window.eval(js_code);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

