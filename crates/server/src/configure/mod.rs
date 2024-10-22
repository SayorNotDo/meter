use ::tracing::info;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

use config::{ConfigError, Environment};
use secret::ConfigJWT;
use server::ConfigHTTP;
use smtp::ConfigSMTP;
use storage::ConfigStorage;

use crate::utils::dir::get_project_root;

pub mod env;
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

    pub fn read(env: Environment) -> Result<Self, ConfigError> {
        let config_dir = get_setting_dir()?;
        let profile = std::env::var("APP_PROFILE")
            .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
            .unwrap_or_else(|_| Ok(Profile::Dev))?;
        let profile_filename = format!("{profile}.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(config::File::from(config_dir.join(profile_filename)))
            .add_source(env)
            .build()?;
        info!("Successfully read config profile: {profile}.");
        config.try_deserialize()
    }
}

pub fn get_static_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("static"))
}

pub fn get_setting_dir() -> Result<std::path::PathBuf, ConfigError> {
    Ok(get_project_root()
        .map_err(|e| ConfigError::Message(e.to_string()))?
        .join("settings"))
}

#[derive(
    Debug,
    Deserialize,
    strum::Display,
    strum::EnumString,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "test")]
    #[strum(serialize = "test")]
    Test,
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "prod")]
    #[strum(serialize = "prod")]
    Prod,
}
