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
    pub tags: Vec<String>,
    pub status: CaseStatus,
    pub edit_type: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub fields: Vec<CaseField>,
    pub attach_info: Option<String>,
}

impl FunctionalCase {
    pub fn new(name: &str, module: FileModule, template_id: i32, tags: Vec<String>) -> Self {
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
            edit_type: String::from("STEP"),
            created_at: Utc::now(),
            created_by: "".to_string(),
            fields: vec![],
            updated_at: None,
            updated_by: None,
            attach_info: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
pub struct CaseExecuteRecord {
    pub id: i32,
    pub case_id: i32,
    pub result: CaseResult,
    pub attach_info: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaseResult {
    UnExecuted,
    Passed,
    Blocked,
    Skipped,
    Failed,
    Unknown,
}

impl CaseResult {
    pub fn from_str(result: &str) -> Self {
        match result {
            "UN_EXECUTED" => CaseResult::UnExecuted,
            "PASSED" => CaseResult::Passed,
            "BLOCKED" => CaseResult::Blocked,
            "SKIPPED" => CaseResult::Skipped,
            "FAILED" => CaseResult::Failed,
            _ => CaseResult::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaseStatus {
    UnReviewed,
    Unknown,
}

impl ToString for CaseStatus {
    fn to_string(&self) -> String {
        let status_str = match self {
            Self::UnReviewed => "UN_REVIEWED",
            Self::Unknown => "UNKNOWN",
        };
        format!("{}", status_str)
    }
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
    pub field_id: i32,
    pub value: String,
    pub position: i32,
}

pub trait FieldInfo {
    fn required(&self) -> bool;
    fn id(&self) -> i32;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct CaseField {
    pub id: i32,
    pub field_name: String,
    pub label: String,
    pub project_id: i32,
    pub field_id: i32,
    pub field_type: FieldType,
    pub remark: Option<String>,
    pub field_value: FieldValue,
    pub options: Option<Vec<FieldOption>>,
    pub internal: bool,
    pub required: bool,
    pub unique_required: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectedField {
    pub id: i32,
    pub required: bool,
    pub value: FieldValue,
}

impl FieldInfo for SelectedField {
    fn id(&self) -> i32 {
        self.id
    }

    fn required(&self) -> bool {
        self.required
    }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub enum FieldType {
    Select,
    Input,
    Unknown,
}

impl FieldType {
    pub fn from_str(field_type: &str) -> Self {
        match field_type {
            "INPUT" => FieldType::Input,
            "SELECT" => FieldType::Select,
            _ => FieldType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub enum FieldValue {
    Select(i32),
    Input(String),
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
    pub label: String,
    pub required: bool,
    pub unique_required: bool,
    pub field_type: String,
    pub internal: bool,
    pub default_value: Option<String>,
    pub options: Vec<FieldOption>,
}

impl FieldInfo for TemplateField {
    fn id(&self) -> i32 {
        self.id
    }

    fn required(&self) -> bool {
        self.required
    }
}
