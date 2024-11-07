use crate::errors::{AppResponseError, AppResult};
use crate::state::AppState;
use crate::utils::claim::UserClaims;
use crate::{dto::request::*, dto::response::*, service};
use axum::extract::Extension;
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

/// User Register
#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/auth/register",
    responses(
    (status = 200, description = "Success register user", body = [MessageResponse]),
    (status = 400, description = "Invalid data input", body = [AppResponseError]),
    (status = 409, description = "User already exists", body = [AppResponseError]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn register(
    Extension(state): Extension<AppState>,
    _user: UserClaims,
    Json(request): Json<RegisterRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("Register new user with request: {request:?}");
    request.user_info_list.validate()?;
    match service::user::batch_register(&state, request).await {
        Ok(_) => Ok(Json(MessageResponse::new("Success register user"))),
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
    (status = 401, description = "User already exists", body = [AppResponseError]),
    (status = 403, description = "Disabled user forbidden", body = [AppResponseError]),
    (status = 404, description = "User not found", body = [AppResponseError]),
    (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn login(
    Extension(state): Extension<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    info!("Login user with request: {request:?}.");
    request.validate()?;
    match service::user::login(&state, request).await {
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
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn logout(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<MessageResponse>> {
    info!("Logout user's uuid: {}", user.uid);
    match service::user::logout(&state, user.uid, user.sid).await {
        Ok(_) => {
            info!("Logout successfully");
            Ok(Json(MessageResponse::new("This user has logged out.")))
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
        (status = 200, description = "User is login", body = [LoginResponse]),
        (status = 401, description = "User Unauthorized", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn is_login(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<LoginResponse>> {
    info!("Check if user is already login: {}", user.uid);
    /* 获取用户的access token
     * 检验成功后返回刷新的access token
     */
    match service::user::is_login(&state, user.uid).await {
        Ok(resp) => {
            info!("User is already login, refresh token: {resp:?}");
            Ok(Json(resp))
        }
        Err(e) => {
            info!("User is not login: {e:?}");
            Err(e)
        }
    }
}

/// RefreshToken Refresh
#[utoipa::path(
    post,
    path = "/auth/token/refresh",
    responses(
        (status = 200, description = "Success get new access token and refresh token", body = [TokenResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn token_refresh(
    Extension(state): Extension<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> AppResult<Json<LoginResponse>> {
    info!("Refresh user's token info");
    /* 刷新用户的refresh token */
    match service::token::refresh(&state, request).await {
        Ok(resp) => {
            info!("refresh token successfully");
            Ok(Json(LoginResponse::Token(resp)))
        }
        Err(e) => {
            info!("Failed to refresh token");
            Err(e)
        }
    }
}
