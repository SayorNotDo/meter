use crate::{
    constant::BEARER,
    entity::user::{UserRole, UserRolePermission, UserRoleRelation},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub enum LoginResponse {
    Token(TokenResponse),
    Code { message: String, expire_in: u64 },
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
pub struct GetUserInfoResponse {
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_project_id: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
    pub user_role_permissions: Vec<UserRolePermission>,
    pub user_role_relations: Vec<UserRoleRelation>,
    pub user_roles: Vec<UserRole>,
}
