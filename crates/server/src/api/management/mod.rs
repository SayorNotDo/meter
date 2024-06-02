use axum::Router;
use axum::routing::get;

mod project;

pub fn app() -> Router {
    Router::new()
        .route("/project/:project_id", get(project::info))
}