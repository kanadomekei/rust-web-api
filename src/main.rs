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
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use url::Url;
use tokio::time::{sleep, Duration};
use sqlx::migrate;

#[tokio::main]
async fn main() {
    // 1. 永続化: SQLx + TiDB(MySQL) 接続とマイグレーション
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://root:@127.0.0.1:4000/test".to_string());

    // Ensure target database exists (TiDB/MySQL)
    let parsed = Url::parse(&database_url).expect("invalid DATABASE_URL");
    let db_name = parsed.path().trim_start_matches('/').to_string();
    let mut server_url = parsed.clone();
    // Use default 'mysql' database for server-level connection (valid DSN)
    server_url.set_path("/mysql");

    // connect to server (no db) with retry and create database if missing
    let server_pool: MySqlPool = {
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            match MySqlPoolOptions::new()
                .max_connections(3)
                .connect(server_url.as_str())
                .await
            {
                Ok(p) => break p,
                Err(e) => {
                    if attempt >= 20 {
                        panic!("failed to connect to TiDB server: {}", e);
                    }
                    eprintln!("waiting for TiDB server... attempt {}: {}", attempt, e);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    };
    let create_db_sql = format!(
        "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci",
        db_name
    );
    sqlx::query(&create_db_sql)
        .execute(&server_pool)
        .await
        .unwrap();
    drop(server_pool);

    // now connect to target database with retry
    let pool: MySqlPool = {
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            match MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
            {
                Ok(p) => break p,
                Err(e) => {
                    if attempt >= 20 {
                        panic!("failed to connect to database '{}': {}", db_name, e);
                    }
                    eprintln!("waiting for database connection... attempt {}: {}", attempt, e);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        }
    };
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
