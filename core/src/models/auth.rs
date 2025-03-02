pub struct Auth {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            access_token: None,
            refresh_token: None,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.access_token.is_some()
    }
}
