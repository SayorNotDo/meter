use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct Plan {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub project_id: i32,
    pub module_id: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
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
            updated_at: None,
            updated_by: None,
            created_at: Utc::now(),
            created_by,
            status: "NEW".into(),
            description,
            start_date,
            end_date,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub module_setting: Option<String>,
}

impl Project {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        created_by: Uuid,
        description: Option<String>,
        module_setting: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            name,
            created_at: Utc::now(),
            created_by,
            updated_at: None,
            updated_by: None,
            deleted_by: None,
            deleted_at: None,
            description,
            module_setting,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct ProjectInfo {
    pub id: i32,
    pub name: String,
    pub member_count: i32,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub enable: bool,
    pub description: Option<String>,
    pub module_setting: Option<String>,
}
