use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub auth_service_api_url: String,
    pub user_service_api_url: String,
    pub message_service_api_url: String,
    pub message_websocket_url: String,
}

pub fn load_core_config<P: AsRef<Path>>(path: P) -> Result<CoreConfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
