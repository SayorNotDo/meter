use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::entity::case::FieldOption;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFunctionalCaseRequest {
    #[garde(length(min = 1))]
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

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFunctionalCaseRequest {
    #[garde(skip)]
    pub case_id: i32,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub module_id: i32,
    #[garde(skip)]
    pub tags: Option<String>,
    #[garde(length(min = 1))]
    pub fields: Vec<SelectedField>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectedField {
    pub id: i32,
    pub value: FieldValue,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum FieldValue {
    Select(i32),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFieldRequest {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub field_type: String,
    #[garde(skip)]
    pub remark: Option<String>,
    #[garde(length(min = 1))]
    pub options: Option<Vec<FieldOption>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct QueryCaseParam {
    pub case_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFieldRequest {
    #[garde(skip)]
    pub id: i32,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub field_type: String,
    #[garde(skip)]
    pub remark: Option<String>,
    #[garde(length(min = 1))]
    pub options: Option<Vec<FieldOption>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFieldRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QueryFieldParam {
    pub field_id: Option<i32>,
}
