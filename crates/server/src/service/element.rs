use crate::{
    dto::{request::CreateElementRequest, response::ElementResponse},
    errors::AppResult,
    state::AppState,
};

pub async fn create(state: &AppState, request: CreateElementRequest) -> AppResult<ElementResponse> {
    Ok(ElementResponse {})
}
