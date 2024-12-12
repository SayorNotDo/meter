use crate::{
    configure::Config,
    errors::{AppError, AppResult},
};

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

pub fn parse_ids(input: &str) -> AppResult<Vec<i32>> {
    input
        .split(',')
        .map(|s| s.parse::<i32>().map_err(AppError::from))
        .collect()
}
