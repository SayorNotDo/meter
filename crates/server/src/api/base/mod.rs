mod auth;
pub mod openapi;
pub mod user;

use axum::routing::{get, post};
use axum::Router;

pub fn app() -> Router {
    Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", get(auth::logout))
        .route("/auth/is-login", get(auth::is_login))
        .route("/auth/token/refresh", post(auth::token_refresh))
        .route("/user/info", get(user::info))
        .route("/user/list", get(user::list))
        .route("/user/role/list/:project_id", get(user::role_list))
}
