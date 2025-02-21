use crate::infrastructure::cryptography::*;
use std::str::FromStr;

use axum::response::{IntoResponse, Response};
use chrono::Utc;
use hyper::StatusCode;

use super::encoding::*;
use crate::infrastructure::cryptography::error::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct AuthToken {
    pub identifier: String, // Username or email for example
    pub expiration: chrono::DateTime<Utc>,
    pub signature: String, // b64 encoded signature, usually a UUID
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

//TODO: Convert this func into a constructor with the new method
/// Create a new web token
/// the identifier is usually the username or email address
pub fn new_web_token(
    identifier: &str,
    expiration: chrono::DateTime<Utc>,
    salt: &str,
    key: &str,
) -> Result<AuthToken, CryptograhyError> {
    let signature = token_sign_into_base64url(
        identifier,
        expiration.to_rfc3339().as_str(),
        salt,
        key.as_bytes(),
    )?;
    Ok(AuthToken {
        identifier: identifier.to_owned(),
        expiration,
        signature,
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_failed_to_decode_token_identier_converts_to_correct_string() {
        let err = TokenValidationError::FailedToDecodeTokenIdentifier;
        assert_eq!(err.to_string(), "failed to decode token identifier");
    }
}
