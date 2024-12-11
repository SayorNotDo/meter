use crate::dao::entity::Step;
use chrono::{DateTime, Utc};
use fake::{
    faker::internet::en::{SafeEmail, Username},
    Dummy,
};

use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub mod case;
pub mod file;
pub mod user;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub user_info_list: Vec<UserInfo>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParam {
    pub idle: bool,
}

#[derive(Debug, Deserialize, Serialize, Dummy, Validate, ToSchema)]
pub struct UserInfo {
    #[dummy(faker = "Username()")]
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[dummy(faker = "SafeEmail()")]
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserInfoUpdateRequest {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct DeleteEntityRequest {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Validate)]
pub struct DeleteRoleRequest {
    #[garde(length(min = 1))]
    pub ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    #[garde(length(min = 30))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ProjectQueryParam {
    pub organization_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct PlanQueryParam {
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct QueryTemplateParam {
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListQueryParam {
    pub module_id: Option<i32>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CaseQueryParam {
    pub deleted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleRequest {
    #[garde(length(min = 3))]
    pub name: String,
    #[garde(skip)]
    pub description: Option<String>,
    #[garde(length(min = 1))]
    pub permission_list: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub module_id: i32,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ElementQueryParam {
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateElementRequest {
    pub name: String,
    pub value: String,
    pub element_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationRequest {
    pub case_id: i32,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub issue_id: String,
    pub source: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateScriptRequest {
    pub name: String,
    pub case_id: i32,
    pub environment: String,
    pub pre_processors: Vec<Step>,
    pub steps: Vec<Step>,
    pub after_processors: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiagnoseRequest {
    pub machine_id: i32,
    pub script_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddMemberRequest {
    pub uid: i32,
    pub role: String,
}
