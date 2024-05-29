use serde::Deserialize;
use std::fs;

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
    pub secret: String,
    pub expires: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigStorage {
    pub database_url: String,
    pub redis_url: String
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
