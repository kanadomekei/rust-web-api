use crate::{domain::user::User, handlers};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello::handler,
        handlers::hello::hello_name,
        handlers::user::json_handler,
    ),
    components(
        schemas(User)
    ),
    tags(
        (name = "rust-web-api", description = "A simple Rust web API")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

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