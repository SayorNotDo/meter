// use uuid::Uuid;

use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::constant::BEARER;
use crate::dao::user::{UserRole, UserRolePermission, UserRoleRelation};

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
pub struct ProjectInfoResponse {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
    pub description: Option<String>,
    pub module_setting: Option<String>,
    pub member_count: i32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
    pub adminList: Vec<()>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectInfoResponse>,
}


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
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_organization_id: Option<i32>,
    pub last_project_id: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
    pub user_role_permissions: Vec<UserRolePermission>,
    pub user_role_relations: Vec<UserRoleRelation>,
    pub user_roles: Vec<UserRole>,
}