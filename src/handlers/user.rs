use crate::domain::user::{CreateUser, User};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type Db = Arc<RwLock<HashMap<u64, User>>>;

pub async fn create_user(
    State(db): State<Db>,
    Json(input): Json<CreateUser>,
) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    let id = db.keys().max().unwrap_or(&0) + 1;
    let user = User {
        id,
        name: input.name,
        email: input.email,
    };
    db.insert(id, user.clone());
    (StatusCode::CREATED, Json(user))
}

pub async fn get_users(State(db): State<Db>) -> impl IntoResponse {
    let db = db.read().unwrap();
    let users: Vec<User> = db.values().cloned().collect();
    (StatusCode::OK, Json(users))
}

pub async fn get_user(State(db): State<Db>, Path(id): Path<u64>) -> impl IntoResponse {
    let db = db.read().unwrap();
    if let Some(user) = db.get(&id) {
        (StatusCode::OK, Json(user.clone())).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn update_user(
    State(db): State<Db>,
    Path(id): Path<u64>,
    Json(input): Json<CreateUser>,
) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    if let Some(user) = db.get_mut(&id) {
        user.name = input.name;
        user.email = input.email;
        (StatusCode::OK, Json(user.clone())).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn delete_user(State(db): State<Db>, Path(id): Path<u64>) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    if db.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(db)
} 