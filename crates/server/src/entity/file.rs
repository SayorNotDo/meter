use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct FileModule {
    pub id: i32,
    pub name: String,
    pub module_type: String,
    pub position: i32,
    pub parent_id: Option<i32>,
}

#[derive(
    Debug, Serialize, strum::Display, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema,
)]
pub enum ModuleType {
    Case,
    Bug,
    Plan,
    Element,
    Unknown,
}

impl ModuleType {
    pub fn from_str(module_type: &str) -> Self {
        match module_type {
            "CASE" => ModuleType::Case,
            "BUG" => ModuleType::Bug,
            "Plan" => ModuleType::Plan,
            "ELEMENT" => ModuleType::Element,
            _ => ModuleType::Unknown,
        }
    }
}
