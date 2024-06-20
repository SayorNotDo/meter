use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_organization_id: Option<i32>,
    pub last_project_id: Option<i32>,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str, gen_uuid: bool) -> Self {
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
            created_at: Utc::now(),
            updated_at: None,
            last_project_id: None,
            last_organization_id: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserRoleRelation {
    pub id: i32,
    pub user_id: Uuid,
    pub role_id: i32,
    pub organization_id: i32,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserRolePermission {
    pub user_role: UserRole,
    pub user_role_permissions: Vec<Permission>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectMember {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_project_id: Option<i32>,
    pub last_organization_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Permission {
    pub id: i32,
    pub role_id: i32,
    pub permission: String,
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
    pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CustomField {
    pub id: i32,
    pub name: String,
    pub required: bool,
    pub field_type: String,
    pub internal: bool,
    pub default_value: Option<String>,
    pub options: Vec<FieldOption>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FieldOption {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub position: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CaseInfo {
    pub id: i32,
    pub name: String,
    pub module_id: i32,
    pub template_id: i32,
    pub tags: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CaseModuleInfo {
    pub id: i32,
    pub name: String,
    pub case_count: i64,
}
