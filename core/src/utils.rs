use std::collections::HashMap;
use serde::Deserialize;

pub fn from_map<T: for<'de> Deserialize<'de>>(map: &HashMap<String, String>) -> Result<T, String> {
    serde_json::from_value(serde_json::json!(map)).map_err(|e| e.to_string())
}
