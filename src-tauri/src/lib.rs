#![allow(unused_imports)]
mod hd_interface;
mod tauri_components;

use hd_interface::process_hd;
use histdiff_core::*;
use tauri_components::open_control_selector_win;

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
        .invoke_handler(tauri::generate_handler![
            greet,
            open_control_selector_win,
            process_hd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
