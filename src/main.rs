use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handler,
        hello_name,
        json_handler
    ),
    components(
        schemas(User)
    ),
    tags(
        (name = "rust-web-api", description = "A simple Rust web API")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("api_key"))),
        )
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(handler))
        .route("/hello/:name", get(hello_name))
        .route("/json", get(json_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Returns a static greeting message", body = String)
    )
)]
async fn handler() -> &'static str {
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
async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello, {}!!", name)
}

#[utoipa::path(
    get,
    path = "/json",
    responses(
        (status = 200, description = "Returns a user object as JSON", body = User)
    )
)]
async fn json_handler() -> Json<User> {
    let user = User {
        id: 1,
        name: "axum user".to_string(),
        email: "axum@example.com".to_string(),
    };
    Json(user)
}
