use crate::storage::{AuthManager, SharedStorage};

pub fn get_auth_manager(storage: SharedStorage) -> AuthManager {
    AuthManager::new(storage)
}
