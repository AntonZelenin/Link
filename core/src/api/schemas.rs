use crate::helpers::types::{ChatId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
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
    pub  refresh_token: String,
    pub  user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
    pub detail: String,
}
