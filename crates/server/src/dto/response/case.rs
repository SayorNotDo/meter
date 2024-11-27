use crate::entity::{case::Field, file::FileModule};
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
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub attach_info: Option<String>,
    pub fields: Vec<Field>,
}
