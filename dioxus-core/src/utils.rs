use std::collections::HashMap;
use dioxus::prelude::FormValue;

pub fn form_values_to_string(map: &HashMap<String, FormValue>) -> HashMap<String, String> {
    map.iter().map(|(k, v)| (k.clone(), v.as_value())).collect()
}
