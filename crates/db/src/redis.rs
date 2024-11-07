use std::future::Future;
use std::time::Duration;

use redis::{Client, RedisError};
pub type RedisClient = Client;

pub trait RedisClientExt {
    fn ping(&self) -> impl Future<Output = Result<Option<String>, RedisError>>;

    fn set(
        &self,
        key: &str,
        value: &str,
        expire: Duration,
    ) -> impl Future<Output = Result<(), RedisError>>;

    fn lpush(
        &self,
        key: &str,
        value: &str,
        expire: Duration,
    ) -> impl Future<Output = Result<(), RedisError>>;

    fn lrem(
        &self,
        key: &str,
        value: &str,
        count: i32,
    ) -> impl Future<Output = Result<(), RedisError>>;

    fn lrange(
        &self,
        key: &str,
        start: i32,
        stop: i32,
    ) -> impl Future<Output = Result<Option<Vec<String>>, RedisError>>;

    fn exist(&self, key: &str) -> impl Future<Output = Result<bool, RedisError>>;

    fn get(&self, key: &str) -> impl Future<Output = Result<Option<String>, RedisError>>;

    fn del(&self, key: &str) -> impl Future<Output = Result<bool, RedisError>>;

    fn ttl(&self, key: &str) -> impl Future<Output = Result<i64, RedisError>>;
}

impl RedisClientExt for Client {
    async fn ping(&self) -> Result<Option<String>, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(value)
    }

    async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let _msg: String = redis::cmd("SET")
            .arg(&[key, value])
            .query_async(&mut conn)
            .await?;
        let _msg: i32 = redis::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    async fn lpush(&self, key: &str, value: &str, expire: Duration) -> Result<(), RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let _msg: String = redis::cmd("LPUSH")
            .arg(&[key, value])
            .query_async(&mut conn)
            .await?;
        let _msg: i32 = redis::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    async fn lrem(&self, key: &str, value: &str, count: i32) -> Result<(), RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let _msg: i32 = redis::cmd("LREM")
            .arg(&[key, &count.to_string(), value])
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    async fn lrange(
        &self,
        key: &str,
        start: i32,
        stop: i32,
    ) -> Result<Option<Vec<String>>, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value = redis::cmd("LRANGE")
            .arg(&[key, &start.to_string(), &stop.to_string()])
            .query_async(&mut conn)
            .await?;
        Ok(value)
    }
    async fn exist(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value = redis::cmd("EXISTS")
            .arg(&[key])
            .query_async(&mut conn)
            .await?;
        Ok(value)
    }

    async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value = redis::cmd("GET").arg(&[key]).query_async(&mut conn).await?;
        Ok(value)
    }

    async fn del(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        Ok(value == 1)
    }

    async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value = redis::cmd("TTL").arg(&[key]).query_async(&mut conn).await?;
        Ok(value)
    }
}
