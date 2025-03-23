use lcore::traits::Storage;

pub struct WebStorage;

impl Storage for WebStorage {
    fn set(&self, key: &str, value: &str) {
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            let _ = storage.set_item(key, value);
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()?
            .get_item(key)
            .ok()
            .flatten()
    }

    fn remove(&self, key: &str) {
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            let _ = storage.remove_item(key);
        }
    }
}

pub fn get_storage() -> WebStorage {
    WebStorage
}
