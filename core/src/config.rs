use serde::Deserialize;

pub const CORE_CONFIG_TOML: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));

pub fn load_core_config() -> Result<CoreConfig, Box<dyn std::error::Error>> {
    let config: CoreConfig = toml::from_str(CORE_CONFIG_TOML)?;
    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    pub auth_service_api_url: String,
    pub user_service_api_url: String,
    pub message_service_api_url: String,
    pub message_websocket_url: String,

    pub apps: Apps,
}

#[derive(Debug, Deserialize)]
pub struct Apps {
    enabled: Vec<String>,
}

impl Apps {
    pub fn is_app_enabled(&self, app_name: &str) -> bool {
        self.enabled.iter().any(|name| name == app_name)
    }
}
