use tauri::{Manager, Window};

mod integrations;
mod api;

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
            api::get_enabled_integrations,
            api::get_meta_integrations,
            api::get_enabled_integrations_configs
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap(); // Ensure window starts hidden
            Ok(())
        })
        .on_page_load(|window, _| {
            // Listen for page load to finish, then show window
            let window_ = window.clone();
            tauri::async_runtime::spawn(async move {
                // Optional: wait a short moment to avoid race condition with rendering
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                let _ = window_.show();
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
