use crate::{
    dto::{request::CreateElementRequest, response::ElementResponse},
    errors::AppResult,
    service::element,
    state::AppState,
    utils::claim::UserClaims,
};
use axum::{extract::Extension, Json};
use tracing::info;

#[utoipa::path(
    post,
    path = "/management/element",
    request_body = CreateElementRequest,
    responses(
        (status = 200, description = "Element created", body = [ElementResponse])
    ),
    security(("jwt" = []))
)]
pub async fn create(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<CreateElementRequest>,
) -> AppResult<Json<ElementResponse>> {
    info!("controller layer create element with request: {request:?}");
    match element::create(&state, user.uid, request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

pub async fn info(Extension(state): Extension<AppState>) -> AppResult<Json<ElementResponse>> {
    info!("controller layer query element information with params");
    Ok(Json(ElementResponse {}))
}
