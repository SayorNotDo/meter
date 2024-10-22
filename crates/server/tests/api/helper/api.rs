use super::result::AppResponseResult;
use log_derive::logfn;
use reqwest::StatusCode;
use server::configure::server::ConfigHTTP;
use server::constant::HTTP;
use server::dto::request::*;
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
        Ok((resp.status(), resp.json().await?))
    }
}
