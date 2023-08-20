use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

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

    #[error("invalid return url")]
    InvalidReturnUrl,

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
        let status_code = match self {
            AuthError::WithoutCode => StatusCode::BAD_REQUEST,
            AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
            AuthError::AuthTokenNotFound => StatusCode::UNAUTHORIZED,
            AuthError::TokenValidationFailed => StatusCode::UNAUTHORIZED,
            AuthError::InvalidReturnUrl => StatusCode::BAD_REQUEST,
            AuthError::TokenSignatureNotMatching => StatusCode::UNAUTHORIZED,
            AuthError::CsrfStateMismatch => StatusCode::FORBIDDEN,
            AuthError::InvalidTokenFormat => StatusCode::BAD_REQUEST,
            AuthError::EmailAddressNotVerified => StatusCode::FORBIDDEN,
            AuthError::StateStoreFailed => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::FailedToGetUser => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::CryptograhyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = self.to_string();
        (status_code, body).into_response()
    }
}
