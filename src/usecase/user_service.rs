use crate::domain::user::{CreateUser, User, UserRepository};
use std::sync::Arc;

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn create_user(&self, user: CreateUser) -> User {
        self.user_repo.create_user(user).await
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        self.user_repo.find_all().await
    }

    pub async fn get_user_by_id(&self, id: u64) -> Option<User> {
        self.user_repo.find_by_id(id).await
    }

    pub async fn update_user(&self, id: u64, user: CreateUser) -> Option<User> {
        self.user_repo.update_user(id, user).await
    }

    pub async fn delete_user(&self, id: u64) -> bool {
        self.user_repo.delete_user(id).await
    }
} 