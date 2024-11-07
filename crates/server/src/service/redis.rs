use crate::errors::AppResult;
use db::redis::{RedisClient, RedisClientExt};
use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use crate::constant::EXPIRE_SESSION_CODE_SECS;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

pub trait RedisKey: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIRE_TIME: Duration;

    #[allow(dead_code)]
    fn expire(&self) -> Duration {
        Self::EXPIRE_TIME
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SessionKey {
    pub uuid: Uuid,
}

impl RedisKey for SessionKey {
    type Value = Uuid;
    const EXPIRE_TIME: Duration = EXPIRE_SESSION_CODE_SECS;
}

impl Display for SessionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SESSION_KEY:{}", self.uuid)
    }
}

#[allow(dead_code)]
pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> AppResult<()>
where
    K: RedisKey,
{
    info!("Set value to redis key: {key:?} value: {value:?}");
    let value = serde_json::to_string(value)?;
    client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
    Ok(())
}

pub async fn lpush<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> AppResult<()>
where
    K: RedisKey,
{
    info!("LPUSH value to redis key: {key:?} value: {value:?}");
    let value = serde_json::to_string(value)?;
    client
        .lpush(&key.to_string(), &value, K::EXPIRE_TIME)
        .await?;
    Ok(())
}

pub async fn lrem<K>(
    client: &RedisClient,
    (key, value): (&K, &K::Value),
    count: i32,
) -> AppResult<()>
where
    K: RedisKey,
{
    info!("LREM value to redis key: {key:?} value: {value:?}");
    let value = serde_json::to_string(value)?;
    client.lrem(&key.to_string(), &value, count).await?;
    Ok(())
}

pub async fn lrange<K>(
    client: &RedisClient,
    key: &K,
    start: i32,
    stop: i32,
) -> AppResult<Option<Vec<String>>>
where
    K: RedisKey,
{
    info!("LRANGE value from redis key: {key:?}");
    Ok(client.lrange(&key.to_string(), start, stop).await?)
}

pub async fn get<K>(client: &RedisClient, key: &K) -> AppResult<Option<K::Value>>
where
    K: RedisKey,
{
    info!("Get value from redis key: {key:?}");
    Ok(client
        .get(&key.to_string())
        .await?
        .map(|v| serde_json::from_str::<K::Value>(&v))
        .transpose()?)
}

pub async fn del(client: &RedisClient, key: &impl RedisKey) -> AppResult<bool> {
    info!("Delete redis key: {key:?}");
    Ok(client.del(&key.to_string()).await?)
}

#[allow(dead_code)]
pub async fn get_ttl(client: &RedisClient, key: &impl RedisKey) -> AppResult<i64> {
    info!("Get redis key: {key:?} ttl");
    Ok(client.ttl(&key.to_string()).await?)
}

#[allow(dead_code)]
pub async fn check_exist(client: &RedisClient, key: &impl RedisKey) -> AppResult<bool> {
    info!("Check redis key: {key:?} exist");
    Ok(client.exist(&key.to_string()).await?)
}
