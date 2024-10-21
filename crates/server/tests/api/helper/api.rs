use super::result::AppResponseResult;
use server::configure::server::ConfigHTTP;
use log_derive::logfn;

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
    pub async fn register(&self, req: &RegisterRequest) -> anyhow::Result<(StatusCode, AppResponseResult<RegisterResponse>)> {

    }
}
