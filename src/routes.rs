use crate::usecase::user_service::UserService;
use crate::presentation::user_handlers;
use axum::{Router, routing::get};
use std::sync::Arc;

async fn root() -> &'static str {
    "Hello World"
}

pub fn create_router(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(user_handlers::routes(user_service))
}
