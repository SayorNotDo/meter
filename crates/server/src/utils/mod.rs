use crate::{configure::Config, errors::AppResult};

pub mod claim;
pub mod dir;
pub mod hash;
pub mod header;
pub mod http;
pub mod password;
pub mod smtp;
pub mod task;
pub mod time;

pub trait ClientBuilder: Sized {
    fn build_from_config(config: &Config) -> AppResult<Self>;
}
