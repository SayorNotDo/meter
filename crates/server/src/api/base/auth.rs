use axum::extract::{Extension, State};
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
    (status = 400, description = "Invalid data input", body = [AppResponseError]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
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
    (status = 404, description = "User not found", body = [AppResponseError]),
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
            info!("Success login user: {resp:?}");
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
    ),
    security(("jwt" = []))
)]
pub async fn logout(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<MessageResponse>> {
    info!("Logout user's uuid: {}", user.uid);
    match service::user::logout(&state, user.uid).await {
        Ok(_) => {
            info!("Logout successfully");
            Ok(Json(MessageResponse::new(
                "This user has logged out.",
            )))
        }
        Err(e) => {
            warn!("Failed to logout: {e:?}");
            Err(e)
        }
    }
}

/// User is-Login
#[utoipa::path(
get,
path = "/auth/is-login",
responses(
    (status = 200, description = "User is login", body = [()]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
),
security(("jwt" = []))
)]
pub async fn is_login(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<LoginResponse>> {
    info!("Check if user is already login: {}",user.uid);
    match service::user::is_login(&state, user.uid).await {
        Ok(resp) => {
            info!("User is already login, refresh token: {resp:?}");
            Ok(Json(resp))
        },
        Err(e) => {
            Err(e)
        }
    }
}
