use std::{fmt::format, sync::Mutex};

use colored::*;
use lazy_static::lazy_static;
use log::{self, set_boxed_logger, set_max_level, Level, LevelFilter, SetLoggerError};
use tauri::{AppHandle, Emitter, Manager};

lazy_static! {
    static ref LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[derive(Clone)]
pub struct Logger {
    app_handle: AppHandle,
}

#[allow(dead_code)]
impl Logger {
    pub fn new(app_handle: AppHandle) -> Self {
        Logger { app_handle }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let line = format!("[{:?}] {}", record.level(), record.args());
            {
                let mut logs = LOGS.lock().unwrap();
                logs.push(line.clone());
            }

            // emit logs
            if let Some(_win) = self.app_handle.get_webview_window("Logs") {
                _ = self.app_handle.emit("rust-log", &line);
                // safe_emit(&self.app_handle, "rust-log", &line);}
            }

            let level_clr = match record.level() {
                Level::Warn => record.level().to_string().yellow().bold(),
                Level::Info => record.level().to_string().purple().bold(),
                Level::Trace => record.level().to_string().cyan().bold(),
                Level::Error => record.level().to_string().red().bold(),
                Level::Debug => record.level().to_string().blue().bold(),
            };

            let msg_str = format!(
                "{:5.5} | [{:25.25}-{:03}] | {}",
                level_clr,
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                record.args(),
            );

            println!("{}", msg_str);
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(app: AppHandle) -> Result<(), SetLoggerError> {
    let logger = Logger::new(app.clone());
    set_boxed_logger(Box::new(logger))?;
    set_max_level(LevelFilter::Trace);
    return Ok(());
}

#[tauri::command]
pub fn get_logs() -> Vec<String> {
    return LOGS.lock().unwrap().clone();
}

#[tauri::command]
pub fn clear_logs() {
    let mut logs = LOGS.lock().unwrap();
    logs.clear();
}

#[tauri::command]
pub fn test_log() {
    log::info!("TESTING TESTING 123!");
}

fn safe_emit(app: &AppHandle, event: &str, payload: &str) {
    if let Some(win) = app.get_webview_window("Logs") {
        _ = win.emit(event, payload);
    }
}
