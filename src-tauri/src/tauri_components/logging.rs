use log::{self, set_boxed_logger, set_max_level, Level, LevelFilter, SetLoggerError};
use tauri::{AppHandle, Emitter, Manager};

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
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let line = format!("[{:?}] {}", record.level(), record.args());

            // emit logs
            _ = self.app_handle.emit("rust-log", &line);
            println!("{}", &line);
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
