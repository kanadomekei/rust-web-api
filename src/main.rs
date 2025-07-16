use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .route("/hello/:name", get(hello_name))
        .route("/json", get(json_handler));

    // run it with hyper on localhost:8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, axum!!"
}

async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!!", name)
}

async fn json_handler() -> Json<User> {
    let user = User {
        id: 1,
        name: "axum user".to_string(),
        email: "axum@example.com".to_string(),
    };
    Json(user)
}
