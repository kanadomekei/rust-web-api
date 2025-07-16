use crate::{handlers, openapi::ApiDoc};
use axum::Router;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::user::User;

type Db = Arc<RwLock<HashMap<u64, User>>>;

pub fn create_router() -> Router {
    let db = Db::default();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(handlers::hello::routes())
        .merge(handlers::user::routes(db))
} 