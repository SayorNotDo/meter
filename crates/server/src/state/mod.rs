#[derive(Clone)]
pub struct AppState {
    pub pool: db::Pool,
}

impl AppState {
    pub fn new(pool: db::Pool) -> Self {
        Self { pool }
    }
}
