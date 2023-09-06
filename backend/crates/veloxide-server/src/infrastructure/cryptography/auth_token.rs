use crate::infrastructure::cryptography::*;
use std::str::FromStr;

use axum::response::{IntoResponse, Response};
use chrono::Utc;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with_macros::serde_as;

use super::encoding::*;
use crate::infrastructure::cryptography::error::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct AuthToken {
    pub identifier: String, // Username or email for example
    pub expiration: chrono::DateTime<Utc>,
    pub signature: String, // b64 encoded signature, usually a UUID
}

impl AuthToken {
    #[tracing::instrument(level = "info", skip(salt, key), err)]
    pub fn new(
        identifier: &str,
        expiration: chrono::DateTime<Utc>,
        salt: &str,
        key: &str,
    ) -> Result<Self, CryptograhyError> {
        let signature = token_sign_into_base64url(
            identifier,
            expiration.to_rfc3339().as_str(),
            salt,
            key.as_bytes(),
        )?;
        Ok(Self {
            identifier: identifier.to_owned(),
            expiration,
            signature,
        })
    }
}

impl FromStr for AuthToken {
    type Err = TokenValidationError;
    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Self::Err::InvalidTokenFormat);
        }
        let (identifier_b64u, expiration_b64u, signature_b64u) = (splits[0], splits[1], splits[2]);
        let decoded_expiration_string = base64url_decode(expiration_b64u)
            .map_err(|_| Self::Err::FailedToDecodeTokenExpiration)?;
        let expiration: chrono::DateTime<Utc> =
            chrono::DateTime::parse_from_rfc3339(&decoded_expiration_string)
                .map_err(|_| Self::Err::FailedToParseTokenExpiration)?
                .with_timezone(&Utc);

        Ok(Self {
            identifier: base64url_decode(identifier_b64u)
                .map_err(|_| Self::Err::FailedToDecodeTokenIdentifier)?,
            expiration,
            signature: signature_b64u.to_string(),
        })
    }
}

/// Converts the DateTime<Utc> to an RFC3339 string, then base64url encode it and the identifier.
impl Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}.{}.{}",
            base64urlsafe_encode(&self.identifier),
            base64urlsafe_encode(&self.expiration.to_rfc3339()),
            self.signature
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TokenValidationError {
    #[error("invalid token format")]
    InvalidTokenFormat,

    #[error("failed to decode token identifier")]
    FailedToDecodeTokenIdentifier,

    #[error("failed to decode token expiration")]
    FailedToDecodeTokenExpiration,

    #[error("failed to parse token expiration")]
    FailedToParseTokenExpiration,
}

impl IntoResponse for TokenValidationError {
    fn into_response(self) -> Response {
        let status = match self {
            TokenValidationError::InvalidTokenFormat
            | TokenValidationError::FailedToDecodeTokenIdentifier
            | TokenValidationError::FailedToDecodeTokenExpiration
            | TokenValidationError::FailedToParseTokenExpiration => StatusCode::BAD_REQUEST,
        };
        (status, self.to_string()).into_response()
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct AuthTokenConfiguration {
    pub token_key: String,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub token_duration_minutes: chrono::Duration,
    pub cookie_domain: String,
    pub cookie_name: String,
    pub cookie_secure: bool,
}

impl AuthTokenConfiguration {
    pub fn from_env() -> AuthTokenConfiguration {
        let token_key = dotenvy::var("TOKEN_KEY").expect("TOKEN_KEY must be set");
        let token_duration_minutes: i64 = dotenvy::var("TOKEN_DURATION_MINUTES")
            .unwrap_or("480".to_string())
            .parse()
            .expect("TOKEN_DURATION_MINUTES must be a valid integer");

        let token_duration = chrono::Duration::minutes(token_duration_minutes);
        let cookie_domain =
            dotenvy::var("AUTH_TOKEN_COOKIE_DOMAIN").expect("AUTH_TOKEN_COOKIE_DOMAIN must be set");
        let cookie_name =
            dotenvy::var("AUTH_TOKEN_COOKIE_NAME").expect("AUTH_TOKEN_COOKIE_NAME must be set");
        let cookie_secure = dotenvy::var("HTTPS")
            .unwrap_or("true".to_string())
            .parse::<bool>()
            .expect("expected to be able to parse HTTPS env var");
        Self {
            token_key,
            token_duration_minutes: token_duration,
            cookie_domain,
            cookie_name,
            cookie_secure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_failed_to_decode_token_identier_converts_to_correct_string() {
        let err = TokenValidationError::FailedToDecodeTokenIdentifier;
        assert_eq!(err.to_string(), "failed to decode token identifier");
    }
}
