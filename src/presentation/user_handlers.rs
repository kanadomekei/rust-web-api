use crate::usecase::user_service::UserService;
use crate::domain::user::CreateUser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

pub async fn create_user(
    State(user_service): State<Arc<UserService>>,
    Json(input): Json<CreateUser>,
) -> impl IntoResponse {
    let user = user_service.create_user(input).await;
    (StatusCode::CREATED, Json(user))
}

pub async fn get_users(State(user_service): State<Arc<UserService>>) -> impl IntoResponse {
    let users = user_service.get_all_users().await;
    (StatusCode::OK, Json(users))
}

pub async fn get_user(
    State(user_service): State<Arc<UserService>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    match user_service.get_user_by_id(id).await {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_user(
    State(user_service): State<Arc<UserService>>,
    Path(id): Path<u64>,
    Json(input): Json<CreateUser>,
) -> impl IntoResponse {
    match user_service.update_user(id, input).await {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_user(
    State(user_service): State<Arc<UserService>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    if user_service.delete_user(id).await {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub fn routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(user_service)
} 