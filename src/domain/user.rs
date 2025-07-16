use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
} 