use crate::auth::schemas::Auth;
use crate::traits::Storage;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct SharedStorage(Arc<RwLock<dyn Storage + Send + Sync>>);

impl SharedStorage {
    pub fn new(storage: impl Storage + Send + Sync + 'static) -> Self {
        Self(Arc::new(RwLock::new(storage)))
    }

    pub fn set(&self, key: &str, value: &str) {
        self.0.write().unwrap().set(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.0.read().unwrap().get(key)
    }

    pub fn remove(&self, key: &str) {
        self.0.write().unwrap().remove(key);
    }
}

pub struct AuthManager {
    storage: SharedStorage,
}

impl AuthManager {
    pub fn new(storage: SharedStorage) -> Self {
        AuthManager { storage }
    }

    pub fn is_authenticated(&self) -> bool {
        self.get_auth().is_some()
    }

    pub fn get_auth(&self) -> Option<Auth> {
        let refresh_token = self.storage.get("refresh_token");
        let access_token = self.storage.get("access_token");

        if refresh_token.is_none() || access_token.is_none() {
            return None;
        }

        Some(Auth::new(&access_token.unwrap(), &refresh_token.unwrap()))
    }

    pub fn update_auth(&mut self, auth: Auth) {
        self.storage.set("refresh_token", &auth.refresh_token);
        self.storage.set("access_token", &auth.access_token);
    }

    pub fn delete_auth(&mut self) {
        self.storage.remove("refresh_token");
        self.storage.remove("access_token");
    }
}
