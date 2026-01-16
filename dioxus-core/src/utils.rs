use std::collections::HashMap;
use dioxus::prelude::FormValue;

pub fn form_values_to_string(values: &[(String, FormValue)]) -> HashMap<String, String> {
    values
        .iter()
        .filter_map(|(k, v)| match v {
            FormValue::Text(s) => Some((k.clone(), s.clone())),
            FormValue::File(_) => None,
        })
        .collect()
}
