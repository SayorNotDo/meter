use axum::Json;

pub async fn save_baseurl() -> Json<()> {
    Json(())
}
