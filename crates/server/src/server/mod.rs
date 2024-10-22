use axum::extract::Extension;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method, StatusCode,
};
use db::create_pool;
use std::sync::Arc;
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
    utils::smtp,
};

pub struct AppServer {
    pub state: AppState,
    pub tcp: tokio::net::TcpListener,
}

impl AppServer {
    pub async fn new(mut config: Config) -> AppResult<Self> {
        let tcp = tokio::net::TcpListener::bind(config.http.get_socket_addr()?).await?;
        let addr = tcp.local_addr()?;
        tracing::info!("🚀 Server started on {addr} successfully!");
        config.http.http_port = addr.port();

        let pool = create_pool(&config.storage.database_url);
        let redis = Arc::new(db::redis_client_builder(&config.storage.redis_url));
        let email = Arc::new(smtp::email_client_builder(&config.smtp));

        let state = AppState::new(pool, redis, email).await?;
        Ok(Self { state, tcp })
    }

    pub async fn run(self) -> AppResult<()> {
        let authorization = ServiceBuilder::new().layer(AuthLayer);
        let _access = ServiceBuilder::new().layer(AccessLayer);

        let cors = CorsLayer::new()
            .allow_origin(
                "http://localhost:3000"
                    .parse::<HeaderValue>()
                    .expect("Invalid header value"),
            )
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::OPTIONS,
                Method::DELETE,
                Method::PUT,
            ])
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
