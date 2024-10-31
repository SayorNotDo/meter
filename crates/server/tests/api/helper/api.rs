use super::result::AppResponseResult;
use crate::unwrap;
use log_derive::logfn;
use reqwest::StatusCode;
use server::{
    configure::server::ConfigHTTP,
    constant::HTTP,
    dto::{request::*, response::*},
    utils::http::HttpClientExt,
};
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
        token: &str,
        req: &RegisterRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let resp = HTTP
            .post(&format!("{}/auth/register", self.addr))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
            .json(req)
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn login(
        &self,
        req: &LoginRequest,
    ) -> anyhow::Result<(StatusCode, AppResponseResult<LoginResponse>)> {
        let resp = HTTP
            .post_request(&format!("{}/auth/login", self.addr), req)
            .await?;
        Ok((resp.status(), resp.json().await?))
    }

    #[logfn(Info)]
    pub async fn get_token(&self, req: &LoginRequest) -> anyhow::Result<TokenResponse> {
        let (_, resp) = self.login(req).await?;
        let resp = unwrap!(resp);
        match resp {
            LoginResponse::Token(token) => Ok(token),
            LoginResponse::Code { .. } => Err(anyhow::anyhow!("Get token failed...")),
        }
    }

    #[logfn(Info)]
    pub async fn logout(&self, token: &str) -> anyhow::Result<(StatusCode, AppResponseResult)> {
        let resp = HTTP
            .get(&format!("{}/auth/logout", self.addr))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?;
        Ok((resp.status(), resp.json().await?))
    }
}
