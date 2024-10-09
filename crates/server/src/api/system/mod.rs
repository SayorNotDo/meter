use axum::routing::get;
use axum::Router;

mod parameter;
mod user;

pub fn app() -> Router {
    Router::new()
        .route("/parameter/save/base-url", get(parameter::save_baseurl))
        .route("/user/list", get(user::list))
}
