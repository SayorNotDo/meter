mod api;
mod dao;
mod dto;
mod errors;
mod logger;
mod service;
mod state;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Extensions, HeaderValue, Method
};
use db::create_pool;
use std::net::SocketAddr;

use tracing::info;

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    /* Logger */
    logger::init();

    /* Config */
    let config = config::Config::parse("./config.toml");

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



    let app = api::create_router().layer(cors).layer(Extensions(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!(
        "🚀 Server started on {} successfully",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
