use tracing::info;
use uuid::Uuid;
use crate::constant::{ACCESS_TOKEN_ENCODE_KEY, EXPIRE_SESSION_CODE_SECS, REFRESH_TOKEN_ENCODE_KEY};
use crate::errors::AppResult;
use crate::dto::response::TokenResponse;
use crate::dto::request::RefreshTokenRequest;
use crate::state::AppState;
use crate::constant::REFRESH_TOKEN_DECODE_KEY;
use crate::utils::claim::UserClaims;
use crate::dao::user;
use crate::service;

pub async fn refresh(state: &AppState, request: RefreshTokenRequest) -> AppResult<TokenResponse> {
    let user_claims = UserClaims::decode(
        &request.refresh_token, &REFRESH_TOKEN_DECODE_KEY,
    )?.claims;
    info!("Refresh token: {user_claims:?}.");
    let user_id = service::session::check(
        &state.redis, &user_claims,
    ).await?;
    let client = state.pool.get().await?;
    let user_dao = user::UserDao::new(client);
    let user = user_dao.find_by_uid(user_id).await?;
    let session_id = service::session::set(&state.redis, user.uuid).await?;
    info!("Set new session for user: {}", user.uuid);
    let resp = generate_tokens(user.uuid, session_id)?;
    Ok(resp)
}

pub fn generate_tokens(
    uuid: Uuid,
    session_id: Uuid,
) -> AppResult<TokenResponse> {
    let access_token = UserClaims::new(EXPIRE_SESSION_CODE_SECS, uuid, session_id)
        .encode(&ACCESS_TOKEN_ENCODE_KEY)?;
    let refresh_token = UserClaims::new(EXPIRE_SESSION_CODE_SECS, uuid, session_id)
        .encode(&REFRESH_TOKEN_ENCODE_KEY)?;
    Ok(TokenResponse::new(
        access_token,
        refresh_token,
        EXPIRE_SESSION_CODE_SECS.as_secs(),
    ))
}