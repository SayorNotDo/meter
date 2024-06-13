use axum::Router;
use axum::routing::get;

mod project;
mod case;

pub fn app() -> Router {
    Router::new()
        .route("/project/:project_id", get(project::info))
        .route("/project/list", get(project::list))
        .route("/project/has-permission/:project_id", get(project::permission))
        .route("/case/module/tree/:project_id", get(case::tree))
        .route("/case/template/field/:project_id", get(case::template_field))
        .route("/case/list/:project_id", get(case::list))
}