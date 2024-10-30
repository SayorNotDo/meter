use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigSMTP {
    pub host: String,
    pub port: u16,
    pub protocol: Protocol,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, strum::Display, strum::EnumString)]
pub enum Protocol {
    #[serde(rename = "starttls")]
    #[strum(serialize = "starttls")]
    STARTTLS,
    #[serde(rename = "local")]
    #[strum(serialize = "local")]
    LOCAL,
}
