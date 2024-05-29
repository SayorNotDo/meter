use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::Extension;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;
use tracing::info;

use db::create_pool;

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


#[tokio::main]
async fn main() {
    /* Logger */
    logger::init();

    /* Config */
    let config = config::Config::parse("./config.toml").unwrap();

    let pool = create_pool(&config.storage.database_url);

    /* CORS */
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    /* State */
    let redis = Arc::new(db::redis_client_builder(&config.storage.redis_url));
    let state = AppState::new(pool, redis);

    let app = api::create_router()
        .layer(cors)
        .layer(Extension(state.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!(
        "🚀 Server started on {} successfully!",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
