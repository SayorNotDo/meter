use crate::{dto::request::AddMemberRequest, utils::claim::UserClaims};
use
use axum::{Extension, Json};

use crate::{errors::AppResult, state::AppState};

#[utoipa::path(
    post,
    path = "management/member/add",
    responses(),
    security(("jwt" = []))
)]
pub async fn add(
    Extension(state): Extension<AppState>,
    Json(request): Json<AddMemberRequest>,
    user: UserClaims,
) -> AppResult {
    info!("controller layer add member with request: {request:?}");
    Ok(())
}
