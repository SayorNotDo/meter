mod base;
mod system;

use axum::routing::*;

pub fn create_router() -> Router {
    Router::new()
        .nest("/", base::app())
        .nest("/system", system::app())
}
