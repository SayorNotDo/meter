use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::file::FileModule;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct FunctionalCase {
    pub id: i32,
    pub name: String,
    pub module: FileModule,
    pub template_id: i32,
    pub tags: Option<String>,
    pub status: CaseStatus,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub fields: Vec<Field>,
    pub attach_info: Option<String>,
}

#[derive(
    Debug, Serialize, strum::Display, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema,
)]
pub enum CaseStatus {
    UnReviewed,
    Unknown,
}

impl CaseStatus {
    pub fn from_str(status: &str) -> Self {
        match status {
            "UN_REVIEWED" => CaseStatus::UnReviewed,
            _ => CaseStatus::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct FieldOption {
    pub id: i32,
    pub value: String,
    pub position: i32,
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
pub struct TemplateField {
    pub id: i32,
    pub name: String,
    pub required: bool,
    pub field_type: String,
    pub internal: bool,
    pub default_value: Option<String>,
    pub options: Vec<FieldOption>,
}

impl FunctionalCase {
    pub fn new(name: &str, module: FileModule, template_id: i32, tags: Option<String>) -> Self {
        FunctionalCase {
            id: 0,
            name: name.to_string(),
            module: FileModule {
                id: module.id,
                name: module.name,
                module_type: module.module_type,
                position: module.position,
                parent_id: module.parent_id,
            },
            template_id,
            tags,
            status: CaseStatus::UnReviewed,
            created_at: Utc::now(),
            created_by: "".to_string(),
            updated_at: None,
            updated_by: None,
            fields: Vec::new(),
            attach_info: None,
        }
    }
}
