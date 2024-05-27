mod auth;
pub mod openapi;

use axum::routing::{get, post};
use axum::Router;

pub fn app() -> Router {
    Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", get(auth::logout))
        .route("/auth/is-login", get(auth::is_login))
}
