use axum::routing::get;
use axum::Router;

mod case;
mod project;

pub fn app() -> Router {
    Router::new()
        .route("/project/:project_id", get(project::info))
        .route("/project/list", get(project::list))
        .route(
            "/project/has-permission/:project_id",
            get(project::permission),
        )
        .route("/project/member/list/:project_id", get(project::members))
        .route("/case/module/tree/:project_id", get(case::tree))
        .route("/case/count/:project_id", get(case::count))
        .route("/case/template/:project_id", get(case::template))
        .route("/case/field/:project_id", get(case::field))
        .route("/case/list/:project_id", get(case::list))
        .route("/case/detail/:case_id", get(case::detail))
        .route("/case/info/requirement/:project_id", get(case::info))
}
