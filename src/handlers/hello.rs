use axum::{extract::Path, routing::get, Router};

pub async fn handler() -> &'static str {
    "Hello, axum!!"
}

pub async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!!", name)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/hello/:name", get(hello_name))
} 