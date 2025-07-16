use crate::usecase::user_service::UserService;
use crate::presentation::user_handlers;
use axum::Router;
use std::sync::Arc;

pub fn create_router(user_service: Arc<UserService>) -> Router {
    Router::new().merge(user_handlers::routes(user_service))
} 