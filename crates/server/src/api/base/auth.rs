use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
    pub authenticate: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub csrf_token: String,
    pub session_id: String,
    pub token: String,
}

pub async fn login(request: Json<Request>) -> Json<Response> {
    Json(Response {
        csrf_token: "".to_string(),
        session_id: "".to_string(),
        token: "".to_string(),
    })
}

pub async fn logout() -> Json<Response> {
    Json(Response {
        csrf_token: "".to_string(),
        session_id: "".to_string(),
        token: "".to_string(),
    })
}

pub async fn get_public_key() -> String {
    "".to_string()
}
