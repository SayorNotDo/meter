pub use cornucopia_async::GenericClient;

pub use cornucopia_async::Params;
use std::{str::FromStr, sync::Arc};

pub use deadpool_postgres::{Client, Pool, PoolError, Transaction};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls_pki_types::{CertificateDer, ServerName, UnixTime};
pub use tokio_postgres::Error as TokioPostgresError;

pub mod redis;

pub fn create_pool(database_url: &str) -> Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();

    let manager = if config.get_ssl_mode() != tokio_postgres::config::SslMode::Disable {
        let tls_config = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(DummyTlsVerifier))
            .with_no_client_auth();

        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
        deadpool_postgres::Manager::new(config, tls)
    } else {
        deadpool_postgres::Manager::new(config, tokio_postgres::NoTls)
    };
    Pool::builder(manager).build().unwrap()
}

pub fn redis_client_builder(redis_url: &str) -> redis::RedisClient {
    redis::RedisClient::open(redis_url).unwrap()
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));

#[derive(Debug)]
struct DummyTlsVerifier;

impl ServerCertVerifier for DummyTlsVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer,
        _intermediates: &[CertificateDer],
        _server_name: &ServerName,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        Vec::new()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn load_users() {
//         let db_url = std::env::var("DATABASE_URL").unwrap();
//         let pool = create_pool(&db_url);

//         let client = pool.get().await.unwrap();

//         let users = crate::queries::user::get_users()
//             .bind(&client)
//             .all()
//             .await
//             .unwrap();

//         dbg!(users);
//     }
// }
