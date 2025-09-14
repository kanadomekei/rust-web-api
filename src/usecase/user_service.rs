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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::in_memory_user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn service_crud_flow() {
        let repo = Arc::new(InMemoryUserRepository::new());
        let svc = UserService::new(repo);

        // create
        let u = svc
            .create_user(CreateUser { name: "A".into(), email: "a@example.com".into() })
            .await;
        assert!(u.id > 0);

        // get by id
        let fetched = svc.get_user_by_id(u.id).await;
        assert!(fetched.is_some());

        // update
        let updated = svc
            .update_user(u.id, CreateUser { name: "B".into(), email: "b@example.com".into() })
            .await
            .unwrap();
        assert_eq!(updated.name, "B");

        // list
        let all = svc.get_all_users().await;
        assert_eq!(all.len(), 1);

        // delete
        let ok = svc.delete_user(u.id).await;
        assert!(ok);
        assert!(svc.get_user_by_id(u.id).await.is_none());
    }
}