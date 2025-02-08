use log::{Level, Metadata, Record, SetLoggerError};
use chrono::Local;
use once_cell::sync::Lazy;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;

pub static LOGGER: Lazy<PhantomLogger> = Lazy::new(|| PhantomLogger::new("phantom.log"));

pub struct PhantomLogger {
    file: Mutex<File>,
}

impl PhantomLogger {
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .expect("Failed to open log file");
        Self {
            file: Mutex::new(file),
        }
    }

    pub fn log(&self, level: &str, message: &str, data: Option<String>) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = match data {
            Some(d) => format!(
                "[{}] {}: {} {}\n",
                timestamp,
                level,
                message,
                serde_json::to_string(&d).unwrap_or_else(|_| "{}".to_string())
            ),
            None => format!("[{}] {}: {}\n", timestamp, level, message),
        };

        let log_line = format!("{}\n", log_entry.to_string());

        if let Ok(mut file) = self.file.lock() {
            let _ = file.write_all(log_line.as_bytes());
        }
    }
}

impl log::Log for PhantomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.log(
                &record.level().to_string(),
                &record.args().to_string(),
                None,
            );
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&*LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info))
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        $crate::LOGGER.log("INFO", $msg, None);
    };
    ($msg:expr, data: $data:expr) => {
        $crate::LOGGER.log("INFO", $msg, Some(serde_json::to_string(&$data).unwrap()));
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        $crate::LOGGER.log("WARN", $msg, None);
    };
    ($msg:expr, data: $data:expr) => {
        $crate::LOGGER.log("WARN", $msg, Some(serde_json::to_string(&$data).unwrap()));
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::LOGGER.log("ERROR", $msg, None);
    };
    ($msg:expr, data: $data:expr) => {
        $crate::LOGGER.log("ERROR", $msg, Some(serde_json::to_string(&$data).unwrap()));
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        $crate::LOGGER.log("DEBUG", $msg, None);
    };
    ($msg:expr, data: $data:expr) => {
        $crate::LOGGER.log("DEBUG", $msg, Some(serde_json::to_string(&$data).unwrap()));
    };
}

