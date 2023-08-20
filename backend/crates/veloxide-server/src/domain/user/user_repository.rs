use super::User;
use crate::prelude::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_email(&self, email: &str) -> Result<User>;

    async fn create_user(&self, user: &User) -> Result<()>;

    async fn get_user_by_id(&self, id: &Uuid) -> Result<User>;
}
