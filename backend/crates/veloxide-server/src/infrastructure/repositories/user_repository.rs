use async_trait::async_trait;
use sqlx::PgPool;
use tracing::instrument;
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
    #[instrument(skip(self), err)]
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email);
        let state = query.fetch_one(&self.pool).await?;
        Ok(state)
    }

    #[instrument(skip(self), err)]
    async fn get_user_by_id(&self, id: &Uuid) -> Result<User> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id);
        let state = query.fetch_one(&self.pool).await?;
        Ok(state)
    }

    #[instrument(skip(self), err)]
    async fn create_user(&self, user: &User) -> Result<()> {
        let query = sqlx::query!(
            "INSERT INTO users (id, email, verified_email, created_at, updated_at, token_salt, given_name, family_name, name, picture, locale) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            Uuid::new_v4(),
            user.email,
            user.verified_email,
            user.created_at,
            user.updated_at,
            Uuid::new_v4(),
            user.given_name,
            user.family_name,
            user.name,
            user.picture,
            user.locale,
        );

        query.execute(&self.pool).await?;
        Ok(())
    }
}

pub type UserRepositoryImpl = crate::infrastructure::repositories::PostgresUserRepository;
