use crate::storage::AuthManager;
use crate::traits::SharedStorage;

pub fn get_auth_manager(storage: SharedStorage) -> AuthManager {
    AuthManager::new(storage)
}
