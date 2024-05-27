// use uuid::Uuid;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: i32,
    // pub uuid: Uuid,
}
