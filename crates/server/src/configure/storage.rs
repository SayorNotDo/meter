use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigStorage {
    pub database_url: String,
    pub redis_url: String,
    pub script_path: String,
    pub template_path: String,
}
