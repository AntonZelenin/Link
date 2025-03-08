use serde::Deserialize;
use std::collections::HashMap;

#[macro_export]
macro_rules! f {
    ($($arg:tt)*) => {
        format!($($arg)*)
    };
}

/// Convert a HashMap<String, String> to a struct that is specified by T
///
/// Example:
/// from_map::<LoginRequest>(&map)
pub fn from_map<T: for<'de> Deserialize<'de>>(map: &HashMap<String, String>) -> Result<T, String> {
    serde_json::from_value(serde_json::json!(map)).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct LoginRequest {
        username: String,
        password: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct UserProfile {
        name: String,
        age: u32,
    }

    #[test]
    fn test_successful_deserialization() {
        let mut map = HashMap::new();
        map.insert("username".to_string(), "test_user".to_string());
        map.insert("password".to_string(), "secure_password".to_string());

        let result: Result<LoginRequest, String> = from_map(&map);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            LoginRequest {
                username: "test_user".to_string(),
                password: "secure_password".to_string(),
            }
        );
    }

    #[test]
    fn test_missing_field() {
        let mut map = HashMap::new();
        map.insert("username".to_string(), "test_user".to_string());

        let result: Result<LoginRequest, String> = from_map(&map);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("missing field `password`"));
    }

    #[test]
    fn test_invalid_type() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "Alice".to_string());
        map.insert("age".to_string(), "not_a_number".to_string());

        let result: Result<UserProfile, String> = from_map(&map);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("invalid digit"));
    }

    #[test]
    fn test_empty_map() {
        let map: HashMap<String, String> = HashMap::new();

        let result: Result<UserProfile, String> = from_map(&map);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("missing field `name`"));
    }

    #[test]
    fn test_macro_f() {
        let name = "Alice";
        let age = 25;
        let msg = f!("Hello, {name}, you are {age} years old!");
        assert_eq!(msg, "Hello, Alice, you are 25 years old!");
    }
}
