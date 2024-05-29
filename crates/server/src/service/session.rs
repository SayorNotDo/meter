use uuid::Uuid;
use db::redis::RedisClient;
use crate::errors::AppResult;

pub async fn set(redis: &RedisClient, uuid: Uuid) -> AppResult<Uuid> {
    let (key, value) = generate(uuid);

}

pub fn generate(uuid: Uuid) -> (SessionKey, Uuid) {

}