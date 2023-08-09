use super::User;
use crate::prelude::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository {
    async fn get_user_by_email(&mut self, email: &str) -> Result<User>;

    async fn create_user(&mut self, user: &User) -> Result<()>;

    async fn get_user_by_id(&mut self, id: &Uuid) -> Result<User>;
}
