#![allow(unused_imports)]
mod hd_interface;
mod tauri_components;

use hd_interface::process_hd;
use histdiff_core::*;
use tauri_components::{init_logger, open_control_selector_win, open_logging_window};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            init_logger(app.handle().clone()).expect("Failed to initialize logger");
            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            open_control_selector_win,
            process_hd,
            open_logging_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
