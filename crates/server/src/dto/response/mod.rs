use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    dao::entity::ElementDetail,
    entity::{
        file::ModuleType,
        project::{Plan, Project},
        user::User,
    },
    errors::AppResponseError,
};

pub mod case;
pub mod file;
pub mod user;

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
    pub description: Option<String>,
    pub module_list: Vec<String>,
    pub creator_is_admin: bool,
    pub member_count: i32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub admin_list: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectListResponse {
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEntityResponse {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UriPermission {
    pub uri: String,
    pub method: String,
    pub permission: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserRoleOption {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FileModuleResponse {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub parent_id: Option<i32>,
    pub module_type: ModuleType,
    pub count: i32,
    pub children: Vec<FileModuleResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListUserResponse {
    pub list: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListElementResponse {
    pub next_page_token: String,
    pub list: Vec<ElementDetail>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListPlanResponse {
    pub next_page_token: String,
    pub list: Vec<Plan>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RequirementInfoResponse {}

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum AppResultResponse<R> {
    Err(AppResponseError),
    Ok(R),
}

impl<R> AppResultResponse<R> {
    pub const fn is_ok(&self) -> bool {
        matches!(*self, AppResultResponse::Ok(_))
    }

    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }
}
