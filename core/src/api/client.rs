use crate::api::schemas;
use crate::api::schemas::{
    AuthError, AuthResponse, LoginRequest, RegisterError, RegisterRequest, RequestParams,
};
use crate::auth::schemas::Auth;
use crate::f;
use crate::helpers::types::{ChatId, UserId};
use crate::storage::AuthManager;
use reqwest::{Response, StatusCode};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;
// type WriteMessageWs = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
// type ReadMessageWs = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(Clone)]
pub struct SharedApiClient(Arc<RwLock<ApiClient>>);

impl SharedApiClient {
    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AuthError> {
        let mut client = self.0.write().await;
        client.login(req).await
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, RegisterError> {
        let mut client = self.0.write().await;
        client.register(req).await
    }

    pub fn new(client: ApiClient) -> Self {
        Self(Arc::new(RwLock::new(client)))
    }
}

pub struct ApiClient {
    client: reqwest::Client,
    auth: Option<Auth>,
    auth_manager: AuthManager,

    auth_service_api_url: String,
    user_service_api_url: String,
    message_service_api_url: String,
    // write_message_ws: Option<WriteMessageWs>,
    // read_message_ws: Option<ReadMessageWs>,
}

impl ApiClient {
    pub fn new(
        client: reqwest::Client,
        auth: Option<Auth>,
        auth_service_api_url: String,
        user_service_api_url: String,
        message_service_api_url: String,
        auth_manager: AuthManager,
    ) -> Self {
        let obj = Self {
            client,
            auth,
            auth_manager,

            auth_service_api_url,
            user_service_api_url,
            message_service_api_url,
            // write_message_ws: None,
            // read_message_ws: None,
        };

        // if obj.auth_tokens.is_some() {
        //     obj.connect_to_message_ws(RequestParams::default()).await;
        // }

        obj
    }

    pub fn is_authenticated(&self) -> bool {
        self.auth.is_some()
    }

    pub async fn login(&mut self, login_req: LoginRequest) -> Result<AuthResponse, AuthError> {
        let res = self
            .client
            .post(&self.auth_url("login"))
            .json(&login_req)
            .send()
            .await
            .map_err(|e| AuthError::ApiError(e.to_string()))?;
        let status = res.status();
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| AuthError::ApiError(e.to_string()))?;
        if !status.is_success() {
            return Err(AuthError::ApiError(
                data["detail"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string(),
            ));
        }
        let auth_response: AuthResponse =
            serde_json::from_value(data).map_err(|e| AuthError::ApiError(e.to_string()))?;
        self.set_auth_tokens(Auth::new(
            &auth_response.access_token,
            &auth_response.refresh_token,
        ));
        Ok(auth_response)
    }

    pub async fn register(
        &mut self,
        register_req: RegisterRequest,
    ) -> Result<AuthResponse, RegisterError> {
        let res = self
            .client
            .post(&self.user_url("users"))
            .json(&register_req)
            .send()
            .await
            .map_err(|e| RegisterError::ApiError(e.to_string()))?;
        let status = res.status();
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| RegisterError::ApiError(e.to_string()))?;
        if !status.is_success() {
            if let Some(errors) = data.get("errors") {
                let errors_map: HashMap<String, String> = serde_json::from_value(errors.clone())
                    .map_err(|e| RegisterError::ApiError(e.to_string()))?;
                return Err(RegisterError::ValidationErrors(errors_map));
            }
            return Err(RegisterError::ApiError(
                data["detail"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string(),
            ));
        }
        let auth_response: AuthResponse =
            serde_json::from_value(data).map_err(|e| RegisterError::ApiError(e.to_string()))?;
        self.set_auth_tokens(Auth::new(
            &auth_response.access_token,
            &auth_response.refresh_token,
        ));
        Ok(auth_response)
    }

    pub async fn get_users_by_ids(
        &mut self,
        user_ids: Vec<UserId>,
    ) -> ApiResult<schemas::UserSearchResults> {
        let rp = RequestParams {
            uri: self.user_url("users/batch-query"),
            body: Some(serde_json::to_value(&schemas::GetUsersByIdsRequest { user_ids }).unwrap()),
            ..Default::default()
        };
        let res = self.post(rp).await?;
        let data = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;

        Ok(serde_json::from_str(&data.to_string()).unwrap())
    }

    pub async fn get_chats(&mut self) -> ApiResult<schemas::ChatSearchResults> {
        let rp = RequestParams {
            uri: self.message_url("chats"),
            ..Default::default()
        };
        match self.get(rp).await {
            Ok(res) => {
                let data = res
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| ApiError::DataError(e.to_string()))?;
                let chats: schemas::ChatSearchResults =
                    serde_json::from_str(&data.to_string()).unwrap();
                Ok(chats)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_chat(&mut self, chat_id: ChatId) -> ApiResult<schemas::ChatModel> {
        let rp = RequestParams {
            uri: self.message_url(&f!("chats/{chat_id}")),
            ..Default::default()
        };
        let res = self.get(rp).await?;
        let data = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())
            .expect("Failed to parse response");

        Ok(serde_json::from_str(&data.to_string()).unwrap())
    }

    pub async fn mark_chat_as_read(&mut self, chat_id: ChatId) {
        let rp = RequestParams {
            uri: self.message_url(&f!("chats/{chat_id}/read")),
            ..Default::default()
        };
        match self.post(rp).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    pub async fn search_users(
        &mut self,
        username: String,
    ) -> ApiResult<schemas::UserSearchResults> {
        let rp = RequestParams {
            uri: self.user_url("users"),
            query_params: vec![("username".parse().unwrap(), username)],
            ..Default::default()
        };
        let res = self.get(rp).await?;
        let data = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;
        let users: schemas::UserSearchResults = serde_json::from_str(&data.to_string()).unwrap();

        Ok(users)
    }

    // pub async fn send_message(&mut self, message: schemas::NewMessage) {
    //     let send_message = self
    //         .write_message_ws
    //         .as_mut()
    //         .expect("Unauthenticated")
    //         .send(Message::Text(Utf8Bytes::from(
    //             serde_json::to_string(&message).unwrap(),
    //         )));
    //     send_message.await.expect("Failed to send message");
    // }

    pub async fn create_chat(
        &mut self,
        chat: schemas::NewChatModel,
    ) -> ApiResult<schemas::ChatModel> {
        let rp = RequestParams {
            uri: self.message_url("chats"),
            body: Some(serde_json::to_value(&chat).unwrap()),
            ..Default::default()
        };
        let res = self.post(rp).await?;
        let status = res.status();
        let data = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())
            .expect("Failed to parse response");
        if !status.is_success() {
            return Err(ApiError::RequestError(data["detail"].to_string()));
        }

        Ok(serde_json::from_str(&data.to_string()).unwrap())
    }

    // pub async fn receive_message(&mut self) -> Option<schemas::MessageModel> {
    //     if let Some(message) = self
    //         .read_message_ws
    //         .as_mut()
    //         .expect("Unauthenticated")
    //         .next()
    //         .await
    //     {
    //         match message {
    //             Ok(Message::Text(text)) => {
    //                 match serde_json::from_str::<schemas::MessageModel>(&text) {
    //                     Ok(parsed_message) => {
    //                         return Some(parsed_message);
    //                     }
    //                     Err(e) => {
    //                         panic!("Failed to parse message: {}", e);
    //                     }
    //                 }
    //             }
    //             Ok(Message::Binary(_)) => {
    //                 panic!("Received a binary message");
    //             }
    //             Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => {}
    //             Ok(Message::Close(_)) => {
    //                 unimplemented!("Handle close messages, possibly clean up or reconnect");
    //             }
    //             Err(e) => {
    //                 panic!("Failed to receive message: {}", e);
    //             }
    //             _ => {
    //                 panic!("Received unexpected message type");
    //             }
    //         }
    //     };
    //
    //     None
    // }

    async fn post(&mut self, mut rp: RequestParams) -> ApiResult<Response> {
        loop {
            let url = Url::parse_with_params(&rp.uri, rp.query_params.clone()).unwrap();
            let res = self
                .client
                .post(url)
                .header("Authorization", self.get_authorization_header())
                .json(&rp.body)
                .send()
                .await
                .map_err(|e| ApiError::RequestError(e.to_string()))?;

            if self.should_refresh_tokens(&mut rp, &res) {
                match self.refresh_tokens(&mut rp).await {
                    Ok(_) => continue,
                    Err(e) => {
                        self.log_out();
                        return Err(e);
                    }
                }
            }
            if !res.status().is_success() {
                let data = res
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| ApiError::DataError(e.to_string()))?;
                return Err(ApiError::RequestError(data["detail"].to_string()));
            }

            return Ok(res);
        }
    }

    fn should_refresh_tokens(&mut self, rp: &mut RequestParams, res: &Response) -> bool {
        res.status() == StatusCode::UNAUTHORIZED && rp.can_reauthenticate && self.auth.is_some()
    }

    async fn get(&mut self, mut rp: RequestParams) -> ApiResult<Response> {
        loop {
            let url = Url::parse_with_params(&rp.uri, rp.query_params.clone()).unwrap();
            let res = self
                .client
                .get(url)
                .header("Authorization", self.get_authorization_header())
                .send()
                .await
                .map_err(|e| ApiError::RequestError(e.to_string()))?;

            if self.should_refresh_tokens(&mut rp, &res) {
                match self.refresh_tokens(&mut rp).await {
                    Ok(_) => continue,
                    Err(_) => {
                        self.log_out();
                        return Err(ApiError::Unauthenticated);
                    }
                }
            }
            if !res.status().is_success() {
                let data = res
                    .json::<serde_json::Value>()
                    .await
                    .map_err(|e| ApiError::DataError(e.to_string()))?;
                return Err(ApiError::RequestError(data["detail"].to_string()));
            }

            return Ok(res);
        }
    }

    async fn refresh_tokens(&mut self, rp: &mut RequestParams) -> ApiResult<()> {
        if !rp.can_reauthenticate {
            panic!("Cannot reauthenticate");
        }
        rp.set_cant_reauthenticate();

        let refresh_token_data = schemas::RefreshTokenRequest {
            refresh_token: self
                .auth
                .as_ref()
                .expect("Unauthenticated")
                .refresh_token
                .clone(),
        };
        let res = self
            .client
            .post(&self.auth_url("refresh-token"))
            .json(&refresh_token_data)
            .send()
            .await
            .map_err(|e| ApiError::RequestError(e.to_string()))?;

        let status = res.status();
        let data = res
            .json::<serde_json::Value>()
            .await
            .map_err(|e| ApiError::DataError(e.to_string()))?;
        if !status.is_success() {
            if status == StatusCode::UNAUTHORIZED {
                return Err(ApiError::Unauthenticated);
            }
            return Err(ApiError::RequestError(data["detail"].to_string()));
        }

        // todo duplicate code, same thing in login
        let jwt = data["access_token"].to_string();
        let refresh_token = data["refresh_token"].to_string();
        if jwt.is_empty() || refresh_token.is_empty() {
            panic!("JWT or refresh token is empty");
        }
        let jwt = jwt.trim_matches('"').to_string();
        let refresh_token = refresh_token.trim_matches('"').to_string();

        self.set_auth_tokens(Auth::new(&jwt, &refresh_token));
        Ok(())
    }

    // async fn connect_to_message_ws(&mut self, mut rp: RequestParams) {
    //     loop {
    //         let request = Request::builder()
    //             .uri(MESSAGE_WEBSOCKET_URL)
    //             .header(AUTHORIZATION, self.get_authorization_header())
    //             .header("sec-websocket-key", helpers::generate_sec_websocket_key())
    //             .header("host", HOST)
    //             .header("upgrade", "websocket")
    //             .header("connection", "upgrade")
    //             .header("sec-websocket-version", 13)
    //             .body(())
    //             .expect("Failed to build request.");
    //
    //         match connect_async(request).await {
    //             Ok((ws_stream, _)) => {
    //                 let (write_ws, read_ws) = ws_stream.split();
    //                 self.write_message_ws = Some(write_ws);
    //                 self.read_message_ws = Some(read_ws);
    //                 break;
    //             }
    //             Err(Error::Http(response)) => {
    //                 if response.status() == tungstenite::http::StatusCode::UNAUTHORIZED
    //                     && rp.can_reauthenticate
    //                 {
    //                     if self.auth_tokens.is_some() {
    //                         match self.refresh_tokens(&mut rp).await {
    //                             Ok(_) => continue,
    //                             Err(_) => {
    //                                 self.unauthenticate();
    //                                 return;
    //                             }
    //                         }
    //                     } else {
    //                         panic!("Can't connect to ws: Unauthenticated");
    //                     }
    //                 }
    //                 panic!(
    //                     "Failed to connect to message websocket: {}",
    //                     response.status()
    //                 );
    //             }
    //             Err(e) => {
    //                 panic!("Failed to connect to message websocket: {}", e);
    //             }
    //         }
    //     }
    // }

    fn set_auth_tokens(&mut self, tokens: Auth) {
        self.auth = Some(tokens.clone());
        self.auth_manager.update(tokens);
    }

    fn log_out(&mut self) {
        self.auth = None;
        self.auth_manager.delete();
    }

    fn get_authorization_header(&mut self) -> String {
        format!(
            "Bearer {}",
            self.auth.as_ref().expect("Unauthenticated").access_token
        )
    }

    fn auth_url(&self, endpoint: &str) -> String {
        self.build_url(self.auth_service_api_url.as_str(), endpoint)
    }

    fn user_url(&self, endpoint: &str) -> String {
        self.build_url(self.user_service_api_url.as_str(), endpoint)
    }

    fn message_url(&self, endpoint: &str) -> String {
        self.build_url(self.message_service_api_url.as_str(), endpoint)
    }

    fn build_url(&self, base: &str, endpoint: &str) -> String {
        let base = base.trim_end_matches('/');
        let endpoint = endpoint.trim_start_matches('/');
        format!("{}/{}", base, endpoint)
    }
}

type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Clone)]
pub enum ApiError {
    Unauthenticated,
    RequestError(String),
    DataError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthenticated => write!(f, "Unauthenticated"),
            ApiError::RequestError(e) => write!(f, "Request error: {}", e),
            ApiError::DataError(e) => write!(f, "Data error: {}", e),
        }
    }
}
