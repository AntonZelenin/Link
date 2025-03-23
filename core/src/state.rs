// todo dioxus in core!
use crate::storage::SharedStorage;
use dioxus::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

const ACTIVE_APP_KEY: &str = "active_app";

pub type AppComponent = fn() -> Element;

pub static IS_AUTHENTICATED: GlobalSignal<bool> = Global::new(|| false);
pub static ACTIVE_APP: GlobalSignal<Option<String>> = Global::new(|| None);

static APP_REGISTRY: Lazy<RwLock<HashMap<String, AppComponent>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn register_app(name: &str, app: AppComponent) {
    APP_REGISTRY.write().unwrap().insert(name.to_string(), app);
}

pub fn remove_app(name: &str) {
    APP_REGISTRY.write().unwrap().remove(name);
}

pub fn get_app(name: &str) -> Option<AppComponent> {
    APP_REGISTRY.read().unwrap().get(name).copied()
}

pub fn set_active_app(app_name: &str, storage: SharedStorage) {
    storage.set(ACTIVE_APP_KEY, app_name);
    *ACTIVE_APP.write() = Some(app_name.to_string());
}

pub fn clear_active_app(storage: SharedStorage) {
    storage.remove(ACTIVE_APP_KEY);
    *ACTIVE_APP.write() = None;
}

pub fn get_active_app() -> Option<AppComponent> {
    ACTIVE_APP.read().as_ref().and_then(|name| get_app(name))
}

pub fn load_active_app(storage: SharedStorage) {
    if let Some(app_name) = storage.get(ACTIVE_APP_KEY) {
        *ACTIVE_APP.write() = Some(app_name);
    }
}
