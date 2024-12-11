use crate::entity::{
    case::{CaseField, CaseResult, CaseStatus, TemplateField},
    file::FileModule,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FunctionalCaseResponse {
    pub id: i32,
    pub name: String,
    pub tags: Vec<String>,
    pub template_id: i32,
    pub module: FileModule,
    pub status: CaseStatus,
    pub edit_type: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub attach_info: Option<String>,
    pub fields: Vec<CaseField>,
    pub last_execute_result: CaseResult,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetTemplateResponse {
    pub id: i32,
    pub name: String,
    pub internal: bool,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub fields: Vec<TemplateField>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListFunctionalCaseResponse {
    pub total: i32,
    pub next_page_token: String,
    pub list: Vec<FunctionalCaseResponse>,
}
