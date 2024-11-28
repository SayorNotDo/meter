use crate::configure::{env::get_env_source, get_static_dir, template::EmailTemplateEngine};
use crate::utils::{http::HttpClient, ClientBuilder};
use axum::http::{HeaderValue, Method};
use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub const ENV_PREFIX: &str = "APP";

// Development mode change 120 to 604800
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(604800);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(604800);
pub const BEARER: &str = "Bearer";
pub const AUTHORIZATION: &str = "Authorization";
pub const PROJECT_ID: &str = "Project";

pub const WHITE_LIST: [&str; 3] = ["/auth/login", "/swagger-ui", "/auth/is-login"];
pub const ALLOW_METHOD: [Method; 6] = [
    Method::GET,
    Method::POST,
    Method::PATCH,
    Method::OPTIONS,
    Method::DELETE,
    Method::PUT,
];
pub const ALLOW_ORIGIN: [HeaderValue; 1] = [HeaderValue::from_static("http://localhost:3000")];
pub const ACCESS_WHITE_LIST: [&str; 5] = [
    "/auth/login",
    "/auth/logout",
    "/swagger-ui",
    "/user/info",
    "/auth/is-login",
];
pub const EMAIL_ADDR: &str = "chenwentao@datatower.ai";
pub const REGISTER_EMAIL_SUBJECT: &str = "<DTest-测试平台> 注册邮件通知";

pub static CONFIG: Lazy<crate::configure::Config> =
    Lazy::new(|| crate::configure::Config::read(get_env_source(ENV_PREFIX)).unwrap());

pub static HTTP: Lazy<reqwest::Client> =
    Lazy::new(|| HttpClient::build_from_config(&CONFIG).unwrap());

pub static DOCTOR_SCRIPT_PATH: &str = "./static/scripts/doctor";

pub static ACCESS_TOKEN_ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    let key = CONFIG.jwt.read_private_access_key().unwrap();
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static ACCESS_TOKEN_DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    let key = CONFIG.jwt.read_public_access_key().unwrap();
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static REFRESH_TOKEN_ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| {
    let key = CONFIG.jwt.read_private_refresh_key().unwrap();
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static REFRESH_TOKEN_DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    let key = CONFIG.jwt.read_public_refresh_key().unwrap();
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

pub static PAGE_ENCODE_KEY: Lazy<EncodingKey> =
    Lazy::new(|| EncodingKey::from_base64_secret("U29tZVNlY3JldEtleQ==".into()).unwrap());

pub static PAGE_DECODE_KEY: Lazy<DecodingKey> =
    Lazy::new(|| DecodingKey::from_base64_secret("U29tZVNlY3JldEtleQ==".into()).unwrap());

pub static TEMPLATE_ENGINE: Lazy<EmailTemplateEngine> = Lazy::new(|| {
    let path = get_static_dir()
        .expect("failed to get static dir")
        .join("templates/**/*")
        .into_os_string()
        .into_string()
        .expect("failed to get template path");
    EmailTemplateEngine::new(&path).expect("failed to get contant template engine")
});
