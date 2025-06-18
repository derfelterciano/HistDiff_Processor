#![allow(unused_imports)]
mod analysis;
mod hd_interface;
mod tauri_components;

use analysis::{cluster_hd, get_cluster_res};
use hd_interface::{
    get_hd_scores, get_neg_controls, process_hd, reset_state, write_res, ClusterState,
    HistDiffState, NegControlState,
};
use histdiff_core::*;
use std::sync::Arc;
use tauri::Manager;
use tauri_components::updater::update;
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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            init_logger(app.handle().clone()).expect("Failed to initialize logger");

            // updater
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });

            app.manage(HistDiffState::new());
            app.manage(ClusterState::new());
            app.manage(NegControlState::new());
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
            get_cluster_res,
            get_hd_scores,
            reset_state,
            get_neg_controls
        ])
        .on_window_event(|win: &tauri::Window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let label = win.label();

                if label == "main" {
                    let handle = win.app_handle();
                    for (label, w) in handle.webview_windows() {
                        if label != "main" {
                            let _ = w.close();
                        }
                    }
                    win.app_handle().exit(0);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
