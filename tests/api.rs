use std::sync::Arc;

use axum::{http::{Request, StatusCode}, Router};
use axum::body::Body;
use tower::ServiceExt; // for `oneshot`

use rust_web_api::{
    routes::create_router,
    usecase::user_service::UserService,
    infra::in_memory_user_repository::InMemoryUserRepository,
    domain::user::User,
};

fn app() -> Router {
    let repo = Arc::new(InMemoryUserRepository::new());
    let svc = Arc::new(UserService::new(repo));
    create_router(svc)
}

fn json_body<T: serde::Serialize>(val: &T) -> Body {
    let s = serde_json::to_string(val).unwrap();
    Body::from(s)
}

const BODY_LIMIT: usize = 1024 * 1024; // 1 MiB

#[tokio::test]
async fn root_responds_ok() {
    let app = app();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_and_list_users() {
    let app = app();

    // create
    let payload = serde_json::json!({"name":"Alice","email":"alice@example.com"});
    let req = Request::builder()
        .method("POST")
        .uri("/users")
        .header("content-type", "application/json")
        .body(json_body(&payload))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let bytes = axum::body::to_bytes(resp.into_body(), BODY_LIMIT).await.unwrap();
    let created: User = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(created.name, "Alice");
    assert_eq!(created.email, "alice@example.com");
    assert!(created.id > 0);

    // list
    let req = Request::builder().uri("/users").body(Body::empty()).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), BODY_LIMIT).await.unwrap();
    let users: Vec<User> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Alice");
}

#[tokio::test]
async fn get_update_delete_user_flow() {
    let app = app();

    // create Bob
    let payload = serde_json::json!({"name":"Bob","email":"bob@example.com"});
    let req = Request::builder()
        .method("POST")
        .uri("/users")
        .header("content-type", "application/json")
        .body(json_body(&payload))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let bytes = axum::body::to_bytes(resp.into_body(), BODY_LIMIT).await.unwrap();
    let created: User = serde_json::from_slice(&bytes).unwrap();

    // get
    let req = Request::builder()
        .uri(format!("/users/{}", created.id))
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // update
    let payload = serde_json::json!({"name":"Bobby","email":"bobby@example.com"});
    let req = Request::builder()
        .method("PUT")
        .uri(format!("/users/{}", created.id))
        .header("content-type", "application/json")
        .body(json_body(&payload))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), BODY_LIMIT).await.unwrap();
    let updated: User = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(updated.name, "Bobby");
    assert_eq!(updated.email, "bobby@example.com");

    // delete
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/users/{}", created.id))
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // get after delete -> 404
    let req = Request::builder()
        .uri(format!("/users/{}", created.id))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}


