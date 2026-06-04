use std::{
    backtrace::Backtrace,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use tauri::Manager;
use thiserror::Error;

const CRASH_FILE: &str = "crashes.log";

#[derive(Error, Debug)]
pub enum CrashLogError {
    #[error("log dir unavailable: {0}")]
    LogDir(String),
    #[error("failed to write crash log: {0}")]
    Write(#[from] std::io::Error),
}

pub fn resolve_log_dir(app: &tauri::AppHandle) -> Result<PathBuf, CrashLogError> {
    app.path()
        .app_log_dir()
        .map_err(|error| CrashLogError::LogDir(error.to_string()))
}

pub fn append_crash_entry(
    log_dir: &Path,
    source: &str,
    message: &str,
    detail: Option<&str>,
) -> Result<(), CrashLogError> {
    std::fs::create_dir_all(log_dir)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join(CRASH_FILE))?;

    writeln!(file, "[{}] [{}] {}", jiff::Zoned::now(), source, message)?;
    if let Some(detail) = detail.filter(|value| !value.is_empty()) {
        writeln!(file, "{detail}")?;
    }
    writeln!(file)?;
    file.flush()?;
    Ok(())
}

pub fn install_panic_hook(log_dir: PathBuf) {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let backtrace = Backtrace::force_capture().to_string();
        let _ = append_crash_entry(
            &log_dir,
            "panic",
            &info.to_string(),
            if backtrace.contains("<disabled>") || backtrace.trim().is_empty() {
                None
            } else {
                Some(backtrace.as_str())
            },
        );
        prev(info);
    }));
}

#[tauri::command]
pub fn log_crash(
    app: tauri::AppHandle,
    source: String,
    message: String,
    stack: Option<String>,
) -> Result<(), String> {
    append_crash_entry(
        &resolve_log_dir(&app).map_err(|error| error.to_string())?,
        &source,
        &message,
        stack.as_deref(),
    )
    .map_err(|error| error.to_string())
}
