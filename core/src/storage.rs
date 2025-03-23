use crate::auth::schemas::Auth;
use crate::traits::SharedStorage;

pub struct AuthManager {
    storage: SharedStorage,
}

impl AuthManager {
    pub fn new(storage: SharedStorage) -> Self {
        AuthManager { storage }
    }

    pub fn get(&self) -> Option<Auth> {
        let refresh_token = self.storage.get("refresh_token");
        let access_token = self.storage.get("access_token");

        if refresh_token.is_none() || access_token.is_none() {
            return None;
        }

        Some(Auth::new(&access_token.unwrap(), &refresh_token.unwrap()))
    }

    pub fn update(&mut self, auth: Auth) {
        self.storage.set("refresh_token", &auth.refresh_token);
        self.storage.set("access_token", &auth.access_token);
    }

    pub fn delete(&mut self) {
        self.storage.remove("refresh_token");
        self.storage.remove("access_token");
    }
}
