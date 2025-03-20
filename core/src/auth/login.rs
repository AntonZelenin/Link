use crate::api::client::SharedClient;
use crate::api::schemas::{AuthError, LoginRequest};
use crate::traits::Storage;

pub async fn login(
    login_request: LoginRequest,
    client: &mut SharedClient,
    storage: &impl Storage,
) -> Result<(), AuthError> {
    let auth_response = client.login(login_request).await?;

    storage.set_item("access_token", &auth_response.access_token);
    storage.set_item("refresh_token", &auth_response.refresh_token);
    storage.set_item("user_id", &auth_response.user_id);

    Ok(())
}
