use crate::{
    dto::{request::CreateElementRequest, response::ElementResponse},
    errors::AppResult,
    service::element,
    state::AppState,
};
use axum::{Extension, Json};
use tracing::info;

#[utoipa::path(
    post,
    path = "/management/element",
    request_body = ElementRequest,
    responses(
        (status = 200, description = "Element created", body = [ElementResponse])
    ),
    security(("jwt" = []))
)]
pub async fn create(
    Extension(state): Extension<AppState>,
    Json(request): Json<CreateElementRequest>,
) -> AppResult<Json<ElementResponse>> {
    info!("controller layer create element with requst: {request:?}");
    match element::create(&state, request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

pub async fn info(Extension(state): Extension<AppState>) -> AppResult<Json<ElementResponse>> {
    Ok(Json(ElementResponse {}))
}
