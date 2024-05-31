use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::utils;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub http: ConfigHTTP,
    pub jwt: ConfigJWT,
    pub storage: ConfigStorage,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigHTTP {
    pub host: String,
    pub http_port: u16,
    pub enable_https: bool,
    pub https_port: u16,
    pub cors: Vec<String>,
    pub tls_cert: String,
    pub tls_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigJWT {
    pub private_access_key: PathBuf,
    pub public_access_key: PathBuf,
    pub private_refresh_key: PathBuf,
    pub public_refresh_key: PathBuf,
}

impl ConfigJWT {
    pub fn read_private_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.private_access_key))
    }
    pub fn read_public_access_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.public_access_key))
    }
    pub fn read_private_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.private_refresh_key))
    }
    #[allow(dead_code)]
    pub fn read_public_refresh_key(&self) -> Result<String, std::io::Error> {
        fs::read_to_string(utils::dir::get_project_root()?.join(&self.public_refresh_key))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigStorage {
    pub database_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn parse(path: &str) -> anyhow::Result<Self> {
        let config_str = fs::read_to_string(path)?;

        let config = toml::from_str(&config_str)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        Config::parse("./config.toml").expect("Failed to parse configuration file");
    }
}
