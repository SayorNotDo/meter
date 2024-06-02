use axum::Router;
use axum::routing::get;

mod parameter;


pub fn app() -> Router {
    Router::new()
        .route("/parameter/save/base-url", get(parameter::save_baseurl))
}
