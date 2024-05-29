use uuid::Uuid;
use crate::constant::{ACCESS_TOKEN_ENCODE_KEY, EXPIRE_SESSION_CODE_SECS, REFRESH_TOKEN_ENCODE_KEY};
use crate::errors::AppResult;
use crate::dto::response::TokenResponse;
use crate::utils::claim::UserClaims;

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