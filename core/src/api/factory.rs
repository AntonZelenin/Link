use crate::api::client::{ApiClient, SharedApiClient};
use crate::traits::SharedStorage;
use crate::{auth, config};

pub fn get_shared_api_client(storage: SharedStorage) -> SharedApiClient {
    SharedApiClient::new(get_api_client(storage))
}

fn get_api_client(storage: SharedStorage) -> ApiClient {
    let config = config::load_core_config().expect("Failed to load core config");

    ApiClient::new(
        reqwest::Client::new(),
        config.auth_service_api_url.clone(),
        config.user_service_api_url.clone(),
        config.message_service_api_url.clone(),
        auth::factory::get_auth_manager(storage),
    )
}
