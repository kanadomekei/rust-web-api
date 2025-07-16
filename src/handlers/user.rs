use crate::domain::user::User;
use axum::{routing::get, Json, Router};

#[utoipa::path(
    get,
    path = "/json",
    responses(
        (status = 200, description = "Returns a user object as JSON", body = User)
    )
)]
pub async fn json_handler() -> Json<User> {
    let user = User {
        id: 1,
        name: "axum user".to_string(),
        email: "axum@example.com".to_string(),
    };
    Json(user)
}

pub fn routes() -> Router {
    Router::new().route("/json", get(json_handler))
} 