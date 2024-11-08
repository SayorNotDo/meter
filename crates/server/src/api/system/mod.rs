use axum::routing::{delete, get, post, put};
use axum::Router;

mod parameter;
mod user;

pub fn app() -> Router {
    Router::new()
        .route("/parameter/save/base-url", get(parameter::save_baseurl))
        .route("/user/list", get(user::list))
        .route("/user/status", put(user::update_status))
        .route("/user", delete(user::delete))
        .route("/role/permission/list", get(user::role_permission_list))
        .route("/role", post(user::create_role))
        .route("/role/permission/:role_id", get(user::role_permission))
}
