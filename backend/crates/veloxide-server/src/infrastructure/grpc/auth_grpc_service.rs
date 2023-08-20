use auth::authentication_server::Authentication;
use auth::{GetCurrentUserRequest, GetUserResponse};
use tonic::{Request, Response, Status};
use tracing::instrument;

use crate::application::{AuthServiceError, AuthenticationApplicationService};
use crate::infrastructure::cryptography::AuthToken;

pub mod auth {
    tonic::include_proto!("auth");
}
pub use auth::*;

pub struct GRpcAuthService {
    app_service: Box<dyn AuthenticationApplicationService>,
}

impl GRpcAuthService {
    pub fn new(app_service: Box<dyn AuthenticationApplicationService>) -> Self {
        GRpcAuthService { app_service }
    }
}

#[tonic::async_trait]
impl Authentication for GRpcAuthService {
    // request is skipped as it's message field is expected to include the user's auth token
    #[instrument(skip(self, request), ret, err, fields(metadata=?request.metadata()))]
    async fn get_current_user(
        &self,
        request: Request<GetCurrentUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let token = request.into_inner().token;
        let auth_token: AuthToken = token
            .parse()
            .map_err(|_| Status::unauthenticated("Invalid token"))?;

        let user_view = self
            .app_service
            .get_current_user_by_email(&auth_token.identifier)
            .await?;
        let reply = GetUserResponse {
            user: Some(user_view),
        };
        Ok(Response::new(reply))
    }
}

const GENERIC_ERROR: &str = "An internal error occurred";
impl From<AuthServiceError> for tonic::Status {
    fn from(error: AuthServiceError) -> Self {
        match error {
            AuthServiceError::UserNotFound(err) => Status::not_found(err.to_string()),
            _ => Status::internal(GENERIC_ERROR),
        }
    }
}
