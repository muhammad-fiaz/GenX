use tauri::{Manager, Window, WindowEvent};
use tauri_plugin_log::{Builder as LogBuilder, Target, TargetKind};
use log::{info, LevelFilter};

mod api;
mod auth;
mod integrations;

#[tauri::command]
fn show_main_window(window: Window) {
    let _ = window.show(); // Best-effort show
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            LogBuilder::default()
                .level(LevelFilter::Info) // Set minimum log level
                .clear_targets()
                .target(Target::new(TargetKind::Stdout)) // Show in terminal
                .target(Target::new(TargetKind::Webview)) // Show in frontend console
                .target(Target::new(TargetKind::LogDir {
                    file_name: Some("genx.log".into()), // Persist logs
                }))
                .build()
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            auth::github_login,
            api::get_enabled_integrations,
            api::get_meta_integrations,
            api::get_enabled_integrations_configs
        ])
        .setup(|app| {
            // Log system/app info
            info!("🚀 GenX Starting...");
            let pkg = app.package_info();
            info!("📦 App Name: {}", pkg.name);
            info!("📦 Version: {}", pkg.version);
            info!("🖥️ Platform: {}", std::env::consts::OS);
            info!("🔧 Arch: {}", std::env::consts::ARCH);

            // Optionally hide the main window at start
            if let Some(window) = app.get_window("main") {
                let _ = window.hide();
            }
            Ok(())
        })
        .on_page_load(|window, _| {
            let window_ = window.clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                let _ = window_.show();
            });
        })
        .on_window_event(|_app_handle, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                log::info!("🛑 GenX is closing...");
                log::logger().flush();
            }
        })
        .run(tauri::generate_context!())
        .expect("❌ Failed to run Tauri application");
}
