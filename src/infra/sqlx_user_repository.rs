use crate::domain::user::{CreateUser, User, UserRepository};
use async_trait::async_trait;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

pub struct SqlxUserRepository {
    pool: SqlitePool,
}

impl SqlxUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn map_row_to_user(row: &SqliteRow) -> User {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        User {
            id: id as u64,
            name,
            email,
        }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn create_user(&self, user_data: CreateUser) -> User {
        // Insert user and fetch last inserted id
        let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
            .bind(&user_data.name)
            .bind(&user_data.email)
            .execute(&self.pool)
            .await
            .expect("failed to insert user");

        let id = result.last_insert_rowid() as u64;

        // Return the created user
        User {
            id,
            name: user_data.name,
            email: user_data.email,
        }
    }

    async fn find_all(&self) -> Vec<User> {
        let rows = sqlx::query("SELECT id, name, email FROM users ORDER BY id")
            .fetch_all(&self.pool)
            .await
            .expect("failed to fetch users");

        rows.iter().map(SqlxUserRepository::map_row_to_user).collect()
    }

    async fn find_by_id(&self, id: u64) -> Option<User> {
        let row = sqlx::query("SELECT id, name, email FROM users WHERE id = ?")
            .bind(id as i64)
            .fetch_optional(&self.pool)
            .await
            .expect("failed to fetch user by id");

        row.map(|r| SqlxUserRepository::map_row_to_user(&r))
    }

    async fn update_user(&self, id: u64, user_data: CreateUser) -> Option<User> {
        let result = sqlx::query("UPDATE users SET name = ?, email = ? WHERE id = ?")
            .bind(&user_data.name)
            .bind(&user_data.email)
            .bind(id as i64)
            .execute(&self.pool)
            .await
            .expect("failed to update user");

        if result.rows_affected() == 0 {
            None
        } else {
            // Return the updated record
            self.find_by_id(id).await
        }
    }

    async fn delete_user(&self, id: u64) -> bool {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id as i64)
            .execute(&self.pool)
            .await
            .expect("failed to delete user");

        result.rows_affected() > 0
    }
}

