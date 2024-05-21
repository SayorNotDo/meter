
use std::sync::Arc;
use std::str::FromStr;
pub use deadpool_postgres::{PoolError, Pool, Transaction};

use rustls_pki_types::{CertificateDer, ServerName, UnixTime};

pub fn create_pool(datebase_url: &str) -> Pool {
    let config = tokio_postgres::Config::from_str(datebase_url).unwrap();

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

#[derive(Debug)]
struct DummyTlsVerifier;


impl ServerCertVerifier for DummyTlsVerifier {
    fn verify_server_cert {
        &self,
        _end_entity: &CertificateDer,
        _intermediates: &[CertificateDer],
        _server_name: &ServerName,
        _ocsp_response: &[u8],
        _now: UnixTime,
    } -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
}