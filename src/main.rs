mod usecase;
mod domain;
mod infra;
mod presentation;
mod routes;

use crate::{
    usecase::user_service::UserService,
    infra::sqlx_user_repository::SqlxUserRepository,
    routes::create_router,
};
use std::sync::Arc;
use sqlx::{SqlitePool, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use std::str::FromStr;
use sqlx::migrate;

#[tokio::main]
async fn main() {
    // 1. 永続化: SQLx + SQLite 接続とマイグレーション
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://app.db".to_string());
    // DBファイルが存在しない場合は作成して接続
    let connect_opts = SqliteConnectOptions::from_str(&database_url)
        .unwrap()
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .connect_with(connect_opts)
        .await
        .unwrap();
    // `migrations/` のSQLをバイナリに埋め込んで適用
    migrate!().run(&pool).await.unwrap();

    // 2. 依存関係の構築 (Dependency Injection)
    let user_repo = Arc::new(SqlxUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(user_repo));

    // 3. ルーターの作成
    let app = create_router(user_service);

    // 4. サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
