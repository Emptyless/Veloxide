use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{OAuth2State, Oauth2StateRepository};
use crate::prelude::Result;

#[derive(Clone, Debug)]
pub struct PostgresOauth2StateRepository {
    pool: PgPool,
}

impl PostgresOauth2StateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Oauth2StateRepository for PostgresOauth2StateRepository {
    #[tracing::instrument(skip(self), ret, err)]
    async fn get_state(&mut self, csrf_state: &str) -> Result<OAuth2State> {
        let query = sqlx::query_as!(
            OAuth2State,
            "SELECT * FROM oauth2_states WHERE csrf_state = $1",
            csrf_state
        );

        let state = query.fetch_one(&self.pool).await?;

        Ok(state)
    }

    #[tracing::instrument(skip(self), err, state, fields(state_id = %state.id))]
    async fn create_state(&mut self, state: OAuth2State) -> Result<()> {
        let query = sqlx::query!(
            "INSERT INTO oauth2_states (id, csrf_state, code_verifier, return_url, created_at) VALUES ($1, $2, $3, $4, $5)",
            state.id,
            state.csrf_state,
            state.code_verifier,
            state.return_url,
            chrono::Utc::now(),
        );

        query.execute(&self.pool).await?;

        Ok(())
    }
}

pub type OAuth2StateRepositoryImpl = PostgresOauth2StateRepository;
