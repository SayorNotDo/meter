use axum::extract::Extension;
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::{dto::request::*, dto::response::*, service};
use crate::errors::AppResult;
use crate::state::AppState;

/// User Register
#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/auth/register",
    responses(
    (status = 200, description = "Success register user", body = [RegisterResponse]),
    (status = 400, description = "Invalid data input", body = [AppError]),
    (status = 500, description = "Internal server error", body = [AppError])
    )
)]
pub async fn register(
    Extension(state): Extension<AppState>,
    Json(request): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    info!("Register new user with request: {request:?}");
    request.validate(&())?;
    match service::user::register(state, request).await {
        Ok(user_id) => {
            info!("Successfully register user: {user_id}");
            let resp = RegisterResponse { id: user_id };
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Failed to register user: {e:?}");
            Err(e)
        }
    }
}

/// User Login
#[utoipa::path(
    post,
    request_body = LoginRequest,
    path = "/auth/login",
    responses(
    (status = 200, description = "Login success", body = [LoginResponse]),
    (status = 400, description = "Invalid data input", body = [AppError]),
    (status = 500, description = "Internal server error", body = [AppError])
    )
)]
pub async fn login(
    Extension(state): Extension<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    info!("Login user with request: {request:?}.");
    request.validate(&())?;
    match service::user::login(state, request).await {
        Ok(_msg) => {
            info!("Successfully login user");
            let resp = LoginResponse {
                csrf_token: "".to_string(),
                session_id: "".to_string(),
                token: "".to_string(),
            };
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Failed to login user: {e:?}");
            Err(e)
        }
    }
}

pub async fn logout() {}

// check user is already login
pub async fn is_login() -> Json<()> {
    Json(())
}
