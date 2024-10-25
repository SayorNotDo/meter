use serde::{Deserialize, Serialize};
use server::{dto::response::MessageResponse, errors::AppResponseError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppResponseResult<T = MessageResponse> {
    Ok(T),
    Err(AppResponseError),
}

impl<T> AppResponseResult<T> {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(*self, Self::Ok(_))
    }

    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(*self, Self::Err(_))
    }
}

#[macro_export]
macro_rules! unwrap {
    ($result:expr) => {
        match $result {
            $crate::helper::result::AppResponseResult::Ok(resp) => resp,
            $crate::helper::result::AppResponseResult::Err(err) => {
                panic!("Called `common::unwrap!()` on an `Err` value {err:?}.")
            }
        }
    };
}
