use axum::routing::{get, post};
use axum::Router;

mod case;
mod element;
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
        .route("/case/functional-case", post(case::create_functional_case))
        .route(
            "/case/functional-case/:case_id",
            get(case::get_functional_case),
        )
        .route(
            "/case/functional-case/issue-relation",
            post(case::create_issue_relation),
        )
        .route("/case/list/:project_id", get(case::list))
        .route("/case/detail/:case_id", get(case::detail))
        .route("/case/script/generate", post(case::create_script))
        .route("/case/environment/diagnose", post(case::env_diagnose))
        .route("/case/info/requirement/:project_id", get(case::info))
        .route("/element", post(element::create))
        .route("/element/:element_id", get(element::info))
        .route("/element/tree/:project", get(element::tree))
        .route("/element/list/:project_id", get(element::list))
}
