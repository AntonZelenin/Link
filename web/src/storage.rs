use lcore::traits::Storage;

pub struct WebStorage;

impl Storage for WebStorage {
    fn set_item(&self, key: &str, value: &str) {
        if let Some(storage) = web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()
        {
            let _ = storage.set_item(key, value);
        }
    }

    fn get_item(&self, key: &str) -> Option<String> {
        web_sys::window()
            .and_then(|w| w.local_storage().ok())
            .flatten()?
            .get_item(key)
            .ok()
            .flatten()
    }
}

pub fn get_storage() -> WebStorage {
    WebStorage
}
