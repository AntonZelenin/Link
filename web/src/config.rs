use lcore::config::CoreConfig;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub core: CoreConfig,
}

pub fn load_web_config<P: AsRef<Path>>(
    core_path: P,
    web_path: P,
) -> Result<WebConfig, Box<dyn std::error::Error>> {
    let core_config = lcore::config::load_core_config(&core_path)?;
    let contents = fs::read_to_string(web_path)?;
    let mut web_config: WebConfig = toml::from_str(&contents)?;
    web_config.core = core_config;
    Ok(web_config)
}

pub static CONFIG: OnceLock<WebConfig> = OnceLock::new();

pub fn init_config(core_path: &str, web_path: &str) {
    CONFIG
        .set(load_web_config(core_path, web_path).expect("Failed to load config"))
        .unwrap();
}
