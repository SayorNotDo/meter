use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateModuleRequest {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub project_id: i32,
    #[garde(skip)]
    pub parent_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteModuleRequest {
    pub id: i32,
}
