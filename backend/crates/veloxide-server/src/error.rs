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
    #[error(transparent)]
    SetGlobalDefault(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("failed to decode b64 encoded string")]
    Base64DecodeError,

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

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

    #[error(transparent)]
    Tonic(#[from] tonic::transport::Error),

    #[error("invalid token format")]
    InvalidTokenFormat,

    #[error("failed to decode token identifier")]
    FailedToDecodeTokenIdentifier,

    #[error("failed to decode token expiration")]
    FailedToDecodeTokenExpiration,

    #[error("failed to parse token expiration")]
    FailedToParseTokenExpiration,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error() {
        let err = Error::FailedToDecodeTokenIdentifier;
        assert_eq!(err.to_string(), "failed to decode token identifier");
    }
}
