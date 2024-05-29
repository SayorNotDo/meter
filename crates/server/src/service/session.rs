use uuid::Uuid;
use db::redis::RedisClient;
use crate::errors::AppResult;
use crate::service::redis;
use crate::service::redis::SessionKey;

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