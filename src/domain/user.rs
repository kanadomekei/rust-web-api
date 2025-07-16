use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: CreateUser) -> User;
    async fn find_all(&self) -> Vec<User>;
    async fn find_by_id(&self, id: u64) -> Option<User>;
    async fn update_user(&self, id: u64, user: CreateUser) -> Option<User>;
    async fn delete_user(&self, id: u64) -> bool;
} 