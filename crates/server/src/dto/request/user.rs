use fake::Dummy;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserStatusRequest {
    #[garde(length(min = 1))]
    pub select_ids: Vec<i32>,
    #[garde(skip)]
    pub enable: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct DeleteUserRequest {
    #[garde(length(min = 1))]
    pub ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, Dummy)]
pub struct LoginRequest {
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[garde(length(min = 8))]
    pub password: String,
}
