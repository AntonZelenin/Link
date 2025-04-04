use crate::helpers::types::{ChatId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(custom(function = "validate_username"))]
    pub username: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        let mut err = ValidationError::new("username_length");
        err.message = Some("Please use at least 3 characters".into());
        return Err(err);
    }
    if username.len() > 150 {
        let mut err = ValidationError::new("username_length");
        err.message = Some("Maximum length of 150 characters exceeded".into());
        return Err(err);
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        let mut err = ValidationError::new("password_length");
        err.message = Some("Please use at least 8 characters".into());
        return Err(err);
    }
    if password.len() > 64 {
        let mut err = ValidationError::new("password_length");
        err.message = Some("Maximum length of 64 characters exceeded".into());
        return Err(err);
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatModel {
    pub id: ChatId,
    pub name: Option<String>,
    pub member_ids: Vec<String>,
    pub messages: Vec<MessageModel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewChatModel {
    pub name: Option<String>,
    pub member_ids: Vec<String>,
    pub first_message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewMessage {
    // todo shouldn't this also contain created at and is_read? then it could be the single model
    pub chat_id: ChatId,
    pub sender_id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageModel {
    pub chat_id: u32,
    pub sender_id: String,
    pub text: String,
    pub created_at: f64,
    pub is_read: bool,
}

// todo maybe you should separate api schema and actual models
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSearchResults {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatSearchResults {
    pub chats: Vec<ChatModel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetUsersByIdsRequest {
    pub user_ids: Vec<UserId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

#[derive(Debug)]
pub enum AuthError {
    ApiError(String),
}

#[derive(Debug)]
pub enum RegisterError {
    ApiError(String),
    ValidationErrors(HashMap<String, String>),
}

pub struct RequestParams {
    pub uri: String,
    pub query_params: Vec<(String, String)>,
    pub body: Option<serde_json::Value>,
    pub can_reauthenticate: bool,
}

impl RequestParams {
    pub fn set_cant_reauthenticate(&mut self) {
        self.can_reauthenticate = false;
    }
}

impl Default for RequestParams {
    fn default() -> Self {
        Self {
            uri: "".to_string(),
            query_params: vec![],
            body: None,
            can_reauthenticate: true,
        }
    }
}
