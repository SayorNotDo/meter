mod auth;
mod display;

use axum::routing::{get, post};
use axum::Router;

pub fn app() -> Router {
    Router::new()
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", get(auth::logout))
        .route("/auth/get-key", get(auth::get_public_key))
        .route("/display/info", get(display::info))
}
