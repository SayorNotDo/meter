use crate::{dto::request::CreatePlanRequest, service::plan};
use axum::{Extension, Json};

use crate::{errors::AppResult, state::AppState, utils::claim::UserClaims};

use tracing::info;

#[utoipa::path(
    post,
    path = "/management/plan",
    request_body = CreatePlanRequest,
    responses()
)]
pub async fn create(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<CreatePlanRequest>,
) -> AppResult {
    info!("controller layer create plan with request: {request:?}");
    match plan::create(&state, user.uid, request).await {
        Ok(resp) => Ok(resp),
        Err(e) => Err(e),
    }
}
