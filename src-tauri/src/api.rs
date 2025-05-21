use crate::integrations::{load_enabled_integrations_configs, OllamaConfig};
use tauri::command;

/// Return all enabled integrations configs dynamically
#[command]
pub fn get_enabled_integrations() -> Result<Vec<OllamaConfig>, String> {
    load_enabled_integrations_configs()
}
