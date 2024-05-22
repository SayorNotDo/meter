use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::info;
USE crate::{dto::*, service};
use crate::dao::user::User;

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

#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/auth/register",
    response(
        (status = 200, description = "Success register user", body = [RegisterResponse]),
        (status = 400, description = "Invalid data input", body = [CustomError]),
        (status = 500, description = "Internal server error", body = [CustomError])
    )
)]
pub async fn register(State(state): State<AppState>, request: RegisterRequest) -> Json<RegisterResponse> {
    info!("Register new user with request: {request:?}");

    request.validate(&())?;
}

pub async fn login(request: Json<Request>) -> Json<Response> {
    let user = User::new(&request.username, &request.password, false);
    println!("{:?}", user);
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

// check user is already login
pub async fn is_login() -> Json<()> {
    Json(())
}
