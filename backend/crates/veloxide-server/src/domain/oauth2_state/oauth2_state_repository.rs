use super::OAuth2State;
use crate::prelude::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Oauth2StateRepository {
    async fn get_state(&mut self, csrf_state: &str) -> Result<OAuth2State>;

    async fn create_state(&mut self, state: OAuth2State) -> Result<()>;
}
