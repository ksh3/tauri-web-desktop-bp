use chrono::Local;
use serde::Serialize;
use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Debug)]
pub struct LoggerConfig {
    pub output_to_file: bool,
    pub log_file_path: PathBuf,
    pub log_format: LogFormat,
}

pub struct Logger {
    config: LoggerConfig,
    file: Option<File>,
}

impl Logger {
    pub fn new() -> std::io::Result<Self> {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "production".into());
        let is_dev = app_env == "development";

        let config = LoggerConfig {
            output_to_file: is_dev,
            log_file_path: "./debug.log".into(),
            log_format: LogFormat::Text,
        };

        let file = if config.output_to_file {
            Some(
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .write(true)
                    .open(&config.log_file_path)?,
            )
        } else {
            None
        };

        Ok(Self { config, file })
    }

    fn log_to_output(&mut self, message: &str) {
        if let Some(file) = &mut self.file {
            let _ = writeln!(file, "{}", message);
        } else {
            println!("{}", message);
        }
    }

    pub fn info(&mut self, msg: &str) {
        let log = format!(
            "{} [INFO] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            msg
        );
        self.log_to_output(&log);
    }

    pub fn error(&mut self, err: AppError, format: Option<LogFormat>) {
        let fmt = format.unwrap_or(self.config.log_format);
        match fmt {
            LogFormat::Text => self.text_error_log(&err),
            LogFormat::Json => self.json_error_log(&err),
        }
    }

    fn text_error_log(&mut self, err: &AppError) {
        let log = format!(
            "{} [ERROR] IsCritical: {} Code: {}, Message: {}, Cause: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            err.is_critical,
            err.code,
            err.message,
            err.cause
        );
        self.log_to_output(&log);
    }

    fn json_error_log(&mut self, err: &AppError) {
        let json_entry = JsonErrorLogEntry {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            level: "ERROR".into(),
            is_critical: err.is_critical,
            code: err.code,
            message: err.message.clone(),
            cause: format!("{}", err.cause),
        };
        if let Ok(json) = serde_json::to_string(&json_entry) {
            self.log_to_output(&json);
        }
    }

    pub fn close(&mut self) {
        if let Some(file) = self.file.take() {
            let _ = file.sync_all();
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    pub is_critical: bool,
    pub code: u16,
    pub message: String,
    pub cause: String,
}

#[derive(Serialize)]
struct JsonErrorLogEntry {
    timestamp: String,
    level: String,
    is_critical: bool,
    code: u16,
    message: String,
    cause: String,
}
