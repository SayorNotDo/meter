use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
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
pub struct Permission {
    pub id: i32,
    pub role_id: i32,
    pub permission: String,
}