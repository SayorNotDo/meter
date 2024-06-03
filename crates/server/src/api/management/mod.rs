use axum::Router;
use axum::routing::get;

mod project;

pub fn app() -> Router {
    Router::new()
        .route("/project/:project_id", get(project::info))
        .route("/project/list", get(project::list))
        .route("/project/has-permission/:project_id", get(project::permission))
}