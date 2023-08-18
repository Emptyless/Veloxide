//! This is the main (and only for now) application Error type.
//! It's using 'thiserror' as it reduces boilerplate error code while providing rich error typing.
//!
//! Notes:
//!     - The strategy is to start with one Error type for the whole application and then seggregate as needed.
//!     - Since everything is typed from the start, renaming and refactoring become relatively trivial.

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

    #[error("failed to decode b64 encoded string")]
    Base64DecodeError,

    #[error(transparent)]
    TraceError(#[from] opentelemetry::trace::TraceError),

    #[error(transparent)]
    ColorEyreError(#[from] color_eyre::Report),

    #[error(transparent)]
    CryptographyError(#[from] crate::infrastructure::cryptography::error::CryptograhyError),

    #[error(transparent)]
    AuthError(#[from] crate::infrastructure::middleware::error::AuthError),

    #[error(transparent)]
    Tonic(#[from] tonic::transport::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = self.to_string();
        let status = match self {
            Error::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::AuthError(_) => StatusCode::UNAUTHORIZED,
            Error::CryptographyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_error_converts_to_string_correctly() {
        let err = Error::Base64DecodeError;
        assert_eq!(err.to_string(), "failed to decode b64 encoded string");
    }
}
