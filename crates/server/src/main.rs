use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::Extension;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method, StatusCode,
};

use server::errors::AppResult;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use db::create_pool;

use server::api;

use server::configure;
use server::middleware::{access::AccessLayer, auth::AuthLayer};
use server::state::AppState;
use server::utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    /* Logger */
    // logger::init();
    let _file_appender_guard = configure::tracing::init()?;
    info!("The initialization of Tracing was successful.");

    /* Config */
    let config = configure::Config::parse("./config.toml")?;

    let pool = create_pool(&config.storage.database_url);

    /* CORS */
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

    /* Authorization */
    let authorization = ServiceBuilder::new().layer(AuthLayer);
    let _access = ServiceBuilder::new().layer(AccessLayer);

    /* State */
    let redis = Arc::new(db::redis_client_builder(&config.storage.redis_url));
    let email = Arc::new(utils::smtp::email_client_builder(&config.smtp));
    let state = AppState::new(pool, redis, email).await?;

    /* Initialize App */
    let app = api::create_router()
        // .layer(access)
        .layer(authorization)
        .layer(Extension(state.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(
            // Graceful shutdown will wait for outstanding requests to complete.
            // Add a timeout so requests dont't hang over.
            TimeoutLayer::new(Duration::from_secs(10)),
        )
        .layer(cors)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") });

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!(
        "🚀 Server started on {} successfully!",
        listener.local_addr()?
    );
    /* Run server with graceful shutodwn */
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
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
