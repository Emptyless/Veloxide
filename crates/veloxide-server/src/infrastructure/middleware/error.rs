use axum::response::{IntoResponse, Response};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("without state")]
    WithoutState,

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

    #[error("email not found")]
    EmailNotFound,

    #[error("csrf state mismatch")]
    CsrfStateMismatch,

    #[error("failed to exchange code for token")]
    CodeExchangeFailed,

    #[error("failed to get user info")]
    UserInfoFailed,

    #[error("failed to parse user info")]
    UserInfoParseFailed,

    #[error("invalid email address")]
    InvalidEmailAddress,

    #[error("invalid token format")]
    InvalidTokenFormat,

    #[error("email address not verified")]
    EmailAddressNotVerified,

    #[error("failed to parse verified email")]
    VerifiedEmailParseFailed,

    #[error("failed to store state")]
    StateStoreFailed,

    #[error("failed to modify session store")]
    FailedToModifySessionStore,

    #[error("failed to get session")]
    FailedToGetSession,

    #[error("failed to get user")]
    FailedToGetUser,

    #[error("failed to parse opa response")]
    FailedToParseOpaResponse,

    #[error("failed to decode identifier")]
    FailedToDecodeIdentifier,

    #[error(transparent)]
    CryptograhyError(#[from] crate::infrastructure::cryptography::error::CryptograhyError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = self.to_string();
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
