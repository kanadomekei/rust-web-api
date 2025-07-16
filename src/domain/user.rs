use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Clone, Debug, ToSchema)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
} 