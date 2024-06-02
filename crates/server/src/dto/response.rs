// use uuid::Uuid;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::constant::BEARER;


#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into() }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectInfoResponse {}


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub enum LoginResponse {
    Token(TokenResponse),
    Code {
        message: String,
        expire_in: u64,
    },
}

impl From<TokenResponse> for LoginResponse {
    fn from(value: TokenResponse) -> Self {
        LoginResponse::Token(value)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: u64,
}

impl TokenResponse {
    pub fn new(access_token: String, refresh_token: String, expire_in: u64) -> Self {
        Self {
            token_type: BEARER.to_string(),
            access_token,
            refresh_token,
            expire_in,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfoResponse {
    pub last_organization_id: String,
    pub last_project_id: String,
}