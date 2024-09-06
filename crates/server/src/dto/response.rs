use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::constant::BEARER;
use crate::dao::entity::{
    CaseDetail, CustomField, Element, ElementDetail, User, UserRole, UserRolePermission,
    UserRoleRelation,
};
use crate::dao::project::ProjectInfo;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectInfoResponse {
    pub id: i32,
    pub name: String,
    pub organization: String,
    pub description: Option<String>,
    pub module_list: Vec<String>,
    pub creator_is_admin: bool,
    pub member_count: i32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
    pub admin_list: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectInfo>,
}

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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FileModuleResponse {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub parent_id: Option<i32>,
    pub module_type: String,
    pub count: i64,
    pub children: Vec<FileModuleResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListCaseResponse {
    pub next_page_token: String,
    pub list: Vec<CaseDetail>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListElementResoponse {
    pub next_page_token: String,
    pub list: Vec<ElementDetail>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TemplateResponse {
    pub id: i32,
    pub name: String,
    pub internal: bool,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RequirementInfoResponse {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CaseDetailResponse {
    pub id: i32,
    pub name: String,
    pub tags: Vec<String>,
    pub template_id: i32,
    pub module_name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub attach_info: Option<String>,
    pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ElementResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub element_type: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateScriptResponse {
    pub id: i32,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DiagnoseResponse {
    pub msg: String,
}
