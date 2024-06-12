use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(120);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(604800);
pub const BEARER: &str = "Bearer";
pub const AUTHORIZATION: &str = "Authorization";

pub const WHITE_LIST: [&str; 2] = ["/auth/login", "/auth/register"];

pub static CONFIG: Lazy<crate::config::Config> = Lazy::new(|| crate::config::Config::parse("./config.toml").unwrap());
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