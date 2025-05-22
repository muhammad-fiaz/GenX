use crate::integrations::{load_enabled_integrations_configs, OllamaConfig, load_meta, MetaIntegration};
use tauri::command;

/// Return all enabled integrations configs dynamically
#[command]
pub fn get_enabled_integrations() -> Result<Vec<OllamaConfig>, String> {
    load_enabled_integrations_configs()
}

/// Get meta information about all integrations
#[command]
pub fn get_meta_integrations() -> Result<Vec<MetaIntegration>, String> {
    let meta = load_meta()?;
    Ok(meta.integrations)
}

/// Get detailed configs for all enabled integrations
#[command]
pub fn get_enabled_integrations_configs() -> Result<Vec<OllamaConfig>, String> {
    load_enabled_integrations_configs()
}
