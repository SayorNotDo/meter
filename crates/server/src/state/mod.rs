use axum::Extension;

#[derive(Clone)]
pub struct AppState {
    pub pool: db::Pool,
}

impl AppState {
    pub async fn new(Extension(pool): Extension<db::Pool>) -> Self {
        Self { pool }
    }
}
