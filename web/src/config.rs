use lcore::config::CoreConfig;
use serde::Deserialize;
use std::sync::OnceLock;
use web_sys::console;

pub const WEB_CONFIG_TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));

#[derive(Debug, Deserialize)]
pub struct WebConfig {}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub core: CoreConfig,
    pub web: WebConfig,
}

fn load_web_config() -> Result<Config, Box<dyn std::error::Error>> {
    let core_config = lcore::config::load_core_config()?;
    let web_config: WebConfig = toml::from_str(WEB_CONFIG_TOML)?;

    Ok(Config {
        core: core_config,
        web: web_config,
    })
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config() {
    let config_result = load_web_config();
    if let Err(e) = &config_result {
        console::error_1(&format!("Config load failed: {:?}", e).into());
    }
    CONFIG
        .set(config_result.expect("Failed to load config"))
        .unwrap();
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("CONFIG is not initialized")
}
