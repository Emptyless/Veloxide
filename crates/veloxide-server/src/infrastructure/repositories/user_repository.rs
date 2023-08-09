use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{User, UserRepository},
    prelude::Result,
};

#[derive(Clone, Debug)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_user_by_email(&mut self, email: &str) -> Result<User> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email);
        let state = query.fetch_one(&self.pool).await?;
        Ok(state)
    }

    async fn get_user_by_id(&mut self, id: &Uuid) -> Result<User> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id);
        let state = query.fetch_one(&self.pool).await?;
        Ok(state)
    }

    async fn create_user(&mut self, user: &User) -> Result<()> {
        let query = sqlx::query!(
            "INSERT INTO users (id, email, created_at, updated_at, token_salt) VALUES ($1, $2, $3, $4, $5)",
            Uuid::new_v4(),
            user.email,
            user.created_at,
            user.updated_at,
            Uuid::new_v4()
        );

        query.execute(&self.pool).await?;
        Ok(())
    }
}

pub type UserRepositoryImpl = crate::infrastructure::repositories::PostgresUserRepository;
