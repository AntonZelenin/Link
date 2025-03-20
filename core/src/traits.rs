use serde::Serialize;
use serde_json::Value;

pub trait Storage {
    fn set_item(&self, key: &str, value: &str);
    fn get_item(&self, key: &str) -> Option<String>;
}

pub trait ToJson {
    fn to_json(&self) -> Value;
}

impl<T: Serialize> ToJson for T {
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
