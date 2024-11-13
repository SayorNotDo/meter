use axum::{
    routing::{delete, get, post, put},
    Router,
};

mod parameter;
mod user;

pub fn app() -> Router {
    Router::new()
        .route("/parameter/save/base-url", get(parameter::save_baseurl))
        .route("/user/list", get(user::list))
        .route("/user/status", put(user::update_status))
        .route("/user", delete(user::delete))
        .route(
            "/user/role/permission/list",
            get(user::role_permission_list),
        )
        .route("/user/role", post(user::create_role))
        .route("/user/role/:role_id", get(user::get_role))
        .route("/user/role/permission/:role_id", get(user::role_permission))
        .route("/user/role", delete(user::delete_role))
}
