use crate::api::client::SharedClient;
use crate::api::schemas::{AuthError, LoginRequest, RegisterError, RegisterRequest};
use crate::traits::SharedStorage;

pub async fn login(
    login_request: LoginRequest,
    client: SharedClient,
    storage: SharedStorage,
) -> Result<(), AuthError> {
    let auth_response = client.login(login_request).await?;

    storage.set_item("access_token", &auth_response.access_token);
    storage.set_item("refresh_token", &auth_response.refresh_token);
    storage.set_item("user_id", &auth_response.user_id);

    Ok(())
}

pub async fn register(
    register_request: RegisterRequest,
    client: SharedClient,
    storage: SharedStorage,
) -> Result<(), RegisterError> {
    let auth_response = client.register(register_request).await?;

    storage.set_item("access_token", &auth_response.access_token);
    storage.set_item("refresh_token", &auth_response.refresh_token);
    storage.set_item("user_id", &auth_response.user_id);

    Ok(())
}
