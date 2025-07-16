use crate::{handlers, openapi::ApiDoc};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(handlers::hello::routes())
        .merge(handlers::user::routes())
} 