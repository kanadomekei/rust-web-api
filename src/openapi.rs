use crate::{
    domain::user::{CreateUser, User},
    handlers,
};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello::handler,
        handlers::hello::hello_name,
        handlers::user::create_user,
        handlers::user::get_users,
        handlers::user::get_user,
        handlers::user::update_user,
        handlers::user::delete_user,
    ),
    components(
        schemas(User, CreateUser)
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