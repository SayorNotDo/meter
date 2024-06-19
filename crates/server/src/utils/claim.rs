use std::time::Duration;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::constant::ACCESS_TOKEN_DECODE_KEY;
use crate::errors::{AppError, AppResult};

pub static DECODE_HEADER: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::RS256));
pub static ENCODE_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::RS256));

pub static PAGE_ENCODE_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::HS256));
pub static PAGE_DECODE_HEADER: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    validation.required_spec_claims.clear();
    validation
});

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, ToSchema)]
pub struct UserClaims {
    pub iat: i64,
    pub exp: i64,
    pub uid: Uuid,
    pub sid: Uuid,
}

impl UserClaims {
    pub fn new(duration: Duration, uuid: Uuid, session_id: Uuid) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + (duration.as_secs() as i64),
            uid: uuid,
            sid: session_id,
        }
    }

    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<UserClaims>(token, key, &DECODE_HEADER)
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&ENCODE_HEADER, self, key)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;
        let user_claims = UserClaims::decode(bearer.token(), &ACCESS_TOKEN_DECODE_KEY)?.claims;
        Ok(user_claims)
    }
}

pub trait UserClaimsRequest {
    #[allow(dead_code)]
    fn get_user_id(&self) -> AppResult<Uuid>;
    #[allow(dead_code)]
    fn get_user_claims(&self) -> AppResult<UserClaims>;
}

impl UserClaimsRequest for axum::extract::Request {
    fn get_user_id(&self) -> AppResult<Uuid> {
        self.extensions()
            .get::<UserClaims>()
            .map(|u| u.uid)
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }
    fn get_user_claims(&self) -> AppResult<UserClaims> {
        self.extensions()
            .get::<UserClaims>()
            .cloned()
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, ToSchema)]
pub struct PageClaims {
    pub page_size: i64,
    pub page_num: i64,
}

impl PageClaims {
    pub fn new(page_size: i64, page_num: i64) -> Self {
        Self {
            page_size,
            page_num,
        }
    }

    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<PageClaims>(token, key, &PAGE_DECODE_HEADER)
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&PAGE_ENCODE_HEADER, self, key)
    }
}
