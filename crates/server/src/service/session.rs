use crate::{
    errors::{AppError, AppResult},
    service::redis::{self, SessionKey},
    utils::claim::UserClaims,
};
use chrono::Utc;
use db::redis::RedisClient;
use log::info;
use std::time::Duration;
use uuid::Uuid;

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
    let session_key = SessionKey { uuid: claims.uid };
    let session_ids = redis::lrange(redis, &session_key, 0, -1)
        .await?
        .ok_or_else(|| {
            AppError::NotFoundError(crate::errors::Resource {
                details: vec![("session_key".to_string(), claims.sid.to_string())],
                resource_type: crate::errors::ResourceType::Session,
            })
        })?
        .iter()
        .map(|item| Uuid::parse_str(item.trim_matches('"')).expect("Failed to parse session_id..."))
        .collect::<Vec<Uuid>>();
    if !session_ids.contains(&claims.sid) {
        info!("Session id invalid so delete it: {session_ids:?}.");
        redis::del(redis, &session_key).await?;
        return Err(AppError::InvalidSessionError(
            "Invalid session id".to_string(),
        ));
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
    if (claims.exp) < Utc::now().timestamp() {
        info!("access_token expired so delete it: {session_ids:?}.");
        redis::lrem(redis, (&session_key, &claims.sid), 0).await?;
        return Err(AppError::UnauthorizedError(
            "access_token is expired".to_string(),
        ));
    }
    Ok(claims.uid)
}

pub async fn delete(redis: &RedisClient, uid: Uuid, sid: Uuid) -> AppResult {
    let session_key = SessionKey { uuid: uid };
    redis::lrem(redis, (&session_key, &sid), 0).await?;
    Ok(())
}

pub async fn destroy(redis: &RedisClient, uid: Uuid) -> AppResult {
    let session_key = SessionKey { uuid: uid };
    redis::del(redis, &session_key).await?;
    Ok(())
}

pub async fn set(redis: &RedisClient, uuid: Uuid) -> AppResult<Uuid> {
    let (key, value) = generate(uuid);
    redis::lpush(redis, (&key, &value)).await?;
    Ok(value)
}

pub fn generate(uuid: Uuid) -> (SessionKey, Uuid) {
    let session_id = Uuid::new_v4();
    let key = SessionKey { uuid };
    (key, session_id)
}
