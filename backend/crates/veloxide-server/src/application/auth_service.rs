use crate::domain::{User, UserRepository};
use crate::infrastructure::grpc::auth_grpc_service::UserView;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum AuthServiceError {
    #[error("user not found: {0}")]
    UserNotFound(Uuid),

    #[error("user not found by email: {0}")]
    UserNotFoundWithEmail(String),

    #[error(transparent)]
    Other(#[from] crate::prelude::Error),
}

#[async_trait::async_trait]
pub trait AuthenticationApplicationService: Send + Sync {
    async fn get_current_user_by_id(&self, user_id: Uuid) -> Result<UserView, AuthServiceError>;
    async fn get_current_user_by_email(&self, email: &str) -> Result<UserView, AuthServiceError>;
}

pub struct AuthServiceImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl AuthenticationApplicationService for AuthServiceImpl {
    // Do not add ret to the instrument macro as that will leak the user's token salt
    #[instrument(skip(self), err)]
    async fn get_current_user_by_id(&self, user_id: Uuid) -> Result<UserView, AuthServiceError> {
        let user: User = self
            .user_repository
            .get_user_by_id(&user_id)
            .await
            .map_err(|_| AuthServiceError::UserNotFound(user_id))?;

        Ok(user.into())
    }

    // Do not add ret to the instrument macro as that will leak the user's token salt
    #[instrument(skip(self), err)]
    async fn get_current_user_by_email(&self, email: &str) -> Result<UserView, AuthServiceError> {
        let user: User = self
            .user_repository
            .get_user_by_email(email)
            .await
            .map_err(|_| AuthServiceError::UserNotFoundWithEmail(email.to_string()))?;

        Ok(user.into())
    }
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        UserView {
            id: user.id.to_string(),
            name: String::from("N/A"),
            email: user.email,
            email_verified: true,
            image: String::from("N/A"),
        }
    }
}
