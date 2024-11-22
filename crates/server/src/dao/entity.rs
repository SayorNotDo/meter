use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::errors::ResourceType;

pub trait AppEntity {
    const RESOURCE: ResourceType;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub enable: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_project_id: Option<i32>,
}

impl User {
    pub fn new(
        username: &str,
        password: &str,
        email: &str,
        created_by: Uuid,
        gen_uuid: bool,
    ) -> Self {
        let username = username.to_lowercase();

        // generate UUID
        let uuid = if gen_uuid {
            Uuid::new_v4()
        } else {
            Uuid::nil()
        };

        Self {
            id: 0,
            uuid,
            username,
            hashed_password: password.to_string(),
            email: email.into(),
            enable: false,
            created_at: Utc::now(),
            created_by,
            updated_at: None,
            last_project_id: None,
        }
    }
}

impl AppEntity for User {
    const RESOURCE: ResourceType = ResourceType::User;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserRole {
    pub id: i32,
    pub name: String,
    pub role_type: String,
    pub internal: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

impl AppEntity for UserRole {
    const RESOURCE: ResourceType = ResourceType::Role;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct UserRoleRelation {
    pub id: i32,
    pub user_id: Uuid,
    pub role_id: i32,
    pub project_id: i32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserRoleOption {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectMember {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_project_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct UserRolePermission {
    pub user_role: UserRole,
    pub permission_list: Vec<Permission>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema, Clone)]
pub struct Permission {
    pub id: i32,
    pub module: String,
    pub scope: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FileModule {
    pub id: i32,
    pub name: String,
    pub module_type: String,
    pub position: i32,
    pub parent_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub internal: bool,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub fields: Vec<TemplateField>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct Field {
    pub id: i32,
    pub name: String,
    pub project_id: i32,
    pub field_type: String,
    pub remark: Option<String>,
    pub internal: bool,
    pub options: Vec<FieldOption>,
}

impl Field {
    pub fn new(name: &str, field_type: &str, remark: Option<String>, project_id: i32) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            project_id,
            field_type: field_type.to_string(),
            remark,
            internal: false,
            options: vec![],
        }
    }
}

pub enum FieldType {
    Text,
    Select,
    Unknown,
}

impl FieldType {
    pub fn from_str(field_type: &str) -> Self {
        match field_type {
            "TEXT" => FieldType::Text,
            "SELECT" => FieldType::Select,
            _ => FieldType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct TemplateField {
    pub id: i32,
    pub name: String,
    pub required: bool,
    pub field_type: String,
    pub internal: bool,
    pub default_value: Option<String>,
    pub options: Vec<FieldOption>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct FieldOption {
    pub id: i32,
    pub value: String,
    pub position: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct PlanDetail {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub belong_project: String,
    pub belong_module: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Plan {
    pub id: i32,
    pub name: String,
    pub project_id: i32,
    pub module_id: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl Plan {
    pub fn new(
        name: &str,
        project_id: i32,
        module_id: i32,
        created_by: Uuid,
        description: Option<String>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Self {
        Plan {
            id: 0,
            name: name.to_string(),
            project_id,
            module_id,
            created_at: Utc::now(),
            created_by,
            status: "NEW".into(),
            description,
            start_date,
            end_date,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FunctionalCase {
    pub id: i32,
    pub name: String,
    pub module_id: i32,
    pub template_id: i32,
    pub tags: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub custom_fields: Vec<Field>,
}

impl FunctionalCase {
    pub fn new(
        name: &str,
        module_id: i32,
        template_id: i32,
        tags: Option<String>,
        created_by: Uuid,
    ) -> Self {
        FunctionalCase {
            id: 0,
            name: name.to_string(),
            module_id,
            template_id,
            tags,
            status: "UN_REVIEWED".into(),
            created_at: Utc::now(),
            created_by,
            custom_fields: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct CaseDetail {
    pub id: i32,
    pub name: String,
    pub module_name: String,
    pub template_id: i32,
    pub tags: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub custom_fields: Vec<Field>,
    pub attach_info: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CaseModuleInfo {
    pub id: i32,
    pub name: String,
    pub case_count: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Element {
    pub id: i32,
    pub name: String,
    pub module: String,
    pub value: String,
    pub element_type: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub operation_options: Vec<OperationOption>,
}

impl Element {
    pub fn new(
        name: &str,
        value: &str,
        element_type: &str,
        description: Option<&str>,
        created_by: Uuid,
    ) -> Self {
        let description = description.map(|s| s.to_string());
        Element {
            id: 0,
            name: name.into(),
            value: value.into(),
            module: "".into(),
            element_type: element_type.into(),
            description,
            created_at: Utc::now(),
            created_by,
            operation_options: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct ElementDetail {
    pub id: i32,
    pub name: String,
    pub module: String,
    pub value: String,
    pub element_type: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub operation_options: Vec<OperationOption>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ElementInfo {
    pub name: String,
    pub action: String,
    pub element_type: String,
    pub selector: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct OperationOption {
    pub id: i32,
    pub name: String,
    pub internal: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script {
    pub case_id: i32,
    pub path: String,
    pub environment: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub position: i32,
    pub element_id: i32,
    pub option_id: i32,
    pub attach_info: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    pub addr: String,
    pub authentication: String,
    pub user: String,
    pub password: String,
}
