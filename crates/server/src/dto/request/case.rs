use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::dao::entity::FieldOption;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFunctionalCaseRequest {
    #[garde(skip)]
    pub name: String,
    #[garde(skip)]
    pub module_id: i32,
    #[garde(skip)]
    pub template_id: i32,
    #[garde(skip)]
    pub tags: Option<String>,
    #[garde(length(min = 1))]
    pub fields: Vec<SelectedField>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectedField {
    pub field_id: i32,
    pub option_id: Option<i32>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFieldRequest {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub field_type: String,
    #[garde(skip)]
    pub project_id: i32,
    #[garde(skip)]
    pub remark: Option<String>,
    #[garde(skip)]
    pub options: Option<Vec<FieldOption>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFieldRequest {
    #[garde(skip)]
    pub id: i32,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub project_id: i32,
    #[garde(skip)]
    pub field_type: String,
    #[garde(skip)]
    pub remark: Option<String>,
    #[garde(skip)]
    pub options: Option<Vec<FieldOption>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryFieldParam {
    pub field_id: Option<i32>,
}
