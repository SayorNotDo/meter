use utoipa::OpenApi;

#[allow(dead_code)]
#[derive(OpenApi)]
#[openapi(info(title = "Meter API", description = "Meter API", version = "1.0.0"))]
pub struct ApiDoc;
