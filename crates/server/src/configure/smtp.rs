use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigSMTP {
    pub host: String,
    pub port: u16,
    // pub tls_off: bool,
    pub username: String,
    pub password: String,
}
