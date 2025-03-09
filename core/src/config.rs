use serde::Deserialize;

pub const CORE_CONFIG_TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub auth_service_api_url: String,
    pub user_service_api_url: String,
    pub message_service_api_url: String,
    pub message_websocket_url: String,
}

pub fn load_core_config() -> Result<CoreConfig, Box<dyn std::error::Error>> {
    let config: CoreConfig = toml::from_str(CORE_CONFIG_TOML)?;
    Ok(config)
}
