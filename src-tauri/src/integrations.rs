use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents each integration entry inside meta.json
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaIntegration {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    #[serde(rename = "type")]
    pub integration_type: String,
    #[serde(rename = "configPath")]
    pub config_path: String,
    pub enabled: bool,
}

/// Represents the root meta.json structure
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaFile {
    pub version: String,
    pub description: String,
    pub identifier: String,
    pub integrations: Vec<MetaIntegration>,
}

/// Endpoint inside integration config
#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    pub key: String,
    pub path: String,
    pub method: String,
    pub description: String,
}

/// Setting inside integration config
#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub input_type: String,
    pub default: serde_json::Value,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub step: Option<f64>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    pub description: String,
}

/// Full config struct for one integration (example: Ollama)
#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub host: String,
    pub endpoints: Vec<Endpoint>,
    pub settings: Vec<Setting>,
}

/// Load the meta.json file and parse it
pub fn load_meta() -> Result<MetaFile, String> {
    let path = Path::new("integrations/meta.json"); // adjust base path as needed
    let data = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read meta.json at {}: {}", path.display(), e))?;
    let meta: MetaFile = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse meta.json: {}", e))?;
    Ok(meta)
}

/// Load an individual integration config file by its path
pub fn load_integration_config(path_str: &str) -> Result<OllamaConfig, String> {
    let path = Path::new(path_str);
    let data = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config at {}: {}", path.display(), e))?;
    let config: OllamaConfig = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse config at {}: {}", path.display(), e))?;
    Ok(config)
}

/// Load configs of all enabled integrations dynamically
pub fn load_enabled_integrations_configs() -> Result<Vec<OllamaConfig>, String> {
    let meta = load_meta()?;
    let mut configs = Vec::new();

    for integration in meta.integrations.iter().filter(|i| i.enabled) {
        // Optional: You can resolve relative paths here, e.g., relative to meta.json directory
        let cfg = load_integration_config(&integration.config_path)?;
        configs.push(cfg);
    }

    Ok(configs)
}
