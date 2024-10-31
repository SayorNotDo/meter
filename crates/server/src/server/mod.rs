use crate::constant::{ALLOW_METHOD, ALLOW_ORIGIN};
use axum::extract::Extension;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use std::time::Duration;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};

use crate::{
    api,
    configure::Config,
    errors::AppResult,
    middleware::{access::AccessLayer, auth::AuthLayer},
    state::AppState,
};

pub struct AppServer {
    pub state: AppState,
    tcp: tokio::net::TcpListener,
}

impl AppServer {
    pub async fn new(mut config: Config) -> AppResult<Self> {
        let tcp = tokio::net::TcpListener::bind(config.http.get_socket_addr()?).await?;
        let addr = tcp.local_addr()?;
        tracing::info!("🚀 Server started on {addr} successfully!");
        config.http.http_port = addr.port();

        let state = AppState::new(config).await?;
        Ok(Self { state, tcp })
    }

    pub async fn run(self) -> AppResult<()> {
        /* Middleware */
        let authorization = ServiceBuilder::new().layer(AuthLayer);
        let _access = ServiceBuilder::new().layer(AccessLayer);

        let cors = CorsLayer::new()
            .allow_origin(ALLOW_ORIGIN)
            .allow_methods(ALLOW_METHOD)
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let app = api::create_router()
            .layer(authorization)
            .layer(Extension(self.state))
            .layer(TraceLayer::new_for_http())
            .layer(TimeoutLayer::new(Duration::from_secs(10)))
            .layer(cors)
            .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") });

        axum::serve(self.tcp, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
