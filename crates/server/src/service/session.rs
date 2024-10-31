use crate::{
    errors::{AppError, AppResult},
    service::redis::{self, SessionKey},
    utils::claim::UserClaims,
};
use db::redis::RedisClient;
use tracing::info;
use uuid::Uuid;

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
    let session_key = SessionKey { uuid: claims.uid };
    let session_id = redis::get(redis, &session_key).await?.ok_or_else(|| {
        AppError::NotFoundError(crate::errors::Resource {
            details: vec![("session_key".to_string(), claims.sid.to_string())],
            resource_type: crate::errors::ResourceType::Session,
        })
    })?;
    if claims.sid != session_id {
        info!("Session id invalid so delete it: {session_id:?}.");
        redis::del(redis, &session_key).await?;
        return Err(AppError::InvalidSessionError(
            "Invalid session id".to_string(),
        ));
    }
    Ok(claims.uid)
}

pub async fn set(redis: &RedisClient, uuid: Uuid) -> AppResult<Uuid> {
    let (key, value) = generate(uuid);
    redis::set(redis, (&key, &value)).await?;
    Ok(value)
}

pub fn generate(uuid: Uuid) -> (SessionKey, Uuid) {
    let session_id = Uuid::new_v4();
    let key = SessionKey { uuid };
    (key, session_id)
}
