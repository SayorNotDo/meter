use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
