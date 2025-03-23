use crate::api::client::Client;
use crate::config;
use crate::storage::AuthManager;
use crate::traits::SharedStorage;

pub fn get_api_client(storage: SharedStorage) -> Client {
    let config = config::load_core_config().expect("Failed to load core config");

    Client::new(
        reqwest::Client::new(),
        None,
        config.auth_service_api_url.clone(),
        config.user_service_api_url.clone(),
        config.message_service_api_url.clone(),
        AuthManager::new(storage),
    )
}
