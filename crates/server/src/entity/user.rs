use super::{permission::Permission, AppEntity};
use crate::errors::ResourceType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

impl AppEntity for User {
    const RESOURCE: ResourceType = ResourceType::User;
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

impl AppEntity for UserRole {
    const RESOURCE: ResourceType = ResourceType::Role;
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct UserRolePermission {
    pub user_role: UserRole,
    pub permission_list: Vec<Permission>,
}
