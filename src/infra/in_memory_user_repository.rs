use crate::domain::user::{CreateUser, User, UserRepository};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct InMemoryUserRepository {
    db: Arc<RwLock<HashMap<u64, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create_user(&self, user_data: CreateUser) -> User {
        let mut db = self.db.write().unwrap();
        let id = db.keys().max().unwrap_or(&0) + 1;
        let new_user = User {
            id,
            name: user_data.name,
            email: user_data.email,
        };
        db.insert(id, new_user.clone());
        new_user
    }

    async fn find_all(&self) -> Vec<User> {
        let db = self.db.read().unwrap();
        db.values().cloned().collect()
    }

    async fn find_by_id(&self, id: u64) -> Option<User> {
        let db = self.db.read().unwrap();
        db.get(&id).cloned()
    }

    async fn update_user(&self, id: u64, user_data: CreateUser) -> Option<User> {
        let mut db = self.db.write().unwrap();
        if let Some(user) = db.get_mut(&id) {
            user.name = user_data.name;
            user.email = user_data.email;
            Some(user.clone())
        } else {
            None
        }
    }

    async fn delete_user(&self, id: u64) -> bool {
        let mut db = self.db.write().unwrap();
        db.remove(&id).is_some()
    }
} 