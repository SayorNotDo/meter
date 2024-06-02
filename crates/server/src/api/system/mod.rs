mod parameter;
mod project;

use axum::Router;

use axum::routing::get;

pub fn app() -> Router {
    Router::new()
        .route("/parameter/save/base-url", get(parameter::save_baseurl))
        .route("/project/:project_id", get(project::info))
}
