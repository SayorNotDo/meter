use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use db::redis::{RedisClient, RedisClientExt};
use crate::errors::AppResult;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tracing::info;
use uuid::Uuid;

use crate::constant::EXPIRE_SESSION_CODE_SECS;

pub trait RedisKey: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIRE_TIME: Duration;

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
        write!(f, "SESSION_KEY_{}", self.uuid)
    }
}

pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> AppResult<()>
    where
        K: RedisKey,
{
    info!("Set value to redis key: {key:?} value: {value:?}");
    let value = serde_json::to_string(value)?;
    client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
    Ok(())
}

pub async fn get<K>(client: &RedisClient, key: &K) -> AppResult<Option<K::Value>>
    where
        K: RedisKey,
{
    info!("Get value from redis key: {key:?}");
    Ok(
        client.get(&key.to_string())
            .await?
            .map(|v| serde_json::from_str::<K::Value>(&v))
            .transpose()?,
    )
}

pub async fn del(client: &RedisClient, key: &impl RedisKey) -> Result<bool, redis::RedisError> {
    info!("Delete redis key: {key:?}");
    client.del(&key.to_string()).await
}
