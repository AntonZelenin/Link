#[derive(Clone)]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: String,
}

impl Auth {
    pub fn new(token: &str, refresh_token: &str) -> Self {
        Self {
            access_token: token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }
}
