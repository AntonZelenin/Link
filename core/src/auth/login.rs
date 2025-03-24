use crate::api::client::SharedApiClient;
use crate::api::schemas::{AuthError, LoginRequest, RegisterError, RegisterRequest};
use crate::state::IS_AUTHENTICATED;
use crate::storage::SharedStorage;

pub async fn login(
    login_request: LoginRequest,
    client: SharedApiClient,
    storage: SharedStorage,
) -> Result<(), AuthError> {
    let auth_response = client.login(login_request).await?;

    storage.set("access_token", &auth_response.access_token);
    storage.set("refresh_token", &auth_response.refresh_token);
    storage.set("user_id", &auth_response.user_id);

    *IS_AUTHENTICATED.write() = true;

    Ok(())
}

pub async fn register(
    register_request: RegisterRequest,
    client: SharedApiClient,
    storage: SharedStorage,
) -> Result<(), RegisterError> {
    let auth_response = client.register(register_request).await?;

    storage.set("access_token", &auth_response.access_token);
    storage.set("refresh_token", &auth_response.refresh_token);
    storage.set("user_id", &auth_response.user_id);

    *IS_AUTHENTICATED.write() = true;

    Ok(())
}

pub fn logout(storage: SharedStorage) {
    storage.remove("access_token");
    storage.remove("refresh_token");
    storage.remove("user_id");
    
    *IS_AUTHENTICATED.write() = false;
}
