use crate::{
    dao::entity::User, dto::response::ListUserResponse, errors::AppResult, service,
    state::AppState, utils::claim::UserClaims,
};
use axum::{Extension, Json};
use tracing::info;

#[utoipa::path(
    get,
    path = "/user/list/:project_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn list(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<ListUserResponse>> {
    info!("controller layer get user list");
    match service::user::list(&state, user.uid).await {
        Ok(resp) => Ok(Json(ListUserResponse { list: resp })),
        Err(e) => Err(e),
    }
}
