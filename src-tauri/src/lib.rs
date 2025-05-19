#![allow(unused_imports)]
mod analysis;
mod hd_interface;
mod tauri_components;

use analysis::{cluster_hd, get_cluster_res};
use hd_interface::{process_hd, write_res, ClusterState, HistDiffState};
use histdiff_core::*;
use std::sync::Arc;
use tauri::Manager;
use tauri_components::{
    clear_logs, get_logs, init_logger, open_analysis, open_control_selector_win,
    open_logging_window, test_log,
};

#[tauri::command]
fn terminal(msg: &str) {
    log::warn!("{:?}", msg);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            init_logger(app.handle().clone()).expect("Failed to initialize logger");

            app.manage(HistDiffState::new());
            app.manage(ClusterState::new());
            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![
            open_control_selector_win,
            process_hd,
            open_logging_window,
            test_log,
            get_logs,
            clear_logs,
            write_res,
            terminal,
            open_analysis,
            cluster_hd,
            get_cluster_res
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
