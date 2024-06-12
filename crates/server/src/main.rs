use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::Extension;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use db::create_pool;
use crate::middleware::auth::AuthLayer;

use crate::state::AppState;

mod api;
mod config;
mod dao;
mod dto;
mod errors;
mod logger;
mod service;
mod state;
mod utils;
mod constant;
mod middleware;


#[tokio::main]
async fn main() {
    /* Logger */
    logger::init();

    /* Config */
    let config = config::Config::parse("./config.toml").expect("Failed to parse configuration file");

    let pool = create_pool(&config.storage.database_url);

    /* CORS */
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().expect("Invalid header value"))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    /* Authorization */
    let authorization = ServiceBuilder::new().layer(
       AuthLayer
    );

    /* State */
    let redis = Arc::new(db::redis_client_builder(&config.storage.redis_url));
    let state = AppState::new(pool, redis).await.expect("Failed to create state.");

    let app = api::create_router()
        .layer(Extension(state.clone()))
        .layer(authorization)
        .layer(cors);


    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind to address.");
    info!(
        "🚀 Server started on {} successfully!",
        listener.local_addr().expect("Failed to get local address.")
    );
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server.");

    /* TODO: graceful shutdown */
}
