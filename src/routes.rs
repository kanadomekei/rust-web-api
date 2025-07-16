use crate::handlers;
use axum::Router;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::domain::user::User;

type Db = Arc<RwLock<HashMap<u64, User>>>;

pub fn create_router() -> Router {
    let db = Db::default();

    Router::new()
        .merge(handlers::hello::routes())
        .merge(handlers::user::routes(db))
} 