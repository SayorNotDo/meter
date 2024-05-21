use axum::Json;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub param_key: String,
    pub session_value: String,
    pub info_type: String,
    pub sort: u16,
    pub file_name: String,
    pub file: String,
}

pub async fn info() -> Json<Response> {
    Json(Response {
        param_key: "".to_string(),
        session_value: "".to_string(),
        info_type: "".to_string(),
        sort: 1,
        file_name: "".to_string(),
        file: "".to_string(),
    })
}
