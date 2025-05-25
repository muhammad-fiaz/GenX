use tauri::{Manager, Window};

mod integrations;
mod api;
mod auth;

// Show the main window
#[tauri::command]
fn show_main_window(window: Window) {
    let _ = window.show(); // Best-effort show
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            auth::github_login,
            api::get_enabled_integrations,
            api::get_meta_integrations,
            api::get_enabled_integrations_configs
        ])
        .setup(|app| {
            // Hide the main window on startup
            if let Some(window) = app.get_window("main") {
                let _ = window.hide();
            }
            Ok(())
        })
        .on_page_load(|window, _| {
            // Show the window after the page loads, with a slight delay to avoid race conditions
            let window_ = window.clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                let _ = window_.show();
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}