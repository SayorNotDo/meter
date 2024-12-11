use crate::{
    constant::{
        ACCESS_TOKEN_ENCODE_KEY, EXPIRE_REFRESH_TOKEN_SECS, EXPIRE_SESSION_CODE_SECS,
        PAGE_DECODE_KEY, PAGE_ENCODE_KEY, REFRESH_TOKEN_DECODE_KEY, REFRESH_TOKEN_ENCODE_KEY,
    },
    dao::user,
    dto::{request::RefreshTokenRequest, response::user::TokenResponse},
    errors::AppResult,
    service,
    state::AppState,
    utils::claim::{PageClaims, UserClaims},
};

use tracing::info;
use uuid::Uuid;

pub async fn refresh(state: &AppState, request: RefreshTokenRequest) -> AppResult<TokenResponse> {
    let user_claims = UserClaims::decode(&request.refresh_token, &REFRESH_TOKEN_DECODE_KEY)?.claims;
    info!("Refresh token: {user_claims:?}.");
    let client = state.pool.get().await?;
    let user_dao = user::UserDao::new(&client);
    let user = user_dao.find_by_uid(&user_claims.uid).await?;
    let session_id = service::session::set(&state.redis, user.uuid).await?;
    info!("Set new session for user: {}", user.uuid);
    let resp = generate_tokens(user.uuid, session_id)?;
    Ok(TokenResponse::new(
        resp.access_token,
        request.refresh_token,
        EXPIRE_SESSION_CODE_SECS.as_secs(),
    ))
}

pub fn generate_tokens(uuid: Uuid, session_id: Uuid) -> AppResult<TokenResponse> {
    let access_token = UserClaims::new(EXPIRE_SESSION_CODE_SECS, uuid, session_id)
        .encode(&ACCESS_TOKEN_ENCODE_KEY)?;
    let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, uuid, session_id)
        .encode(&REFRESH_TOKEN_ENCODE_KEY)?;
    Ok(TokenResponse::new(
        access_token,
        refresh_token,
        EXPIRE_SESSION_CODE_SECS.as_secs(),
    ))
}

pub fn generate_page_token(page_size: i64, page_num: i64, last_item_id: i32) -> AppResult<String> {
    let page_token = PageClaims::new(page_size, page_num, last_item_id).encode(&PAGE_ENCODE_KEY)?;
    Ok(page_token)
}

pub fn parse_page_token(page_token: String) -> AppResult<(i64, i64, i32)> {
    let page_claims = PageClaims::decode(&page_token, &PAGE_DECODE_KEY)?.claims;
    Ok((
        page_claims.page_size,
        page_claims.page_num,
        page_claims.last_item_id,
    ))
}
