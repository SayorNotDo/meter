mod base;
mod system;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// use axum::routing::*;

pub fn create_router() -> Router {
    let router = Router::new().merge(
        SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", base::openapi::ApiDoc::openapi()),
    );
    router.nest("/", base::app())
        .nest("/system", system::app())
}
