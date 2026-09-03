// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use std::sync::Mutex;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--minimized"])))
        .manage(commands::system::SystemState {
            sys: Mutex::new(sys),
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::get_system_stats,
            commands::power::power_action,
            commands::games::launch_game,
            commands::games::scan_games,
            commands::sensors::get_hwinfo_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
