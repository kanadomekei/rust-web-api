mod domain;
mod handlers;
mod openapi;
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
