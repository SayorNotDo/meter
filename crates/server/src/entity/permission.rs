use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema, Clone)]
pub struct Permission {
    pub id: i32,
    pub module: String,
    pub scope: String,
}
