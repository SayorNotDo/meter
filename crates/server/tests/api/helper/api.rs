use super::result::AppResponseResult;
use log_derive::logfn;
use reqwest::StatusCode;
use server::configure::server::ConfigHTTP;
use server::constant::HTTP;
use server::dto::request::*;
// use server::dto::response::MessageResponse;
use server::utils::http::HttpClientExt;

pub struct Api {
    addr: String,
}

impl Api {
    pub fn new(config: &ConfigHTTP) -> Self {
        Self {
            addr: config.get_http_addr(),
        }
    }

    #[logfn(Info)]
    pub async fn register(
        &self,
        req: &RegisterRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let resp = HTTP
            .post_request(&format!("{}/auth/register", self.addr), req)
            .await?;
        let status = resp.status();
        let json_body = resp.json().await?;
        Ok((status, json_body))
    }

    #[logfn(Info)]
    pub async fn login(
        &self,
        req: &LoginRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let resp = HTTP
            .post_request(&format!("{}/auth/login", self.addr), req)
            .await?;
        let status = resp.status();
        let json_body = resp.json().await?;
        Ok((status, json_body))
    }
}
