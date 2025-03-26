use crate::api::client::SharedApiClient;
use crate::api::schemas::{AuthError, LoginRequest, RegisterError, RegisterRequest};
use crate::storage::SharedStorage;
use crate::traits::AuthState;

pub async fn login(
    login_request: LoginRequest,
    client: SharedApiClient,
    storage: SharedStorage,
    auth_state: impl AuthState,
) -> Result<(), AuthError> {
    let auth_response = client.login(login_request).await?;

    storage.set("access_token", &auth_response.access_token);
    storage.set("refresh_token", &auth_response.refresh_token);
    storage.set("user_id", &auth_response.user_id);

    auth_state.set_authenticated();

    Ok(())
}

pub async fn register(
    register_request: RegisterRequest,
    client: SharedApiClient,
    storage: SharedStorage,
    auth_state: impl AuthState,
) -> Result<(), RegisterError> {
    let auth_response = client.register(register_request).await?;

    storage.set("access_token", &auth_response.access_token);
    storage.set("refresh_token", &auth_response.refresh_token);
    storage.set("user_id", &auth_response.user_id);

    auth_state.set_authenticated();

    Ok(())
}

pub async fn logout(
    client: SharedApiClient,
    storage: SharedStorage,
    auth_state: impl AuthState,
) -> Result<(), AuthError> {
    client.logout().await?;

    storage.remove("access_token");
    storage.remove("refresh_token");
    storage.remove("user_id");

    auth_state.set_not_authenticated();

    Ok(())
}
