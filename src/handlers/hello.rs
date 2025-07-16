use axum::{extract::Path, routing::get, Router};

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Returns a static greeting message", body = String)
    )
)]
pub async fn handler() -> &'static str {
    "Hello, axum!!"
}

#[utoipa::path(
    get,
    path = "/hello/{name}",
    params(
        ("name" = String, Path, description = "Name of the person to greet")
    ),
    responses(
        (status = 200, description = "Returns a personalized greeting message", body = String)
    )
)]
pub async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!!", name)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/hello/:name", get(hello_name))
} 