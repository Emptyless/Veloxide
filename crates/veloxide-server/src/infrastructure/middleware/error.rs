use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("without code")]
    WithoutCode,

    #[error("auth token expired")]
    TokenExpired,

    #[error("auth token not found")]
    AuthTokenNotFound,

    #[error("token validation failed")]
    TokenValidationFailed,

    #[error("token signature not matching")]
    TokenSignatureNotMatching,

    #[error("csrf state mismatch")]
    CsrfStateMismatch,

    #[error("invalid token format")]
    InvalidTokenFormat,

    #[error("email address not verified")]
    EmailAddressNotVerified,

    #[error("failed to store state")]
    StateStoreFailed,

    #[error("failed to get user")]
    FailedToGetUser,

    #[error(transparent)]
    CryptograhyError(#[from] crate::infrastructure::cryptography::error::CryptograhyError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = self.to_string();
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
