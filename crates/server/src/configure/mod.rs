use serde::Deserialize;
use std::fs;

use config::ConfigError;
use secret::ConfigJWT;
use server::ConfigHTTP;
use smtp::ConfigSMTP;
use storage::ConfigStorage;

use crate::utils::dir::get_project_root;

pub mod secret;
pub mod server;
pub mod smtp;
pub mod storage;
pub mod template;
pub mod tracing;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
    pub jwt: ConfigJWT,
    pub storage: ConfigStorage,
    pub smtp: ConfigSMTP,
}

impl Config {
    pub fn parse(path: &str) -> anyhow::Result<Self> {
        let config_str = fs::read_to_string(path)?;

        let config = toml::from_str(&config_str)?;

        Ok(config)
    }
}

pub fn get_static_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|err| ConfigError::Message(err.to_string()))?
        .join("static"))
}
