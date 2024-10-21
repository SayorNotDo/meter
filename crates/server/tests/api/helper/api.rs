use server::configure::server::ConfigHTTP;

pub struct Api {
    addr: String,
}

impl Api {
    pub fn new(config: &ConfigHTTP) -> Self {
        Self {
            addr: config.get_http_addr(),
        }
    }
}
