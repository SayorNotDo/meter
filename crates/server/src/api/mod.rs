mod base;
mod engine;
mod management;
mod system;
use axum::Router;
// use base::openapi::ApiDoc;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;
// use axum::routing::*;

pub fn create_router() -> Router {
    // let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi()).split_for_parts();
    // router
    // .merge(SwaggerUi::new("swagger-ui").url("/api-docs/openapi.jsoin", api.clone()))
    Router::new()
        .merge(base::app())
        .nest("/system", system::app())
        .nest("/management", management::app())
        .nest("/engine", engine::app())
}
