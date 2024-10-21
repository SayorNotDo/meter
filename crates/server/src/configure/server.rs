use std::net::{AddrParseError, SocketAddr};

use serde::Deserialize;

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

impl ConfigHTTP {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.http_port)
    }

    pub fn get_http_addr(&self) -> String {
        format!("http://{}:{}", self.host, self.http_port)
    }

    pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
        self.get_addr().parse()
    }
}
