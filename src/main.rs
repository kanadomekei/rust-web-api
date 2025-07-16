mod usecase;
mod domain;
mod infra;
mod presentation;
mod routes;

use crate::{
    usecase::user_service::UserService, infra::in_memory_user_repository::InMemoryUserRepository,
    routes::create_router,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 1. 依存関係の構築 (Dependency Injection)
    let user_repo = Arc::new(InMemoryUserRepository::new());
    let user_service = Arc::new(UserService::new(user_repo));

    // 2. ルーターの作成
    let app = create_router(user_service);

    // 3. サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
