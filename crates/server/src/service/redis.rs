use std::fmt::{Display, Debug};
use chrono::Duration;
use serde::{Deserialize, Serialize};

pub trait RedisKey: Debug + Display {
    type Value: Serialize + Deserialize + Debug;
    const EXPIRE_TIME: Duration;

    fn expire(&self) -> Duration {
        Self::EXPIRE_TIME
    }
}