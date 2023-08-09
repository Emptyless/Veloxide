//! This is the main (and only for now) application Error type.
//! It's using 'thiserror' as it reduces boilerplate error code while providing rich error typing.
//!
//! Notes:
//!     - The strategy is to start with one Error type for the whole application and then seggregate as needed.
//!     - Since everything is typed from the start, renaming and refactoring become relatively trivial.
//!     - By best practices, `anyhow` is not used in application code, but can be used in unit or integration test (will be in dev_dependencies when used)
//!

use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String), // The goal is to get rid of this generic error eventually

    #[error(transparent)]
    SetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("no email found in claims")]
    NoEmail,

    #[error("no name found in claims")]
    NoName,

    #[error("no id token found in claims")]
    NoIdToken,

    #[error("failed to decode b64 encoded string")]
    Base64DecodeError,

    #[error("code exchange failed")]
    CodeExchangeFailed,

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error("invalid state")]
    InvalidState,

    #[error(transparent)]
    ClaimsVerificationError(#[from] openidconnect::ClaimsVerificationError),

    #[error(transparent)]
    TraceError(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    ColorEyreError(#[from] color_eyre::Report),

    #[error(transparent)]
    CryptograhyError(#[from] crate::infrastructure::cryptography::error::CryptograhyError),

    #[error(transparent)]
    AuthError(#[from] crate::infrastructure::middleware::error::AuthError),

    #[error("invalid token format")]
    InvalidTokenFormat,

    #[error("failed to decode token identifier")]
    FailedToDecodeTokenIdentifier,

    #[error("failed to decode token expiration")]
    FailedToDecodeTokenExpiration,

    #[error("failed to parse token expiration")]
    FailedToParseTokenExpiration,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = match self {
            Error::Generic(msg) => msg,
            _ => self.to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error() {
        let err = Error::Generic("test".to_string());
        assert_eq!(err.to_string(), "Generic: test");
    }
}
