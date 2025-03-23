use serde::Serialize;
use serde_json::Value;
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

pub trait Storage {
    fn set(&self, key: &str, value: &str);
    fn get(&self, key: &str) -> Option<String>;
    fn remove(&self, key: &str);
}

pub trait ToJson {
    fn to_json(&self) -> Value;
}

impl<T: Serialize> ToJson for T {
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
