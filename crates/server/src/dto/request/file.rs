use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateModuleRequest {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub parent_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateModuleRequest {
    #[garde(skip)]
    pub id: i32,
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(skip)]
    pub parent_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteModuleRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct QueryModuleParam {
    pub module_id: Option<i32>,
}
