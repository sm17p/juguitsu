mod crash_log;

use color_eyre::eyre::Context;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::LogDir {
                        file_name: Some("juguitsu".into()),
                    }),
                    #[cfg(debug_assertions)]
                    Target::new(TargetKind::Stdout),
                    #[cfg(debug_assertions)]
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] [{}] [{}] {message}",
                        crash_log::now_zoned(),
                        record.level(),
                        record.target(),
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let log_dir =
                crash_log::resolve_log_dir(app.handle()).context("failed to resolve log dir")?;
            crash_log::install_panic_hook(log_dir);

            tracing::info!("juguitsu started");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, crash_log::log_crash])
        .run(tauri::generate_context!())
        .context("error while running tauri application")
        .expect("error while running tauri application");
}
