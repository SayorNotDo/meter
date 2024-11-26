use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectMember {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_project_id: Option<i32>,
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
