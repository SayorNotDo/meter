use axum::routing::{delete, get, post, put};
use axum::Router;

mod case;
mod element;
mod plan;
mod project;

pub fn app() -> Router {
    Router::new()
        .route("/project/:project_id", get(project::info))
        .route("/project", get(project::get_project_list))
        .route(
            "/project/has-permission/:project_id",
            get(project::permission),
        )
        .route("/project/member/list/:project_id", get(project::members))
        .route("/case/module", get(case::get_module_list))
        .route("/case/module", post(case::create_module))
        .route("/case/module", delete(case::delete_module))
        .route("/case/count/:project_id", get(case::count))
        .route("/case/template/:project_id", get(case::template))
        .route("/case/field/:project_id", get(case::get_field_list))
        .route("/case/field", post(case::create_field))
        .route("/case/field", put(case::update_field))
        .route("/case/field", delete(case::delete_field))
        .route("/case/functional-case", post(case::create_functional_case))
        .route("/case/functional-case", get(case::get_functional_case_list))
        .route(
            "/case/functional-case/:case_id",
            get(case::get_functional_case),
        )
        .route("/case/functional-case", put(case::update_functional_case))
        .route(
            "/case/functional-case/issue-relation",
            post(case::create_issue_relation),
        )
        // .route("/case/list/:project_id", get(case::list))
        .route("/case/script/generate", post(case::create_script))
        .route("/case/environment/diagnose", post(case::env_diagnose))
        .route("/case/info/requirement/:project_id", get(case::info))
        .route("/element", post(element::create))
        .route("/element/:element_id", get(element::info))
        .route("/element/module/tree/:project", get(element::tree))
        .route("/element/list/:project_id", get(element::list))
        .route("/element/count/:project_id", get(element::count))
        .route("/test-plan", post(plan::create))
        .route("/test-plan/module/tree/:project_id", get(plan::tree))
        .route("/test-plan/module/count/:project_id", get(plan::count))
        .route("/test-plan/module", post(plan::create_module))
        .route("/test-plan/list/:project_id", get(plan::list))
}
