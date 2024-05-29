use axum::extract::Extension;
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::errors::AppResult;
use crate::state::AppState;
use crate::utils::claim::UserClaims;
use crate::{dto::request::*, dto::response::*, service};

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
    (status = 400, description = "Invalid data input", body = [AppResponseError]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn login(
    Extension(state): Extension<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    info!("Login user with request: {request:?}.");
    request.validate(&())?;
    match service::user::login(state, request).await {
        Ok(resp) => {
            info!("Successfully login user");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Failed to login user: {e:?}");
            Err(e)
        }
    }
}

/// User Logout
#[utoipa::path(
get,
path = "/auth/logout",
responses(
    (status = 200, description = "Logout success", body = [MessageResponse]),
    (status = 400, description = "Unauthorized user", body = [AppResponseError]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
)
security(("jwt" = []))
)]
pub async fn logout(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<MessageResponse>> {
    info!("Logout user's uuid: {}", user.uid);
    match service::user::logout(&state).await {
        Ok(resp) => {
            info!("Successfully logout");
            Ok(Json(MessageResponse::new(
                "This user has successfully logout.",
            )))
        }
        Err(e) => {
            warn!("Failed to logout: {e:?}");
            Err(e)
        }
    }
}

// check user is already login
pub async fn is_login() -> Json<()> {
    Json(())
}
